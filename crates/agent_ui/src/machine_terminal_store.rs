use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    process::Command,
    time::Duration,
};

use anyhow::{Context as _, Result};
use gpui::{App, AppContext as _, Context, Entity, Global, Task};
use paths::APP_NAME;

use crate::terminal_thread_metadata_store::{
    TerminalAgentKind, detect_terminal_agent_command, detect_terminal_agent_kind,
};

const MACHINE_TERMINAL_REFRESH_INTERVAL: Duration = Duration::from_secs(5);

/// A terminal that belongs to another application on the same machine.
///
/// This is deliberately observation-only. Dez does not own the PTY, cannot
/// send input to it, and does not persist this record. The process table is
/// rescanned periodically and rows disappear when their TTY disappears.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservedMachineTerminal {
    pub id: String,
    pub tty: String,
    pub foreground_pid: u32,
    pub foreground_command: String,
    pub owning_application: Option<String>,
    pub owning_application_pid: Option<u32>,
    pub working_directory: Option<PathBuf>,
    pub detected_agent_kind: Option<TerminalAgentKind>,
}

impl ObservedMachineTerminal {
    pub fn display_title(&self) -> String {
        self.detected_agent_kind
            .map(TerminalAgentKind::display_name)
            .unwrap_or_else(|| display_process_name(&self.foreground_command))
            .to_owned()
    }

    pub fn owner_label(&self) -> &str {
        self.owning_application.as_deref().unwrap_or("External app")
    }

    pub fn copy_details(&self) -> String {
        let mut details = format!(
            "{}\nOwner: {}\nTTY: {}\nPID: {}",
            self.display_title(),
            self.owner_label(),
            self.tty,
            self.foreground_pid,
        );
        if let Some(working_directory) = &self.working_directory {
            details.push_str("\nWorking directory: ");
            details.push_str(&working_directory.to_string_lossy());
        }
        details.push_str(
            "\n\nObserved only. Dez does not own this PTY and cannot send input or restore it.",
        );
        details
    }

    pub fn reveal_owning_application(&self) -> Result<()> {
        let application = self
            .owning_application
            .as_deref()
            .context("the owning terminal application could not be identified")?;
        let status = Command::new("/usr/bin/open")
            .args(["-a", application])
            .status()
            .with_context(|| format!("failed to reveal {application}"))?;
        if !status.success() {
            anyhow::bail!("open exited with {status}");
        }
        Ok(())
    }
}

struct GlobalMachineTerminalStore(Entity<MachineTerminalStore>);
impl Global for GlobalMachineTerminalStore {}

pub struct MachineTerminalStore {
    terminals: Vec<ObservedMachineTerminal>,
    _refresh_task: Task<()>,
}

impl MachineTerminalStore {
    pub fn init_global(cx: &mut App) {
        if APP_NAME == "Zed" || cx.has_global::<GlobalMachineTerminalStore>() {
            return;
        }

        let store = cx.new(Self::new);
        cx.set_global(GlobalMachineTerminalStore(store));
    }

    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalMachineTerminalStore>()
            .map(|store| store.0.clone())
    }

    pub fn terminals(&self) -> &[ObservedMachineTerminal] {
        &self.terminals
    }

    fn new(cx: &mut Context<Self>) -> Self {
        let refresh_task = cx.spawn(async move |this, cx| {
            loop {
                let scan = cx
                    .background_executor()
                    .spawn(async { scan_machine_terminals() })
                    .await;

                if this
                    .update(cx, |store, cx| {
                        let terminals = scan.unwrap_or_else(|error| {
                            log::debug!("failed to observe machine terminals: {error:#}");
                            Vec::new()
                        });
                        if store.terminals != terminals {
                            store.terminals = terminals;
                            cx.notify();
                        }
                    })
                    .is_err()
                {
                    break;
                }

                cx.background_executor()
                    .timer(MACHINE_TERMINAL_REFRESH_INTERVAL)
                    .await;
            }
        });

        Self {
            terminals: Vec::new(),
            _refresh_task: refresh_task,
        }
    }
}

