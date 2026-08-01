use std::{
    collections::{BTreeMap, HashMap, HashSet},
    future::Future,
    io::ErrorKind,
    path::PathBuf,
    process::Output,
    time::Duration,
};

use anyhow::{Context as _, Result};
use futures::{FutureExt as _, Stream, StreamExt as _};
use gpui::{App, AppContext as _, BackgroundExecutor, Context, Entity, Global, Task};
use paths::APP_NAME;
use serde_json::Value;

use crate::terminal_thread_metadata_store::{
    TerminalAgentKind, detect_terminal_agent_command, detect_terminal_agent_kind,
};

const MULTIPLEXER_REFRESH_INTERVAL: Duration = Duration::from_secs(5);
const MULTIPLEXER_COMMAND_TIMEOUT: Duration = Duration::from_secs(4);
const HERDR_ENDPOINT_TIMEOUT: Duration = Duration::from_secs(2);
const HERDR_SOURCE_TIMEOUT: Duration = Duration::from_secs(4);
const HERDR_MAX_CONCURRENT_ENDPOINT_QUERIES: usize = 8;
// tmux sanitizes control characters when Dez is launched from Finder's
// minimal, locale-free environment. Keep this printable so the exact same
// parser works from a shell, Finder, and a signed application bundle.
const TMUX_FIELD_SEPARATOR: &str = "|:dez:|";
const TMUX_FORMAT: &str = "#{session_id}|:dez:|#{session_name}|:dez:|#{session_attached}|:dez:|#{window_id}|:dez:|#{window_name}|:dez:|#{window_active}|:dez:|#{pane_id}|:dez:|#{pane_active}|:dez:|#{pane_current_path}|:dez:|#{pane_current_command}|:dez:|#{pane_title}";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MultiplexerKind {
    Tmux,
    Herdr,
    Cmux,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultiplexerSourceIssue {
    pub kind: MultiplexerKind,
    pub summary: String,
    pub had_successful_scan: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MultiplexerSourceAvailability {
    MissingExecutable,
    AvailableEmpty,
    Failed,
    Ready,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultiplexerSourceStatus {
    pub kind: MultiplexerKind,
    pub availability: MultiplexerSourceAvailability,
    pub session_count: usize,
    pub summary: Option<String>,
    pub had_successful_scan: bool,
}

impl MultiplexerKind {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Tmux => "tmux",
            Self::Herdr => "Herdr",
            Self::Cmux => "cmux",
        }
    }

    fn workspace_priority(self) -> u8 {
        match self {
            Self::Cmux => 0,
            Self::Herdr => 1,
            Self::Tmux => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MultiplexerSessionState {
    Available,
    Attached,
    Detached,
    Idle,
    Working,
    NeedsAttention,
    Completed,
    Unknown,
}

impl MultiplexerSessionState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Attached => "attached",
            Self::Detached => "detached",
            Self::Idle => "idle",
            Self::Working => "working",
            Self::NeedsAttention => "needs input",
            Self::Completed => "done",
            Self::Unknown => "state unknown",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExternalSessionOpenMode {
    AttachInTerminal,
    RevealExternal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternalSessionCommand {
    pub program: String,
    pub args: Vec<String>,
    pub label: String,
    pub mode: ExternalSessionOpenMode,
}

/// An externally owned terminal session or Workspace that Dez can safely open.
///
/// The external multiplexer remains authoritative. tmux and Herdr sessions open
/// as native terminal clients, while cmux Workspaces remain in cmux and are
/// revealed through its public CLI.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternalMultiplexerSession {
    pub id: String,
    pub kind: MultiplexerKind,
    pub target: String,
    pub title: String,
    pub owner_context: Option<String>,
    pub working_directory: Option<PathBuf>,
    pub foreground_command: Option<String>,
    pub detected_agent_kind: Option<TerminalAgentKind>,
    pub latest_activity: Option<String>,
    pub listening_ports: Vec<u16>,
    pub state: MultiplexerSessionState,
    pub attached_clients: Option<usize>,
    discovery_stale: bool,
    executable: PathBuf,
    herdr_session_name: Option<String>,
}

impl ExternalMultiplexerSession {
    pub fn is_last_known(&self) -> bool {
        self.discovery_stale
    }

    pub fn source_label(&self) -> String {
        self.owner_context
            .as_ref()
            .map(|context| format!("{} · {context}", self.kind.display_name()))
            .unwrap_or_else(|| self.kind.display_name().to_owned())
    }

    pub fn state_label(&self) -> String {
        let label = match self.attached_clients {
            Some(1) => format!("{} · 1 client", self.state.label()),
            Some(count) if count > 1 => format!("{} · {count} clients", self.state.label()),
            _ => self.state.label().to_owned(),
        };
        if self.discovery_stale {
            format!("{label} · last known")
        } else {
            label
        }
    }

    pub fn location_label(&self) -> String {
        let location = self
            .working_directory
            .as_ref()
            .map(|path| path.to_string_lossy().into_owned());
        let ports = concise_port_label(&self.listening_ports);
        match (location, ports) {
            (Some(location), Some(ports)) => format!("{location} · {ports}"),
            (Some(location), None) => location,
            (None, Some(ports)) => ports,
            (None, None) => "Working directory unavailable".to_owned(),
        }
    }

    pub fn open_command(&self) -> ExternalSessionCommand {
        let program = self.executable.to_string_lossy().into_owned();
        let (args, label, mode) = match self.kind {
            MultiplexerKind::Tmux => (
                vec![
                    "attach-session".to_owned(),
                    "-t".to_owned(),
                    self.target.clone(),
                ],
                format!("Attach {}", self.title),
                ExternalSessionOpenMode::AttachInTerminal,
            ),
            MultiplexerKind::Herdr => {
                let mut args = Vec::new();
                if let Some(session_name) = &self.herdr_session_name {
                    args.extend(["--session".to_owned(), session_name.clone()]);
                }
                if self.detected_agent_kind.is_some() {
                    args.extend(["agent".to_owned(), "attach".to_owned(), self.target.clone()]);
                } else {
                    args.extend([
                        "terminal".to_owned(),
                        "attach".to_owned(),
                        self.target.clone(),
                    ]);
                }
                (
                    args,
                    format!("Attach {}", self.title),
                    ExternalSessionOpenMode::AttachInTerminal,
                )
            }
            MultiplexerKind::Cmux => (
                vec![
                    "select-workspace".to_owned(),
                    "--workspace".to_owned(),
                    self.target.clone(),
                ],
                format!("Open {} in cmux", self.title),
                ExternalSessionOpenMode::RevealExternal,
            ),
        };

        ExternalSessionCommand {
            program,
            args,
            label,
            mode,
        }
    }
}

struct GlobalMultiplexerSessionStore(Entity<MultiplexerSessionStore>);
impl Global for GlobalMultiplexerSessionStore {}

pub struct MultiplexerSessionStore {
    sessions: Vec<ExternalMultiplexerSession>,
    source_issues: Vec<MultiplexerSourceIssue>,
    source_statuses: Vec<MultiplexerSourceStatus>,
    successful_sources: HashSet<MultiplexerKind>,
    refreshing: bool,
    refresh_pending: bool,
    _refresh_task: Task<()>,
}

struct AvailableMultiplexerScan {
    sessions: Vec<ExternalMultiplexerSession>,
    warnings: Vec<MultiplexerScanWarning>,
    successful_endpoint_count: usize,
}

struct MultiplexerScanWarning {
    herdr_session_name: Option<String>,
    summary: String,
}

enum MultiplexerScanOutcome {
    MissingExecutable,
    Available(AvailableMultiplexerScan),
}

struct ExternalMultiplexerScan {
    tmux: Result<MultiplexerScanOutcome>,
    herdr: Result<MultiplexerScanOutcome>,
    cmux: Result<MultiplexerScanOutcome>,
}

impl AvailableMultiplexerScan {
    fn complete(sessions: Vec<ExternalMultiplexerSession>) -> Self {
        Self {
            sessions,
            warnings: Vec::new(),
            successful_endpoint_count: 1,
        }
    }
}

impl MultiplexerSessionStore {
    pub fn init_global(cx: &mut App) {
        if APP_NAME == "Zed" || cx.has_global::<GlobalMultiplexerSessionStore>() {
            return;
        }

        let store = cx.new(Self::new);
        cx.set_global(GlobalMultiplexerSessionStore(store));
    }

    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalMultiplexerSessionStore>()
            .map(|store| store.0.clone())
    }

    pub fn sessions(&self) -> &[ExternalMultiplexerSession] {
        &self.sessions
    }

    pub fn is_refreshing(&self) -> bool {
        self.refreshing
    }

    pub fn source_issues(&self) -> &[MultiplexerSourceIssue] {
        &self.source_issues
    }

    pub fn source_statuses(&self) -> &[MultiplexerSourceStatus] {
        &self.source_statuses
    }

    pub fn source_status(&self, kind: MultiplexerKind) -> Option<&MultiplexerSourceStatus> {
        self.source_statuses
            .iter()
            .find(|status| status.kind == kind)
    }

    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        if self.refreshing {
            self.refresh_pending = true;
            return;
        }

        self.refreshing = true;
        cx.notify();
        cx.spawn(async move |this, cx| {
            let scan_executor = cx.background_executor().clone();
            let scan = cx
                .background_executor()
                .spawn(async move { scan_external_multiplexer_sessions(&scan_executor).await })
                .await;
            if let Err(error) = this.update(cx, |store, cx| {
                store.refreshing = false;
                store.apply_scan(scan, cx);
                if std::mem::take(&mut store.refresh_pending) {
                    store.refresh(cx);
                } else {
                    cx.notify();
                }
            }) {
                log::debug!("external terminal refresh finished after its store closed: {error:#}");
            }
        })
        .detach();
    }

    fn apply_scan(&mut self, scan: ExternalMultiplexerScan, cx: &mut Context<Self>) {
        let (sessions, source_issues, source_statuses, successful_sources) =
            reconcile_external_multiplexer_sessions(&self.sessions, scan, &self.successful_sources);
        self.successful_sources.extend(successful_sources);
        if self.sessions != sessions
            || self.source_issues != source_issues
            || self.source_statuses != source_statuses
        {
            self.sessions = sessions;
            self.source_issues = source_issues;
            self.source_statuses = source_statuses;
            cx.notify();
        }
    }

    fn new(cx: &mut Context<Self>) -> Self {
        let refresh_task = cx.spawn(async move |this, cx| {
            loop {
                if this.update(cx, |store, cx| store.refresh(cx)).is_err() {
                    break;
                }

                cx.background_executor()
                    .timer(MULTIPLEXER_REFRESH_INTERVAL)
                    .await;
            }
        });

        Self {
            sessions: Vec::new(),
            source_issues: Vec::new(),
            source_statuses: Vec::new(),
            successful_sources: HashSet::new(),
            refreshing: false,
            refresh_pending: false,
            _refresh_task: refresh_task,
        }
    }
}

pub fn init(cx: &mut App) {
    MultiplexerSessionStore::init_global(cx);
}

async fn scan_external_multiplexer_sessions(
    executor: &BackgroundExecutor,
) -> ExternalMultiplexerScan {
    let tmux = bounded_multiplexer_scan("tmux", scan_tmux_sessions(), executor);
    let herdr = scan_herdr_sessions(executor);
    let cmux = bounded_multiplexer_scan("cmux", scan_cmux_workspaces(), executor);
    let (tmux, herdr, cmux) = futures::join!(tmux, herdr, cmux);
    ExternalMultiplexerScan { tmux, herdr, cmux }
}

async fn bounded_multiplexer_scan(
    source: &'static str,
    scan: impl Future<Output = Result<MultiplexerScanOutcome>>,
    executor: &BackgroundExecutor,
) -> Result<MultiplexerScanOutcome> {
    run_bounded_multiplexer_operation(
        format!("{source} discovery"),
        scan,
        executor,
        MULTIPLEXER_COMMAND_TIMEOUT,
    )
    .await
}

async fn run_bounded_multiplexer_operation<T>(
    operation_name: impl Into<String>,
    operation: impl Future<Output = Result<T>>,
    executor: &BackgroundExecutor,
    timeout_duration: Duration,
) -> Result<T> {
    let operation_name = operation_name.into();
    let timeout = executor.timer(timeout_duration);
    futures::pin_mut!(operation, timeout);
    match futures::future::select(operation, timeout).await {
        futures::future::Either::Left((result, _)) => result,
        futures::future::Either::Right(_) => {
            anyhow::bail!("{operation_name} timed out after {timeout_duration:?}")
        }
    }
}

fn reconcile_external_multiplexer_sessions(
    previous_sessions: &[ExternalMultiplexerSession],
    scan: ExternalMultiplexerScan,
    successful_sources: &HashSet<MultiplexerKind>,
) -> (
    Vec<ExternalMultiplexerSession>,
    Vec<MultiplexerSourceIssue>,
    Vec<MultiplexerSourceStatus>,
    Vec<MultiplexerKind>,
) {
    let mut sessions = Vec::new();
    let mut source_issues = Vec::new();
    let mut source_statuses = Vec::new();
    let mut newly_successful_sources = Vec::new();
    for (kind, result) in [
        (MultiplexerKind::Tmux, scan.tmux),
        (MultiplexerKind::Herdr, scan.herdr),
        (MultiplexerKind::Cmux, scan.cmux),
    ] {
        match result {
            Ok(MultiplexerScanOutcome::MissingExecutable) => {
                let last_known_sessions = previous_sessions
                    .iter()
                    .filter(|session| session.kind == kind)
                    .map(|session| {
                        let mut session = session.clone();
                        session.discovery_stale = true;
                        session
                    })
                    .collect::<Vec<_>>();
                let session_count = last_known_sessions.len();
                sessions.extend(last_known_sessions);
                source_statuses.push(MultiplexerSourceStatus {
                    kind,
                    availability: MultiplexerSourceAvailability::MissingExecutable,
                    session_count,
                    summary: Some(format!("{} executable was not found", kind.display_name())),
                    had_successful_scan: successful_sources.contains(&kind),
                });
            }
            Ok(MultiplexerScanOutcome::Available(mut scan)) if scan.warnings.is_empty() => {
                let session_count = scan.sessions.len();
                let availability = if scan.sessions.is_empty() {
                    MultiplexerSourceAvailability::AvailableEmpty
                } else {
                    MultiplexerSourceAvailability::Ready
                };
                sessions.append(&mut scan.sessions);
                newly_successful_sources.push(kind);
                source_statuses.push(MultiplexerSourceStatus {
                    kind,
                    availability,
                    session_count,
                    summary: None,
                    had_successful_scan: true,
                });
            }
            Ok(MultiplexerScanOutcome::Available(mut scan)) => {
                let current_session_ids = scan
                    .sessions
                    .iter()
                    .map(|session| session.id.clone())
                    .collect::<HashSet<_>>();
                let failed_herdr_session_names = scan
                    .warnings
                    .iter()
                    .map(|warning| warning.herdr_session_name.clone())
                    .collect::<Vec<_>>();
                scan.sessions.extend(
                    previous_sessions
                        .iter()
                        .filter(|session| {
                            session.kind == kind
                                && !current_session_ids.contains(&session.id)
                                && failed_herdr_session_names.iter().any(|name| {
                                    session.herdr_session_name.as_ref() == name.as_ref()
                                })
                        })
                        .map(|session| {
                            let mut session = session.clone();
                            session.discovery_stale = true;
                            session
                        }),
                );
                let summary = concise_discovery_message(
                    &scan
                        .warnings
                        .iter()
                        .map(|warning| warning.summary.as_str())
                        .collect::<Vec<_>>()
                        .join("; "),
                );
                let had_successful_scan =
                    successful_sources.contains(&kind) || scan.successful_endpoint_count > 0;
                if scan.successful_endpoint_count > 0 {
                    newly_successful_sources.push(kind);
                }
                let session_count = scan.sessions.len();
                sessions.append(&mut scan.sessions);
                source_issues.push(MultiplexerSourceIssue {
                    kind,
                    summary: summary.clone(),
                    had_successful_scan,
                });
                source_statuses.push(MultiplexerSourceStatus {
                    kind,
                    availability: MultiplexerSourceAvailability::Failed,
                    session_count,
                    summary: Some(summary),
                    had_successful_scan,
                });
            }
            Err(error) => {
                log::debug!(
                    "{} discovery failed; preserving its last known sessions: {error:#}",
                    kind.display_name()
                );
                let last_known_sessions = previous_sessions
                    .iter()
                    .filter(|session| session.kind == kind)
                    .map(|session| {
                        let mut session = session.clone();
                        session.discovery_stale = true;
                        session
                    })
                    .collect::<Vec<_>>();
                let session_count = last_known_sessions.len();
                sessions.extend(last_known_sessions);
                let summary = concise_discovery_error(&error);
                source_issues.push(MultiplexerSourceIssue {
                    kind,
                    summary: summary.clone(),
                    had_successful_scan: successful_sources.contains(&kind),
                });
                source_statuses.push(MultiplexerSourceStatus {
                    kind,
                    availability: MultiplexerSourceAvailability::Failed,
                    session_count,
                    summary: Some(summary),
                    had_successful_scan: successful_sources.contains(&kind),
                });
            }
        }
    }
    sessions.sort_by(|left, right| {
        left.kind
            .workspace_priority()
            .cmp(&right.kind.workspace_priority())
            .then_with(|| left.title.cmp(&right.title))
            .then_with(|| left.id.cmp(&right.id))
    });
    (
        sessions,
        source_issues,
        source_statuses,
        newly_successful_sources,
    )
}

fn concise_discovery_error(error: &anyhow::Error) -> String {
    concise_discovery_message(&format!("{error:#}"))
}

fn concise_discovery_message(message: &str) -> String {
    const MAX_CHARS: usize = 180;
    let message = message.split_whitespace().collect::<Vec<_>>().join(" ");
    if message.chars().count() <= MAX_CHARS {
        message
    } else {
        format!("{}…", message.chars().take(MAX_CHARS).collect::<String>())
    }
}

async fn scan_tmux_sessions() -> Result<MultiplexerScanOutcome> {
    let Some((executable, output)) = run_first_available(
        &["/opt/homebrew/bin/tmux", "/usr/local/bin/tmux", "tmux"],
        &["list-panes", "-a", "-F", TMUX_FORMAT],
    )
    .await?
    else {
        return Ok(MultiplexerScanOutcome::MissingExecutable);
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if tmux_failure_is_no_server(output.status.code(), &stderr) {
            return Ok(MultiplexerScanOutcome::Available(
                AvailableMultiplexerScan::complete(Vec::new()),
            ));
        }
        anyhow::bail!("tmux discovery failed: {}", stderr.trim());
    }

    Ok(MultiplexerScanOutcome::Available(
        AvailableMultiplexerScan::complete(parse_tmux_sessions(
            &String::from_utf8_lossy(&output.stdout),
            executable,
        )?),
    ))
}

fn tmux_failure_is_no_server(exit_code: Option<i32>, stderr: &str) -> bool {
    if exit_code != Some(1) {
        return false;
    }
    let stderr = stderr.to_ascii_lowercase();
    stderr.contains("no server running on")
        || stderr.contains("no server running")
        || stderr.contains("error connecting to") && stderr.contains("no such file or directory")
}

async fn scan_cmux_workspaces() -> Result<MultiplexerScanOutcome> {
    let Some((executable, current_output)) = run_first_available(
        &[
            "/Applications/cmux.app/Contents/Resources/bin/cmux",
            "/opt/homebrew/bin/cmux",
            "/usr/local/bin/cmux",
            "cmux",
        ],
        &["workspace", "list", "--json"],
    )
    .await?
    else {
        return Ok(MultiplexerScanOutcome::MissingExecutable);
    };

    let (workspace_output, used_legacy_workspace_verb) = if current_output.status.success() {
        (current_output, false)
    } else {
        let legacy_output = run_multiplexer_command(
            &executable,
            &["list-workspaces", "--json"],
            "cmux compatibility workspace discovery",
        )
        .await?;
        if !legacy_output.status.success() {
            anyhow::bail!(
                "cmux discovery failed: {}; compatibility fallback failed: {}",
                concise_command_stderr(&current_output),
                concise_command_stderr(&legacy_output)
            );
        }
        (legacy_output, true)
    };

    let notification_output = run_multiplexer_command(
        &executable,
        &["list-notifications", "--json"],
        "cmux notification discovery",
    )
    .await?;
    let (notification_json, notification_warning) = if notification_output.status.success() {
        (
            Some(String::from_utf8_lossy(&notification_output.stdout).into_owned()),
            None,
        )
    } else if used_legacy_workspace_verb {
        log::debug!(
            "cmux compatibility discovery does not expose notifications: {}",
            concise_command_stderr(&notification_output)
        );
        (None, None)
    } else {
        (
            None,
            Some(format!(
                "cmux notifications could not be refreshed: {}",
                concise_command_stderr(&notification_output)
            )),
        )
    };

    let sessions = parse_cmux_workspaces(
        &String::from_utf8_lossy(&workspace_output.stdout),
        notification_json.as_deref(),
        executable,
    )?;
    let mut scan = AvailableMultiplexerScan::complete(sessions);
    if let Some(summary) = notification_warning {
        scan.warnings.push(MultiplexerScanWarning {
            herdr_session_name: None,
            summary,
        });
    }

    Ok(MultiplexerScanOutcome::Available(scan))
}

fn parse_cmux_workspaces(
    output: &str,
    notification_output: Option<&str>,
    executable: PathBuf,
) -> Result<Vec<ExternalMultiplexerSession>> {
    let value: Value = serde_json::from_str(output).context("invalid cmux workspace JSON")?;
    let result = value.get("result").unwrap_or(&value);
    let workspaces = result
        .get("workspaces")
        .and_then(Value::as_array)
        .or_else(|| result.as_array())
        .context("cmux response did not contain workspaces")?;
    let activity_by_workspace = notification_output
        .map(parse_cmux_notifications)
        .transpose()?
        .unwrap_or_default();

    Ok(workspaces
        .iter()
        .filter_map(|workspace| {
            let target = workspace
                .get("id")
                .and_then(Value::as_str)
                .or_else(|| workspace.get("ref").and_then(Value::as_str))?
                .to_owned();
            let working_directory = workspace
                .get("current_directory")
                .and_then(Value::as_str)
                .or_else(|| workspace.get("cwd").and_then(Value::as_str))
                .filter(|path| !path.is_empty())
                .map(PathBuf::from);
            let title = workspace
                .get("title")
                .and_then(Value::as_str)
                .filter(|title| !title.is_empty())
                .map(str::to_owned)
                .or_else(|| {
                    working_directory
                        .as_deref()
                        .and_then(std::path::Path::file_name)
                        .and_then(|name| name.to_str())
                        .map(str::to_owned)
                })
                .unwrap_or_else(|| "cmux Workspace".to_owned());
            let owner_context = workspace
                .get("description")
                .and_then(Value::as_str)
                .filter(|description| !description.is_empty() && *description != title)
                .map(str::to_owned);
            let activity = activity_by_workspace.get(&target.to_ascii_lowercase());
            let detected_agent_kind = cmux_workspace_agent_label(workspace)
                .and_then(detect_terminal_agent_kind)
                .or_else(|| activity.and_then(|activity| activity.detected_agent_kind))
                .or_else(|| cmux_workspace_detected_agent_kind(workspace));
            let needs_attention = workspace
                .get("unread_count")
                .and_then(Value::as_u64)
                .is_some_and(|count| count > 0)
                || workspace
                    .get("needs_attention")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
                || activity.is_some_and(|activity| activity.needs_attention);
            let latest_activity = activity
                .and_then(|activity| activity.latest_activity.clone())
                .or_else(|| cmux_workspace_latest_activity(workspace));
            let listening_ports = cmux_workspace_listening_ports(workspace);

            Some(ExternalMultiplexerSession {
                id: format!("cmux:{target}"),
                kind: MultiplexerKind::Cmux,
                target,
                title,
                owner_context,
                working_directory,
                foreground_command: None,
                detected_agent_kind,
                latest_activity,
                listening_ports,
                state: if needs_attention {
                    MultiplexerSessionState::NeedsAttention
                } else {
                    MultiplexerSessionState::Available
                },
                attached_clients: None,
                discovery_stale: false,
                executable: executable.clone(),
                herdr_session_name: None,
            })
        })
        .collect())
}

#[derive(Default)]
struct CmuxWorkspaceActivity {
    needs_attention: bool,
    latest_activity: Option<String>,
    latest_created_at: Option<String>,
    latest_index: usize,
    detected_agent_kind: Option<TerminalAgentKind>,
}

fn parse_cmux_notifications(output: &str) -> Result<HashMap<String, CmuxWorkspaceActivity>> {
    let value: Value = serde_json::from_str(output).context("invalid cmux notification JSON")?;
    let result = value.get("result").unwrap_or(&value);
    let notifications = result
        .get("notifications")
        .and_then(Value::as_array)
        .or_else(|| result.as_array())
        .context("cmux response did not contain notifications")?;
    let mut activity_by_workspace = HashMap::<String, CmuxWorkspaceActivity>::new();

    for (index, notification) in notifications.iter().enumerate() {
        let Some(workspace_id) = notification
            .get("workspace_id")
            .and_then(Value::as_str)
            .filter(|workspace_id| !workspace_id.is_empty())
        else {
            continue;
        };
        let activity = activity_by_workspace
            .entry(workspace_id.to_ascii_lowercase())
            .or_default();
        activity.needs_attention |= !notification
            .get("is_read")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        if activity.detected_agent_kind.is_none() {
            activity.detected_agent_kind = cmux_notification_detected_agent_kind(notification);
        }

        let created_at = notification
            .get("created_at")
            .and_then(Value::as_str)
            .filter(|created_at| !created_at.is_empty());
        let is_latest = match (&activity.latest_created_at, created_at) {
            (Some(previous), Some(candidate)) => candidate >= previous.as_str(),
            (None, Some(_)) => true,
            (Some(_), None) => false,
            (None, None) => index >= activity.latest_index,
        };
        if is_latest {
            activity.latest_activity = cmux_notification_activity(notification);
            activity.latest_created_at = created_at.map(str::to_owned);
            activity.latest_index = index;
        }
    }

    Ok(activity_by_workspace)
}

fn cmux_notification_activity(notification: &Value) -> Option<String> {
    ["body", "subtitle", "title", "tab_title"]
        .into_iter()
        .find_map(|field| {
            notification
                .get(field)
                .and_then(Value::as_str)
                .and_then(concise_cmux_activity)
        })
}

fn cmux_notification_detected_agent_kind(notification: &Value) -> Option<TerminalAgentKind> {
    ["tab_title", "title", "subtitle", "body"]
        .into_iter()
        .find_map(|field| {
            notification
                .get(field)
                .and_then(Value::as_str)
                .and_then(detect_terminal_agent_kind)
        })
}

fn cmux_workspace_detected_agent_kind(workspace: &Value) -> Option<TerminalAgentKind> {
    [
        "latest_conversation_message",
        "latest_submitted_message",
        "title",
        "description",
    ]
    .into_iter()
    .find_map(|field| {
        workspace
            .get(field)
            .and_then(Value::as_str)
            .and_then(detect_terminal_agent_kind)
    })
}

fn cmux_workspace_latest_activity(workspace: &Value) -> Option<String> {
    ["latest_conversation_message", "latest_submitted_message"]
        .into_iter()
        .find_map(|field| {
            workspace
                .get(field)
                .and_then(Value::as_str)
                .and_then(concise_cmux_activity)
        })
}

fn cmux_workspace_listening_ports(workspace: &Value) -> Vec<u16> {
    let mut ports = workspace
        .get("listening_ports")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|port| {
            port.as_u64()
                .and_then(|port| u16::try_from(port).ok())
                .or_else(|| port.as_str().and_then(|port| port.parse::<u16>().ok()))
        })
        .collect::<Vec<_>>();
    ports.sort_unstable();
    ports.dedup();
    ports
}

fn concise_cmux_activity(activity: &str) -> Option<String> {
    const MAX_CHARS: usize = 96;
    let activity = activity.split_whitespace().collect::<Vec<_>>().join(" ");
    if activity.is_empty() {
        None
    } else if activity.chars().count() <= MAX_CHARS {
        Some(activity)
    } else {
        Some(format!(
            "{}…",
            activity.chars().take(MAX_CHARS).collect::<String>()
        ))
    }
}

fn concise_port_label(ports: &[u16]) -> Option<String> {
    if ports.is_empty() {
        return None;
    }
    let visible_ports = ports
        .iter()
        .take(3)
        .map(|port| format!(":{port}"))
        .collect::<Vec<_>>()
        .join(", ");
    let remaining_count = ports.len().saturating_sub(3);
    Some(if remaining_count > 0 {
        format!("{visible_ports} +{remaining_count}")
    } else {
        visible_ports
    })
}

fn concise_command_stderr(output: &Output) -> String {
    let stderr = concise_discovery_message(&String::from_utf8_lossy(&output.stderr));
    if stderr.is_empty() {
        format!("command exited with {}", output.status)
    } else {
        stderr
    }
}

fn cmux_workspace_agent_label(workspace: &Value) -> Option<&str> {
    if let Some(agent) = workspace.get("agent").and_then(Value::as_str) {
        return Some(agent);
    }

    let tags = workspace.get("tags")?;
    if let Some(agent) = tags.get("agent").and_then(Value::as_str) {
        return Some(agent);
    }

    tags.as_array()?
        .iter()
        .find(|tag| tag.get("key").and_then(Value::as_str) == Some("agent"))
        .and_then(|tag| tag.get("value"))
        .and_then(Value::as_str)
}

#[derive(Clone, Debug)]
struct TmuxPaneRecord {
    session_id: String,
    session_name: String,
    attached_clients: usize,
    window_name: String,
    window_active: bool,
    pane_active: bool,
    working_directory: Option<PathBuf>,
    command: Option<String>,
    title: Option<String>,
}

fn parse_tmux_sessions(
    output: &str,
    executable: PathBuf,
) -> Result<Vec<ExternalMultiplexerSession>> {
    let mut sessions = BTreeMap::<String, TmuxPaneRecord>::new();
    let mut nonempty_line_count = 0usize;

    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }
        nonempty_line_count += 1;
        let fields = line.split(TMUX_FIELD_SEPARATOR).collect::<Vec<_>>();
        if fields.len() != 11 {
            continue;
        }

        let record = TmuxPaneRecord {
            session_id: fields[0].to_owned(),
            session_name: fields[1].to_owned(),
            attached_clients: fields[2].parse().unwrap_or_default(),
            window_name: fields[4].to_owned(),
            window_active: fields[5] == "1",
            pane_active: fields[7] == "1",
            working_directory: (!fields[8].is_empty()).then(|| PathBuf::from(fields[8])),
            command: (!fields[9].is_empty()).then(|| fields[9].to_owned()),
            title: (!fields[10].is_empty()).then(|| fields[10].to_owned()),
        };

        sessions
            .entry(record.session_id.clone())
            .and_modify(|current| {
                let current_score =
                    usize::from(current.window_active) + usize::from(current.pane_active);
                let candidate_score =
                    usize::from(record.window_active) + usize::from(record.pane_active);
                if candidate_score > current_score {
                    *current = record.clone();
                }
            })
            .or_insert(record);
    }

    if nonempty_line_count > 0 && sessions.is_empty() {
        anyhow::bail!("tmux returned an unsupported pane-list format")
    }

    Ok(sessions
        .into_values()
        .map(|record| {
            let command = record.command.clone();
            let detected_agent_kind = command
                .as_deref()
                .and_then(detect_terminal_agent_command)
                .or_else(|| record.title.as_deref().and_then(detect_terminal_agent_kind));
            let state = if record.attached_clients > 0 {
                MultiplexerSessionState::Attached
            } else {
                MultiplexerSessionState::Detached
            };

            ExternalMultiplexerSession {
                id: format!("tmux:{}", record.session_id),
                kind: MultiplexerKind::Tmux,
                target: record.session_id,
                title: record.session_name,
                owner_context: Some(record.window_name),
                working_directory: record.working_directory,
                foreground_command: command,
                detected_agent_kind,
                latest_activity: None,
                listening_ports: Vec::new(),
                state,
                attached_clients: Some(record.attached_clients),
                discovery_stale: false,
                executable: executable.clone(),
                herdr_session_name: None,
            }
        })
        .collect())
}

