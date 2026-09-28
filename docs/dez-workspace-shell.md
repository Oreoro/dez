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

**There is exactly one sidebar, and it is the rail.** One scrollable list of
workspace groups — each group's header (name, branch, changed-file count)
followed by the live sessions tied to that group's workspaces: agent threads
(dot marker, labelled with the agent kind) and plain shells (terminal marker,
titled from the terminal's OSC title, falling back to the foreground-process
title the terminal itself derives when the program never set one).

The rail has no Files / Git / Settings tabs. Earlier builds had them, and every
one of them only dispatched a toggle for the corresponding Zed docked panel, so
"switching tabs" opened a second left column beside the rail instead of changing
what the rail showed. Those tools stay in Zed's own panels — reachable through
the View menu and the command palette — and the rail stays one surface.

Sessions live in the rail rather than in a second panel. Three surfaces once
competed to answer "what is running" — the navigator, the Agent Threads panel,
and the terminal tabs — so the rule now is:

- The **rail supervises live sessions**: status, title, actor, and a click that
  focuses the session through the store's own focus path.
- The **Agent Threads panel** keeps only what the rail doesn't do: resuming and
  archiving *historical* sessions. It is reachable from a single quiet
  `Agent thread history` row at the foot of the rail, never as the default way
  to see what is running.
- The **bar** still owns tabs for open surfaces.

Sessions are collected per workspace and never pooled, so a session running in
one workspace can never appear under another workspace's header — the
workspace/terminal isolation dez v0 established survives the port.

States, mapped from `agent_threads` at the presentation layer only (never fork
the state machine):

| Rail state | Agent thread source | Shell source |
|---|---|---|
| `Running` | `ThreadDisplayStatus::Busy` (also the no-signal-yet default) | `ProjectAttentionStatus::Working` |
| `Needs input` | `ThreadDisplayStatus::Blocked` — covers "question" and "permission" until manifest evidence can split them | `ProjectAttentionStatus::Blocked` |
| `Finished` | `ThreadDisplayStatus::Finished` — completed, not yet looked at | `ProjectAttentionStatus::Finished` |
| `Idle` | `ThreadDisplayStatus::Idle` — completed and already looked at | `ProjectAttentionStatus::Idle` |
| `Review-ready` | Not implemented; derived from `git_ui` changed-file counts per thread worktree | — |

Shell state is the same `attention_detection::classify_any` call the Agent
Threads panel's cross-project rollup already makes against a terminal's screen
tail, so the rail and the panel can't disagree about whether a shell needs the
user. The rail observes terminal activity separately from the agent store,
because a shell's state is derived live rather than stored.

Remaining revisions to land:

1. **Session discovery-and-attach** — list tmux/Herdr/cmux sessions and attach
   (read + send input) without owning them. dez attaches to anything; Flint
   manages its own threads. Both coexist.
2. **Cross-project attention rollup in the bar** — the rail header already
   carries a Needs-input rollup; Flint's rollup should also surface as badges
   on workspace headers in the bar.
3. **Review-ready** (see the table above).

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
- [x] Only one path to the Agent Threads panel remains (the rail's footer
      row); the rail itself is the supervision surface
- [x] The Files / Git / Settings rail tabs are gone, so the rail and a docked
      panel can no longer fight over the same column
- [x] Width persistence; `dez_sidebar.starts_open` setting
- [x] Sessions render inline in the rail, nested under their owning workspace
      group, attention-first, with a Needs-input rollup in the rail header
- [x] Per-session states (Running / Needs input / Finished / Idle) projected
      from `agent_threads` at the presentation layer
- [x] Plain shells listed alongside agent threads, from the panel's own
      terminal summaries, so the rail and the panel agree on what is running
- [ ] Surfaces: draggable/splittable tabs in the main work area
- [ ] "Waiting for permission" split out of `Needs input` via manifest evidence
- [ ] Session discovery-and-attach for tmux, Herdr, cmux
- [ ] Review-ready detection from git changed-file counts

## Acceptance for Phase 1

- Parity with dez v0's README feature list (workspace navigator, activity
  states, discovery-attach, multi-root headers).
- Zero `Fix`-of-`Fix` commits during the port.
- `script/dez-identity-check` + `cargo check -p dez_sidebar -p dez_workspace_shell`
  green.
