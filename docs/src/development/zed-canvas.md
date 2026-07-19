# Zed Canvas

Zed Canvas is the product and architecture direction for a pane-first,
agent-aware workspace. It preserves Zed's existing workspace, pane, tab, and
workspace item vocabulary while extending the workspace into a persistent
development multiplexer.

This page is an internal product and architecture specification for Superzed
work. It should guide UI, agent, terminal, persistence, automation, and design
system changes that touch the workspace model.

## Product definition {#product-definition}

Zed Canvas is a native development environment for managing code, terminals,
documentation, browsers, collaboration, and multiple coding agents inside one
persistent workspace.

It combines these behavioral references:

- Zed for native performance, GPUI architecture, language intelligence,
  collaboration, extensions, and existing pane semantics.
- Notion for calm hierarchy, progressive disclosure, readable content, and
  command discovery.
- tmux for sessions, directional panes, zoom, named layouts, detach, attach,
  copy mode, and optional prefix workflows.
- cmux for workspace metadata, agent attention, browser panes, vertical
  navigation, and automation.
- Herdr for real terminal agents, durable sessions, lifecycle visibility,
  reattachment, hooks, and agent-controlled panes.

Do not copy these products visually or by name. Use them as behavioral
references.

## Goals and non-goals {#goals-and-non-goals}

Goals:

- Make parallel agent work understandable at a glance.
- Treat editors, terminals, agents, browsers, documentation, diffs, Git,
  diagnostics, tasks, and collaboration surfaces as equal workspace items.
- Replace rigid side and bottom panels with a composable pane grid.
- Preserve running work across restart, window close, crash, remote changes,
  and supported detach or attach flows.
- Keep application chrome quiet and content primary.
- Support direct manipulation, keyboard workflows, command discovery, and
  safe automation.
- Distinguish authoritative agent state from observed or inferred state.
- Keep Zed's existing editor, language, Git, debugging, task, extension,
  remote, and collaboration capabilities.

Non-goals:

- Recreate Notion, tmux, cmux, or Herdr pixel for pixel.
- Replace full browsers.
- Hide real terminal output behind an agent-specific abstraction.
- Let agents operate invisibly or without visible permission boundaries.
- Convert every surface into a card.
- Require users to understand terminal multiplexers before using the product.
- Claim agent success or process survival from terminal heuristics alone.

## Design principles {#design-principles}

Use these principles when designing or reviewing Canvas work:

- Content before chrome. Code, terminal output, documents, previews, and agent
  work are primary. Controls collapse or move to overflow before content becomes
  unusable.
- One composable canvas. The workspace center is a recursive pane grid. Any
  compatible workspace item can move, split, zoom, float, or join a saved layout.
- Calm by default. Use neutral surfaces and restrained color. Reserve
  saturation and motion for focus, diagnostics, attention, and security.
- Dense, not cramped. High density is acceptable only when grouping, hierarchy,
  hit targets, and readability remain clear.
- Keyboard and pointer parity. Every important operation needs a semantic
  action, Command Palette entry, pointer affordance, context menu where useful,
  and automation API where safe.
- Reversible manipulation. Pane splits, pane closes, tab movement, layout
  changes, settings changes, and agent-applied edits should be undoable or
  restorable where state still exists.
- Honest agent interfaces. Always label whether state is reported by a
  provider, observed from a process, inferred from heuristics, or unknown.

## Conceptual model {#conceptual-model}

```text
Application
└── Window
    └── Workspace
        ├── Session Rail
        ├── Pane Grid
        │   └── Pane
        │       └── Tab
        │           └── WorkspaceItem
        ├── Status Strip
        └── Overlay Layer
```

Application owns global commands, accounts, themes, providers, sessions,
notifications, security policy, automation endpoints, and extension registry.

Workspace owns worktrees, pane-tree geometry, tabs, agents, terminals, tasks,
debugging, browser state, collaboration state, notifications, layout presets,
workspace settings, and restoration metadata.

Pane Grid is a recursive tree:

```text
PaneNode =
  PaneLeaf
  | HorizontalSplit(children, ratios)
  | VerticalSplit(children, ratios)
```

The pane grid must support arbitrary nesting, directional focus, resizing,
equalization, rotation, mirroring, zoom, swap, move, collapse, restoration, and
minimum-size enforcement.

