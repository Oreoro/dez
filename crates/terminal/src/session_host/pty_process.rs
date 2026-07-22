use std::{
    collections::{HashMap, VecDeque},
    io::{self, ErrorKind, Read as _, Write as _},
    num::NonZeroUsize,
    path::PathBuf,
    sync::{Arc, mpsc},
    thread,
};

use alacritty_terminal::{
    event::{OnResize as _, WindowSize},
    tty::{self, ChildEvent, EventedPty as _, EventedReadWrite as _},
};
use polling::{Event, Events, PollMode, Poller};

const PTY_READ_BUFFER_BYTES: usize = 64 * 1024;
// `EventedPty` assigns these fixed polling keys during registration, but the
// current Alacritty revision no longer exports their names to consumers.
const PTY_READ_WRITE_TOKEN: usize = 0;
const PTY_CHILD_EVENT_TOKEN: usize = 1;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TerminalHostPtyEvent {
    Output(Vec<u8>),
    Exited { exit_code: Option<i32> },
    Failed(String),
}

enum TerminalHostPtyCommand {
    Input(Vec<u8>),
    Resize { columns: u16, rows: u16 },
    Terminate,
}

/// Process handle owned by a terminal host rather than a GPUI entity.
///
/// The underlying PTY and child live on their I/O thread. Dropping the final
/// handle closes the command channel, which deliberately terminates the PTY;
/// a long-lived helper must retain one handle per hosted session.
pub struct TerminalHostPtyHandle {
    command_tx: mpsc::Sender<TerminalHostPtyCommand>,
    poller: Arc<Poller>,
    process_id: Option<u32>,
}

impl Drop for TerminalHostPtyHandle {
    fn drop(&mut self) {
        if self
            .command_tx
            .send(TerminalHostPtyCommand::Terminate)
            .is_ok()
            && let Err(error) = self.poller.notify()
        {
            log::debug!("failed to wake terminal host PTY for shutdown: {error}");
        }
    }
}

impl TerminalHostPtyHandle {
    pub fn spawn(
        working_directory: Option<PathBuf>,
        shell: Option<super::TerminalSessionShell>,
        environment: HashMap<String, String>,
        columns: u16,
        rows: u16,
        event_handler: impl Fn(TerminalHostPtyEvent) + Send + 'static,
    ) -> io::Result<Self> {
        let options = tty::Options {
            shell: shell.map(|shell| tty::Shell::new(shell.program, shell.args)),
            working_directory,
            drain_on_exit: true,
            env: environment,
            #[cfg(not(windows))]
            child_signal_mask: Some(tty::SignalMask::current()?),
            #[cfg(windows)]
            escape_args: false,
        };
        let window_size = WindowSize {
            num_lines: rows.max(1),
            num_cols: columns.max(1),
            cell_width: 8,
            cell_height: 16,
        };
        let pty = tty::new(&options, window_size, 0)?;
        let process_id = pty_process_id(&pty);
        let (command_tx, command_rx) = mpsc::channel();
        let poller = Arc::new(Poller::new()?);
        thread::Builder::new()
            .name("dez terminal host PTY".to_owned())
            .spawn({
                let poller = poller.clone();
                move || run_pty(pty, command_rx, poller, event_handler)
            })?;
        Ok(Self {
            command_tx,
            poller,
            process_id,
        })
    }

    pub fn process_id(&self) -> Option<u32> {
        self.process_id
    }

    pub fn input(&self, bytes: Vec<u8>) -> io::Result<()> {
        self.send_command(TerminalHostPtyCommand::Input(bytes))
    }

    pub fn resize(&self, columns: u16, rows: u16) -> io::Result<()> {
        self.send_command(TerminalHostPtyCommand::Resize { columns, rows })
    }

    pub fn terminate(&self) -> io::Result<()> {
        self.send_command(TerminalHostPtyCommand::Terminate)
    }

    fn send_command(&self, command: TerminalHostPtyCommand) -> io::Result<()> {
        self.command_tx
            .send(command)
            .map_err(|_| io::Error::new(ErrorKind::BrokenPipe, "terminal host PTY closed"))?;
        self.poller.notify()
    }
}

