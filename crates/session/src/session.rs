use std::collections::BTreeMap;

use db::kvp::KeyValueStore;
use gpui::{App, AppContext as _, Context, EventEmitter, Subscription, Task, WindowId};
use serde::{Deserialize, Serialize};
use util::ResultExt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkspaceRestoreState {
    Pending,
    Restoring,
    Ready,
}

impl WorkspaceRestoreState {
    fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Pending, Self::Restoring) | (Self::Restoring, Self::Ready)
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppSessionEvent {
    WorkspaceRestoreStateChanged(WorkspaceRestoreState),
    DurableWorkspaceMembershipChanged,
    DurableWorkspaceSelectionChanged,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableWorkspaceResolution {
    Resolved,
    /// The database identity exists, but its live window could not be
    /// materialized. This remains distinct from an identity that has simply
    /// not been considered by the active restore policy.
    RestoreFailed,
    #[default]
    Unresolved,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DurableWorkspaceMembership {
    pub workspace_id: i64,
    /// Compatibility hint for durable states written before viewport
    /// composition had its own record. New state is authoritative in
    /// `DurableViewportRecord`; this value is normalized to `None` after
    /// migration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewport_id: Option<u64>,
    #[serde(default)]
    pub resolution: DurableWorkspaceResolution,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DurableViewportRecord {
    pub viewport_id: u64,
    #[serde(default)]
    pub workspace_ids: Vec<i64>,
    #[serde(default)]
    pub active_workspace_id: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
struct DurableAppSessionState {
    memberships: Vec<DurableWorkspaceMembership>,
    #[serde(default)]
    viewports: Vec<DurableViewportRecord>,
    /// Compatibility field for compact state written before viewport records.
    /// It is consumed during migration and omitted from new state.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    active_workspace_by_viewport: BTreeMap<u64, i64>,
}

pub struct Session {
    session_id: String,
    old_session_id: Option<String>,
    old_window_ids: Option<Vec<WindowId>>,
    old_durable_workspace_state: DurableAppSessionState,
}

const SESSION_ID_KEY: &str = "session_id";
const SESSION_WINDOW_STACK_KEY: &str = "session_window_stack";
const DURABLE_WORKSPACE_STATE_KEY: &str = "dez_durable_workspace_state";

impl Session {
    pub async fn new(session_id: String, db: KeyValueStore) -> Self {
        let old_session_id = db.read_kvp(SESSION_ID_KEY).ok().flatten();

        db.write_kvp(SESSION_ID_KEY.to_string(), session_id.clone())
            .await
            .log_err();

        let old_window_ids = db
            .read_kvp(SESSION_WINDOW_STACK_KEY)
            .ok()
            .flatten()
            .and_then(|json| serde_json::from_str::<Vec<u64>>(&json).ok())
            .map(|vec: Vec<u64>| {
                vec.into_iter()
                    .map(WindowId::from)
                    .collect::<Vec<WindowId>>()
            });

        let mut old_durable_workspace_state = db
            .read_kvp(DURABLE_WORKSPACE_STATE_KEY)
            .ok()
            .flatten()
            .and_then(|json| serde_json::from_str::<DurableAppSessionState>(&json).ok())
            .unwrap_or_default();
        old_durable_workspace_state.viewports = migrate_durable_viewports(
            &old_durable_workspace_state.memberships,
            old_durable_workspace_state.viewports,
            &old_durable_workspace_state.active_workspace_by_viewport,
        );
        for membership in &mut old_durable_workspace_state.memberships {
            membership.resolution = resolution_without_live_attachment(membership.resolution);
            membership.viewport_id = None;
        }
        old_durable_workspace_state
            .active_workspace_by_viewport
            .clear();

        Self {
            session_id,
            old_session_id,
            old_window_ids,
            old_durable_workspace_state,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            old_session_id: None,
            old_window_ids: None,
            old_durable_workspace_state: DurableAppSessionState::default(),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_with_old_session(old_session_id: String) -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            old_session_id: Some(old_session_id),
            old_window_ids: None,
            old_durable_workspace_state: DurableAppSessionState::default(),
        }
    }

    pub fn id(&self) -> &str {
        &self.session_id
    }
}

pub struct AppSession {
    session: Session,
    workspace_restore_state: WorkspaceRestoreState,
    durable_workspace_memberships: Vec<DurableWorkspaceMembership>,
    durable_viewports: Vec<DurableViewportRecord>,
    _workspace_serialization_task: Option<Task<()>>,
    _serialization_task: Task<()>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<AppSessionEvent> for AppSession {}

impl AppSession {
    pub fn new(session: Session, cx: &Context<Self>) -> Self {
        let _subscriptions = vec![cx.on_app_quit(Self::app_will_quit)];
        let durable_workspace_memberships =
            collect_workspace_memberships(session.old_durable_workspace_state.memberships.clone());
        let durable_viewports = collect_durable_viewports(
            session.old_durable_workspace_state.viewports.clone(),
            &durable_workspace_memberships,
        );

        let _serialization_task = if cfg!(not(any(test, feature = "test-support"))) {
            let db = KeyValueStore::global(cx);
            cx.spawn(async move |_, cx| {
                // Disabled in tests: the infinite loop bypasses "parking forbidden" checks,
                // causing tests to hang instead of panicking.
                {
                    let mut current_window_stack = Vec::new();
                    loop {
                        if let Some(windows) = cx.update(|cx| window_stack(cx))
                            && windows != current_window_stack
                        {
                            store_window_stack(db.clone(), &windows).await;
                            current_window_stack = windows;
                        }

                        cx.background_executor()
                            .timer(std::time::Duration::from_millis(500))
                            .await;
                    }
                }
            })
        } else {
            Task::ready(())
        };

        Self {
            session,
            workspace_restore_state: WorkspaceRestoreState::Pending,
            durable_workspace_memberships,
            durable_viewports,
            _workspace_serialization_task: None,
            _subscriptions,
            _serialization_task,
        }
    }

    fn app_will_quit(&mut self, cx: &mut Context<Self>) -> Task<()> {
        let window_stack = window_stack(cx);
        let state = self.durable_workspace_state();
        let db = KeyValueStore::global(cx);
        cx.background_spawn(async move {
            if let Some(window_stack) = window_stack {
                store_window_stack(db.clone(), &window_stack).await;
            }
            store_durable_workspace_state(db, &state).await;
        })
    }

    pub fn id(&self) -> &str {
        self.session.id()
    }

    pub fn last_session_id(&self) -> Option<&str> {
        self.session.old_session_id.as_deref()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn replace_session_for_test(&mut self, session: Session) {
        self.session = session;
        self.durable_workspace_memberships.clear();
        self.durable_viewports.clear();
        self._workspace_serialization_task = None;
    }

    pub fn last_session_window_stack(&self) -> Option<Vec<WindowId>> {
        self.session.old_window_ids.clone()
    }

    pub fn workspace_restore_state(&self) -> WorkspaceRestoreState {
        self.workspace_restore_state
    }

    pub fn durable_workspace_memberships(
        &self,
    ) -> impl Iterator<Item = DurableWorkspaceMembership> + '_ {
        self.durable_workspace_memberships.iter().copied()
    }

    pub fn durable_viewports(&self) -> impl Iterator<Item = &DurableViewportRecord> {
        self.durable_viewports.iter()
    }

    pub fn active_durable_workspace(&self, viewport_id: u64) -> Option<i64> {
        self.durable_viewports
            .iter()
            .find(|viewport| viewport.viewport_id == viewport_id)
            .and_then(|viewport| viewport.active_workspace_id)
    }

    pub fn replace_durable_workspace_memberships(
        &mut self,
        memberships: impl IntoIterator<Item = DurableWorkspaceMembership>,
        cx: &mut Context<Self>,
    ) {
        let resolved_memberships = collect_workspace_memberships(memberships)
            .into_iter()
            .map(|membership| DurableWorkspaceMembership {
                viewport_id: None,
                ..membership
            })
            .collect::<Vec<_>>();
        let memberships = reconcile_workspace_memberships(
            &self.durable_workspace_memberships,
            resolved_memberships,
        );
        let viewports = collect_durable_viewports(self.durable_viewports.clone(), &memberships);
        let viewports_changed = self.durable_viewports != viewports;
        if self.durable_workspace_memberships == memberships && !viewports_changed {
            return;
        }
        self.durable_workspace_memberships = memberships;
        cx.emit(AppSessionEvent::DurableWorkspaceMembershipChanged);
        if viewports_changed {
            self.durable_viewports = viewports;
            cx.emit(AppSessionEvent::DurableWorkspaceSelectionChanged);
        }
        self.serialize_durable_workspace_state(cx);
        cx.notify();
    }

    pub fn register_durable_workspace(
        &mut self,
        mut membership: DurableWorkspaceMembership,
        cx: &mut Context<Self>,
    ) {
        let viewport_id = membership.viewport_id.take();
        if let Some(index) = self
            .durable_workspace_memberships
            .iter()
            .position(|existing| existing.workspace_id == membership.workspace_id)
        {
            if self.durable_workspace_memberships[index] == membership {
                if let Some(viewport_id) = viewport_id {
                    self.attach_durable_workspace_to_viewport(
                        viewport_id,
                        membership.workspace_id,
                        false,
                        cx,
                    );
                }
                return;
            }
            self.durable_workspace_memberships[index] = membership;
        } else {
            self.durable_workspace_memberships.push(membership);
        }
        cx.emit(AppSessionEvent::DurableWorkspaceMembershipChanged);
        if let Some(viewport_id) = viewport_id {
            self.attach_durable_workspace_to_viewport(
                viewport_id,
                membership.workspace_id,
                false,
                cx,
            );
        }
        self.serialize_durable_workspace_state(cx);
        cx.notify();
    }

    pub fn replace_durable_viewports(
        &mut self,
        viewports: impl IntoIterator<Item = DurableViewportRecord>,
        cx: &mut Context<Self>,
    ) {
        let resolved = collect_durable_viewports(viewports, &self.durable_workspace_memberships);
        let viewports = reconcile_durable_viewports(
            &self.durable_viewports,
            resolved,
            &self.durable_workspace_memberships,
        );
        if self.durable_viewports == viewports {
            return;
        }
        self.durable_viewports = viewports;
        cx.emit(AppSessionEvent::DurableWorkspaceSelectionChanged);
        self.serialize_durable_workspace_state(cx);
        cx.notify();
    }

    /// Records that a database-backed Workspace identity could not be
    /// materialized during restoration. Its ordered membership and viewport
    /// placement are intentionally retained so a recovery surface can retry or
    /// explicitly remove it instead of silently forgetting user state.
    pub fn mark_durable_workspace_restore_failed(
        &mut self,
        workspace_id: i64,
        cx: &mut Context<Self>,
    ) -> bool {
        if !set_workspace_resolution(
            &mut self.durable_workspace_memberships,
            workspace_id,
            DurableWorkspaceResolution::RestoreFailed,
        ) {
            return false;
        }
        cx.emit(AppSessionEvent::DurableWorkspaceMembershipChanged);
        self.serialize_durable_workspace_state(cx);
        cx.notify();
        true
    }

    pub fn attach_durable_workspace_to_viewport(
        &mut self,
        viewport_id: u64,
        workspace_id: i64,
        make_active: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        if !self
            .durable_workspace_memberships
            .iter()
            .any(|membership| membership.workspace_id == workspace_id)
        {
            return false;
        }

        if !attach_workspace_to_viewport_records(
            &mut self.durable_viewports,
            viewport_id,
            workspace_id,
            make_active,
        ) {
            return false;
        }

        cx.emit(AppSessionEvent::DurableWorkspaceSelectionChanged);
        self.serialize_durable_workspace_state(cx);
        cx.notify();
        true
    }

    pub fn set_active_durable_workspace(
        &mut self,
        viewport_id: u64,
        workspace_id: i64,
        cx: &mut Context<Self>,
    ) -> bool {
        self.attach_durable_workspace_to_viewport(viewport_id, workspace_id, true, cx)
    }

    pub fn remove_durable_workspace(&mut self, workspace_id: i64, cx: &mut Context<Self>) -> bool {
        let Some(index) = self
            .durable_workspace_memberships
            .iter()
            .position(|membership| membership.workspace_id == workspace_id)
        else {
            return false;
        };
        self.durable_workspace_memberships.remove(index);
        let previous_viewports = self.durable_viewports.clone();
        self.durable_viewports = collect_durable_viewports(
            self.durable_viewports.clone(),
            &self.durable_workspace_memberships,
        );
        let selections_changed = self.durable_viewports != previous_viewports;
        cx.emit(AppSessionEvent::DurableWorkspaceMembershipChanged);
        if selections_changed {
            cx.emit(AppSessionEvent::DurableWorkspaceSelectionChanged);
        }
        self.serialize_durable_workspace_state(cx);
        cx.notify();
        true
    }

    /// Removes one viewport's presentation of a Workspace. Global membership
    /// is removed only when no other viewport still presents that Workspace.
    pub fn remove_durable_workspace_from_viewport(
        &mut self,
        viewport_id: u64,
        workspace_id: i64,
        cx: &mut Context<Self>,
    ) -> bool {
        if !remove_workspace_from_viewport_records(
            &mut self.durable_viewports,
            viewport_id,
            workspace_id,
        ) {
            return false;
        }

        let remains_presented = self
            .durable_viewports
            .iter()
            .any(|viewport| viewport.workspace_ids.contains(&workspace_id));
        if !remains_presented {
            self.durable_workspace_memberships
                .retain(|membership| membership.workspace_id != workspace_id);
            cx.emit(AppSessionEvent::DurableWorkspaceMembershipChanged);
        }
        cx.emit(AppSessionEvent::DurableWorkspaceSelectionChanged);
        self.serialize_durable_workspace_state(cx);
        cx.notify();
        true
    }

    pub fn begin_workspace_restore(&mut self, cx: &mut Context<Self>) -> bool {
        self.transition_workspace_restore(WorkspaceRestoreState::Restoring, cx)
    }

    pub fn finish_workspace_restore(&mut self, cx: &mut Context<Self>) -> bool {
        self.transition_workspace_restore(WorkspaceRestoreState::Ready, cx)
    }

    fn transition_workspace_restore(
        &mut self,
        next: WorkspaceRestoreState,
        cx: &mut Context<Self>,
    ) -> bool {
        if !self.workspace_restore_state.can_transition_to(next) {
            return false;
        }

        self.workspace_restore_state = next;
        cx.emit(AppSessionEvent::WorkspaceRestoreStateChanged(next));
        cx.notify();
        true
    }

    fn durable_workspace_state(&self) -> DurableAppSessionState {
        DurableAppSessionState {
            memberships: self.durable_workspace_memberships.clone(),
            viewports: self.durable_viewports.clone(),
            active_workspace_by_viewport: BTreeMap::new(),
        }
    }

    fn serialize_durable_workspace_state(&mut self, cx: &mut Context<Self>) {
        let state = self.durable_workspace_state();
        let db = KeyValueStore::global(cx);
        self._workspace_serialization_task = Some(cx.background_spawn(async move {
            store_durable_workspace_state(db, &state).await;
        }));
    }
}

fn attach_workspace_to_viewport_records(
    viewports: &mut Vec<DurableViewportRecord>,
    viewport_id: u64,
    workspace_id: i64,
    make_active: bool,
) -> bool {
    let viewport = if let Some(index) = viewports
        .iter()
        .position(|viewport| viewport.viewport_id == viewport_id)
    {
        &mut viewports[index]
    } else {
        viewports.push(DurableViewportRecord {
            viewport_id,
            workspace_ids: Vec::new(),
            active_workspace_id: None,
        });
        viewports.last_mut().expect("viewport was just inserted")
    };

    let mut changed = false;
    if !viewport.workspace_ids.contains(&workspace_id) {
        viewport.workspace_ids.push(workspace_id);
        changed = true;
    }
    if make_active && viewport.active_workspace_id != Some(workspace_id) {
        viewport.active_workspace_id = Some(workspace_id);
        changed = true;
    }
    changed
}

fn set_workspace_resolution(
    memberships: &mut [DurableWorkspaceMembership],
    workspace_id: i64,
    resolution: DurableWorkspaceResolution,
) -> bool {
    let Some(membership) = memberships
        .iter_mut()
        .find(|membership| membership.workspace_id == workspace_id)
    else {
        return false;
    };
    if membership.resolution == resolution {
        return false;
    }
    membership.resolution = resolution;
    true
}

fn resolution_without_live_attachment(
    previous: DurableWorkspaceResolution,
) -> DurableWorkspaceResolution {
    match previous {
        DurableWorkspaceResolution::RestoreFailed => DurableWorkspaceResolution::RestoreFailed,
        DurableWorkspaceResolution::Resolved | DurableWorkspaceResolution::Unresolved => {
            DurableWorkspaceResolution::Unresolved
        }
    }
}

fn collect_workspace_memberships(
    memberships: impl IntoIterator<Item = DurableWorkspaceMembership>,
) -> Vec<DurableWorkspaceMembership> {
    let mut positions = BTreeMap::new();
    let mut collected = Vec::new();
    for membership in memberships {
        if let Some(index) = positions.get(&membership.workspace_id).copied() {
            collected[index] = membership;
        } else {
            positions.insert(membership.workspace_id, collected.len());
            collected.push(membership);
        }
    }
    collected
}

fn migrate_durable_viewports(
    memberships: &[DurableWorkspaceMembership],
    viewports: Vec<DurableViewportRecord>,
    legacy_active_workspace_by_viewport: &BTreeMap<u64, i64>,
) -> Vec<DurableViewportRecord> {
    if !viewports.is_empty() {
        return viewports;
    }

    let mut migrated = Vec::<DurableViewportRecord>::new();
    for membership in memberships {
        let Some(viewport_id) = membership.viewport_id else {
            continue;
        };
        let viewport = if let Some(index) = migrated
            .iter()
            .position(|viewport| viewport.viewport_id == viewport_id)
        {
            &mut migrated[index]
        } else {
            migrated.push(DurableViewportRecord {
                viewport_id,
                workspace_ids: Vec::new(),
                active_workspace_id: legacy_active_workspace_by_viewport
                    .get(&viewport_id)
                    .copied(),
            });
            migrated.last_mut().expect("viewport was just inserted")
        };
        if !viewport.workspace_ids.contains(&membership.workspace_id) {
            viewport.workspace_ids.push(membership.workspace_id);
        }
    }

    for (&viewport_id, &workspace_id) in legacy_active_workspace_by_viewport {
        let viewport = if let Some(index) = migrated
            .iter()
            .position(|viewport| viewport.viewport_id == viewport_id)
        {
            &mut migrated[index]
        } else {
            migrated.push(DurableViewportRecord {
                viewport_id,
                workspace_ids: Vec::new(),
                active_workspace_id: None,
            });
            migrated.last_mut().expect("viewport was just inserted")
        };
        if !viewport.workspace_ids.contains(&workspace_id) {
            viewport.workspace_ids.push(workspace_id);
        }
        viewport.active_workspace_id = Some(workspace_id);
    }

    migrated
}

fn collect_durable_viewports(
    viewports: impl IntoIterator<Item = DurableViewportRecord>,
    memberships: &[DurableWorkspaceMembership],
) -> Vec<DurableViewportRecord> {
    let valid_workspace_ids = memberships
        .iter()
        .map(|membership| membership.workspace_id)
        .collect::<std::collections::BTreeSet<_>>();
    let mut positions = BTreeMap::new();
    let mut collected = Vec::new();

    for mut viewport in viewports {
        let mut seen = std::collections::BTreeSet::new();
        viewport.workspace_ids.retain(|workspace_id| {
            valid_workspace_ids.contains(workspace_id) && seen.insert(*workspace_id)
        });
        if viewport
            .active_workspace_id
            .is_some_and(|workspace_id| !viewport.workspace_ids.contains(&workspace_id))
        {
            viewport.active_workspace_id = None;
        }

        if let Some(index) = positions.get(&viewport.viewport_id).copied() {
            collected[index] = viewport;
        } else {
            positions.insert(viewport.viewport_id, collected.len());
            collected.push(viewport);
        }
    }

    collected
}

fn reconcile_durable_viewports(
    previous: &[DurableViewportRecord],
    resolved: Vec<DurableViewportRecord>,
    memberships: &[DurableWorkspaceMembership],
) -> Vec<DurableViewportRecord> {
    let resolved_order = resolved
        .iter()
        .map(|viewport| viewport.viewport_id)
        .collect::<Vec<_>>();
    let mut resolved_by_id = resolved
        .into_iter()
        .map(|viewport| (viewport.viewport_id, viewport))
        .collect::<BTreeMap<_, _>>();
    let mut reconciled = Vec::new();

    for previous_viewport in previous {
        reconciled.push(
            resolved_by_id
                .remove(&previous_viewport.viewport_id)
                .unwrap_or_else(|| previous_viewport.clone()),
        );
    }
    for viewport_id in resolved_order {
        if let Some(viewport) = resolved_by_id.remove(&viewport_id) {
            reconciled.push(viewport);
        }
    }

    collect_durable_viewports(reconciled, memberships)
}

fn remove_workspace_from_viewport_records(
    viewports: &mut Vec<DurableViewportRecord>,
    viewport_id: u64,
    workspace_id: i64,
) -> bool {
    let Some(viewport_index) = viewports
        .iter()
        .position(|viewport| viewport.viewport_id == viewport_id)
    else {
        return false;
    };
    if !viewports[viewport_index]
        .workspace_ids
        .contains(&workspace_id)
    {
        return false;
    }

    let viewport = &mut viewports[viewport_index];
    viewport
        .workspace_ids
        .retain(|candidate| *candidate != workspace_id);
    if viewport.active_workspace_id == Some(workspace_id) {
        viewport.active_workspace_id = viewport.workspace_ids.first().copied();
    }
    if viewport.workspace_ids.is_empty() {
        viewports.remove(viewport_index);
    }
    true
}

fn reconcile_workspace_memberships(
    previous: &[DurableWorkspaceMembership],
    resolved: Vec<DurableWorkspaceMembership>,
) -> Vec<DurableWorkspaceMembership> {
    let resolved_order = resolved
        .iter()
        .map(|membership| membership.workspace_id)
        .collect::<Vec<_>>();
    let mut resolved_by_id = resolved
        .into_iter()
        .map(|membership| (membership.workspace_id, membership))
        .collect::<BTreeMap<_, _>>();
    let mut reconciled = Vec::new();

    for previous_membership in previous {
        if let Some(membership) = resolved_by_id.remove(&previous_membership.workspace_id) {
            reconciled.push(membership);
        } else {
            let resolution = resolution_without_live_attachment(previous_membership.resolution);
            reconciled.push(DurableWorkspaceMembership {
                resolution,
                ..*previous_membership
            });
        }
    }

    for workspace_id in resolved_order {
        if let Some(membership) = resolved_by_id.remove(&workspace_id) {
            reconciled.push(membership);
        }
    }
    reconciled
}

fn window_stack(cx: &App) -> Option<Vec<u64>> {
    Some(
        cx.window_stack()?
            .into_iter()
            .map(|window| window.window_id().as_u64())
            .collect(),
    )
}

async fn store_window_stack(db: KeyValueStore, windows: &[u64]) {
    if let Ok(window_ids_json) = serde_json::to_string(windows) {
        db.write_kvp(SESSION_WINDOW_STACK_KEY.to_string(), window_ids_json)
            .await
            .log_err();
    }
}

async fn store_durable_workspace_state(db: KeyValueStore, state: &DurableAppSessionState) {
    if let Ok(json) = serde_json::to_string(state) {
        db.write_kvp(DURABLE_WORKSPACE_STATE_KEY.to_string(), json)
            .await
            .log_err();
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DurableAppSessionState, DurableViewportRecord, DurableWorkspaceMembership,
        DurableWorkspaceResolution, WorkspaceRestoreState, attach_workspace_to_viewport_records,
        collect_durable_viewports, collect_workspace_memberships, migrate_durable_viewports,
        reconcile_durable_viewports, reconcile_workspace_memberships,
        remove_workspace_from_viewport_records, resolution_without_live_attachment,
        set_workspace_resolution,
    };

    #[test]
    fn workspace_restore_state_has_one_ordered_path() {
        assert!(WorkspaceRestoreState::Pending.can_transition_to(WorkspaceRestoreState::Restoring));
        assert!(WorkspaceRestoreState::Restoring.can_transition_to(WorkspaceRestoreState::Ready));
        assert!(!WorkspaceRestoreState::Pending.can_transition_to(WorkspaceRestoreState::Ready));
        assert!(!WorkspaceRestoreState::Ready.can_transition_to(WorkspaceRestoreState::Restoring));
    }

    #[test]
    fn failed_restore_is_distinct_from_unresolved_without_losing_order() {
        let mut memberships = vec![
            DurableWorkspaceMembership {
                workspace_id: 20,
                viewport_id: None,
                resolution: DurableWorkspaceResolution::Resolved,
            },
            DurableWorkspaceMembership {
                workspace_id: 10,
                viewport_id: None,
                resolution: DurableWorkspaceResolution::Resolved,
            },
        ];

        assert!(set_workspace_resolution(
            &mut memberships,
            20,
            DurableWorkspaceResolution::RestoreFailed,
        ));
        assert_eq!(
            memberships
                .iter()
                .map(|membership| membership.workspace_id)
                .collect::<Vec<_>>(),
            [20, 10]
        );
        assert_eq!(
            memberships[0].resolution,
            DurableWorkspaceResolution::RestoreFailed
        );
        assert_eq!(
            memberships[1].resolution,
            DurableWorkspaceResolution::Resolved
        );
        assert!(!set_workspace_resolution(
            &mut memberships,
            20,
            DurableWorkspaceResolution::RestoreFailed,
        ));
        assert!(!set_workspace_resolution(
            &mut memberships,
            999,
            DurableWorkspaceResolution::RestoreFailed,
        ));
    }

    #[test]
    fn only_actual_restore_failures_survive_as_failures() {
        assert_eq!(
            resolution_without_live_attachment(DurableWorkspaceResolution::Resolved),
            DurableWorkspaceResolution::Unresolved
        );
        assert_eq!(
            resolution_without_live_attachment(DurableWorkspaceResolution::Unresolved),
            DurableWorkspaceResolution::Unresolved
        );
        assert_eq!(
            resolution_without_live_attachment(DurableWorkspaceResolution::RestoreFailed),
            DurableWorkspaceResolution::RestoreFailed
        );
    }

    #[test]
    fn live_viewport_attachment_is_idempotent_and_preserves_order() {
        let mut viewports = Vec::new();
        assert!(attach_workspace_to_viewport_records(
            &mut viewports,
            1,
            10,
            false,
        ));
        assert!(!attach_workspace_to_viewport_records(
            &mut viewports,
            1,
            10,
            false,
        ));
        assert!(attach_workspace_to_viewport_records(
            &mut viewports,
            2,
            10,
            false,
        ));
        assert!(attach_workspace_to_viewport_records(
            &mut viewports,
            2,
            10,
            true,
        ));

        assert_eq!(viewports.len(), 2);
        assert_eq!(viewports[0].viewport_id, 1);
        assert_eq!(viewports[0].workspace_ids, [10]);
        assert_eq!(viewports[0].active_workspace_id, None);
        assert_eq!(viewports[1].viewport_id, 2);
        assert_eq!(viewports[1].workspace_ids, [10]);
        assert_eq!(viewports[1].active_workspace_id, Some(10));
    }

    #[test]
    fn durable_workspace_membership_preserves_order_and_updates_in_place() {
        let memberships = collect_workspace_memberships([
            DurableWorkspaceMembership {
                workspace_id: 20,
                viewport_id: Some(3),
                resolution: DurableWorkspaceResolution::Resolved,
            },
            DurableWorkspaceMembership {
                workspace_id: 10,
                viewport_id: Some(1),
                resolution: DurableWorkspaceResolution::Resolved,
            },
            DurableWorkspaceMembership {
                workspace_id: 20,
                viewport_id: Some(4),
                resolution: DurableWorkspaceResolution::Resolved,
            },
        ]);

        assert_eq!(memberships.len(), 2);
        assert_eq!(
            memberships[0],
            DurableWorkspaceMembership {
                workspace_id: 20,
                viewport_id: Some(4),
                resolution: DurableWorkspaceResolution::Resolved,
            }
        );
        assert_eq!(
            memberships
                .iter()
                .map(|membership| membership.workspace_id)
                .collect::<Vec<_>>(),
            [20, 10]
        );
    }

    #[test]
    fn workspace_reconciliation_retains_unresolved_membership_in_order() {
        let previous = [
            DurableWorkspaceMembership {
                workspace_id: 20,
                viewport_id: Some(3),
                resolution: DurableWorkspaceResolution::Resolved,
            },
            DurableWorkspaceMembership {
                workspace_id: 10,
                viewport_id: Some(1),
                resolution: DurableWorkspaceResolution::Resolved,
            },
        ];
        let resolved = vec![
            DurableWorkspaceMembership {
                workspace_id: 10,
                viewport_id: Some(1),
                resolution: DurableWorkspaceResolution::Resolved,
            },
            DurableWorkspaceMembership {
                workspace_id: 40,
                viewport_id: Some(4),
                resolution: DurableWorkspaceResolution::Resolved,
            },
            DurableWorkspaceMembership {
                workspace_id: 30,
                viewport_id: Some(3),
                resolution: DurableWorkspaceResolution::Resolved,
            },
        ];

        let reconciled = reconcile_workspace_memberships(&previous, resolved);

        assert_eq!(reconciled.len(), 4);
        assert_eq!(reconciled[0].workspace_id, 20);
        assert_eq!(
            reconciled[0].resolution,
            DurableWorkspaceResolution::Unresolved
        );
        assert_eq!(reconciled[1].workspace_id, 10);
        assert_eq!(
            reconciled[1].resolution,
            DurableWorkspaceResolution::Resolved
        );
        assert_eq!(reconciled[2].workspace_id, 40);
        assert_eq!(reconciled[3].workspace_id, 30);
    }

    #[test]
    fn legacy_membership_state_migrates_to_explicit_viewports() {
        let memberships = [
            DurableWorkspaceMembership {
                workspace_id: 10,
                viewport_id: Some(1),
                resolution: DurableWorkspaceResolution::Resolved,
            },
            DurableWorkspaceMembership {
                workspace_id: 20,
                viewport_id: Some(2),
                resolution: DurableWorkspaceResolution::Unresolved,
            },
        ];
        let selections = [(1, 10), (2, 20)].into_iter().collect();

        assert_eq!(
            migrate_durable_viewports(&memberships, Vec::new(), &selections,),
            vec![
                DurableViewportRecord {
                    viewport_id: 1,
                    workspace_ids: vec![10],
                    active_workspace_id: Some(10),
                },
                DurableViewportRecord {
                    viewport_id: 2,
                    workspace_ids: vec![20],
                    active_workspace_id: Some(20),
                },
            ]
        );
    }

    #[test]
    fn workspace_can_belong_to_multiple_viewports_without_duplicate_membership() {
        let memberships = [DurableWorkspaceMembership {
            workspace_id: 10,
            viewport_id: None,
            resolution: DurableWorkspaceResolution::Resolved,
        }];
        let viewports = collect_durable_viewports(
            [
                DurableViewportRecord {
                    viewport_id: 1,
                    workspace_ids: vec![10, 10],
                    active_workspace_id: Some(10),
                },
                DurableViewportRecord {
                    viewport_id: 2,
                    workspace_ids: vec![10],
                    active_workspace_id: Some(10),
                },
            ],
            &memberships,
        );

        assert_eq!(viewports.len(), 2);
        assert_eq!(viewports[0].workspace_ids, [10]);
        assert_eq!(viewports[1].workspace_ids, [10]);
    }

    #[test]
    fn duplicate_viewport_updates_in_place_without_reordering() {
        let memberships = [10, 20, 30].map(|workspace_id| DurableWorkspaceMembership {
            workspace_id,
            viewport_id: None,
            resolution: DurableWorkspaceResolution::Resolved,
        });
        let viewports = collect_durable_viewports(
            [
                DurableViewportRecord {
                    viewport_id: 1,
                    workspace_ids: vec![10],
                    active_workspace_id: Some(10),
                },
                DurableViewportRecord {
                    viewport_id: 2,
                    workspace_ids: vec![20],
                    active_workspace_id: Some(20),
                },
                DurableViewportRecord {
                    viewport_id: 1,
                    workspace_ids: vec![30, 30, 999],
                    active_workspace_id: Some(999),
                },
            ],
            &memberships,
        );

        assert_eq!(viewports.len(), 2);
        assert_eq!(viewports[0].viewport_id, 1);
        assert_eq!(viewports[0].workspace_ids, [30]);
        assert_eq!(viewports[0].active_workspace_id, None);
        assert_eq!(viewports[1].viewport_id, 2);
        assert_eq!(viewports[1].workspace_ids, [20]);
    }

    #[test]
    fn viewport_reconciliation_preserves_prior_order_and_unresolved_composition() {
        let memberships = [
            DurableWorkspaceMembership {
                workspace_id: 10,
                viewport_id: None,
                resolution: DurableWorkspaceResolution::Resolved,
            },
            DurableWorkspaceMembership {
                workspace_id: 20,
                viewport_id: None,
                resolution: DurableWorkspaceResolution::Unresolved,
            },
        ];
        let previous = [DurableViewportRecord {
            viewport_id: 2,
            workspace_ids: vec![20],
            active_workspace_id: Some(20),
        }];
        let resolved = vec![DurableViewportRecord {
            viewport_id: 1,
            workspace_ids: vec![10],
            active_workspace_id: Some(10),
        }];

        let reconciled = reconcile_durable_viewports(&previous, resolved, &memberships);

        assert_eq!(
            reconciled,
            vec![
                DurableViewportRecord {
                    viewport_id: 2,
                    workspace_ids: vec![20],
                    active_workspace_id: Some(20),
                },
                DurableViewportRecord {
                    viewport_id: 1,
                    workspace_ids: vec![10],
                    active_workspace_id: Some(10),
                },
            ]
        );
    }

    #[test]
    fn removing_one_viewport_copy_preserves_the_other_copy() {
        let mut viewports = vec![
            DurableViewportRecord {
                viewport_id: 1,
                workspace_ids: vec![10],
                active_workspace_id: Some(10),
            },
            DurableViewportRecord {
                viewport_id: 2,
                workspace_ids: vec![10, 20],
                active_workspace_id: Some(10),
            },
        ];

        assert!(remove_workspace_from_viewport_records(
            &mut viewports,
            1,
            10
        ));
        assert_eq!(viewports.len(), 1);
        assert_eq!(viewports[0].viewport_id, 2);
        assert_eq!(viewports[0].workspace_ids, [10, 20]);

        assert!(remove_workspace_from_viewport_records(
            &mut viewports,
            2,
            10
        ));
        assert_eq!(viewports[0].workspace_ids, [20]);
        assert_eq!(viewports[0].active_workspace_id, Some(20));
    }

    #[test]
    fn durable_workspace_state_round_trips_without_paths_or_entities() {
        let state = DurableAppSessionState {
            memberships: vec![DurableWorkspaceMembership {
                workspace_id: 10,
                viewport_id: None,
                resolution: DurableWorkspaceResolution::Unresolved,
            }],
            viewports: vec![DurableViewportRecord {
                viewport_id: 1,
                workspace_ids: vec![10],
                active_workspace_id: Some(10),
            }],
            active_workspace_by_viewport: Default::default(),
        };

        let json = serde_json::to_string(&state).expect("serialize durable workspace state");
        let restored: DurableAppSessionState =
            serde_json::from_str(&json).expect("deserialize durable workspace state");

        assert_eq!(restored, state);
    }
}