Pane is a spatial container with a tab bar, optional context bar, one active
workspace item, inactive tabs, focus state, attention state, and transient
overlays.

Tab is a persistent reference to a workspace item. Tab metadata can include
title, icon, type, dirty state, process state, attention, Git state, remote host,
agent state, pinned state, and preview state.

WorkspaceItem is the common unit for editor, multibuffer, terminal, structured
agent thread, terminal agent, browser, Markdown preview, diff, Git view,
diagnostic view, task, debugger, media, settings, collaboration, and future
extension-provided items.

## Information architecture {#information-architecture}

### Session Rail {#session-rail}

The Session Rail is a collapsible vertical navigator. It provides orientation
across workspaces and sessions; it is not a fixed content panel.

Possible sections:

- Current workspace.
- Favorites and pinned workspaces.
- Active and recent workspaces.
- Running agents.
- Agent attention.
- Running tasks.
- Collaboration sessions.
- Remote hosts.
- Saved layouts.
- Utility actions.

Workspace entries may show configurable metadata:

- Name, project, repository, working directory, or worktree.
- Git branch, pull request, diagnostics, or dirty state.
- Remote host and connection identity.
- Listening ports.
- Running agents and tasks.
- Participants.
- Active or saved layout.
- Latest attention or notification.

`workspace_bar.show_agent_attention` controls whether visual agent-attention
badges appear at the workspace and rail levels. `session_rail.metadata` remains
the source of truth for whether latest-attention timestamps are shown.
When `session_rail.metadata` contains `layout` or `saved_layout`, project
headers show the active Canvas recipe name and saved-layout count when row
labels are visible.

`workspace_bar.visibility = "hidden"` suppresses the existing layout
command/menu affordances, the centered command-search trigger, and workspace/rail
attention chrome. `workspace_bar.center_command_search` renders a compact
Command Search button that opens the Command Palette when the workspace bar is
visible. `workspace_bar.height` controls the rendered workspace-bar row density:
`minimal`, `compact`, or `comfortable`.

Rail modes:

- Hidden.
- Icon.
- Compact.
- Detailed.
- Overlay.
- Auto.

`session_rail.sort_by` supports attention-first, agent-state, creation-time,
recent-activity, and project row ordering. Project sorting orders thread and
terminal rows by their first worktree label or path inside each project group,
then falls back to recent activity. Manual ordering preserves the visible
thread and terminal row order across rail rebuilds and persists user-authored
row order in sidebar state. `Move Selected Entry Up` and `Move Selected Entry
Down` move the selected thread or terminal within its project group; new rows
fall back to recent activity until explicitly reordered.

`session_rail.visibility = "hidden"` removes the rail from layout. Setting
`visibility` to `icon`, `compact`, or `detailed` overrides the rail display
density directly. Overlay and auto modes share the current rail surface until
distinct render modes are added.

`session_rail.mode = "icon"` narrows the rail and hides row labels/metadata
unless a concrete visibility mode overrides it, `compact` clamps it to a
narrower readable width, and `detailed` keeps a wider minimum width. The first
pass keeps the same row component so status and attention icons remain
consistent across modes.

`session_rail.mode = "always"` keeps the rail open on startup, restore, toggle,
and close actions unless the rail is explicitly hidden.

`session_rail.position` is the effective left/right side for Canvas rail
placement. The existing sidebar side menu writes both `sidebar.side` and
`session_rail.position` for compatibility.

Automatic rail sorting must not move a click or drag target while the user is
interacting with it.

### Pane Canvas {#pane-canvas}

The Pane Canvas is the main work surface.

Rules:

- Project, Git, outline, collaboration, agents, terminals, Markdown previews,
  diffs, diagnostics, settings, and future tool surfaces should be regular pane
  tabs by default.
- The visible default workspace should not reserve fixed left, right, or bottom
  content docks. Compatibility docks may exist internally for migration,
  actions, extensions, and restoration, but Canvas should host their contents as
  movable pane items.
- Runtime Canvas layout flows close every dock returned by the workspace dock
  list after migrating panel contents into pane-hosted tabs. Current bottom
  panel compatibility maps through the existing dock model until a distinct
  bottom-dock entity exists.