async fn scan_herdr_sessions(executor: &BackgroundExecutor) -> Result<MultiplexerScanOutcome> {
    let source_deadline = executor.timer(HERDR_SOURCE_TIMEOUT).shared();
    let executable_discovery =
        first_available_program(&["/opt/homebrew/bin/herdr", "/usr/local/bin/herdr", "herdr"]);
    let executable_deadline = source_deadline.clone();
    futures::pin_mut!(executable_discovery, executable_deadline);
    let executable = match futures::future::select(executable_discovery, executable_deadline).await
    {
        futures::future::Either::Left((result, _)) => result?,
        futures::future::Either::Right(_) => anyhow::bail!(
            "Herdr discovery reached its source-wide deadline after {:?}",
            HERDR_SOURCE_TIMEOUT
        ),
    };
    let Some(executable) = executable else {
        return Ok(MultiplexerScanOutcome::MissingExecutable);
    };

    let session_discovery = list_running_herdr_sessions(&executable);
    let session_deadline = source_deadline.clone();
    futures::pin_mut!(session_discovery, session_deadline);
    let server_names = match futures::future::select(session_discovery, session_deadline).await {
        futures::future::Either::Left((result, _)) => result?,
        futures::future::Either::Right(_) => anyhow::bail!(
            "Herdr discovery reached its source-wide deadline after {:?}",
            HERDR_SOURCE_TIMEOUT
        ),
    };

    ensure_herdr_source_deadline_open(source_deadline.clone()).await?;
    if server_names.is_empty() {
        return Ok(MultiplexerScanOutcome::Available(
            AvailableMultiplexerScan::complete(Vec::new()),
        ));
    }

    let expected_server_names = server_names.clone();
    let endpoint_scans = futures::stream::iter(server_names.into_iter().map(|server_name| {
        scan_herdr_endpoint_with_timeout(executable.clone(), server_name, executor)
    }))
    .buffer_unordered(HERDR_MAX_CONCURRENT_ENDPOINT_QUERIES);
    let mut endpoint_scans = collect_herdr_endpoint_results_until_deadline(
        endpoint_scans,
        expected_server_names,
        source_deadline,
    )
    .await;
    endpoint_scans.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sessions = Vec::new();
    let mut warnings = Vec::new();
    let mut successful_endpoint_count = 0usize;
    for (server_name, result) in endpoint_scans {
        let server_label = server_name.clone().unwrap_or_else(|| "default".to_owned());
        match result {
            Ok(scanned_sessions) => {
                sessions.extend(scanned_sessions);
                successful_endpoint_count += 1;
            }
            Err(error) => warnings.push(MultiplexerScanWarning {
                herdr_session_name: server_name,
                summary: format!("Herdr session discovery failed for {server_label}: {error:#}"),
            }),
        }
    }

    Ok(MultiplexerScanOutcome::Available(
        AvailableMultiplexerScan {
            sessions,
            warnings,
            successful_endpoint_count,
        },
    ))
}

