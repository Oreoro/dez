# dez Workspace Shell — design spec

*Status: draft for Phase 1. Written before code, per the Flint method.*

## Purpose

dez organizes the workspace around agent sessions the way a browser organizes
itself around tabs. The workspace shell is dez's flagship differentiator: a
Chrome-like bar and sidebar that make every workspace surface — files, diffs,
terminals, agent threads, settings — a first-class, movable, addressable tab.

The agent owns its own config (Flint rule, inherited): dez never becomes
another native AI client. dez owns *the workspace*.

## Product shape

First screen: a workspace whose primary chrome is the **bar** (top) and the
**sidebar** (left), not a project panel with tabs bolted on.

### The bar (Chrome-like)

- Tabs for every open surface: editors, diffs, terminals, agent threads,
  Home, Files, Git, Settings.
- Tabs are draggable, splittable into the main work area ("surfaces").
- Each tab shows live status where applicable: agent threads show their
  attention state (Working/Idle/Blocked) as a badge; git tabs show changed-file
  counts; multi-root workspaces show branch name + changed count in the header.
- New-tab affordance opens the quick-open palette.

### The sidebar

Tabs (Home / Files / Git / Settings) live in the sidebar as workspace-owned
surfaces, per dez v0's navigator, with these revisions:

1. **Activity section** — agent sessions with states:
   - `Running`, `Needs Input`, `Waiting for Permission` mapped from
     `agent_threads::AttentionState::{Working, Blocked}` (Blocked refines into
     Needs Input vs Waiting for Permission via terminal-bell/manifest evidence;
     presentation-layer only, never fork the state machine).
   - `Review-ready`: NEW — derived from `git_ui` changed-file counts per
     thread worktree.
2. **Session discovery-and-attach** — list tmux/Herdr/cmux sessions and attach
   (read + send input) without owning them. dez attaches to anything; Flint
   manages its own threads. Both coexist.
3. **Cross-project attention rollup** — Flint's existing rollup surfaces as
   badges on workspace headers in the bar.

## Non-goals

- No model/provider/auth UI (agents own their config).
- No chat rendering, no transcript persistence beyond session state.
- No fork of the attention state machine — presentation mapping only.
- No new settings surface beyond `workspace_shell` basics.

## Architecture

```
crates/dez_workspace_shell/     # bar + surfaces + workspace tabs
crates/dez_sidebar/             # navigator / activity / discovery / headers
  navigator.rs                  # Home/Files/Git/Settings tabs
  surfaces.rs                   # movable/splittable main work area
  activity.rs                   # session states via agent_threads
  discovery.rs                  # tmux/Herdr/cmux attach
  headers.rs                    # multi-root workspace headers
```

- Flint already registers `TerminalView` as a serializable center-pane
  workspace item — extend that registration pattern for all shell surfaces.
- All dez logic stays inside `dez*` crates. Touches to `workspace`/`terminal_view`
  are ≤50-line marked hooks, each listed in `FORK.md`.
- The old dez `sidebar.rs` (21,471 lines) is the reference implementation but
  is ported module-by-module, split, and re-pointed from ACP
  (`acp_thread::ThreadStatus`, `agent::ThreadStore`, `agent_ui::*`) onto
  `agent_threads` (see MERGE.md §2.2 mapping). Tests port per-module;
  ACP-plumbing tests are deleted.

## Implementation status

- [x] `dez_sidebar` crate implementing `workspace::Sidebar` (placement,
      resizing, and persistence owned by `MultiWorkspace`)
- [x] Browser-like view model: Home / Files / Git / Settings
- [x] Labeled tab strip (icon + label) over the four views
- [x] Home navigator over project groups with joined names and active branch
- [x] Workspace rows activate their workspace
- [x] Workspace headers show the active branch plus a changed-file count with
      a conflict/modified/added/deleted/clean status indicator
- [x] Files / Git / Settings views list their workspace actions and dispatch
      the corresponding Zed panel or settings actions
- [x] Git view surfaces the Git Graph (`git_ui::git_graph`) next to Changes;
      this is Flint's graph, the same lineage as Gram's `git_graph`
- [x] Activity entry point in Home opens the Agent Threads panel and mirrors
      the panel's attention rollup count
- [x] Width and active-view persistence; `dez_sidebar.starts_open` setting
- [ ] Surfaces: draggable/splittable tabs in the main work area
- [ ] Activity section with per-thread states (Running / Needs Input /
      Waiting for Permission / Review-ready)
- [ ] Session discovery-and-attach for tmux, Herdr, cmux
- [ ] Review-ready detection from git changed-file counts

## Acceptance for Phase 1

- Parity with dez v0's README feature list (workspace navigator, activity
  states, discovery-attach, multi-root headers).
- Zero `Fix`-of-`Fix` commits during the port.
- `script/dez-identity-check` + `cargo check -p dez_sidebar -p dez_workspace_shell`
  green.