- Automatic dock-to-pane migration is controlled by `pane_grid.panel_surface`,
  `pane_grid.draggable_panel_tabs`, and `pane_grid.show_legacy_docks`.
- No item type owns a privileged location.
- Pane-hosted project and agent surfaces retain semantic pane identity for
  compatibility and persistence, but participate in normal tab-host operations
  such as close, move, clone, project/external file drops, and tab drop targets.
- Focused pane receives keyboard actions first.
- Pane borders stay subtle except for focus, drag target, or attention.
- `pane_grid.focus_indicator` controls whether active-pane focus uses the
  existing border treatment or relies on the active tab/title state only.
- `pane_grid.attention_ring = false` suppresses Canvas pane overlay-ring chrome
  even when `pane_grid.focus_indicator` selects a border-style treatment.
- `pane_grid.tab_overflow = "stack"` compacts a pane to pinned tabs plus the
  active unpinned tab and an overflow tab menu. `searchable` keeps the
  scrollable tab strip and adds the same direct tab selector as a stable
  vertical overflow affordance. `scroll` keeps only the horizontal strip.
- `pane_grid.layout_history = true` captures a bounded in-memory snapshot before
  Canvas layout recipes. `Restore Previous Canvas Layout` restores visibility
  and focus for panes that still exist, without deserializing workspace items or
  killing live processes. Runtime snapshots also retain the last applied
  Canvas recipe identity so menu state can follow restores. Recognized active
  recipe ids persist as workspace metadata across restart; persisted semantic
  layout history is still required.
- `Save Canvas Layout: Slot 1/2/3` stores durable saved-layout snapshots with
  pane visibility, focus, and active recipe identity. Each slot persists pane
  intent by pane kind and occurrence order, and the matching restore action
  applies that snapshot to panes that still exist. Saved snapshots now include
  a display label derived from the active Canvas recipe or `Custom Canvas
  Layout`, pane tree shape with split axes and flex weights, tab titles,
  serializable item kind/id when available, tab active, preview, dirty, pinned,
  project-path metadata, and explicit restore-planning intent for serializable,
  project-path, or live-only tabs. The Panel Layout menu shows the derived
  label on restore actions. Restore applies the saved pane-tree shape when
  every saved center pane still exists, without closing panes or relaunching
  processes. Restore now also reopens missing project-path-backed tabs and
  missing serializable tabs into their saved panes in saved order, then
  reapplies saved pinned and active-tab metadata. The Panel Layout menu can
  rename or clear stale fixed slots, save the current layout under a free-form
  name, and restore, rename, or clear free-form named layouts. The saved-layout
  manager modal lists all fixed slots plus named layouts with save-to-slot,
  restore, rename, duplicate, and clear controls, and shows pane/tab counts
  plus restore coverage for project-path, serializable, live-only, pinned, and
  dirty tabs.
  The manager and Panel Layout menu can also clear all saved Canvas layouts
  after a warning confirmation, and copy the saved-layout JSON to the
  clipboard for manual export. Saved-layout JSON can be imported from the
  clipboard; when imported keys conflict with existing saved layouts, the user
  can import only new layouts or replace the conflicts.
  `workspace::RenameSavedCanvasLayoutSlot`,
  `workspace::SaveCurrentCanvasLayoutAs`, and
  `workspace::ManageSavedCanvasLayouts` provide the text-entry and manager
  surfaces. Live process/session restoration is still future work.
- Manual structural layout changes clear the active recipe identity and show
  `Custom Canvas Layout` in the Panel Layout menu while keeping the pane tree
  and tabs intact.
- `pane_grid.auto_reflow` is available at runtime. On recipe application and
  resize, narrow or portrait workspaces reflow horizontal recipe splits into
  vertical splits without closing workspace items. Resize-driven reflow only
  acts on active Canvas recipes and preserves tabs/live processes by inverting
  pane axes when the recipe's desired root orientation changes. Ultrawide
  workspaces reflow vertical-first recipes such as Main Top, Code/Run/Observe,
  Debug, and Even Rows into horizontal-first layouts while preserving the
  explicit Portrait Display recipe. Four-Agent Matrix, Six-Agent Supervisor,
  and Worktree Matrix also prefer column-first split directions when newly
  applied on ultrawide workspaces, and already-open many-agent layouts flatten
  nested rows into existing horizontal columns on ultrawide resize without
  closing panes or tabs. When those already-open layouts leave ultrawide, they
  reshape the same pane entities back into the recipe's nested matrix, or a
  vertical stack for narrow/portrait workspaces.
