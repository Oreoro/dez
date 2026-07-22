use std::{
    fs::OpenOptions,
    io::{ErrorKind, Read as _, Write as _},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::Arc,
    time::Duration,
};

use anyhow::{Context as _, Result};
use gpui::{App, AppContext as _, BackgroundExecutor, Entity, Global};
use terminal::session_host::{
    TerminalHostId,
    transport::{
        EXPERIMENTAL_TERMINAL_HOST_ENV, TerminalHostAuthToken, TerminalHostConnection,
        TerminalHostHandshakeRejection, TerminalHostStartupState, TerminalHostStartupStatus,
        TerminalHostTransportError, terminal_host_executable_path,
    },
};
use util::ResultExt as _;
use uuid::Uuid;

const TERMINAL_HOST_CONNECT_ATTEMPTS: usize = 40;
const TERMINAL_HOST_CONNECT_INTERVAL: Duration = Duration::from_millis(50);

struct GlobalTerminalHostRuntime(Entity<TerminalHostRuntime>);

impl Global for GlobalTerminalHostRuntime {}

pub struct TerminalHostRuntime;

impl TerminalHostRuntime {
    pub fn init(host_id: TerminalHostId, cx: &mut App) -> Entity<Self> {
        if let Some(runtime) = Self::try_global(cx) {
            return runtime;
        }

        terminal::session_host::transport::TerminalHostSnapshotRevision::init(cx);
        TerminalHostStartupStatus::init(cx);
        let enabled = std::env::var(EXPERIMENTAL_TERMINAL_HOST_ENV).as_deref() == Ok("1");
        let runtime = cx.new(|_| Self);
        cx.set_global(GlobalTerminalHostRuntime(runtime.clone()));
        if !enabled {
            return runtime;
        }
        TerminalHostStartupStatus::set(TerminalHostStartupState::Connecting, cx);

        let background_executor = cx.background_executor().clone();
        let runtime_handle = runtime.downgrade();
        cx.spawn(async move |cx| {
            let result = connect_or_launch(host_id, &background_executor).await;
            runtime_handle
                .update(cx, |_runtime, cx| {
                    match result {
                        Ok(connection) => {
                            let connection = Arc::new(connection);
                            TerminalHostConnection::set_global(connection, cx);
                        }
                        Err(error) => {
                            let message = format!("{error:#}");
                            log::error!("durable terminal host startup failed: {message}");
                            TerminalHostStartupStatus::set(
                                TerminalHostStartupState::Failed {
                                    message: message.clone(),
                                },
                                cx,
                            );
                        }
                    }
                    cx.notify();
                })
                .log_err();
        })
        .detach();
        runtime
    }

    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalTerminalHostRuntime>()
            .map(|runtime| runtime.0.clone())
    }
}

async fn connect_or_launch(
    host_id: TerminalHostId,
    background_executor: &BackgroundExecutor,
) -> Result<TerminalHostConnection> {
    let paths = prepare_runtime_paths()?;
    let auth_token = read_or_create_auth_token(&paths.token)?;

    match TerminalHostConnection::connect(
        &paths.socket,
        host_id,
        auth_token.clone(),
        background_executor,
    )
    .await
    {
        Ok(connection) => return Ok(connection),
        Err(error) if is_identity_rejection(&error) => return Err(error.into()),
        Err(error) if is_stale_socket_error(&error) => remove_stale_socket(&paths.socket)?,
        Err(error) => return Err(error.into()),
    }

    let helper = terminal_host_executable()?;
    let mut helper_process = Command::new(&helper)
        .arg("serve")
        .arg("--socket")
        .arg(&paths.socket)
        .arg("--token-file")
        .arg(&paths.token)
        .arg("--host-id")
        .arg(host_id.to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .with_context(|| format!("launch terminal host helper {}", helper.display()))?;
    std::thread::Builder::new()
        .name("dez terminal host monitor".to_owned())
        .spawn(move || {
            if let Err(error) = helper_process.wait() {
                log::warn!("failed to wait for terminal host helper: {error}");
            }
        })
        .context("start terminal host process monitor")?;

    let mut last_error = None;
    for _ in 0..TERMINAL_HOST_CONNECT_ATTEMPTS {
        match TerminalHostConnection::connect(
            &paths.socket,
            host_id,
            auth_token.clone(),
            background_executor,
        )
        .await
        {
            Ok(connection) => return Ok(connection),
            Err(error) if is_identity_rejection(&error) => return Err(error.into()),
            Err(error) => last_error = Some(error),
        }
        background_executor
            .timer(TERMINAL_HOST_CONNECT_INTERVAL)
            .await;
    }
    match last_error {
        Some(error) => Err(error).context("connect to launched terminal host helper"),
        None => anyhow::bail!("terminal host helper did not accept a connection"),
    }
}

fn is_identity_rejection(error: &TerminalHostTransportError) -> bool {
    matches!(
        error,
        TerminalHostTransportError::HandshakeRejected(
            TerminalHostHandshakeRejection::AuthenticationFailed
                | TerminalHostHandshakeRejection::HostMismatch
                | TerminalHostHandshakeRejection::ProtocolMismatch { .. }
        ) | TerminalHostTransportError::HostMismatch
            | TerminalHostTransportError::ProtocolMismatch { .. }
    )
}

fn is_stale_socket_error(error: &TerminalHostTransportError) -> bool {
    matches!(
        error,
        TerminalHostTransportError::Io(error)
            if matches!(error.kind(), ErrorKind::NotFound | ErrorKind::ConnectionRefused)
    )
}

struct TerminalHostRuntimePaths {
    socket: PathBuf,
    token: PathBuf,
}

fn prepare_runtime_paths() -> Result<TerminalHostRuntimePaths> {
    let directory = paths::state_dir().join("terminal-host");
    create_private_directory(&directory)?;
    Ok(TerminalHostRuntimePaths {
        socket: directory.join("local.sock"),
        token: directory.join("auth.token"),
    })
}

#[cfg(unix)]
fn create_private_directory(path: &Path) -> Result<()> {
    use std::os::unix::fs::{DirBuilderExt as _, PermissionsExt as _};

    std::fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(path)
        .with_context(|| format!("create terminal host runtime directory {}", path.display()))?;
    let metadata = std::fs::symlink_metadata(path)
        .with_context(|| format!("inspect terminal host runtime directory {}", path.display()))?;
    anyhow::ensure!(
        metadata.file_type().is_dir(),
        "terminal host runtime path is not a real directory"
    );
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))
        .with_context(|| format!("secure terminal host runtime directory {}", path.display()))
}

