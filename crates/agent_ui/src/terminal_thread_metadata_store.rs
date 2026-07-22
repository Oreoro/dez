use std::path::{Path, PathBuf};

use anyhow::Context as _;
use chrono::{DateTime, Utc};
use collections::{HashMap, HashSet};
use db::{
    sqlez::{
        bindable::Column, domain::Domain, statement::Statement,
        thread_safe_connection::ThreadSafeConnection,
    },
    sqlez_macros::sql,
};
use futures::{FutureExt, future::Shared};
use gpui::{AppContext as _, Entity, Global, Task};
use remote::{RemoteConnectionOptions, same_remote_connection_identity};
use terminal::session_host::{TerminalHostId, TerminalSessionId, TerminalSessionRef};
use ui::{App, Context, SharedString};
use util::ResultExt as _;
use workspace::PathList;

use crate::{TerminalId, thread_metadata_store::WorktreePaths};

pub fn init(cx: &mut App) {
    TerminalThreadMetadataStore::init_global(cx);
}

struct GlobalTerminalThreadMetadataStore(Entity<TerminalThreadMetadataStore>);
impl Global for GlobalTerminalThreadMetadataStore {}

#[cfg(any(test, feature = "test-support"))]
pub struct TestTerminalMetadataDbName(pub String);
#[cfg(any(test, feature = "test-support"))]
impl Global for TestTerminalMetadataDbName {}