async fn list_running_herdr_sessions(executable: &std::path::Path) -> Result<Vec<Option<String>>> {
    let mut command = util::command::new_command(executable);
    command
        .args(["session", "list", "--json"])
        .kill_on_drop(true);
    let output = command
        .output()
        .await
        .context("run Herdr session discovery")?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let detail = stderr.trim();
        if detail.is_empty() {
            anyhow::bail!("Herdr session discovery exited with {}", output.status);
        }
        anyhow::bail!("Herdr session discovery failed: {detail}");
    }

    parse_running_herdr_sessions(&String::from_utf8_lossy(&output.stdout))
}

fn parse_running_herdr_sessions(output: &str) -> Result<Vec<Option<String>>> {
    let value: Value = serde_json::from_str(output).context("invalid Herdr session-list JSON")?;
    let result = value.get("result").unwrap_or(&value);
    let sessions = result
        .get("sessions")
        .and_then(Value::as_array)
        .or_else(|| result.as_array())
        .context("Herdr session-list response did not contain a sessions array")?;

    let mut server_names = sessions
        .iter()
        .map(|session| {
            let running = session
                .get("running")
                .and_then(Value::as_bool)
                .context("Herdr session-list entry did not contain a running state")?;
            if !running {
                return Ok(None);
            }

            if session
                .get("default")
                .and_then(Value::as_bool)
                .unwrap_or(false)
            {
                return Ok(Some(None));
            }

            let name = session
                .get("name")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|name| !name.is_empty())
                .context("running Herdr session did not contain a name")?;
            Ok(Some(Some(name.to_owned())))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    server_names.sort();
    server_names.dedup();
    Ok(server_names)
}

async fn ensure_herdr_source_deadline_open(
    source_deadline: impl Future<Output = ()>,
) -> Result<()> {
    let deadline = source_deadline.fuse();
    let continue_scan = futures::future::ready(()).fuse();
    futures::pin_mut!(deadline, continue_scan);
    futures::select_biased! {
        _ = deadline => anyhow::bail!(
            "Herdr discovery reached its source-wide deadline after {:?}",
            HERDR_SOURCE_TIMEOUT
        ),
        _ = continue_scan => Ok(()),
    }
}

async fn collect_herdr_endpoint_results_until_deadline<T>(
    endpoint_scans: impl Stream<Item = (Option<String>, Result<T>)>,
    mut pending_server_names: Vec<Option<String>>,
    source_deadline: impl Future<Output = ()>,
) -> Vec<(Option<String>, Result<T>)> {
    let mut endpoint_scans = Box::pin(endpoint_scans.fuse());
    let deadline = source_deadline.fuse();
    futures::pin_mut!(deadline);
    let mut results = Vec::new();

    loop {
        futures::select_biased! {
            _ = deadline => break,
            result = endpoint_scans.next() => {
                let Some(result) = result else {
                    break;
                };
                pending_server_names.retain(|server_name| server_name != &result.0);
                results.push(result);
            }
        }
    }

    drop(endpoint_scans);
    results.extend(pending_server_names.into_iter().map(|server_name| {
        (
            server_name,
            Err(anyhow::anyhow!(
                "Herdr discovery reached its source-wide deadline after {:?}",
                HERDR_SOURCE_TIMEOUT
            )),
        )
    }));
    results
}

async fn scan_herdr_endpoint_with_timeout(
    executable: PathBuf,
    server_name: Option<String>,
    executor: &BackgroundExecutor,
) -> (Option<String>, Result<Vec<ExternalMultiplexerSession>>) {
    let result = run_bounded_herdr_endpoint_query(
        scan_herdr_endpoint(executable, server_name.clone()),
        executor,
    )
    .await;
    (server_name, result)
}

async fn run_bounded_herdr_endpoint_query<T>(
    query: impl Future<Output = Result<T>>,
    executor: &BackgroundExecutor,
) -> Result<T> {
    run_bounded_multiplexer_operation(
        "Herdr endpoint query",
        query,
        executor,
        HERDR_ENDPOINT_TIMEOUT,
    )
    .await
}

async fn scan_herdr_endpoint(
    executable: PathBuf,
    server_name: Option<String>,
) -> Result<Vec<ExternalMultiplexerSession>> {
    let mut args = Vec::new();
    if let Some(name) = &server_name {
        args.extend(["--session".to_owned(), name.clone()]);
    }
    args.extend(["api".to_owned(), "snapshot".to_owned()]);

    let mut command = util::command::new_command(&executable);
    command.args(&args).kill_on_drop(true);
    let output = command.output().await.context("run Herdr snapshot query")?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let detail = stderr.trim();
        if detail.is_empty() {
            anyhow::bail!("Herdr snapshot query exited with {}", output.status);
        }
        anyhow::bail!("{detail}");
    }

    parse_herdr_snapshot(
        &String::from_utf8_lossy(&output.stdout),
        executable,
        server_name,
    )
}