- Empty panes offer quick item creation and restore actions.
- Zoom preserves underlying layout.
- Layout modifications persist continuously.
- Narrow windows reflow without closing workspace items.

Implementation boundary:

- Current recipes are geometry-only and reveal existing pane-hosted surfaces.
  They must not spawn agents, terminals, browsers, or external processes.
- Saved-layout support currently provides three fixed durable slots plus
  free-form named layouts with built-in text entry and an inspectable manager
  modal with duplicate, confirmation-backed clear-all, and copy-to-clipboard
  JSON export/import workflows. Actual process restoration remains future work.
  Saved layouts use pane-tree metadata to reshape existing panes, reopen
  project-path-backed and serializable tabs, and store restore-planning intent
  for live-only/process-backed tabs.
- Process lifetime stays separate from tab lifetime so closing a tab is not
  silently treated as killing or resuming a process.

### Status Strip {#status-strip}

The Status Strip shows workspace-wide or focused-item state only. Examples:
mode, Git branch, diagnostics, language server state, remote host, collaboration
state, task progress, agent totals, security boundary, cursor position,
encoding, and line endings.

Do not duplicate the Pane Context Bar.

### Overlay Layer {#overlay-layer}

The Overlay Layer hosts temporary UI: Command Palette, Quick Open, workspace
switcher, tab switcher, layout picker, slash menu, notifications, permission
prompts, context menus, tooltips, and modals.

## Navigation and manipulation {#navigation-and-manipulation}

Focus moves through:

```text
Application → Window → Workspace → Pane → Tab → WorkspaceItem → Local control
```

Escape should move outward predictably. It should not close unrelated state.

Pane focus uses spatial adjacency:

```text
pane::FocusLeft
pane::FocusRight
pane::FocusUp
pane::FocusDown
pane::FocusPrevious
pane::FocusLastActive
```

Pane manipulation actions:

```text
pane::SplitLeft
pane::SplitRight
pane::SplitUp
pane::SplitDown
pane::CloseActiveItem (alias: pane::Close)
pane::ReopenClosedItem (aliases: pane::RestoreClosed, tab::Reopen)
workspace::ToggleZoom (alias: pane::Zoom)
pane::Swap { direction: "left|right|up|down" }
pane::Move { direction: "left|right|up|down" }
pane::Rotate
workspace::ResetPaneSizes (alias: pane::Equalize)
pane::Resize { direction: "left|right|up|down" }
pane::BreakToWindow
pane::JoinFromWindow
```

Generic Canvas aliases for `pane::BreakToWindow` and `pane::JoinFromWindow`
remain future until they have exact action payloads or runtime implementations.
`pane::Swap`, `pane::Move`, and `pane::Resize` default to `"right"` when the
direction payload is omitted. `pane::Rotate` inverts horizontal and vertical
axes across the center pane tree.

Tab manipulation actions:

```text
workspace::NewFile (alias: tab::New)
pane::CloseActiveItem (alias: tab::Close)
pane::ReopenClosedItem (alias: tab::Reopen)
pane::TogglePinTab (alias: tab::Pin)
pane::SwapItemLeft (alias: tab::MoveLeft)
pane::SwapItemRight (alias: tab::MoveRight)
pane::DeploySearch (alias: tab::Search)
workspace::MoveItemToPane (alias: tab::MoveToPane)
workspace::MoveItemToPaneInDirection
tab::Duplicate
```

`tab::Duplicate` uses the same clone path as split/clone and only duplicates
tabs whose item type supports cloning. `tab::MoveToWorkspace` remains a future
Canvas command alias until exact runtime behavior exists.

Layout actions:

```text
workspace::ApplyCanvasLayoutRecipe { name: "four_agent_matrix" }
workspace::CycleCanvasLayout
```

`workspace::ApplyCanvasLayoutRecipe` accepts the same normalized recipe names
and aliases used by `multiplexer.layout_cycle`, no-ops on unknown names, and
preserves the current workspace/window mode instead of forcing title-bar Canvas
chrome.