pub fn init(cx: &mut App) {
    MachineTerminalStore::init_global(cx);
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ProcessRecord {
    pid: u32,
    parent_pid: u32,
    process_group_id: u32,
    foreground_process_group_id: u32,
    tty: String,
    command: String,
    command_line: String,
}

fn scan_machine_terminals() -> Result<Vec<ObservedMachineTerminal>> {
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("/bin/ps")
            .args(["-x", "-ww", "-o", "pid=,ppid=,pgid=,tpgid=,tty=,command="])
            .output()
            .context("failed to run ps")?;
        if !output.status.success() {
            anyhow::bail!("ps exited with {}", output.status);
        }

        let processes = parse_ps_output(&String::from_utf8_lossy(&output.stdout));
        let current_process_id = std::process::id();
        let mut terminals = observed_terminals_from_processes(&processes, current_process_id);
        let working_directories = working_directories_for(
            terminals
                .iter()
                .map(|terminal| terminal.foreground_pid)
                .collect(),
        );
        for terminal in &mut terminals {
            terminal.working_directory = working_directories.get(&terminal.foreground_pid).cloned();
        }
        terminals.sort_by(|left, right| {
            left.owner_label()
                .cmp(right.owner_label())
                .then_with(|| left.tty.cmp(&right.tty))
                .then_with(|| left.foreground_pid.cmp(&right.foreground_pid))
        });
        Ok(terminals)
    }

    #[cfg(not(target_os = "macos"))]
    {
        Ok(Vec::new())
    }
}

fn parse_ps_output(output: &str) -> Vec<ProcessRecord> {
    output
        .lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let pid = fields.next()?.parse().ok()?;
            let parent_pid = fields.next()?.parse().ok()?;
            let process_group_id = fields.next()?.parse().ok()?;
            let foreground_process_group_id = fields.next()?.parse().ok()?;
            let tty = fields.next()?.to_owned();
            let command_line = fields.collect::<Vec<_>>().join(" ");
            let command = command_line.split_whitespace().next()?.to_owned();
            Some(ProcessRecord {
                pid,
                parent_pid,
                process_group_id,
                foreground_process_group_id,
                tty,
                command,
                command_line,
            })
        })
        .collect()
}

fn observed_terminals_from_processes(
    processes: &[ProcessRecord],
    current_process_id: u32,
) -> Vec<ObservedMachineTerminal> {
    let processes_by_pid: HashMap<u32, &ProcessRecord> = processes
        .iter()
        .map(|process| (process.pid, process))
        .collect();
    let mut processes_by_tty: HashMap<&str, Vec<&ProcessRecord>> = HashMap::new();

    for process in processes {
        if process.tty == "??" || process.tty == "-" {
            continue;
        }
        if process_is_descendant_of(process.pid, current_process_id, &processes_by_pid) {
            continue;
        }
        processes_by_tty
            .entry(process.tty.as_str())
            .or_default()
            .push(process);
    }

    processes_by_tty
        .into_iter()
        .filter_map(|(tty, processes)| {
            let foreground = processes
                .iter()
                .copied()
                .filter(|process| !process_is_terminal_helper(process))
                .filter(|process| {
                    process.foreground_process_group_id > 0
                        && process.process_group_id == process.foreground_process_group_id
                })
                .max_by_key(|process| {
                    process_depth(process.pid, &processes_by_pid)
                        .saturating_mul(1_000_000)
                        .saturating_add(process.pid as usize)
                })
                .or_else(|| {
                    processes
                        .iter()
                        .copied()
                        .filter(|process| !process_is_terminal_helper(process))
                        .max_by_key(|process| {
                            process_depth(process.pid, &processes_by_pid)
                                .saturating_mul(1_000_000)
                                .saturating_add(process.pid as usize)
                        })
                })?;

            if process_belongs_to_dez(foreground.pid, &processes_by_pid) {
                return None;
            }

            let (owning_application, owning_application_pid) =
                terminal_application_for(foreground.pid, &processes_by_pid)
                    .map(|(name, pid)| (Some(name.to_owned()), Some(pid)))
                    .unwrap_or((None, None));
            let root_terminal_pid =
                terminal_root_process_id(foreground.pid, tty, &processes_by_pid);
            let foreground_command =
                process_executable_name(&foreground.command, &foreground.command_line);
            let detected_agent_kind = detect_terminal_agent_command(&foreground_command)
                .or_else(|| detect_terminal_agent_kind(&foreground.command_line));

            Some(ObservedMachineTerminal {
                id: format!("{tty}:{root_terminal_pid}"),
                tty: tty.to_owned(),
                foreground_pid: foreground.pid,
                foreground_command,
                owning_application,
                owning_application_pid,
                working_directory: None,
                detected_agent_kind,
            })
        })
        .collect()
}

fn process_is_terminal_helper(process: &ProcessRecord) -> bool {
    let executable =
        process_executable_name(&process.command, &process.command_line).to_ascii_lowercase();
    executable.starts_with("gitstatusd")
        || matches!(
            executable.as_str(),
            "starship"
                | "oh-my-posh"
                | "zoxide"
                | "direnv"
                | "atuin"
                | "zsh-async"
                | "zsh-autosuggest"
        )
}