fn parse_herdr_snapshot(
    output: &str,
    executable: PathBuf,
    server_name: Option<String>,
) -> Result<Vec<ExternalMultiplexerSession>> {
    let value: Value = serde_json::from_str(output).context("invalid Herdr snapshot JSON")?;
    let snapshot = value
        .pointer("/result/snapshot")
        .context("Herdr snapshot response did not contain result.snapshot")?;

    let workspace_labels = snapshot
        .get("workspaces")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|workspace| {
            Some((
                workspace.get("workspace_id")?.as_str()?.to_owned(),
                workspace.get("label")?.as_str()?.to_owned(),
            ))
        })
        .collect::<HashMap<_, _>>();
    let tab_labels = snapshot
        .get("tabs")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|tab| {
            Some((
                tab.get("tab_id")?.as_str()?.to_owned(),
                tab.get("label")?.as_str()?.to_owned(),
            ))
        })
        .collect::<HashMap<_, _>>();

    let panes = snapshot
        .get("panes")
        .and_then(Value::as_array)
        .context("Herdr snapshot did not contain a panes array")?;

    let server_key = server_name.as_deref().unwrap_or("default");
    let sessions = panes
        .iter()
        .filter_map(|pane| {
            let pane_id = pane.get("pane_id")?.as_str()?.to_owned();
            let terminal_id = pane
                .get("terminal_id")
                .and_then(Value::as_str)
                .map(str::to_owned);
            let workspace_id = pane.get("workspace_id")?.as_str()?;
            let tab_id = pane.get("tab_id")?.as_str()?;
            let workspace_label = workspace_labels.get(workspace_id).cloned();
            let tab_label = tab_labels.get(tab_id).cloned();
            let agent = pane
                .get("display_agent")
                .and_then(Value::as_str)
                .or_else(|| pane.get("agent").and_then(Value::as_str));
            let title = pane
                .get("title")
                .and_then(Value::as_str)
                .or_else(|| pane.get("terminal_title_stripped").and_then(Value::as_str))
                .or_else(|| pane.get("label").and_then(Value::as_str))
                .or(agent)
                .map(str::to_owned)
                .or(tab_label)
                .or(workspace_label.clone())
                .unwrap_or_else(|| "Herdr terminal".to_owned());
            let foreground_command = agent.map(str::to_owned).or_else(|| {
                pane.get("terminal_title_stripped")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
            });
            let detected_agent_kind = agent.and_then(detect_terminal_agent_kind).or_else(|| {
                foreground_command
                    .as_deref()
                    .and_then(detect_terminal_agent_command)
            });
            let target = if detected_agent_kind.is_some() {
                pane_id.clone()
            } else {
                terminal_id.unwrap_or_else(|| pane_id.clone())
            };
            let working_directory = pane
                .get("foreground_cwd")
                .and_then(Value::as_str)
                .or_else(|| pane.get("cwd").and_then(Value::as_str))
                .filter(|path| !path.trim().is_empty())
                .map(PathBuf::from);
            let state = match pane.get("agent_status").and_then(Value::as_str) {
                Some("idle") => MultiplexerSessionState::Idle,
                Some("working") => MultiplexerSessionState::Working,
                Some("blocked") => MultiplexerSessionState::NeedsAttention,
                Some("done") => MultiplexerSessionState::Completed,
                _ if detected_agent_kind.is_none() => MultiplexerSessionState::Available,
                _ => MultiplexerSessionState::Unknown,
            };

            Some(ExternalMultiplexerSession {
                id: format!("herdr:{server_key}:{pane_id}"),
                kind: MultiplexerKind::Herdr,
                target,
                title,
                owner_context: workspace_label,
                working_directory,
                foreground_command,
                detected_agent_kind,
                latest_activity: None,
                listening_ports: Vec::new(),
                state,
                attached_clients: None,
                discovery_stale: false,
                executable: executable.clone(),
                herdr_session_name: server_name.clone(),
            })
        })
        .collect::<Vec<_>>();

    if !panes.is_empty() && sessions.is_empty() {
        anyhow::bail!("Herdr returned panes in an unsupported snapshot format");
    }

    Ok(sessions)
}