An optional tmux-style Prefix Mode may exist for experts, but ordinary users
must not need it. Prefix Mode needs a visible mode indicator, configurable
prefix, timeout, fixed-step resize commands, prefix pass-through, and command
discovery. The first visible indicator is implemented as a
compact title-bar chip that appears while GPUI is waiting for the next
multi-stroke key.

The runtime settings model now reads `multiplexer.prefix_mode`,
`multiplexer.prefix`, `multiplexer.prefix_timeout_ms`, and
`multiplexer.broadcast_confirmation`; the Panel Layout menu shows the
configured prefix, prefix timeout, and broadcast policy when prefix mode is
enabled. When `multiplexer.prefix_mode = true`, workspace key context exposes
`canvas_prefix_mode` and the default keymaps bind the default `ctrl-b` prefix to
core Canvas commands. The Panel Layout menu also shows disabled discovery rows
for the default prefix command set, and the title bar shows a `PREFIX …` chip
while a multi-stroke prefix sequence is pending:

- `ctrl-b space` cycles Canvas layouts.
- `ctrl-b a` applies Agent Control.
- `ctrl-b f` applies Focus Editor.
- `ctrl-b m` applies Four-Agent Matrix.
- `ctrl-b s` saves Canvas layout slot 1.
- `ctrl-b r` restores Canvas layout slot 1.
- `ctrl-b shift-1/2/3` saves Canvas layout slots 1/2/3.
- `ctrl-b 1/2/3` restores Canvas layout slots 1/2/3.
- `ctrl-b n 1/2/3` renames Canvas layout slots 1/2/3.
- `ctrl-b n m` opens the Canvas saved-layout manager.
- `ctrl-b n s` saves the current Canvas layout under a free-form name.
- `ctrl-b p` restores the previous Canvas layout snapshot.
- `ctrl-b left/down/up/right` focuses adjacent panes.
- `ctrl-b shift-left/down/up/right` swaps adjacent panes.
- `ctrl-b alt-left/down/up/right` moves the active pane to the workspace edge.
- `ctrl-b v` splits the active pane to the right.
- `ctrl-b enter` splits the active pane downward.
- `ctrl-b h/j/k/l` resizes the active pane left/down/up/right in fixed steps.
- `ctrl-b =` equalizes pane sizes without changing the split tree.
- `ctrl-b ctrl-b` sends the default prefix through to the focused item.

`multiplexer.prefix_timeout_ms = 0` disables timeout replay for incomplete
prefix sequences. Custom prefix strings still require user keymap overrides,
and dynamic remapping remains future work.

## Layout system {#layout-system}

Built-in named layouts should include:

| Layout      | Shape                             | Use                    |
| ----------- | --------------------------------- | ---------------------- |
| Focus       | One zoomed pane                   | Editing or review      |
| Columns     | Equal vertical panes              | Parallel agents        |
| Rows        | Equal horizontal panes            | Logs and tasks         |
| Main Left   | Large left with stacked right     | Editor plus tools      |
| Main Right  | Stacked left with large right     | Preview or agent focus |
| Main Top    | Large top with bottom stack       | Editor over terminals  |
| Main Bottom | Top stack with large bottom       | Terminal-heavy work    |
| Grid        | Balanced matrix                   | Agent fleet            |
| Dashboard   | Summary plus detail panes         | Monitoring             |
| Review      | Diff, files, tests, agent         | Change review          |
| Debug       | Editor, stack, variables, console | Debugging              |
| Research    | Browser, notes, agent, terminal   | Investigation          |

Saved layouts should store semantic slots, not only exact tabs. A review layout,
for example, can accept `diff`, `editor`, `git`, `project`, `terminal`, `task`,
or `agent` items by role.

The first runtime recipe set exposes direct actions for Full, Agent Control,
Focus Editor, Even Columns, Even Rows, and the broader geometry-only Canvas
starter layouts, with cycling controlled separately by `multiplexer.layout_cycle`.
The saved-layout slot actions record and restore durable Canvas snapshots
without spawning, closing, or recreating workspace items.

Responsive rules:

- Preserve focused pane first.
- Collapse the Session Rail before hiding active work.
- Move low-priority controls into overflow.
- Stack panes when minimum widths cannot be maintained.
- Never silently close workspace items.
- Offer temporary single-pane mode on narrow screens.
- Restore prior geometry when enough space returns.