fn run_pty(
    mut pty: tty::Pty,
    command_rx: mpsc::Receiver<TerminalHostPtyCommand>,
    poller: Arc<Poller>,
    event_handler: impl Fn(TerminalHostPtyEvent),
) {
    let mut pending_input = VecDeque::<(Vec<u8>, usize)>::new();
    let mut read_buffer = vec![0; PTY_READ_BUFFER_BYTES];
    let mut interest = Event::readable(PTY_READ_WRITE_TOKEN);
    if let Err(error) = unsafe { pty.register(&poller, interest, PollMode::Level) } {
        event_handler(TerminalHostPtyEvent::Failed(error.to_string()));
        return;
    }
    let Some(event_capacity) = NonZeroUsize::new(128) else {
        event_handler(TerminalHostPtyEvent::Failed(
            "terminal host event capacity must be non-zero".to_owned(),
        ));
        if let Err(error) = pty.deregister(&poller) {
            event_handler(TerminalHostPtyEvent::Failed(error.to_string()));
        }
        return;
    };
    let mut events = Events::with_capacity(event_capacity);

    'host: loop {
        events.clear();
        if let Err(error) = poller.wait(&mut events, None) {
            if error.kind() == ErrorKind::Interrupted {
                continue;
            }
            event_handler(TerminalHostPtyEvent::Failed(error.to_string()));
            break;
        }

        loop {
            match command_rx.try_recv() {
                Ok(TerminalHostPtyCommand::Input(bytes)) => {
                    if !bytes.is_empty() {
                        pending_input.push_back((bytes, 0));
                    }
                }
                Ok(TerminalHostPtyCommand::Resize { columns, rows }) => {
                    pty.on_resize(WindowSize {
                        num_lines: rows.max(1),
                        num_cols: columns.max(1),
                        cell_width: 8,
                        cell_height: 16,
                    });
                }
                Ok(TerminalHostPtyCommand::Terminate) => break 'host,
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => break 'host,
            }
        }

        for event in events.iter() {
            match event.key {
                PTY_CHILD_EVENT_TOKEN => {
                    if let Some(ChildEvent::Exited(exit_status)) = pty.next_child_event() {
                        if let Err(error) =
                            drain_pty_output(&mut pty, &mut read_buffer, &event_handler)
                            && !is_closed_pty_error(&error)
                        {
                            event_handler(TerminalHostPtyEvent::Failed(error.to_string()));
                        }
                        event_handler(TerminalHostPtyEvent::Exited {
                            exit_code: exit_status.and_then(|status| status.code()),
                        });
                        break 'host;
                    }
                }
                PTY_READ_WRITE_TOKEN => {
                    if event.is_interrupt() {
                        continue;
                    }
                    if event.readable
                        && let Err(error) =
                            drain_pty_output(&mut pty, &mut read_buffer, &event_handler)
                        && !is_closed_pty_error(&error)
                    {
                        event_handler(TerminalHostPtyEvent::Failed(error.to_string()));
                        break 'host;
                    }
                    if event.writable
                        && let Err(error) = write_pending_input(&mut pty, &mut pending_input)
                    {
                        event_handler(TerminalHostPtyEvent::Failed(error.to_string()));
                        break 'host;
                    }
                }
                _ => {}
            }
        }

        let needs_write = !pending_input.is_empty();
        if needs_write != interest.writable {
            interest.writable = needs_write;
            if let Err(error) = pty.reregister(&poller, interest, PollMode::Level) {
                event_handler(TerminalHostPtyEvent::Failed(error.to_string()));
                break;
            }
        }
    }

    if let Err(error) = pty.deregister(&poller) {
        event_handler(TerminalHostPtyEvent::Failed(error.to_string()));
    }
}

fn drain_pty_output(
    pty: &mut tty::Pty,
    read_buffer: &mut [u8],
    event_handler: &impl Fn(TerminalHostPtyEvent),
) -> io::Result<()> {
    loop {
        match pty.reader().read(read_buffer) {
            Ok(0) => return Ok(()),
            Ok(read) => event_handler(TerminalHostPtyEvent::Output(
                read_buffer.get(..read).unwrap_or_default().to_vec(),
            )),
            Err(error) if error.kind() == ErrorKind::Interrupted => continue,
            Err(error) if error.kind() == ErrorKind::WouldBlock => return Ok(()),
            Err(error) => return Err(error),
        }
    }
}

fn write_pending_input(
    pty: &mut tty::Pty,
    pending_input: &mut VecDeque<(Vec<u8>, usize)>,
) -> io::Result<()> {
    while let Some((bytes, offset)) = pending_input.front_mut() {
        match pty.writer().write(bytes.get(*offset..).unwrap_or_default()) {
            Ok(0) => return Ok(()),
            Ok(written) => {
                *offset += written;
                if *offset >= bytes.len() {
                    pending_input.pop_front();
                }
            }
            Err(error) if error.kind() == ErrorKind::Interrupted => continue,
            Err(error) if error.kind() == ErrorKind::WouldBlock => return Ok(()),
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

#[cfg(not(windows))]
fn pty_process_id(pty: &tty::Pty) -> Option<u32> {
    Some(pty.child().id())
}

#[cfg(windows)]
fn pty_process_id(pty: &tty::Pty) -> Option<u32> {
    pty.child_watcher().pid().map(u32::from)
}

#[cfg(not(windows))]
fn is_closed_pty_error(error: &io::Error) -> bool {
    // Unix PTYs commonly report EIO after the slave side exits.
    error.raw_os_error() == Some(libc::EIO)
}

#[cfg(windows)]
fn is_closed_pty_error(_error: &io::Error) -> bool {
    false
}