fn process_is_descendant_of(
    process_id: u32,
    ancestor_id: u32,
    processes_by_pid: &HashMap<u32, &ProcessRecord>,
) -> bool {
    let mut process_id = process_id;
    let mut visited = HashSet::new();
    while visited.insert(process_id) {
        if process_id == ancestor_id {
            return true;
        }
        let Some(process) = processes_by_pid.get(&process_id) else {
            return false;
        };
        if process.parent_pid == 0 || process.parent_pid == process_id {
            return false;
        }
        process_id = process.parent_pid;
    }
    false
}

fn process_depth(process_id: u32, processes_by_pid: &HashMap<u32, &ProcessRecord>) -> usize {
    let mut process_id = process_id;
    let mut depth = 0usize;
    let mut visited = HashSet::new();
    while visited.insert(process_id) {
        let Some(process) = processes_by_pid.get(&process_id) else {
            break;
        };
        if process.parent_pid == 0 || process.parent_pid == process_id {
            break;
        }
        process_id = process.parent_pid;
        depth = depth.saturating_add(1);
    }
    depth
}

fn terminal_root_process_id(
    process_id: u32,
    tty: &str,
    processes_by_pid: &HashMap<u32, &ProcessRecord>,
) -> u32 {
    let mut process_id = process_id;
    let mut root_process_id = process_id;
    let mut visited = HashSet::new();
    while visited.insert(process_id) {
        let Some(process) = processes_by_pid.get(&process_id) else {
            break;
        };
        if process.tty == tty {
            root_process_id = process.pid;
        }
        if process.parent_pid == 0 || process.parent_pid == process_id {
            break;
        }
        process_id = process.parent_pid;
    }
    root_process_id
}

fn process_belongs_to_dez(
    process_id: u32,
    processes_by_pid: &HashMap<u32, &ProcessRecord>,
) -> bool {
    let mut process_id = process_id;
    let mut visited = HashSet::new();
    while visited.insert(process_id) {
        let Some(process) = processes_by_pid.get(&process_id) else {
            return false;
        };
        let identity = format!("{} {}", process.command, process.command_line).to_ascii_lowercase();
        if identity.contains("/dez.app/") || identity.contains("dez-terminal-host") {
            return true;
        }
        if process.parent_pid == 0 || process.parent_pid == process_id {
            return false;
        }
        process_id = process.parent_pid;
    }
    false
}

fn terminal_application_for(
    process_id: u32,
    processes_by_pid: &HashMap<u32, &ProcessRecord>,
) -> Option<(&'static str, u32)> {
    let mut process_id = process_id;
    let mut visited = HashSet::new();
    while visited.insert(process_id) {
        let process = processes_by_pid.get(&process_id)?;
        if let Some(application) =
            known_terminal_application(&process.command, &process.command_line)
        {
            return Some((application, process.pid));
        }
        if process.parent_pid == 0 || process.parent_pid == process_id {
            return None;
        }
        process_id = process.parent_pid;
    }
    None
}

fn known_terminal_application(command: &str, command_line: &str) -> Option<&'static str> {
    let identity = format!("{command} {command_line}").to_ascii_lowercase();
    [
        ("/terminal.app/", "Terminal"),
        ("iterm", "iTerm"),
        ("warp.app", "Warp"),
        ("ghostty.app", "Ghostty"),
        ("wezterm", "WezTerm"),
        ("alacritty", "Alacritty"),
        ("kitty.app", "kitty"),
        ("hyper.app", "Hyper"),
        ("visual studio code.app", "Visual Studio Code"),
        ("cursor.app", "Cursor"),
        ("zed.app", "Zed"),
    ]
    .into_iter()
    .find_map(|(needle, application)| identity.contains(needle).then_some(application))
}