#[cfg(any(test, feature = "test-support"))]
impl TestTerminalMetadataDbName {
    pub fn global(cx: &App) -> String {
        cx.try_global::<Self>()
            .map(|global| global.0.clone())
            .unwrap_or_else(|| {
                let thread = std::thread::current();
                let test_name = thread.name().unwrap_or("unknown_test");
                format!("TERMINAL_THREAD_METADATA_DB_{}", test_name)
            })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TerminalThreadMetadata {
    pub terminal_id: TerminalId,
    pub title: SharedString,
    pub custom_title: Option<SharedString>,
    pub created_at: DateTime<Utc>,
    pub worktree_paths: WorktreePaths,
    pub remote_connection: Option<RemoteConnectionOptions>,
    pub working_directory: Option<PathBuf>,
    /// Durable attention truth and presentation. Acknowledging an item only
    /// changes its unread presentation; it does not claim the underlying
    /// condition has been resolved.
    pub attention: TerminalAttentionState,
    /// Optional stable computation identity. `None` is reserved for legacy or
    /// remote rows and must not be inferred from `terminal_id`.
    pub session_ref: Option<TerminalSessionRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalAttentionCondition {
    Active,
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalAttentionPresentation {
    Unread,
    Acknowledged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TerminalAttentionPriority {
    Normal,
    Urgent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalAttentionState {
    pub condition: TerminalAttentionCondition,
    pub presentation: TerminalAttentionPresentation,
    pub priority: TerminalAttentionPriority,
    pub muted_until: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    /// Optional expiry for low-confidence or event-style attention. Structured
    /// permission and failure conditions should leave this unset and resolve
    /// from their owning adapter instead.
    pub expires_at: Option<DateTime<Utc>>,
}

impl Default for TerminalAttentionState {
    fn default() -> Self {
        Self {
            condition: TerminalAttentionCondition::Resolved,
            presentation: TerminalAttentionPresentation::Acknowledged,
            priority: TerminalAttentionPriority::Normal,
            muted_until: None,
            resolved_at: None,
            updated_at: None,
            expires_at: None,
        }
    }
}

impl TerminalAttentionState {
    pub fn raise_observed(&mut self, now: DateTime<Utc>) {
        self.condition = TerminalAttentionCondition::Active;
        self.presentation = TerminalAttentionPresentation::Unread;
        self.priority = TerminalAttentionPriority::Normal;
        self.muted_until = None;
        self.resolved_at = None;
        self.updated_at = Some(now);
        self.expires_at = Some(now + chrono::Duration::days(7));
    }

    pub fn acknowledge(&mut self, now: DateTime<Utc>) {
        if self.condition == TerminalAttentionCondition::Active {
            self.presentation = TerminalAttentionPresentation::Acknowledged;
            self.updated_at = Some(now);
        }
    }

    pub fn snooze_until(&mut self, until: DateTime<Utc>, now: DateTime<Utc>) {
        if self.condition == TerminalAttentionCondition::Active {
            self.presentation = TerminalAttentionPresentation::Acknowledged;
            self.muted_until = Some(until);
            self.updated_at = Some(now);
        }
    }

    pub fn resume(&mut self, now: DateTime<Utc>) {
        if self.condition == TerminalAttentionCondition::Active {
            self.presentation = TerminalAttentionPresentation::Unread;
            self.muted_until = None;
            self.updated_at = Some(now);
        }
    }

    pub fn resolve(&mut self, now: DateTime<Utc>) {
        self.condition = TerminalAttentionCondition::Resolved;
        self.presentation = TerminalAttentionPresentation::Acknowledged;
        self.muted_until = None;
        self.resolved_at = Some(now);
        self.updated_at = Some(now);
        self.expires_at = None;
    }

    pub fn is_stale_at(&self, now: DateTime<Utc>) -> bool {
        self.condition == TerminalAttentionCondition::Active
            && self.expires_at.is_some_and(|expires_at| expires_at <= now)
    }

    pub fn is_muted_at(&self, now: DateTime<Utc>) -> bool {
        self.condition == TerminalAttentionCondition::Active
            && self
                .muted_until
                .is_some_and(|muted_until| muted_until > now)
    }

    pub fn requires_action_at(&self, now: DateTime<Utc>) -> bool {
        self.condition == TerminalAttentionCondition::Active
            && !self.is_stale_at(now)
            && !self.is_muted_at(now)
    }

    pub fn is_unread_at(&self, now: DateTime<Utc>) -> bool {
        self.requires_action_at(now) && self.presentation == TerminalAttentionPresentation::Unread
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalAgentKind {
    Claude,
    Codex,
    Gemini,
    Aider,
    Agy,
    OpenCode,
    Amp,
    Crush,
    Devin,
    Droid,
    Goose,
    Grok,
    OpenHands,
    Pi,
    Qwen,
    Cursor,
    Copilot,
}

impl TerminalAgentKind {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Claude => "Claude Code",
            Self::Codex => "Codex",
            Self::Gemini => "Gemini CLI",
            Self::Aider => "Aider",
            Self::Agy => "Agy",
            Self::OpenCode => "OpenCode",
            Self::Amp => "Amp",
            Self::Crush => "Crush",
            Self::Devin => "Devin",
            Self::Droid => "Droid",
            Self::Goose => "Goose",
            Self::Grok => "Grok",
            Self::OpenHands => "OpenHands",
            Self::Pi => "Pi",
            Self::Qwen => "Qwen Code",
            Self::Cursor => "Cursor Agent",
            Self::Copilot => "GitHub Copilot",
        }
    }
}

impl TerminalThreadMetadata {
    pub fn folder_paths(&self) -> &PathList {
        self.worktree_paths.folder_path_list()
    }

    pub fn main_worktree_paths(&self) -> &PathList {
        self.worktree_paths.main_worktree_path_list()
    }

    pub fn display_title(&self) -> SharedString {
        compose_terminal_thread_title(
            self.title.as_ref(),
            self.custom_title.as_ref().map(|title| title.as_ref()),
        )
    }

    pub fn detected_agent_kind(&self) -> Option<TerminalAgentKind> {
        detect_terminal_agent_kind(self.display_title().as_ref())
            .or_else(|| detect_terminal_agent_kind(self.title.as_ref()))
    }
}

pub(crate) fn compose_terminal_thread_title(
    terminal_title: &str,
    custom_title: Option<&str>,
) -> SharedString {
    let Some(custom_title) = custom_title.filter(|title| !title.trim().is_empty()) else {
        return SharedString::from(terminal_title.to_string());
    };

    if let Some(prefix) = terminal_title_prefix(terminal_title) {
        SharedString::from(format!("{prefix}{custom_title}"))
    } else {
        SharedString::from(custom_title.to_string())
    }
}

pub(crate) fn terminal_title_without_prefix(title: &str) -> &str {
    terminal_title_prefix(title)
        .map(|prefix| &title[prefix.len()..])
        .unwrap_or(title)
}

pub fn terminal_title_prefix(title: &str) -> Option<&str> {
    let mut prefix_byte_len = 0;
    let mut saw_prefix_character = false;
    let mut saw_whitespace_after_prefix = false;

    let mut chars = title.chars().peekable();
    while let Some(character) = chars.next() {
        if character.is_alphanumeric() {
            return None;
        }

        if character.is_whitespace() {
            if !saw_prefix_character {
                return None;
            }

            prefix_byte_len += character.len_utf8();
            saw_whitespace_after_prefix = true;

            while let Some(character) = chars.peek() {
                if !character.is_whitespace() {
                    break;
                }

                prefix_byte_len += character.len_utf8();
                chars.next();
            }

            break;
        }

        saw_prefix_character = true;
        prefix_byte_len += character.len_utf8();
    }

    if saw_whitespace_after_prefix {
        Some(&title[..prefix_byte_len])
    } else {
        None
    }
}

pub fn detect_terminal_agent_kind(title: &str) -> Option<TerminalAgentKind> {
    let title = terminal_title_without_prefix(title);
    let normalized = title
        .chars()
        .map(|character| {
            if character.is_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect::<String>();
    let tokens = normalized.split_whitespace().collect::<Vec<_>>();
    if tokens.is_empty() {
        return None;
    }

    let has_token = |token: &str| tokens.contains(&token);
    let has_phrase = |phrase: &[&str]| {
        tokens
            .windows(phrase.len())
            .any(|candidate| candidate == phrase)
    };
    let compact = tokens.join("");

    if has_token("claude") || has_phrase(&["claude", "code"]) {
        Some(TerminalAgentKind::Claude)
    } else if has_token("codex") {
        Some(TerminalAgentKind::Codex)
    } else if has_token("gemini") {
        Some(TerminalAgentKind::Gemini)
    } else if has_token("aider") {
        Some(TerminalAgentKind::Aider)
    } else if has_token("agy") {
        Some(TerminalAgentKind::Agy)
    } else if has_token("opencode") || compact.contains("opencode") {
        Some(TerminalAgentKind::OpenCode)
    } else if has_token("amp") {
        Some(TerminalAgentKind::Amp)
    } else if has_token("crush") {
        Some(TerminalAgentKind::Crush)
    } else if has_token("devin") {
        Some(TerminalAgentKind::Devin)
    } else if has_token("droid") {
        Some(TerminalAgentKind::Droid)
    } else if has_token("goose") {
        Some(TerminalAgentKind::Goose)
    } else if has_token("grok") {
        Some(TerminalAgentKind::Grok)
    } else if has_token("openhands") || compact.contains("openhands") {
        Some(TerminalAgentKind::OpenHands)
    } else if has_token("pi") {
        Some(TerminalAgentKind::Pi)
    } else if has_token("qwen") {
        Some(TerminalAgentKind::Qwen)
    } else if has_token("cursor") {
        Some(TerminalAgentKind::Cursor)
    } else if has_token("copilot") {
        Some(TerminalAgentKind::Copilot)
    } else {
        None
    }
}

pub fn detect_terminal_agent_command(command: &str) -> Option<TerminalAgentKind> {
    let command = command.trim();
    if command.is_empty() {
        return None;
    }

    let command = command.rsplit(['/', '\\']).next().unwrap_or(command);
    let command = command.strip_suffix(".exe").unwrap_or(command);

    detect_terminal_agent_kind(command)
}

pub struct TerminalThreadMetadataStore {
    db: TerminalThreadMetadataDb,
    terminals: HashMap<TerminalId, TerminalThreadMetadata>,
    terminals_by_paths: HashMap<PathList, HashSet<TerminalId>>,
    terminals_by_main_paths: HashMap<PathList, HashSet<TerminalId>>,
    reload_task: Option<Shared<Task<()>>>,
    pending_terminal_ops_tx: async_channel::Sender<DbOperation>,
    _db_operations_task: Task<()>,
}

#[derive(Debug, PartialEq)]
enum DbOperation {
    Upsert(TerminalThreadMetadata),
    Delete(TerminalId),
}

impl DbOperation {
    fn id(&self) -> TerminalId {
        match self {
            DbOperation::Upsert(metadata) => metadata.terminal_id,
            DbOperation::Delete(terminal_id) => *terminal_id,
        }
    }
}

impl TerminalThreadMetadataStore {
    #[cfg(not(any(test, feature = "test-support")))]
    pub fn init_global(cx: &mut App) {
        if cx.has_global::<GlobalTerminalThreadMetadataStore>() {
            return;
        }

        let db = TerminalThreadMetadataDb::global(cx);
        let terminal_store = cx.new(|cx| Self::new(db, cx));
        cx.set_global(GlobalTerminalThreadMetadataStore(terminal_store));
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn init_global(cx: &mut App) {
        let db_name = TestTerminalMetadataDbName::global(cx);
        let db = gpui::block_on(db::open_test_db::<TerminalThreadMetadataDb>(&db_name));
        let terminal_store = cx.new(|cx| Self::new(TerminalThreadMetadataDb(db), cx));
        cx.set_global(GlobalTerminalThreadMetadataStore(terminal_store));
    }

    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalTerminalThreadMetadataStore>()
            .map(|store| store.0.clone())
    }

    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalTerminalThreadMetadataStore>().0.clone()
    }

    pub fn entry(&self, terminal_id: TerminalId) -> Option<&TerminalThreadMetadata> {
        self.terminals.get(&terminal_id)
    }

    pub fn entries(&self) -> impl Iterator<Item = &TerminalThreadMetadata> + '_ {
        self.terminals.values()
    }

    pub fn reload_task(&self) -> Shared<Task<()>> {
        self.reload_task
            .clone()
            .unwrap_or_else(|| Task::ready(()).shared())
    }

    pub fn entries_for_path<'a>(
        &'a self,
        path_list: &PathList,
        remote_connection: Option<&'a RemoteConnectionOptions>,
    ) -> impl Iterator<Item = &'a TerminalThreadMetadata> + 'a {
        self.terminals_by_paths
            .get(path_list)
            .into_iter()
            .flatten()
            .filter_map(|id| self.terminals.get(id))
            .filter(move |terminal| {
                same_remote_connection_identity(
                    terminal.remote_connection.as_ref(),
                    remote_connection,
                )
            })
    }

    pub fn entries_for_main_worktree_path<'a>(
        &'a self,
        path_list: &PathList,
        remote_connection: Option<&'a RemoteConnectionOptions>,
    ) -> impl Iterator<Item = &'a TerminalThreadMetadata> + 'a {
        self.terminals_by_main_paths
            .get(path_list)
            .into_iter()
            .flatten()
            .filter_map(|id| self.terminals.get(id))
            .filter(move |terminal| {
                same_remote_connection_identity(
                    terminal.remote_connection.as_ref(),
                    remote_connection,
                )
            })
    }

    pub fn path_is_referenced_by_terminal(
        &self,
        terminal_id: Option<TerminalId>,
        path: &Path,
        remote_connection: Option<&RemoteConnectionOptions>,
    ) -> bool {
        self.entries().any(|terminal| {
            Some(terminal.terminal_id) != terminal_id
                && same_remote_connection_identity(
                    terminal.remote_connection.as_ref(),
                    remote_connection,
                )
                && terminal
                    .folder_paths()
                    .paths()
                    .iter()
                    .any(|folder_path| folder_path.as_path() == path)
        })
    }

    pub fn save(&mut self, metadata: TerminalThreadMetadata, cx: &mut Context<Self>) {
        self.save_internal(metadata);
        cx.notify();
    }

    pub fn acknowledge_attention(&mut self, terminal_id: TerminalId, cx: &mut Context<Self>) {
        self.update_attention(terminal_id, |attention, now| attention.acknowledge(now));
        cx.notify();
    }

    pub fn raise_attention(&mut self, terminal_id: TerminalId, cx: &mut Context<Self>) {
        self.update_attention(terminal_id, |attention, now| attention.raise_observed(now));
        cx.notify();
    }

    pub fn snooze_attention(
        &mut self,
        terminal_id: TerminalId,
        duration: chrono::Duration,
        cx: &mut Context<Self>,
    ) {
        self.update_attention(terminal_id, |attention, now| {
            attention.snooze_until(now + duration, now)
        });
        cx.notify();
    }

    pub fn resolve_attention(&mut self, terminal_id: TerminalId, cx: &mut Context<Self>) {
        self.update_attention(terminal_id, |attention, now| attention.resolve(now));
        cx.notify();
    }

    pub fn resume_attention(&mut self, terminal_id: TerminalId, cx: &mut Context<Self>) {
        self.update_attention(terminal_id, |attention, now| attention.resume(now));
        cx.notify();
    }

    fn update_attention(
        &mut self,
        terminal_id: TerminalId,
        mutate: impl FnOnce(&mut TerminalAttentionState, DateTime<Utc>),
    ) {
        let Some(mut metadata) = self.terminals.get(&terminal_id).cloned() else {
            return;
        };
        mutate(&mut metadata.attention, Utc::now());
        self.save_internal(metadata);
    }

    pub fn change_worktree_paths(
        &mut self,
        current_folder_paths: &PathList,
        remote_connection: Option<&RemoteConnectionOptions>,
        mutate: impl Fn(&mut WorktreePaths),
        cx: &mut Context<Self>,
    ) {
        let terminal_ids: Vec<_> = self
            .terminals_by_paths
            .get(current_folder_paths)
            .into_iter()
            .flatten()
            .filter(|id| {
                self.terminals.get(id).is_some_and(|terminal| {
                    same_remote_connection_identity(
                        terminal.remote_connection.as_ref(),
                        remote_connection,
                    )
                })
            })
            .copied()
            .collect();

        if terminal_ids.is_empty() {
            return;
        }

        for terminal_id in terminal_ids {
            if let Some(mut terminal) = self.terminals.get(&terminal_id).cloned() {
                mutate(&mut terminal.worktree_paths);
                self.save_internal(terminal);
            }
        }

        cx.notify();
    }

    fn save_internal(&mut self, metadata: TerminalThreadMetadata) {
        if let Some(existing) = self.terminals.get(&metadata.terminal_id) {
            if existing.folder_paths() != metadata.folder_paths()
                && let Some(ids) = self.terminals_by_paths.get_mut(existing.folder_paths())
            {
                ids.remove(&metadata.terminal_id);
            }

            if existing.main_worktree_paths() != metadata.main_worktree_paths()
                && let Some(ids) = self
                    .terminals_by_main_paths
                    .get_mut(existing.main_worktree_paths())
            {
                ids.remove(&metadata.terminal_id);
            }
        }

        self.cache_terminal_metadata(metadata.clone());
        self.pending_terminal_ops_tx
            .try_send(DbOperation::Upsert(metadata))
            .log_err();
    }

    fn cache_terminal_metadata(&mut self, metadata: TerminalThreadMetadata) {
        self.terminals
            .insert(metadata.terminal_id, metadata.clone());

        self.terminals_by_paths
            .entry(metadata.folder_paths().clone())
            .or_default()
            .insert(metadata.terminal_id);

        if !metadata.main_worktree_paths().is_empty() {
            self.terminals_by_main_paths
                .entry(metadata.main_worktree_paths().clone())
                .or_default()
                .insert(metadata.terminal_id);
        }
    }

    pub fn delete(&mut self, terminal_id: TerminalId, cx: &mut Context<Self>) {
        if let Some(terminal) = self.terminals.remove(&terminal_id) {
            if let Some(ids) = self.terminals_by_paths.get_mut(terminal.folder_paths()) {
                ids.remove(&terminal_id);
            }
            if !terminal.main_worktree_paths().is_empty()
                && let Some(ids) = self
                    .terminals_by_main_paths
                    .get_mut(terminal.main_worktree_paths())
            {
                ids.remove(&terminal_id);
            }
        }
        self.pending_terminal_ops_tx
            .try_send(DbOperation::Delete(terminal_id))
            .log_err();
        cx.notify();
    }

    fn new(db: TerminalThreadMetadataDb, cx: &mut Context<Self>) -> Self {
        let (tx, rx) = async_channel::unbounded();
        let _db_operations_task = cx.background_spawn({
            let db = db.clone();
            async move {
                while let Ok(first_update) = rx.recv().await {
                    let mut updates = vec![first_update];
                    while let Ok(update) = rx.try_recv() {
                        updates.push(update);
                    }
                    let updates = Self::dedup_db_operations(updates);
                    for operation in updates {
                        match operation {
                            DbOperation::Upsert(metadata) => {
                                db.save(metadata).await.log_err();
                            }
                            DbOperation::Delete(terminal_id) => {
                                db.delete(terminal_id).await.log_err();
                            }
                        }
                    }
                }
            }
        });

        let mut this = Self {
            db,
            terminals: HashMap::default(),
            terminals_by_paths: HashMap::default(),
            terminals_by_main_paths: HashMap::default(),
            reload_task: None,
            pending_terminal_ops_tx: tx,
            _db_operations_task,
        };
        this.reload(cx);
        this
    }

    fn dedup_db_operations(operations: Vec<DbOperation>) -> Vec<DbOperation> {
        let mut ops = HashMap::default();
        for operation in operations.into_iter().rev() {
            if ops.contains_key(&operation.id()) {
                continue;
            }
            ops.insert(operation.id(), operation);
        }
        ops.into_values().collect()
    }

    fn reload(&mut self, cx: &mut Context<Self>) {
        let db = self.db.clone();
        self.reload_task = Some(
            cx.spawn(async move |this, cx| {
                let rows = cx
                    .background_spawn(async move {
                        db.list()
                            .context("Failed to fetch terminal thread metadata")
                    })
                    .await
                    .log_err()
                    .unwrap_or_default();

                this.update(cx, |this, cx| {
                    this.terminals.clear();
                    this.terminals_by_paths.clear();
                    this.terminals_by_main_paths.clear();

                    for row in rows {
                        this.cache_terminal_metadata(row);
                    }

                    cx.notify();
                })
                .ok();
            })
            .shared(),
        );
    }
}

struct TerminalThreadMetadataDb(ThreadSafeConnection);

impl Domain for TerminalThreadMetadataDb {
    const NAME: &str = stringify!(TerminalThreadMetadataDb);

    const MIGRATIONS: &[&str] = &[
        sql!(
            CREATE TABLE IF NOT EXISTS sidebar_terminal_threads(
                terminal_id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                custom_title TEXT,
                created_at TEXT NOT NULL,
                working_directory TEXT,
                folder_paths TEXT,
                folder_paths_order TEXT,
                main_worktree_paths TEXT,
                main_worktree_paths_order TEXT,
                remote_connection TEXT
            ) STRICT;
        ),
        sql!(
            ALTER TABLE sidebar_terminal_threads ADD COLUMN terminal_host_id TEXT;
            ALTER TABLE sidebar_terminal_threads ADD COLUMN terminal_session_id TEXT;
        ),
        sql!(
            ALTER TABLE sidebar_terminal_threads
            ADD COLUMN has_attention INTEGER NOT NULL DEFAULT 0;
        ),
        sql!(
            ALTER TABLE sidebar_terminal_threads
            ADD COLUMN attention_condition INTEGER NOT NULL DEFAULT 0;
            ALTER TABLE sidebar_terminal_threads
            ADD COLUMN attention_acknowledged INTEGER NOT NULL DEFAULT 1;
            ALTER TABLE sidebar_terminal_threads
            ADD COLUMN attention_priority INTEGER NOT NULL DEFAULT 0;
            ALTER TABLE sidebar_terminal_threads ADD COLUMN attention_muted_until TEXT;
            ALTER TABLE sidebar_terminal_threads ADD COLUMN attention_resolved_at TEXT;
            ALTER TABLE sidebar_terminal_threads ADD COLUMN attention_updated_at TEXT;
            ALTER TABLE sidebar_terminal_threads ADD COLUMN attention_expires_at TEXT;

            UPDATE sidebar_terminal_threads
            SET attention_condition = 1,
                attention_acknowledged = 0,
                attention_updated_at = created_at
            WHERE has_attention = 1;
        ),
    ];
}

db::static_connection!(TerminalThreadMetadataDb, []);

impl TerminalThreadMetadataDb {
    pub fn list(&self) -> anyhow::Result<Vec<TerminalThreadMetadata>> {
        self.select::<TerminalThreadMetadata>(
            "SELECT terminal_id, title, custom_title, created_at, \
            working_directory, folder_paths, folder_paths_order, main_worktree_paths, \
            main_worktree_paths_order, remote_connection, terminal_host_id, terminal_session_id, \
            has_attention, attention_condition, attention_acknowledged, attention_priority, \
            attention_muted_until, attention_resolved_at, attention_updated_at, attention_expires_at \
            FROM sidebar_terminal_threads \
            ORDER BY created_at DESC",
        )?()
    }

    pub async fn save(&self, row: TerminalThreadMetadata) -> anyhow::Result<()> {
        let terminal_id = row.terminal_id.to_key_string();
        let title = row.title.to_string();
        let custom_title = row.custom_title.as_ref().map(ToString::to_string);
        let created_at = row.created_at.to_rfc3339();
        let working_directory = row
            .working_directory
            .as_ref()
            .map(|path| path.to_string_lossy().into_owned());
        let serialized = row.folder_paths().serialize();
        let (folder_paths, folder_paths_order) = if row.folder_paths().is_empty() {
            (None, None)
        } else {
            (Some(serialized.paths), Some(serialized.order))
        };
        let main_serialized = row.main_worktree_paths().serialize();
        let (main_worktree_paths, main_worktree_paths_order) =
            if row.main_worktree_paths().is_empty() {
                (None, None)
            } else {
                (Some(main_serialized.paths), Some(main_serialized.order))
            };
        let remote_connection = row
            .remote_connection
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .context("serialize terminal thread remote connection")?;
        let terminal_host_id = row
            .session_ref
            .map(|session_ref| session_ref.host_id.to_string());
        let terminal_session_id = row
            .session_ref
            .map(|session_ref| session_ref.session_id.to_string());
        let now = Utc::now();
        let has_attention = if row.attention.requires_action_at(now) {
            1_i64
        } else {
            0_i64
        };
        let attention_condition = match row.attention.condition {
            TerminalAttentionCondition::Active => 1_i64,
            TerminalAttentionCondition::Resolved => 0_i64,
        };
        let attention_acknowledged = match row.attention.presentation {
            TerminalAttentionPresentation::Unread => 0_i64,
            TerminalAttentionPresentation::Acknowledged => 1_i64,
        };
        let attention_priority = match row.attention.priority {
            TerminalAttentionPriority::Normal => 0_i64,
            TerminalAttentionPriority::Urgent => 1_i64,
        };
        let attention_muted_until = row.attention.muted_until.map(|value| value.to_rfc3339());
        let attention_resolved_at = row.attention.resolved_at.map(|value| value.to_rfc3339());
        let attention_updated_at = row.attention.updated_at.map(|value| value.to_rfc3339());
        let attention_expires_at = row.attention.expires_at.map(|value| value.to_rfc3339());

        self.write(move |conn| {
            let sql = "INSERT INTO sidebar_terminal_threads(terminal_id, title, custom_title, created_at, working_directory, folder_paths, folder_paths_order, main_worktree_paths, main_worktree_paths_order, remote_connection, terminal_host_id, terminal_session_id, has_attention, attention_condition, attention_acknowledged, attention_priority, attention_muted_until, attention_resolved_at, attention_updated_at, attention_expires_at) \
                       VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20) \
                       ON CONFLICT(terminal_id) DO UPDATE SET \
                           title = excluded.title, \
                           custom_title = excluded.custom_title, \
                           created_at = excluded.created_at, \
                           working_directory = excluded.working_directory, \
                           folder_paths = excluded.folder_paths, \
                           folder_paths_order = excluded.folder_paths_order, \
                           main_worktree_paths = excluded.main_worktree_paths, \
                           main_worktree_paths_order = excluded.main_worktree_paths_order, \
                           remote_connection = excluded.remote_connection, \
                           terminal_host_id = excluded.terminal_host_id, \
                           terminal_session_id = excluded.terminal_session_id, \
                           has_attention = excluded.has_attention, \
                           attention_condition = excluded.attention_condition, \
                           attention_acknowledged = excluded.attention_acknowledged, \
                           attention_priority = excluded.attention_priority, \
                           attention_muted_until = excluded.attention_muted_until, \
                           attention_resolved_at = excluded.attention_resolved_at, \
                           attention_updated_at = excluded.attention_updated_at, \
                           attention_expires_at = excluded.attention_expires_at";
            let mut stmt = Statement::prepare(conn, sql)?;
            let mut i = stmt.bind(&terminal_id, 1)?;
            i = stmt.bind(&title, i)?;
            i = stmt.bind(&custom_title, i)?;
            i = stmt.bind(&created_at, i)?;
            i = stmt.bind(&working_directory, i)?;
            i = stmt.bind(&folder_paths, i)?;
            i = stmt.bind(&folder_paths_order, i)?;
            i = stmt.bind(&main_worktree_paths, i)?;
            i = stmt.bind(&main_worktree_paths_order, i)?;
            i = stmt.bind(&remote_connection, i)?;
            i = stmt.bind(&terminal_host_id, i)?;
            i = stmt.bind(&terminal_session_id, i)?;
            i = stmt.bind(&has_attention, i)?;
            i = stmt.bind(&attention_condition, i)?;
            i = stmt.bind(&attention_acknowledged, i)?;
            i = stmt.bind(&attention_priority, i)?;
            i = stmt.bind(&attention_muted_until, i)?;
            i = stmt.bind(&attention_resolved_at, i)?;
            i = stmt.bind(&attention_updated_at, i)?;
            stmt.bind(&attention_expires_at, i)?;
            stmt.exec()
        })
        .await
    }

    pub async fn delete(&self, terminal_id: TerminalId) -> anyhow::Result<()> {
        let terminal_id = terminal_id.to_key_string();
        self.write(move |conn| {
            let mut stmt = Statement::prepare(
                conn,
                "DELETE FROM sidebar_terminal_threads WHERE terminal_id = ?",
            )?;
            stmt.bind(&terminal_id, 1)?;
            stmt.exec()
        })
        .await
    }
}

impl Column for TerminalThreadMetadata {
    fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
        let (terminal_id, next): (String, i32) = Column::column(statement, start_index)?;
        let (title, next): (String, i32) = Column::column(statement, next)?;
        let (custom_title, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (created_at, next): (String, i32) = Column::column(statement, next)?;
        let (working_directory, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (folder_paths_str, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (folder_paths_order_str, next): (Option<String>, i32) =
            Column::column(statement, next)?;
        let (main_worktree_paths_str, next): (Option<String>, i32) =
            Column::column(statement, next)?;
        let (main_worktree_paths_order_str, next): (Option<String>, i32) =
            Column::column(statement, next)?;
        let (remote_connection_json, next): (Option<String>, i32) =
            Column::column(statement, next)?;
        let (terminal_host_id, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (terminal_session_id, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (has_attention, next): (i64, i32) = Column::column(statement, next)?;
        let (attention_condition, next): (i64, i32) = Column::column(statement, next)?;
        let (attention_acknowledged, next): (i64, i32) = Column::column(statement, next)?;
        let (attention_priority, next): (i64, i32) = Column::column(statement, next)?;
        let (attention_muted_until, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (attention_resolved_at, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (attention_updated_at, next): (Option<String>, i32) = Column::column(statement, next)?;
        let (attention_expires_at, next): (Option<String>, i32) = Column::column(statement, next)?;

        let folder_paths = folder_paths_str
            .map(|paths| {
                PathList::deserialize(&util::path_list::SerializedPathList {
                    paths,
                    order: folder_paths_order_str.unwrap_or_default(),
                })
            })
            .unwrap_or_default();

        let main_worktree_paths = main_worktree_paths_str
            .map(|paths| {
                PathList::deserialize(&util::path_list::SerializedPathList {
                    paths,
                    order: main_worktree_paths_order_str.unwrap_or_default(),
                })
            })
            .unwrap_or_default();

        let remote_connection = remote_connection_json
            .as_deref()
            .map(serde_json::from_str::<RemoteConnectionOptions>)
            .transpose()
            .context("deserialize terminal thread remote connection")?;

        let worktree_paths = WorktreePaths::from_path_lists(main_worktree_paths, folder_paths)
            .unwrap_or_else(|_| WorktreePaths::default());
        let session_ref = match (terminal_host_id, terminal_session_id) {
            (None, None) => None,
            (Some(host_id), Some(session_id)) => Some(TerminalSessionRef {
                host_id: host_id
                    .parse::<TerminalHostId>()
                    .context("deserialize terminal host identity")?,
                session_id: session_id
                    .parse::<TerminalSessionId>()
                    .context("deserialize terminal session identity")?,
            }),
            _ => anyhow::bail!("terminal session identity is only partially stored"),
        };

        let parse_timestamp = |value: Option<String>| -> anyhow::Result<Option<DateTime<Utc>>> {
            value
                .map(|value| {
                    DateTime::parse_from_rfc3339(&value).map(|value| value.with_timezone(&Utc))
                })
                .transpose()
                .map_err(Into::into)
        };
        let legacy_attention = has_attention != 0;
        let attention = TerminalAttentionState {
            condition: if attention_condition != 0 || legacy_attention {
                TerminalAttentionCondition::Active
            } else {
                TerminalAttentionCondition::Resolved
            },
            presentation: if attention_acknowledged == 0 || legacy_attention {
                TerminalAttentionPresentation::Unread
            } else {
                TerminalAttentionPresentation::Acknowledged
            },
            priority: if attention_priority == 1 {
                TerminalAttentionPriority::Urgent
            } else {
                TerminalAttentionPriority::Normal
            },
            muted_until: parse_timestamp(attention_muted_until)?,
            resolved_at: parse_timestamp(attention_resolved_at)?,
            updated_at: parse_timestamp(attention_updated_at)?,
            expires_at: parse_timestamp(attention_expires_at)?,
        };

        Ok((
            TerminalThreadMetadata {
                terminal_id: TerminalId::from_key_string(&terminal_id)?,
                title: SharedString::from(title),
                custom_title: custom_title
                    .filter(|title| !title.trim().is_empty())
                    .map(SharedString::from),
                created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
                worktree_paths,
                remote_connection,
                working_directory: working_directory.map(PathBuf::from),
                attention,
                session_ref,
            },
            next,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use std::path::Path;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            TerminalThreadMetadataStore::init_global(cx);
        });
        cx.run_until_parked();
    }

    fn metadata(title: &str, worktree_paths: WorktreePaths) -> TerminalThreadMetadata {
        let now = Utc::now();
        TerminalThreadMetadata {
            terminal_id: TerminalId::new(),
            title: SharedString::from(title.to_string()),
            custom_title: None,
            created_at: now,
            worktree_paths,
            remote_connection: None,
            working_directory: None,
            attention: TerminalAttentionState::default(),
            session_ref: None,
        }
    }

    #[test]
    fn test_terminal_title_prefix_preserves_non_alphanumeric_prefixes() {
        assert_eq!(terminal_title_prefix("✳ Thinking"), Some("✳ "));
        assert_eq!(terminal_title_prefix(">>>   Thinking"), Some(">>>   "));
        assert_eq!(terminal_title_prefix("⠋ Running"), Some("⠋ "));
        assert_eq!(terminal_title_prefix("* Claude"), Some("* "));
        assert_eq!(terminal_title_prefix("✳Thinking"), None);
        assert_eq!(terminal_title_prefix("Thinking"), None);
        assert_eq!(terminal_title_prefix(" Thinking"), None);
        assert_eq!(terminal_title_prefix("✳"), None);
        assert_eq!(terminal_title_prefix("v1 Running"), None);
    }

    #[test]
    fn test_terminal_thread_display_title_combines_raw_and_custom_titles() {
        let mut metadata = metadata(
            "⠋ Thinking",
            WorktreePaths::from_folder_paths(&PathList::default()),
        );
        metadata.custom_title = Some("Fix bug".into());
        assert_eq!(metadata.display_title().as_ref(), "⠋ Fix bug");

        metadata.title = "Thinking".into();
        assert_eq!(metadata.display_title().as_ref(), "Fix bug");
    }

    #[test]
    fn test_detect_terminal_agent_kind_from_titles() {
        assert_eq!(
            detect_terminal_agent_kind("✳ Claude Code v2.1.207"),
            Some(TerminalAgentKind::Claude)
        );
        assert_eq!(
            detect_terminal_agent_kind("codex hello"),
            Some(TerminalAgentKind::Codex)
        );
        assert_eq!(
            detect_terminal_agent_kind("Gemini CLI"),
            Some(TerminalAgentKind::Gemini)
        );
        assert_eq!(
            detect_terminal_agent_kind("open-code session"),
            Some(TerminalAgentKind::OpenCode)
        );
        assert_eq!(
            detect_terminal_agent_kind("amp"),
            Some(TerminalAgentKind::Amp)
        );
        assert_eq!(
            detect_terminal_agent_kind("agy plan"),
            Some(TerminalAgentKind::Agy)
        );
        assert_eq!(
            detect_terminal_agent_kind("crush"),
            Some(TerminalAgentKind::Crush)
        );
        assert_eq!(
            detect_terminal_agent_kind("devin task"),
            Some(TerminalAgentKind::Devin)
        );
        assert_eq!(
            detect_terminal_agent_kind("droid session"),
            Some(TerminalAgentKind::Droid)
        );
        assert_eq!(
            detect_terminal_agent_kind("grok code"),
            Some(TerminalAgentKind::Grok)
        );
        assert_eq!(
            detect_terminal_agent_kind("OpenHands"),
            Some(TerminalAgentKind::OpenHands)
        );
        assert_eq!(
            detect_terminal_agent_kind("pi"),
            Some(TerminalAgentKind::Pi)
        );
        assert_eq!(detect_terminal_agent_kind("lamp server"), None);
        assert_eq!(detect_terminal_agent_kind("api server"), None);
        assert_eq!(detect_terminal_agent_kind("agent"), None);
        assert_eq!(detect_terminal_agent_kind("zsh"), None);
    }

    #[test]
    fn test_detect_terminal_agent_command() {
        assert_eq!(
            detect_terminal_agent_command("/usr/local/bin/claude"),
            Some(TerminalAgentKind::Claude)
        );
        assert_eq!(
            detect_terminal_agent_command("C:\\Users\\me\\bin\\codex.exe"),
            Some(TerminalAgentKind::Codex)
        );
        assert_eq!(
            detect_terminal_agent_command("open-hands"),
            Some(TerminalAgentKind::OpenHands)
        );
        assert_eq!(detect_terminal_agent_command("agent"), None);
        assert_eq!(detect_terminal_agent_command("python"), None);
    }

    #[gpui::test]
    async fn test_terminal_attention_round_trips_through_database(cx: &mut TestAppContext) {
        init_test(cx);

        let mut expected = metadata(
            "Background agent",
            WorktreePaths::from_folder_paths(&PathList::default()),
        );
        expected.attention.raise_observed(Utc::now());
        let db = cx.update(|cx| TerminalThreadMetadataStore::global(cx).read(cx).db.clone());

        db.save(expected.clone()).await.unwrap();
        let rows = db.list().unwrap();

        assert_eq!(rows, vec![expected]);
    }

    #[test]
    fn acknowledging_attention_does_not_resolve_the_condition() {
        let raised_at = Utc::now();
        let mut attention = TerminalAttentionState::default();
        attention.raise_observed(raised_at);
        attention.acknowledge(raised_at + chrono::Duration::minutes(1));

        assert_eq!(attention.condition, TerminalAttentionCondition::Active);
        assert_eq!(
            attention.presentation,
            TerminalAttentionPresentation::Acknowledged
        );
        assert!(attention.requires_action_at(raised_at + chrono::Duration::minutes(2)));
        assert!(!attention.is_unread_at(raised_at + chrono::Duration::minutes(2)));
    }

    #[test]
    fn snoozed_attention_reappears_without_becoming_unresolved_again() {
        let raised_at = Utc::now();
        let mut attention = TerminalAttentionState::default();
        attention.raise_observed(raised_at);
        attention.snooze_until(
            raised_at + chrono::Duration::hours(1),
            raised_at + chrono::Duration::minutes(1),
        );

        assert!(!attention.requires_action_at(raised_at + chrono::Duration::minutes(30)));
        assert!(attention.requires_action_at(raised_at + chrono::Duration::hours(2)));
        assert!(!attention.is_unread_at(raised_at + chrono::Duration::hours(2)));
    }

    #[test]
    fn resolving_and_expiring_attention_remove_it_from_action_needed() {
        let raised_at = Utc::now();
        let mut resolved = TerminalAttentionState::default();
        resolved.raise_observed(raised_at);
        resolved.resolve(raised_at + chrono::Duration::minutes(1));
        assert!(!resolved.requires_action_at(raised_at + chrono::Duration::minutes(2)));

        let mut expired = TerminalAttentionState::default();
        expired.raise_observed(raised_at);
        assert!(expired.requires_action_at(raised_at + chrono::Duration::days(6)));
        assert!(expired.is_stale_at(raised_at + chrono::Duration::days(8)));
        assert!(!expired.requires_action_at(raised_at + chrono::Duration::days(8)));
    }

    #[gpui::test]
    async fn test_change_worktree_paths_reindexes_terminal_metadata(cx: &mut TestAppContext) {
        init_test(cx);

        let old_main_paths = PathList::new(&[Path::new("/repo")]);
        let old_folder_paths = PathList::new(&[Path::new("/repo-feature")]);
        let new_main_path = Path::new("/repo");
        let new_folder_path = Path::new("/repo-feature-renamed");
        let new_folder_paths = PathList::new(&[new_folder_path]);
        let metadata = metadata(
            "Dev Server",
            WorktreePaths::from_path_lists(old_main_paths.clone(), old_folder_paths.clone())
                .unwrap(),
        );
        let terminal_id = metadata.terminal_id;

        cx.update(|cx| {
            TerminalThreadMetadataStore::global(cx).update(cx, |store, cx| {
                store.save(metadata, cx);
            });
        });

        cx.update(|cx| {
            TerminalThreadMetadataStore::global(cx).update(cx, |store, cx| {
                store.change_worktree_paths(
                    &old_folder_paths,
                    None,
                    |paths| {
                        paths.add_path(new_main_path, new_folder_path);
                        paths.remove_folder_path(Path::new("/repo-feature"));
                    },
                    cx,
                );
            });
        });

        cx.update(|cx| {
            let store = TerminalThreadMetadataStore::global(cx);
            let store = store.read(cx);
            assert!(
                store
                    .entries_for_path(&old_folder_paths, None)
                    .next()
                    .is_none()
            );
            assert_eq!(
                store
                    .entries_for_path(&new_folder_paths, None)
                    .map(|entry| entry.terminal_id)
                    .collect::<Vec<_>>(),
                vec![terminal_id]
            );
            assert_eq!(
                store
                    .entry(terminal_id)
                    .unwrap()
                    .main_worktree_paths()
                    .paths(),
                old_main_paths.paths()
            );
        });
    }
}