async fn run_first_available(
    programs: &[&str],
    args: &[&str],
) -> Result<Option<(PathBuf, Output)>> {
    for program in programs {
        let mut command = util::command::new_command(program);
        command.args(args).kill_on_drop(true);
        match command.output().await {
            Ok(output) => return Ok(Some((PathBuf::from(program), output))),
            Err(error) if error.kind() == ErrorKind::NotFound => continue,
            Err(error) => return Err(error).with_context(|| format!("failed to run {program}")),
        }
    }
    Ok(None)
}

async fn run_multiplexer_command(
    executable: &PathBuf,
    args: &[&str],
    operation: &str,
) -> Result<Output> {
    let mut command = util::command::new_command(executable);
    command.args(args).kill_on_drop(true);
    command
        .output()
        .await
        .with_context(|| format!("failed to run {operation}"))
}

async fn first_available_program(programs: &[&str]) -> Result<Option<PathBuf>> {
    for program in programs {
        let mut command = util::command::new_command(program);
        command.arg("--version").kill_on_drop(true);
        match command.output().await {
            Ok(_) => return Ok(Some(PathBuf::from(program))),
            Err(error) if error.kind() == ErrorKind::NotFound => continue,
            Err(error) => return Err(error).with_context(|| format!("failed to run {program}")),
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn external_session(kind: MultiplexerKind, id: &str) -> ExternalMultiplexerSession {
        ExternalMultiplexerSession {
            id: id.to_owned(),
            kind,
            target: id.to_owned(),
            title: id.to_owned(),
            owner_context: None,
            working_directory: None,
            foreground_command: None,
            detected_agent_kind: None,
            latest_activity: None,
            listening_ports: Vec::new(),
            state: MultiplexerSessionState::Available,
            attached_clients: None,
            discovery_stale: false,
            executable: PathBuf::from(kind.display_name()),
            herdr_session_name: None,
        }
    }

    fn complete_scan(sessions: Vec<ExternalMultiplexerSession>) -> Result<MultiplexerScanOutcome> {
        Ok(MultiplexerScanOutcome::Available(
            AvailableMultiplexerScan::complete(sessions),
        ))
    }

    #[test]
    fn a_failed_integration_scan_preserves_only_its_last_known_sessions() {
        let previous_sessions = vec![
            external_session(MultiplexerKind::Tmux, "old-tmux"),
            external_session(MultiplexerKind::Herdr, "old-herdr"),
            external_session(MultiplexerKind::Cmux, "old-cmux"),
        ];
        let successful_sources = HashSet::from([MultiplexerKind::Herdr]);
        let (sessions, source_issues, source_statuses, newly_successful_sources) =
            reconcile_external_multiplexer_sessions(
                &previous_sessions,
                ExternalMultiplexerScan {
                    tmux: complete_scan(vec![external_session(MultiplexerKind::Tmux, "new-tmux")]),
                    herdr: Err(anyhow::anyhow!("Herdr is temporarily unavailable")),
                    cmux: complete_scan(Vec::new()),
                },
                &successful_sources,
            );

        assert_eq!(
            sessions
                .iter()
                .map(|session| session.id.as_str())
                .collect::<Vec<_>>(),
            ["old-herdr", "new-tmux"]
        );
        assert!(sessions[0].discovery_stale);
        assert_eq!(sessions[0].state_label(), "available · last known");
        assert!(!sessions[1].discovery_stale);
        assert_eq!(source_issues.len(), 1);
        assert_eq!(source_issues[0].kind, MultiplexerKind::Herdr);
        assert_eq!(source_issues[0].summary, "Herdr is temporarily unavailable");
        assert!(source_issues[0].had_successful_scan);
        assert_eq!(source_statuses.len(), 3);
        assert_eq!(
            source_statuses[0].availability,
            MultiplexerSourceAvailability::Ready
        );
        assert_eq!(
            source_statuses[1].availability,
            MultiplexerSourceAvailability::Failed
        );
        assert_eq!(source_statuses[1].session_count, 1);
        assert_eq!(
            source_statuses[2].availability,
            MultiplexerSourceAvailability::AvailableEmpty
        );
        assert_eq!(
            newly_successful_sources,
            [MultiplexerKind::Tmux, MultiplexerKind::Cmux]
        );
    }

    #[test]
    fn source_availability_distinguishes_missing_empty_and_ready() {
        let previous_sessions = vec![external_session(MultiplexerKind::Tmux, "old-tmux")];
        let (sessions, source_issues, source_statuses, newly_successful_sources) =
            reconcile_external_multiplexer_sessions(
                &previous_sessions,
                ExternalMultiplexerScan {
                    tmux: Ok(MultiplexerScanOutcome::MissingExecutable),
                    herdr: complete_scan(Vec::new()),
                    cmux: complete_scan(vec![external_session(
                        MultiplexerKind::Cmux,
                        "ready-cmux",
                    )]),
                },
                &HashSet::from([MultiplexerKind::Tmux]),
            );

        assert!(source_issues.is_empty());
        assert_eq!(
            sessions
                .iter()
                .map(|session| session.id.as_str())
                .collect::<Vec<_>>(),
            ["ready-cmux", "old-tmux"]
        );
        assert!(!sessions[0].discovery_stale);
        assert!(sessions[1].discovery_stale);
        assert_eq!(
            source_statuses
                .iter()
                .map(|status| status.availability)
                .collect::<Vec<_>>(),
            [
                MultiplexerSourceAvailability::MissingExecutable,
                MultiplexerSourceAvailability::AvailableEmpty,
                MultiplexerSourceAvailability::Ready,
            ]
        );
        assert_eq!(source_statuses[0].session_count, 1);
        assert!(source_statuses[0].had_successful_scan);
        assert_eq!(source_statuses[1].session_count, 0);
        assert_eq!(source_statuses[2].session_count, 1);
        assert_eq!(
            newly_successful_sources,
            [MultiplexerKind::Herdr, MultiplexerKind::Cmux]
        );
    }

    #[test]
    fn a_failed_herdr_endpoint_preserves_only_that_exact_endpoints_sessions() {
        let mut previous_failed = external_session(MultiplexerKind::Herdr, "herdr:alpha:old");
        previous_failed.herdr_session_name = Some("alpha".to_owned());
        let mut previous_healthy =
            external_session(MultiplexerKind::Herdr, "herdr:alpha:child:old");
        previous_healthy.herdr_session_name = Some("alpha:child".to_owned());
        let mut current_healthy = external_session(MultiplexerKind::Herdr, "herdr:alpha:child:new");
        current_healthy.herdr_session_name = Some("alpha:child".to_owned());
        let previous_sessions = vec![previous_failed, previous_healthy];
        let (sessions, source_issues, source_statuses, newly_successful_sources) =
            reconcile_external_multiplexer_sessions(
                &previous_sessions,
                ExternalMultiplexerScan {
                    tmux: Ok(MultiplexerScanOutcome::MissingExecutable),
                    herdr: Ok(MultiplexerScanOutcome::Available(
                        AvailableMultiplexerScan {
                            sessions: vec![current_healthy],
                            warnings: vec![MultiplexerScanWarning {
                                herdr_session_name: Some("alpha".to_owned()),
                                summary: "Herdr session discovery failed for alpha: unavailable"
                                    .to_owned(),
                            }],
                            successful_endpoint_count: 1,
                        },
                    )),
                    cmux: Ok(MultiplexerScanOutcome::MissingExecutable),
                },
                &HashSet::new(),
            );

        assert_eq!(
            sessions
                .iter()
                .map(|session| (session.id.as_str(), session.discovery_stale))
                .collect::<Vec<_>>(),
            [("herdr:alpha:child:new", false), ("herdr:alpha:old", true),]
        );
        assert_eq!(source_issues.len(), 1);
        assert_eq!(source_issues[0].kind, MultiplexerKind::Herdr);
        assert!(source_issues[0].had_successful_scan);
        assert_eq!(
            source_statuses[1].availability,
            MultiplexerSourceAvailability::Failed
        );
        assert_eq!(source_statuses[1].session_count, 2);
        assert!(source_statuses[1].had_successful_scan);
        assert_eq!(newly_successful_sources, [MultiplexerKind::Herdr]);
    }

    #[test]
    fn discovery_timeout_finishes_before_the_next_refresh_interval() {
        assert!(MULTIPLEXER_COMMAND_TIMEOUT < MULTIPLEXER_REFRESH_INTERVAL);
        assert!(HERDR_SOURCE_TIMEOUT < MULTIPLEXER_REFRESH_INTERVAL);
        assert!(HERDR_ENDPOINT_TIMEOUT < HERDR_SOURCE_TIMEOUT);
    }

    #[gpui::test]
    async fn an_elapsed_herdr_source_deadline_cannot_report_authoritative_empty() {
        let error = ensure_herdr_source_deadline_open(futures::future::ready(()))
            .await
            .expect_err("an elapsed source deadline should fail before the empty result");
        assert!(error.to_string().contains("source-wide deadline"));

        ensure_herdr_source_deadline_open(futures::future::pending())
            .await
            .expect("an open source deadline should allow the scan to continue");
    }

    #[gpui::test]
    async fn a_hung_herdr_endpoint_does_not_hide_a_healthy_endpoint(
        background_executor: BackgroundExecutor,
    ) {
        let hung = run_bounded_herdr_endpoint_query(
            futures::future::pending::<Result<&'static str>>(),
            &background_executor,
        );
        let healthy = run_bounded_herdr_endpoint_query(
            async { Ok::<_, anyhow::Error>("healthy") },
            &background_executor,
        );
        let (hung, healthy) = futures::join!(hung, healthy);

        let error = hung.expect_err("hung endpoint should reach its own deadline");
        assert!(error.to_string().contains("endpoint query timed out"));
        assert_eq!(
            healthy.expect("healthy endpoint should complete"),
            "healthy"
        );
    }

    #[gpui::test]
    async fn herdr_source_deadline_keeps_completed_endpoint_results(
        background_executor: BackgroundExecutor,
    ) {
        let healthy = async {
            (
                Some("healthy".to_owned()),
                Ok::<_, anyhow::Error>("healthy"),
            )
        }
        .boxed();
        let hung = futures::future::pending::<(Option<String>, Result<&'static str>)>().boxed();
        let endpoint_scans = futures::stream::iter(vec![hung, healthy]).buffer_unordered(2);
        let mut results = collect_herdr_endpoint_results_until_deadline(
            endpoint_scans,
            vec![Some("hung".to_owned()), Some("healthy".to_owned())],
            background_executor.timer(Duration::from_millis(1)),
        )
        .await;
        results.sort_by(|left, right| left.0.cmp(&right.0));

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0.as_deref(), Some("healthy"));
        assert_eq!(
            results[0]
                .1
                .as_ref()
                .expect("healthy endpoint should be preserved"),
            &"healthy"
        );
        assert_eq!(results[1].0.as_deref(), Some("hung"));
        let error = results[1]
            .1
            .as_ref()
            .expect_err("unfinished endpoint should receive a source-timeout warning");
        assert!(error.to_string().contains("source-wide deadline"));
    }

    #[test]
    fn tmux_only_treats_canonical_missing_server_failures_as_empty() {
        assert!(tmux_failure_is_no_server(
            Some(1),
            "no server running on /private/tmp/tmux-501/default"
        ));
        assert!(tmux_failure_is_no_server(
            Some(1),
            "error connecting to /tmp/tmux/default (No such file or directory)"
        ));
        assert!(!tmux_failure_is_no_server(
            Some(1),
            "error connecting to /tmp/tmux/default (Permission denied)"
        ));
        assert!(!tmux_failure_is_no_server(
            Some(2),
            "no server running on /tmp/tmux/default"
        ));
    }

    #[test]
    fn parses_tmux_sessions_by_stable_session_id_and_prefers_active_pane() {
        let separator = TMUX_FIELD_SEPARATOR;
        let output = format!(
            "$1{separator}compiler{separator}0{separator}@1{separator}editor{separator}1{separator}%1{separator}0{separator}/tmp/old{separator}zsh{separator}old\n\
             $1{separator}compiler{separator}0{separator}@1{separator}editor{separator}1{separator}%2{separator}1{separator}/tmp/project{separator}codex{separator}agent"
        );
        let sessions = parse_tmux_sessions(&output, PathBuf::from("/opt/homebrew/bin/tmux"))
            .expect("pane list should parse");

        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id, "tmux:$1");
        assert_eq!(sessions[0].working_directory, Some("/tmp/project".into()));
        assert_eq!(sessions[0].state, MultiplexerSessionState::Detached);
        assert_eq!(
            sessions[0].open_command().args,
            ["attach-session", "-t", "$1"]
        );
    }

    #[test]
    fn rejects_nonempty_tmux_output_when_no_rows_match_the_contract() {
        let error = parse_tmux_sessions(
            "pane output from an incompatible tmux format",
            PathBuf::from("tmux"),
        )
        .expect_err("an incompatible schema must preserve last-known sessions");

        assert!(error.to_string().contains("unsupported pane-list format"));
    }

    #[test]
    fn parses_herdr_snapshot_with_agent_attention_and_named_session() {
        let output = r#"{
          "id": "snapshot",
          "result": {
            "type": "session_snapshot",
            "snapshot": {
              "version": "0.7.5",
              "protocol": 17,
              "workspaces": [{
                "workspace_id": "ws-1",
                "number": 1,
                "label": "compiler",
                "focused": true,
                "pane_count": 1,
                "tab_count": 1,
                "active_tab_id": "tab-1",
                "agent_status": "blocked"
              }],
              "tabs": [{
                "tab_id": "tab-1",
                "workspace_id": "ws-1",
                "number": 1,
                "label": "agent",
                "focused": true,
                "pane_count": 1,
                "agent_status": "blocked"
              }],
              "panes": [{
                "pane_id": "pane-1",
                "terminal_id": "terminal-1",
                "workspace_id": "ws-1",
                "tab_id": "tab-1",
                "focused": true,
                "agent_status": "blocked",
                "revision": 3,
                "display_agent": "Claude Code",
                "foreground_cwd": "/tmp/compiler",
                "title": "Fix parser"
              }],
              "layouts": [],
              "agents": []
            }
          }
        }"#;

        let sessions = parse_herdr_snapshot(
            output,
            PathBuf::from("/opt/homebrew/bin/herdr"),
            Some("team".to_owned()),
        )
        .expect("snapshot should parse");

        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id, "herdr:team:pane-1");
        assert_eq!(sessions[0].state, MultiplexerSessionState::NeedsAttention);
        assert_eq!(
            sessions[0].open_command().args,
            ["--session", "team", "agent", "attach", "pane-1"]
        );
    }

    #[test]
    fn opens_non_agent_herdr_panes_with_the_interactive_terminal_client() {
        let output = r#"{
          "result": {
            "snapshot": {
              "workspaces": [{"workspace_id": "ws-1", "label": "compiler"}],
              "tabs": [{"tab_id": "tab-1", "label": "shell"}],
              "panes": [{
                "pane_id": "pane-1",
                "terminal_id": "terminal-1",
                "workspace_id": "ws-1",
                "tab_id": "tab-1",
                "cwd": "/tmp/compiler"
              }]
            }
          }
        }"#;

        let sessions = parse_herdr_snapshot(output, PathBuf::from("/opt/homebrew/bin/herdr"), None)
            .expect("snapshot should parse");

        assert_eq!(sessions.len(), 1);
        assert_eq!(
            sessions[0].open_command().args,
            ["terminal", "attach", "terminal-1"]
        );
        assert_eq!(sessions[0].state, MultiplexerSessionState::Available);
        assert_eq!(sessions[0].working_directory, Some("/tmp/compiler".into()));
    }

    #[test]
    fn ignores_empty_herdr_working_directories() {
        let output = r#"{
          "result": {
            "snapshot": {
              "workspaces": [{"workspace_id": "ws-1", "label": "compiler"}],
              "tabs": [{"tab_id": "tab-1", "label": "shell"}],
              "panes": [{
                "pane_id": "pane-1",
                "terminal_id": "terminal-1",
                "workspace_id": "ws-1",
                "tab_id": "tab-1",
                "cwd": "   "
              }]
            }
          }
        }"#;

        let sessions = parse_herdr_snapshot(output, PathBuf::from("herdr"), None)
            .expect("snapshot should parse");
        assert_eq!(sessions[0].working_directory, None);
    }

    #[test]
    fn rejects_herdr_snapshots_without_a_panes_array() {
        let output = r#"{"result":{"snapshot":{"workspaces":[],"tabs":[]}}}"#;
        let error = parse_herdr_snapshot(output, PathBuf::from("herdr"), None)
            .expect_err("a missing pane schema must preserve last-known sessions");

        assert!(error.to_string().contains("panes array"));
    }

    #[test]
    fn parses_running_herdr_sessions_from_the_cli_registry() {
        let output = r#"{
          "sessions": [
            {"name": "default", "default": true, "running": true},
            {"name": "team", "default": false, "running": true},
            {"name": "stopped", "default": false, "running": false},
            {"name": "team", "default": false, "running": true}
          ]
        }"#;

        assert_eq!(
            parse_running_herdr_sessions(output).expect("session list should parse"),
            [None, Some("team".to_owned())]
        );
    }

    #[test]
    fn rejects_running_herdr_sessions_without_a_name() {
        let error =
            parse_running_herdr_sessions(r#"{"sessions":[{"default":false,"running":true}]}"#)
                .expect_err("an incomplete live session must not erase last-known sessions");

        assert!(error.to_string().contains("did not contain a name"));
    }

    #[test]
    fn preserves_legacy_cmux_workspace_metadata() {
        let output = r#"{
          "id": "workspace-list",
          "ok": true,
          "result": {
            "window_id": "window-id",
            "window_ref": "window:1",
            "workspaces": [{
              "id": "workspace-id",
              "ref": "workspace:1",
              "title": "parser",
              "description": "Reviewing agent changes",
              "current_directory": "/tmp/parser",
              "unread_count": 2,
              "tags": { "agent": "Codex" },
              "selected": false
            }]
          }
        }"#;

        let sessions = parse_cmux_workspaces(output, None, PathBuf::from("/opt/homebrew/bin/cmux"))
            .expect("workspace list should parse");

        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id, "cmux:workspace-id");
        assert_eq!(sessions[0].working_directory, Some("/tmp/parser".into()));
        assert_eq!(sessions[0].state, MultiplexerSessionState::NeedsAttention);
        assert_eq!(
            sessions[0].detected_agent_kind,
            Some(TerminalAgentKind::Codex)
        );
        assert_eq!(
            sessions[0].open_command(),
            ExternalSessionCommand {
                program: "/opt/homebrew/bin/cmux".to_owned(),
                args: vec![
                    "select-workspace".to_owned(),
                    "--workspace".to_owned(),
                    "workspace-id".to_owned(),
                ],
                label: "Open parser in cmux".to_owned(),
                mode: ExternalSessionOpenMode::RevealExternal,
            }
        );
    }

    #[test]
    fn correlates_current_cmux_workspace_activity_notifications_and_ports() {
        let workspaces = r#"[{
          "id": "9B2C",
          "ref": "workspace:2",
          "title": "compiler",
          "description": "Agent review",
          "current_directory": "/tmp/compiler",
          "listening_ports": [5173, "3000", 5173, 70000],
          "latest_conversation_message": "Codex is checking the patch",
          "latest_submitted_message": "Review the latest changes",
          "selected": false
        }]"#;
        let notifications = r#"[{
          "id": "notification-1",
          "workspace_id": "9b2c",
          "is_read": false,
          "title": "Codex needs input",
          "body": "Approve the proposed command",
          "tab_title": "codex",
          "created_at": "2026-08-01T09:00:00Z"
        }, {
          "id": "notification-2",
          "workspace_id": "unrelated",
          "is_read": false,
          "body": "Do not leak across Workspaces",
          "created_at": "2026-08-01T09:01:00Z"
        }]"#;

        let sessions = parse_cmux_workspaces(
            workspaces,
            Some(notifications),
            PathBuf::from("/Applications/cmux.app/Contents/Resources/bin/cmux"),
        )
        .expect("current workspace and notification lists should correlate");

        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].state, MultiplexerSessionState::NeedsAttention);
        assert_eq!(
            sessions[0].detected_agent_kind,
            Some(TerminalAgentKind::Codex)
        );
        assert_eq!(
            sessions[0].latest_activity.as_deref(),
            Some("Approve the proposed command")
        );
        assert_eq!(sessions[0].listening_ports, [3000, 5173]);
        assert_eq!(sessions[0].location_label(), "/tmp/compiler · :3000, :5173");
    }

    #[test]
    fn cmux_is_the_first_external_workspace_choice() {
        assert!(
            MultiplexerKind::Cmux.workspace_priority()
                < MultiplexerKind::Herdr.workspace_priority()
        );
        assert!(
            MultiplexerKind::Herdr.workspace_priority()
                < MultiplexerKind::Tmux.workspace_priority()
        );
    }
}