#[cfg(not(unix))]
fn create_private_directory(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path)
        .with_context(|| format!("create terminal host runtime directory {}", path.display()))
}

fn read_or_create_auth_token(path: &Path) -> Result<TerminalHostAuthToken> {
    match open_new_token_file(path) {
        Ok(mut file) => {
            let token = Uuid::new_v4().simple().to_string();
            file.write_all(token.as_bytes())
                .context("write terminal host authentication token")?;
            file.sync_all()
                .context("persist terminal host authentication token")?;
            TerminalHostAuthToken::parse(token).context("parse new terminal host token")
        }
        Err(error) if error.kind() == ErrorKind::AlreadyExists => {
            validate_existing_token_file(path)?;
            let mut token = String::new();
            OpenOptions::new()
                .read(true)
                .open(path)
                .with_context(|| format!("open terminal host token {}", path.display()))?
                .read_to_string(&mut token)
                .context("read terminal host authentication token")?;
            TerminalHostAuthToken::parse(token.trim().to_owned())
                .context("parse existing terminal host token")
        }
        Err(error) => {
            Err(error).with_context(|| format!("create terminal host token {}", path.display()))
        }
    }
}

#[cfg(unix)]
fn validate_existing_token_file(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt as _;

    let metadata = std::fs::symlink_metadata(path)
        .with_context(|| format!("inspect terminal host token {}", path.display()))?;
    anyhow::ensure!(
        metadata.file_type().is_file(),
        "terminal host token path is not a regular file"
    );
    anyhow::ensure!(
        metadata.permissions().mode() & 0o077 == 0,
        "terminal host token file must not be accessible by group or other users"
    );
    Ok(())
}

#[cfg(not(unix))]
fn validate_existing_token_file(path: &Path) -> Result<()> {
    anyhow::ensure!(path.is_file(), "terminal host token path is not a file");
    Ok(())
}

#[cfg(unix)]
fn open_new_token_file(path: &Path) -> std::io::Result<std::fs::File> {
    use std::os::unix::fs::OpenOptionsExt as _;

    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
}

#[cfg(not(unix))]
fn open_new_token_file(path: &Path) -> std::io::Result<std::fs::File> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

#[cfg(unix)]
fn remove_stale_socket(path: &Path) -> Result<()> {
    use std::os::unix::fs::FileTypeExt as _;

    let metadata = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error).context("inspect terminal host socket"),
    };
    anyhow::ensure!(
        metadata.file_type().is_socket(),
        "refusing to remove non-socket terminal host path {}",
        path.display()
    );
    std::fs::remove_file(path)
        .with_context(|| format!("remove stale terminal host socket {}", path.display()))
}

#[cfg(not(unix))]
fn remove_stale_socket(path: &Path) -> Result<()> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error)
            .with_context(|| format!("remove stale terminal host socket {}", path.display())),
    }
}

fn terminal_host_executable() -> Result<PathBuf> {
    let helper = terminal_host_executable_path().context("locate Dez terminal host helper")?;
    anyhow::ensure!(
        helper.is_file(),
        "terminal host helper is not installed at {}",
        helper.display()
    );
    Ok(helper)
}
