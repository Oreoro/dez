use std::{
    collections::{BTreeMap, HashMap},
    fs,
    io::ErrorKind,
    path::PathBuf,
    process::{Command, Output},
    time::Duration,
};

use anyhow::{Context as _, Result};
use gpui::{App, AppContext as _, Context, Entity, Global, Task};
use paths::APP_NAME;
use serde_json::Value;

use crate::terminal_thread_metadata_store::{
    TerminalAgentKind, detect_terminal_agent_command, detect_terminal_agent_kind,
};

const MULTIPLEXER_REFRESH_INTERVAL: Duration = Duration::from_secs(5);
// tmux sanitizes control characters when Dez is launched from Finder's
// minimal, locale-free environment. Keep this printable so the exact same
// parser works from a shell, Finder, and a signed application bundle.
const TMUX_FIELD_SEPARATOR: &str = "|:dez:|";
const TMUX_FORMAT: &str = "#{session_id}|:dez:|#{session_name}|:dez:|#{session_attached}|:dez:|#{window_id}|:dez:|#{window_name}|:dez:|#{window_active}|:dez:|#{pane_id}|:dez:|#{pane_active}|:dez:|#{pane_current_path}|:dez:|#{pane_current_command}|:dez:|#{pane_title}";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MultiplexerKind {
    Tmux,
    Herdr,
    Cmux,
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
    pub state: MultiplexerSessionState,
    pub attached_clients: Option<usize>,
    executable: PathBuf,
    herdr_session_name: Option<String>,
}

impl ExternalMultiplexerSession {
    pub fn source_label(&self) -> String {
        self.owner_context
            .as_ref()
            .map(|context| format!("{} · {context}", self.kind.display_name()))
            .unwrap_or_else(|| self.kind.display_name().to_owned())
    }

    pub fn state_label(&self) -> String {
        match self.attached_clients {
            Some(1) => format!("{} · 1 client", self.state.label()),
            Some(count) if count > 1 => format!("{} · {count} clients", self.state.label()),
            _ => self.state.label().to_owned(),
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
    refreshing: bool,
    _refresh_task: Task<()>,
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

    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        if self.refreshing {
            return;
        }

        self.refreshing = true;
        cx.notify();
        cx.spawn(async move |this, cx| {
            let scan = cx
                .background_executor()
                .spawn(async { scan_external_multiplexer_sessions() })
                .await;
            if let Err(error) = this.update(cx, |store, cx| {
                store.refreshing = false;
                store.apply_scan(scan, cx);
                cx.notify();
            }) {
                log::debug!("external terminal refresh finished after its store closed: {error:#}");
            }
        })
        .detach();
    }

    fn apply_scan(
        &mut self,
        scan: Result<Vec<ExternalMultiplexerSession>>,
        cx: &mut Context<Self>,
    ) {
        let sessions = match scan {
            Ok(sessions) => sessions,
            Err(error) => {
                log::debug!("failed to discover external terminal sessions: {error:#}");
                return;
            }
        };
        if self.sessions != sessions {
            self.sessions = sessions;
            cx.notify();
        }
    }

    fn new(cx: &mut Context<Self>) -> Self {
        let refresh_task = cx.spawn(async move |this, cx| {
            loop {
                let scan = cx
                    .background_executor()
                    .spawn(async { scan_external_multiplexer_sessions() })
                    .await;

                if this
                    .update(cx, |store, cx| {
                        store.apply_scan(scan, cx);
                    })
                    .is_err()
                {
                    break;
                }

                cx.background_executor()
                    .timer(MULTIPLEXER_REFRESH_INTERVAL)
                    .await;
            }
        });

        Self {
            sessions: Vec::new(),
            refreshing: false,
            _refresh_task: refresh_task,
        }
    }
}

pub fn init(cx: &mut App) {
    MultiplexerSessionStore::init_global(cx);
}

fn scan_external_multiplexer_sessions() -> Result<Vec<ExternalMultiplexerSession>> {
    let mut sessions = match scan_tmux_sessions() {
        Ok(sessions) => sessions,
        Err(error) => {
            log::debug!("tmux discovery failed without changing the session: {error:#}");
            Vec::new()
        }
    };
    match scan_herdr_sessions() {
        Ok(herdr_sessions) => sessions.extend(herdr_sessions),
        Err(error) => {
            log::debug!("Herdr discovery failed without changing the session: {error:#}");
        }
    }
    match scan_cmux_workspaces() {
        Ok(cmux_workspaces) => sessions.extend(cmux_workspaces),
        Err(error) => {
            log::debug!("cmux discovery failed without changing the session: {error:#}");
        }
    }
    sessions.sort_by(|left, right| {
        left.kind
            .workspace_priority()
            .cmp(&right.kind.workspace_priority())
            .then_with(|| left.title.cmp(&right.title))
            .then_with(|| left.id.cmp(&right.id))
    });
    Ok(sessions)
}

fn scan_tmux_sessions() -> Result<Vec<ExternalMultiplexerSession>> {
    let Some((executable, output)) = run_first_available(
        &["/opt/homebrew/bin/tmux", "/usr/local/bin/tmux", "tmux"],
        &["list-panes", "-a", "-F", TMUX_FORMAT],
    )?
    else {
        return Ok(Vec::new());
    };

    if !output.status.success() {
        // tmux exits with status 1 when no server exists. That is a normal
        // empty state, not a product error.
        if output.status.code() == Some(1) {
            return Ok(Vec::new());
        }
        anyhow::bail!(
            "tmux discovery failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    Ok(parse_tmux_sessions(
        &String::from_utf8_lossy(&output.stdout),
        executable,
    ))
}

fn scan_cmux_workspaces() -> Result<Vec<ExternalMultiplexerSession>> {
    let Some((executable, output)) = run_first_available(
        &[
            "/Applications/cmux.app/Contents/Resources/bin/cmux",
            "/opt/homebrew/bin/cmux",
            "/usr/local/bin/cmux",
            "cmux",
        ],
        &["list-workspaces", "--json"],
    )?
    else {
        return Ok(Vec::new());
    };

    if !output.status.success() {
        anyhow::bail!(
            "cmux discovery failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    parse_cmux_workspaces(&String::from_utf8_lossy(&output.stdout), executable)
}

fn parse_cmux_workspaces(
    output: &str,
    executable: PathBuf,
) -> Result<Vec<ExternalMultiplexerSession>> {
    let value: Value = serde_json::from_str(output).context("invalid cmux workspace JSON")?;
    let result = value.get("result").unwrap_or(&value);
    let workspaces = result
        .get("workspaces")
        .and_then(Value::as_array)
        .or_else(|| result.as_array())
        .context("cmux response did not contain workspaces")?;

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
            let detected_agent_kind =
                cmux_workspace_agent_label(workspace).and_then(detect_terminal_agent_kind);
            let needs_attention = workspace
                .get("unread_count")
                .and_then(Value::as_u64)
                .is_some_and(|count| count > 0)
                || workspace
                    .get("needs_attention")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);

            Some(ExternalMultiplexerSession {
                id: format!("cmux:{target}"),
                kind: MultiplexerKind::Cmux,
                target,
                title,
                owner_context,
                working_directory,
                foreground_command: None,
                detected_agent_kind,
                state: if needs_attention {
                    MultiplexerSessionState::NeedsAttention
                } else {
                    MultiplexerSessionState::Available
                },
                attached_clients: None,
                executable: executable.clone(),
                herdr_session_name: None,
            })
        })
        .collect())
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