Persist:

- Tree structure and split ratios.
- Active pane and active tabs.
- Pinned tabs.
- Zoom state.
- Item restoration metadata.
- Scroll positions where safe.
- Terminal and agent session identifiers.
- Remote connection metadata.

## Agent experience {#agent-experience}

Canvas supports three agent item types:

- Structured Agent Thread. Use when a provider exposes messages, tools, usage,
  permissions, and edits as structured events.
- Agent Terminal. Use for terminal-native agents. It contains a real PTY,
  session metadata, an adapter layer, attention state, detach controls, and no
  rewritten terminal output.
- Agent Dashboard. A compact operational list of many sessions with state,
  task, provider, workspace, duration, latest event, changes, tests,
  permissions, and attention.

Agent states:

```text
starting
working
waiting
blocked
paused
idle
completed
failed
disconnected
restoring
unknown
```

Each displayed state must include provenance:

```text
reported
observed
inferred
unknown
```

Examples:

- `Waiting for input · Reported by provider`.
- `Working · Inferred from terminal activity`.
- `Disconnected · Transport unavailable`.

Attention levels:

| Level    | Meaning                                | Treatment                |
| -------- | -------------------------------------- | ------------------------ |
| Quiet    | Background activity                    | No interruption          |
| Active   | Work is running                        | Stable icon              |
| Notice   | Useful new output                      | Badge                    |
| Action   | Input or permission required           | Attention ring and queue |
| Critical | Security, data loss, or severe failure | Explicit alert           |

Agent composers support multiline instructions, file and symbol references,
image attachments, terminal-output references, pane references, workspace
references, reusable prompt commands, permission profile selection, model
selection, queueing, stop, interrupt, and voice input where supported.

Tool events should be compact, expandable rows. Expanded details may show
sanitized arguments, output, error, affected files, command, exit status, and
timing.

`agent_ui.group_tool_calls` groups adjacent visible tool-call cards with a
subtle transcript rail, keeping bursts of tool activity scannable without
changing provider events or execution order.

`agent_ui.event_verbosity = "summary"` suppresses completed generic tool events
that have no content or raw input. Safety- or state-bearing events remain
visible: subagents, terminal tools, edits, pending work, permission prompts,
failures, cancellations, and rejected calls.

`agent_ui.keep_permissions_expanded` controls whether generic permission
details default open. Approval and rejection controls stay visible either way;
sandbox and confusable-warning details stay expanded so users can inspect the
scope before granting access.

`agent_ui.presentation` controls agent transcript density. `compact` tightens
user and assistant message spacing for multi-pane agent lanes, `chat` keeps a
conversational middle density, and `document` preserves the roomier default
layout.

Agent changes must support file-grouped review, unified or split diffs, hunk
acceptance, hunk rejection, file acceptance, full rollback, conflict state, test
linkage, commit preparation, and attribution. Proposed and applied changes must
never be mixed without explicit labels.

Permission prompts must show scope and consequence. Decision options include
allow once, allow for thread, allow for workspace, always allow matching rule,
deny, deny and explain, and edit command or scope.

## Terminal and TUI behavior {#terminal-and-tui-behavior}

Terminal requirements:

- Real PTY or ConPTY.
- GPU-accelerated rendering where available.
- Shell integration.
- Links, images where supported, search, copy mode, and scrollback.
- Semantic command blocks.
- Working-directory tracking.
- Exit-state reporting.
- Remote-host identity.
- Session persistence.
- Detach and reattach where supported.
- Safe process termination.

TUI compatibility rules:

- Do not intercept application keystrokes while the terminal owns focus.
- Support alternate-screen applications.
- Preserve mouse reporting.
- Distinguish terminal selection from application selection.
- Provide an escape action back to pane navigation.
- Do not paint overlays into terminal cells.
- Ensure resize sends correct terminal dimensions.

Synchronized input is allowed only through explicit Broadcast Groups. Broadcast
Mode requires visible target indicators, a previewable target list, environment
warnings, and immediate cancellation.

## Browser and collaboration {#browser-and-collaboration}

Browser workspace items may include address field, navigation controls,
security state, viewport controls, DevTools entry, inspect mode, external-open,
agent-control indicator, and permission state.