fn working_directories_for(process_ids: Vec<u32>) -> HashMap<u32, PathBuf> {
    #[cfg(target_os = "macos")]
    {
        if process_ids.is_empty() {
            return HashMap::new();
        }
        let process_ids = process_ids
            .into_iter()
            .map(|process_id| process_id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let Ok(output) = Command::new("/usr/sbin/lsof")
            .args(["-a", "-d", "cwd", "-p", &process_ids, "-Fn"])
            .output()
        else {
            return HashMap::new();
        };
        parse_lsof_working_directories(&String::from_utf8_lossy(&output.stdout))
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = process_ids;
        HashMap::new()
    }
}

fn parse_lsof_working_directories(output: &str) -> HashMap<u32, PathBuf> {
    let mut working_directories = HashMap::new();
    let mut current_process_id = None;
    let mut saw_cwd = false;
    for line in output.lines() {
        match line.as_bytes().first().copied() {
            Some(b'p') => {
                current_process_id = line[1..].parse().ok();
                saw_cwd = false;
            }
            Some(b'f') => {
                saw_cwd = line == "fcwd";
            }
            Some(b'n') if saw_cwd => {
                if let Some(process_id) = current_process_id {
                    working_directories.insert(process_id, PathBuf::from(&line[1..]));
                }
                saw_cwd = false;
            }
            _ => {}
        }
    }
    working_directories
}

fn display_process_name(command: &str) -> &str {
    command
        .rsplit('/')
        .next()
        .filter(|command| !command.is_empty())
        .unwrap_or("Terminal")
}

fn process_executable_name(command: &str, command_line: &str) -> String {
    let executable = command_line
        .split_whitespace()
        .next()
        .filter(|command| !command.is_empty())
        .unwrap_or(command)
        .trim_start_matches('-');
    display_process_name(executable).to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_processes() -> Vec<ProcessRecord> {
        parse_ps_output(
            "\
100 1 100 0 ?? /Applications/Utilities/Terminal.app/Contents/MacOS/Terminal
101 100 101 102 ttys001 -zsh
102 101 102 102 ttys001 /opt/homebrew/bin/codex
200 1 200 0 ?? /Applications/Dez.app/Contents/MacOS/dez
201 200 201 201 ttys002 -zsh
300 1 300 0 ?? /Applications/Warp.app/Contents/MacOS/stable
301 300 301 301 ttys003 -zsh
",
        )
    }

    #[test]
    fn discovers_external_ttys_and_excludes_dez_descendants() {
        let terminals = observed_terminals_from_processes(&fixture_processes(), 200);

        assert_eq!(terminals.len(), 2);
        let codex = terminals
            .iter()
            .find(|terminal| terminal.tty == "ttys001")
            .unwrap();
        assert_eq!(codex.foreground_pid, 102);
        assert_eq!(codex.owning_application.as_deref(), Some("Terminal"));
        assert_eq!(codex.detected_agent_kind, Some(TerminalAgentKind::Codex));
        assert_eq!(codex.display_title(), "Codex");

        let warp = terminals
            .iter()
            .find(|terminal| terminal.tty == "ttys003")
            .unwrap();
        assert_eq!(warp.owning_application.as_deref(), Some("Warp"));
        assert_eq!(warp.display_title(), "zsh");
        assert!(terminals.iter().all(|terminal| terminal.tty != "ttys002"));
    }

    #[test]
    fn ignores_prompt_helpers_when_the_foreground_group_is_unavailable() {
        let processes = parse_ps_output(
            "\
100 1 100 0 ?? /Applications/Utilities/Terminal.app/Contents/MacOS/Terminal
101 100 101 0 ttys001 -zsh
102 101 102 0 ttys001 /Users/test/.cache/gitstatus/gitstatusd-darwin-arm64 -G v1
",
        );

        let terminals = observed_terminals_from_processes(&processes, 999);
        assert_eq!(terminals.len(), 1);
        assert_eq!(terminals[0].foreground_pid, 101);
        assert_eq!(terminals[0].display_title(), "zsh");
    }

    #[test]
    fn parses_lsof_cwd_records_without_retaining_other_descriptors() {
        let working_directories = parse_lsof_working_directories(
            "\
p102
fcwd
n/Users/test/Documents/dez
ftxt
n/opt/homebrew/bin/codex
p301
fcwd
n/Users/test
",
        );

        assert_eq!(
            working_directories.get(&102),
            Some(&PathBuf::from("/Users/test/Documents/dez"))
        );
        assert_eq!(
            working_directories.get(&301),
            Some(&PathBuf::from("/Users/test"))
        );
    }

    #[test]
    fn recognizes_supported_terminal_app_ancestors() {
        assert_eq!(
            known_terminal_application(
                "/Applications/iTerm.app/Contents/MacOS/iTerm2",
                "/Applications/iTerm.app/Contents/MacOS/iTerm2"
            ),
            Some("iTerm")
        );
        assert_eq!(
            known_terminal_application(
                "/Applications/Visual Studio Code.app/Contents/MacOS/Electron",
                "Electron"
            ),
            Some("Visual Studio Code")
        );
    }
}