fn parse_tmux_sessions(output: &str, executable: PathBuf) -> Vec<ExternalMultiplexerSession> {
    let mut sessions = BTreeMap::<String, TmuxPaneRecord>::new();

    for line in output.lines() {
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

    sessions
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
                state,
                attached_clients: Some(record.attached_clients),
                executable: executable.clone(),
                herdr_session_name: None,
            }
        })
        .collect()
}

fn scan_herdr_sessions() -> Result<Vec<ExternalMultiplexerSession>> {
    let herdr_root = paths::home_dir().join(".config").join("herdr");
    let mut server_names = Vec::new();
    if herdr_root.join("herdr.sock").exists() {
        server_names.push(None);
    }

    let named_sessions_dir = herdr_root.join("sessions");
    if let Ok(entries) = fs::read_dir(&named_sessions_dir) {
        for entry in entries.flatten() {
            if entry.path().join("herdr.sock").exists()
                && let Some(name) = entry.file_name().to_str()
            {
                server_names.push(Some(name.to_owned()));
            }
        }
    }

    if server_names.is_empty() {
        return Ok(Vec::new());
    }

    let Some(executable) =
        first_available_program(&["/opt/homebrew/bin/herdr", "/usr/local/bin/herdr", "herdr"])?
    else {
        return Ok(Vec::new());
    };

    let mut sessions = Vec::new();
    for server_name in server_names {
        let mut args = Vec::new();
        if let Some(name) = &server_name {
            args.extend(["--session".to_owned(), name.clone()]);
        }
        args.extend(["api".to_owned(), "snapshot".to_owned()]);

        let output = Command::new(&executable)
            .args(&args)
            .output()
            .with_context(|| format!("failed to query {}", executable.display()))?;
        if !output.status.success() {
            // A stale socket is equivalent to no running Herdr session.
            log::debug!(
                "Herdr session discovery skipped {}: {}",
                server_name.as_deref().unwrap_or("default"),
                String::from_utf8_lossy(&output.stderr).trim()
            );
            continue;
        }

        sessions.extend(parse_herdr_snapshot(
            &String::from_utf8_lossy(&output.stdout),
            executable.clone(),
            server_name,
        )?);
    }

    Ok(sessions)
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

    let server_key = server_name.as_deref().unwrap_or("default");
    let sessions = snapshot
        .get("panes")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
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
                state,
                attached_clients: None,
                executable: executable.clone(),
                herdr_session_name: server_name.clone(),
            })
        })
        .collect();

    Ok(sessions)
}

fn run_first_available(programs: &[&str], args: &[&str]) -> Result<Option<(PathBuf, Output)>> {
    for program in programs {
        match Command::new(program).args(args).output() {
            Ok(output) => return Ok(Some((PathBuf::from(program), output))),
            Err(error) if error.kind() == ErrorKind::NotFound => continue,
            Err(error) => return Err(error).with_context(|| format!("failed to run {program}")),
        }
    }
    Ok(None)
}

fn first_available_program(programs: &[&str]) -> Result<Option<PathBuf>> {
    for program in programs {
        match Command::new(program).arg("--version").output() {
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

    #[test]
    fn parses_tmux_sessions_by_stable_session_id_and_prefers_active_pane() {
        let separator = TMUX_FIELD_SEPARATOR;
        let output = format!(
            "$1{separator}compiler{separator}0{separator}@1{separator}editor{separator}1{separator}%1{separator}0{separator}/tmp/old{separator}zsh{separator}old\n\
             $1{separator}compiler{separator}0{separator}@1{separator}editor{separator}1{separator}%2{separator}1{separator}/tmp/project{separator}codex{separator}agent"
        );
        let sessions = parse_tmux_sessions(&output, PathBuf::from("/opt/homebrew/bin/tmux"));

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
    }

    #[test]
    fn parses_cmux_workspaces_as_externally_revealed_project_activity() {
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

        let sessions = parse_cmux_workspaces(output, PathBuf::from("/opt/homebrew/bin/cmux"))
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