Agents may control browser panes only with visible, interruptible activity and
permission boundaries.

Collaboration extensions should support shared pane focus, present-follow mode,
shared terminals with explicit host permission, shared agent threads,
agent-action attribution, per-participant permissions, shared layouts, voice
indicators, collaboration-aware notifications, local-only private panes, and
clear filesystem trust boundaries.

## Persistence, automation, security, and performance {#runtime-requirements}

Durable entities:

```text
WorkspaceSession
PaneTree
PaneState
TabState
WorkspaceItemState
TerminalSession
AgentSession
BrowserSession
LayoutPreset
NotificationRecord
PermissionGrant
```

Restoration states:

```text
restored
restoring
partially_restored
requires_authentication
provider_unavailable
remote_unreachable
process_ended
incompatible_version
failed
```

Recovery rules:

- Save layout changes incrementally.
- Use atomic snapshots and last-known-good snapshots.
- Separate visual restoration from process restoration.
- Never claim a process survived unless its backend confirms it.
- Show failed items in place with retry, replace, and remove actions.
- Preserve terminal scrollback where security policy permits.
- Redact secrets from logs and restoration metadata.

Automation should support querying workspaces and pane trees, splitting and
focusing panes, opening items, starting terminals and agents, sending bounded
terminal input, reading bounded terminal output, querying agent state, waiting
for transitions, opening and inspecting browser panes, reading notifications,
applying layouts, and capturing workspace metadata.

Automation safety requirements:

- Authenticate local clients.
- Use explicit capabilities scoped to workspace and action.
- Log automation mutations.
- Require consent for credentials, network access, and destructive operations.
- Limit output size and history.
- Redact secrets.
- Prevent background automation from stealing focus unless requested.

Performance expectations:

- Typing must remain independent of agent streaming.
- Terminal rendering must not block the UI thread.
- Inactive panes should reduce rendering work.
- Browser panes hidden from view should throttle.
- Agent event lists should virtualize.
- Large scrollback should use bounded memory.
- Persistence should be incremental.
- Failed item restoration must not block the workspace.

## Implementation direction {#implementation-direction}

Prefer evolving existing Zed structures before adding parallel models. The
current workspace already has panes, items, serialization, actions, docks,
terminal metadata, and agent tabs. Canvas work should unify those surfaces
behind common contracts.

Suggested entities:

```text
CanvasWorkspace
PaneTreeModel
PaneModel
TabModel
WorkspaceItemRegistry
SessionRegistry
AgentRuntime
TerminalRuntime
BrowserRuntime
AttentionStore
LayoutStore
AutomationServer
PermissionStore
```

The common item contract should expose:

```text
id
title
icon
kind
focus handle
serialized state
attention state
close disposition
context actions
layout hints
```

Keep these lifetimes separate:

- UI item lifetime.
- Process lifetime.
- Provider-session lifetime.
- Remote-transport lifetime.
- Restoration-record lifetime.

Closing a tab must not automatically terminate a durable process unless the
configured close policy says so.

## Acceptance criteria {#acceptance-criteria}

- Any compatible item can occupy any pane.
- Users can split, resize, move, zoom, and restore panes.
- Layouts survive restart.
- Failed item restoration does not block the workspace.
- Multiple agents can run concurrently.
- Agent states are visible from the rail and dashboard.
- Reported, observed, inferred, and unknown states are distinguishable.
- Permission requests identify scope and consequence.
- Proposed and applied agent changes remain separate.
- Full-screen TUIs work correctly.
- Detach does not terminate durable sessions where the backend supports it.
- Application shortcuts do not corrupt terminal input.
- Synchronized input is visibly indicated.
- Components use semantic tokens.
- State is never communicated by color alone.
- `accessibility.announce_agent_attention` controls OS/window attention
  requests from structured agents and detected terminal agents. In-app
  notification popups remain controlled by agent notification settings.
- Motion respects reduced-motion settings. `accessibility.reduced_motion`
  overrides the root motion setting for Canvas: `reduced` forces reduced
  motion, `full` permits full motion, and `system` follows the existing
  root-level `reduce_motion` setting.
- Pane and item operations are scriptable.
- Automation is authenticated, capability-scoped, and auditable.
- Shared and private collaboration surfaces are visibly distinct.
