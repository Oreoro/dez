# Superzed Canvas implementation plan

This plan integrates the Superzed upstream merge request with the Zed Canvas
product and design-system specifications:

- [Zed Canvas](./zed-canvas.md)
- [Zed Canvas design system](./zed-canvas-design-system.md)

Current phase: Canvas foundation and native defaults. Avoid full builds until
the user asks.

Implemented in this phase:

- Merged latest `zed-industries/zed:main` into the Superzed work branch.
- Added Canvas settings families:
  `design_system`, `workspace_bar`, `session_rail`, `pane_grid`, `agent_ui`,
  `multiplexer`, and `accessibility`.
- Added the Canvas layout action that syncs dock panels into regular pane tabs
  and closes legacy dock chrome.
- Canvas recipes and pane-grid application now close legacy dock chrome through
  the workspace dock list instead of special-casing left/right docks, keeping
  the default Canvas surface pane-first as bottom-dock support evolves.
- Added native Lumin theme assets and made `Lumin Blur` / `Lumin Light` the
  default dark/light pair.
- Added native JetBrains Mono font assets for editor, terminal, Markdown code,
  and code-like agent surfaces.
- Added schema/default switches for pane-tab panel hosting, terminal-agent
  detection, multiple visible agents, session restoration, agent attention
  notifications, and preview-first Markdown.
- Registered Markdown preview as the default project item opener for Markdown
  files while preserving source access through the embedded editor and source
  return action.
- Panel-hosted project and agent surfaces now use the normal pane tab bar and
  close their pane when emptied, so they behave like regular
  draggable/closable workspace tabs.
- Project and agent panel panes now keep their semantic pane kind for
  migration/persistence, but opt into normal tab-host interactions: close
  active item, close matching items, take active item, tab/body drop targets,
  project/external file drops, explicit add-item routing, project-item opens,
  shared-screen opens, and move/clone item helpers.
- Session Rail terminal rows now classify known terminal agent CLIs from title
  metadata, including Claude Code, Codex, Gemini CLI, Aider, Agy, OpenCode,
  Amp, Crush, Devin, Droid, Goose, Grok, OpenHands, Pi, Qwen Code, Cursor
  Agent, and GitHub Copilot.
- Standalone terminal tabs whose titles identify a known agent CLI now appear
  in the Session Rail, and generic-title tabs can also surface when their live
  foreground command identifies a known non-ambiguous agent CLI. Activating the
  rail row focuses the existing terminal tab, and closing the row closes that
  tab instead of spawning or restoring an Agent Panel terminal.
- Standalone agent terminal bell state now marks the Session Rail row as
  notified and clears when the row is activated.
- Added command-palette/menu actions for Canvas layout recipes:
  `Canvas: Full`, `Canvas: Agent Control`, `Canvas: Focus Editor`, and
  `Cycle Canvas Layout`.
- Added additional named Canvas recipe actions for Main + Stack, Main Top,
  Golden Split, Code/Run/Observe, Agent Operations Center, and Four-Agent
  Matrix.
- Added additional geometry-only starter layout actions for Review, Debug,
  Documentation Studio, Browser Development, Six-Agent Supervisor, Worktree
  Matrix, Remote Operations, Pair Programming, Incident Response, and Portrait
  Display. These recipes create or reuse pane geometry only; they do not spawn
  agents, terminals, browsers, or external processes.
- Added direct `Canvas: Even Columns` and `Canvas: Even Rows` layout actions so
  the default cycle recipes are also selectable without cycling.
- Canvas/tmux command aliases now resolve for close and reopen workflows:
  `pane::Close` aliases `pane::CloseActiveItem`, while `pane::RestoreClosed`
  and `tab::Reopen` alias `pane::ReopenClosedItem`.
- Additional tab command aliases now resolve for the implemented Canvas tab
  workflow: `tab::New`, `tab::Close`, `tab::Pin`, `tab::MoveLeft`,
  `tab::MoveRight`, `tab::MoveToPane`, and `tab::Search`.
- Pane command aliases now resolve for exact zoom and equalize workflows:
  `pane::Zoom` aliases `workspace::ToggleZoom`, and `pane::Equalize` aliases
  `workspace::ResetPaneSizes`.
- Generic payload-based Canvas pane commands now resolve for directional
  workflows: `pane::Swap`, `pane::Move`, and `pane::Resize` accept
  `{"direction":"left|right|up|down"}` and reuse the existing directional pane
  implementations.
- `pane::Rotate` now inverts horizontal and vertical axes across the center
  pane tree while preserving existing panes and tabs.
- `workspace_bar.show_layout` now controls whether Canvas layout commands show
  in the Command Palette and Panel Layout chrome.
- `workspace_bar.show_agent_attention` now controls workspace-level attention
  badges plus Session Rail thread, terminal, and collapsed-project attention
  markers; `session_rail.metadata` still controls latest-attention timestamps.
- Runtime workspace bar settings now expose `visibility`, `height`, and
  `center_command_search`; `workspace_bar.visibility = "hidden"` gates the
  existing layout command/menu affordances, centered command-search trigger, and
  workspace/rail attention chrome. `center_command_search` now opens the Command
  Palette from the workspace bar when visible, and `height` controls the
  rendered workspace-bar row density.

## Ground truth {#ground-truth}

- Local repository is `maxktz/superzed` on `main`.
- The fork is ahead of upstream Zed with existing Superzed work:
  branding, isolated app data directories, no self-update behavior, pane-card
  polish, sidebar changes, terminal and debugger tab work, project and agent
  panel-to-pane work, scrollbar changes, and Superzed defaults.
- The fork is also behind `zed-industries/zed:main`. Before implementation,
  fetch enough history to use the real merge base, then merge latest upstream
  main.
- The current fork already contains important foundations:
  `PanelItem`, `PanelPaneKind`, project and agent panel pane hosting, terminal
  metadata storage, terminal-agent command detection, agent thread tabs,
  Markdown preview serialization, and Lumin-compatible blur theming support.

## Product direction {#product-direction}

Superzed should become the opinionated Zed Canvas build:

- Pane-first and tab-first.
- Agents, terminals, editors, diffs, Markdown previews, browsers, project
  trees, Git, diagnostics, and settings are all WorkspaceItems.
- Traditional side and bottom dock panels are compatibility plumbing, not the
  default visible model.
- Session Rail gives orientation across workspaces, agents, tasks, remotes,
  collaboration, and saved layouts.
- Agent state is visible, jumpable, and honest about whether it is reported,
  observed, inferred, or unknown.
- Terminal-native agents stay real PTYs. Superzed must not replace TUI output
  with a lossy transcript.
- Layouts are named, semantic, restorable, and responsive.
- Automation is semantic, authenticated, capability-scoped, bounded, and
  auditable.

## Merge and branch policy {#merge-and-branch-policy}

1. Preserve the current Superzed fork work on a branch.
2. Add or update `upstream` for `https://github.com/zed-industries/zed.git`.
3. Fetch enough fork and upstream history to resolve the real merge base.
4. Merge latest upstream main into Superzed.
5. Resolve conflicts by preserving:
   - Superzed branding and app identity.
   - Isolated config, cache, and data directories.
   - No automatic self-update.
   - Existing pane-first fork work.
   - Existing terminal-agent and agent-thread work.
6. Audit experimental upstream branches by relevance, not by count.

Relevant branch families:

- Agent: `agent-*`, `add-simple-multi-agent-running`,
  `multi-agent-multi-workspace-keyboard`, `terminal-thread`, `subagents`,
  `background-agent`, and related ACP branches.
- Pane and layout: `add-layout-components`, `*layout*`, `*pane*`, panel
  hosting, project-panel, and git-panel layout branches.
- Terminal: `terminal-restore*`, `interactive-terminal`,
  `terminal-osc-133`, `extended-terminal-layout`, and terminal quality branches.
- Markdown: `markdown-*`, preview, parser, image, link, and rendering branches.
- Theme and UI: theme variables, HSL support, font fallback, scrollbar, and
  pane chrome branches.

Skip unrelated, stale, duplicate, or conflicting branches unless a commit
directly supports the Canvas plan.

## Foundation implementation {#foundation-implementation}

Create a Canvas foundation without replacing existing Zed primitives:

- Add schema-backed defaults for Canvas layout settings:
  `design_system`, `workspace_bar`, `session_rail`, `pane_grid`, `agent_ui`,
  `multiplexer`, and `accessibility`.
- Introduce a clear WorkspaceItem contract around existing `Item` behavior:
  item kind, title, icon, focus, layout hints, attention state, close
  disposition, serialization, and context actions.
- Standardize Pane Frame, Pane Tab Bar, optional Pane Context Bar, Empty Pane,
  and attention ring behavior.
- Make tab overflow searchable before tab titles become unreadable.
- Add pane layout history and restorable snapshots.
- Add layout hints for minimum size, compact support, overlay support, stacking,
  priority, and preferred aspect.
- Keep focus actions spatial and deterministic.
- Add Pane Map and pane numbering overlay after the base pane model is stable.

## Visual defaults and design system {#visual-defaults-and-design-system}

Vendor visual defaults as native Superzed assets:

- Bundle the Lumin theme as a native theme with license attribution.
- Default dark theme to `Lumin Blur`.
- Default light theme to `Lumin Light` unless a better light Canvas theme is
  added.
- Bundle JetBrains Mono with license attribution.
- Default editor, terminal, agent buffer, Markdown code, and numeric/code-like
  surfaces to JetBrains Mono where appropriate.

Apply Lumin carefully:

- Use blur and transparency for the window and stable surfaces.
- Avoid blur on large continuously updating terminal or agent surfaces.
- Avoid turning adjacent panes into independent rounded cards.
- Preserve strong enough focus, attention, diagnostic, Git, and security state.
- Use semantic tokens for permanent colors.
- Keep high-contrast and reduced-motion modes first-class.

## Pane-first workspace {#pane-first-workspace}

Use the existing panel-as-pane bridge as the migration path:

- Proper Pane Layout means the visible workspace is the pane grid. Side and
  bottom docks are compatibility plumbing and should not be visible in the
  default Canvas layout.
- Keep panel registration internally so actions, persistence, and plugins keep
  working.
- Host project, Git, outline, collaboration, agent, terminal, and future tool
  surfaces as pane tabs by default.
- Agent pane-hosted panels show a tab bar by default so agent sessions behave
  like regular workspace items instead of a hidden singleton surface.
- Project and agent panel panes should use the default pane tab bar. Avoid
  custom icon-only panel headers unless they preserve normal tab drag, close,
  context-menu, and keyboard behavior.
- The Canvas layout action syncs registered dock panels into pane-hosted tabs,
  opens the project and agent pane surfaces, hides legacy left and right docks,
  and returns focus to the last editor pane.
- Hide legacy left, right, and bottom dock UI in the default Canvas layout.
- Keep a compatibility setting or action to restore the traditional panel model
  during migration.
- Default `pane_grid.panel_surface` to `pane_tab`, default
  `pane_grid.show_legacy_docks` to `false`, and keep
  `pane_grid.draggable_panel_tabs` enabled.
- Runtime startup applies `pane_grid.show_legacy_docks = false` after session
  restore by migrating dock-hosted panels into pane tabs and closing legacy dock
  chrome.
- Runtime panel migration and Canvas recipes now require
  `pane_grid.panel_surface = "pane_tab"`, `pane_grid.draggable_panel_tabs =
  true`, and `pane_grid.show_legacy_docks = false`; opting out disables
  automatic panel-pane creation without deleting existing manual pane state.
- Runtime active-pane border rendering now honors `pane_grid.focus_indicator`:
  `title` suppresses the border, while `border`, `border_and_title`, and
  `ring` use the existing active-pane border treatment.
- Runtime pane overlay-ring rendering now also honors
  `pane_grid.attention_ring`; disabling it suppresses Canvas pane ring chrome
  even when `pane_grid.focus_indicator` selects a border-style treatment.
- Runtime pane rendering now honors `pane_grid.auto_hide_single_tab_bar`; the
  Canvas default remains `false` so single-tab panes still advertise their
  draggable tab surface.
- Runtime tab-bar rendering now reads `pane_grid.tab_overflow`: `stack` shows
  pinned tabs plus the active unpinned tab with a tab menu for direct
  activation, `searchable` keeps the scrollable tab strip and adds the same
  direct tab selector as a stable vertical overflow affordance, and `scroll`
  retains the existing scrollable strip.
- Runtime pane-grid settings now read `layout_history` and `auto_reflow`.
  `layout_history` captures a bounded in-memory snapshot before each Canvas
  recipe and exposes `Restore Previous Canvas Layout`, which restores visibility
  and focus for still-existing panes without recreating items or terminating
  live processes. The Panel Layout menu disables that restore action until a
  previous Canvas layout snapshot exists and shows the current in-memory
  snapshot count. Full persisted semantic layout history and user-authored
  responsive profile rules remain future work.
- The Panel Layout menu now exposes three durable saved-layout slots. Each
  `Save Canvas Layout: Slot N` action records the current pane visibility,
  focus, and active recipe identity as a semantic snapshot by pane kind and
  occurrence order, and each matching restore action restores it for
  still-existing panes without recreating items or touching live processes.
- First `pane_grid.auto_reflow` behavior is implemented for Canvas recipe
  application and resize: when the workspace is narrow or portrait, horizontal
  recipe splits reflow into vertical splits. Active Canvas recipes invert the
  pane tree axes on resize only when the recipe's desired root orientation
  changes, preserving tabs and live processes while avoiding squeezed columns.
  Ultrawide workspaces now reflow vertical-first recipes such as Main Top,
  Code/Run/Observe, Debug, and Even Rows into horizontal-first layouts while
  preserving the explicit Portrait Display recipe. Four-Agent Matrix,
  Six-Agent Supervisor, and Worktree Matrix also prefer column-first split
  directions when newly applied on ultrawide workspaces, and already-open
  many-agent layouts flatten nested rows into existing horizontal columns on
  ultrawide resize, then reshape those same panes back into the recipe matrix
  or narrow vertical stack when the workspace leaves ultrawide.
- Remove the one-visible-agent bottleneck. Multiple agent tabs and terminal
  agents can be visible across panes and windows.
- Support direct pane/tab dragging, keyboard movement, context-menu movement,
  and Command Palette movement.
- Implemented first layout recipe actions:
  - Full Canvas shows both project and agent panel panes and hides legacy dock
    chrome.
  - Agent Control shows project and agent panes and focuses the agent pane.
  - Focus Editor hides project and agent panes and focuses the editor/tab pane.
  - Cycle Canvas Layout toggles between agent-control and focus-editor modes.
- Additional recipe actions now create or reuse regular tabbed panes for
  `Main + Stack`, `Main Top`, `Golden Split`, `Code/Run/Observe`,
  `Review`, `Debug`, `Documentation Studio`, `Browser Development`,
  `Agent Operations Center`, `Four-Agent Matrix`, `Six-Agent Supervisor`,
  `Worktree Matrix`, `Remote Operations`, `Pair Programming`,
  `Incident Response`, and `Portrait Display` without starting processes or
  replacing tab contents.
- `Cycle Canvas Layout` now honors `multiplexer.layout_cycle`, including the
  default `even_columns`, `even_rows`, `main_left`, `main_top`, `tiled`, and
  `agent_control` recipe names, with tolerant matching for label-style names.
- Runtime multiplexer settings now read `prefix_mode`, `prefix`,
  `prefix_timeout_ms`, `layout_cycle`, and `broadcast_confirmation`. When
  prefix mode is enabled, the Panel Layout menu shows the configured prefix,
  prefix timeout, and broadcast confirmation policy as a disabled status row.
  Workspace key context now exposes
  `canvas_prefix_mode`, and the default keymaps bind the default `ctrl-b`
  prefix to core Canvas commands: cycle layout, Agent Control, Focus Editor,
  Four-Agent Matrix, save slot 1, restore slot 1, restore previous layout,
  save/restore all three fixed slots with `shift-1/2/3` and `1/2/3`,
  open the saved-layout manager with `n m`, save a free-form named layout with
  `n s`, rename fixed slots with `n 1/2/3`,
  adjacent-pane focus with arrow keys, adjacent-pane swapping with shift-arrow
  keys, move-to-edge commands with alt-arrow keys, split-right/split-down with
  `v`/`enter`, fixed-step pane resizing with `h/j/k/l`, and pane equalization
  with `=`, plus `ctrl-b ctrl-b` to send the default prefix through to the
  focused item. The Panel Layout menu also shows disabled discovery rows for
  those prefix commands, and the title bar shows a compact `PREFIX …` chip
  while a multi-stroke prefix sequence is pending. `prefix_timeout_ms = 0`
  disables timeout replay. Custom prefix strings and dynamic remapping remain
  future work.
- Canvas recipes now share one canonical runtime recipe-name mapping. The
  workspace records the last applied Canvas recipe, layout history snapshots
  restore that recipe identity with pane visibility/focus, persisted workspace
  metadata restores recognized recipe identity across restart, and the Panel
  Layout menu checks the active recipe entry while the window is in Canvas mode.
  Three durable saved-layout slots are available now; each saved snapshot stores
  a derived display label shown by the Panel Layout restore actions, pane-tree
  shape metadata, tab-role metadata, and explicit serializable/project-path/
  live-only restore intent. Restore now uses the pane-tree shape when every
  saved center pane still exists, reopens missing project-path-backed and
  serializable tabs into saved panes, reapplies pinned/active tab metadata, and
  the Panel Layout menu can rename or clear stale fixed slots, save the current
  layout under a free-form name, and restore, rename, or clear free-form named
  layouts. The saved-layout manager modal lists all fixed slots plus named
  layouts with save-to-slot, restore, rename, and clear controls, and shows
  pane/tab counts plus restore coverage for project-path, serializable,
  live-only, pinned, and dirty tabs. The manager and Panel Layout menu can also
  clear all saved Canvas layouts after a warning confirmation.
  `workspace::RenameSavedCanvasLayoutSlot`,
  `workspace::SaveCurrentCanvasLayoutAs`, and
  `workspace::ManageSavedCanvasLayouts` provide the text-entry and manager
  surfaces. Bulk import/export/duplication workflows and live process/session
  restoration remain future work.
- Manual structural layout changes now clear the active Canvas recipe identity
  so the Panel Layout menu reports `Custom Canvas Layout` after pane splits,
  pane moves, pane joins, pane removal, pane-size changes, or explicit
  project/agent pane visibility toggles.
- `tab::Duplicate` is implemented for tabs whose item type supports the
  existing clone-on-split path; process-backed or live-only tabs remain under
  explicit process-lifetime rules.
- Add explicit close, restore, and "detach process" behavior so closing a tab
  is not confused with killing a process.

### Proper Panes Layout implementation contract {#proper-panes-layout-implementation-contract}

Proper Pane Layout is the Canvas migration target, not just a visual skin:

- The visible work surface is the center pane grid plus optional Session Rail.
  Legacy side or bottom docks remain internal compatibility hosts only.
- Project, Git, outline, collaboration, agent, terminal, Markdown, browser,
  diagnostics, settings, and future tool surfaces must be addressable as
  WorkspaceItems inside pane tabs.
- A recipe may create geometry and reveal existing pane-hosted surfaces, but it
  must not start background processes, kill live terminals/agents, or overwrite
  tab contents.
- Process-backed items need an explicit lifetime policy: close tab, hide tab,
  detach process, terminate process, and restore placeholder are separate user
  outcomes.
- Agent surfaces are not singleton panels. Multiple structured agent threads,
  terminal-native agent tabs, inspectors, timelines, and fleet views can be
  visible in separate panes at the same time.
- Layout history records pane visibility/focus immediately and evolves toward
  durable semantic snapshots: recipe name, pane tree, tab identities, process
  restoration metadata, user-authored saved slot labels, and later multi-name
  layout management.
- The first recipe-identity slice persists the active recipe id separately from
  process lifetime. Restart restore can recover the active recipe label without
  claiming terminal, agent, or browser processes were resumed.
- `pane_grid.auto_reflow` should choose semantic variants for narrow,
  portrait, ultrawide, and many-agent states without closing items. Narrow,
  portrait, first ultrawide orientation variants, first many-agent ultrawide
  recipe-application variants, and ultrawide resize reshaping for already-open
  many-agent layouts are implemented.

Implementation order:

1. Keep closing all known legacy docks through `Workspace::all_docks()` in
   Canvas flows. This is implemented for current recipes and pane-grid
   application.
2. Replace any remaining privileged project/agent pane behavior with normal tab
   affordances: drag, close, context menu, keyboard movement, and overflow.
   Close, open, move, clone, and tab-drop routing now use the tab-host
   capability for existing project/agent panel panes while persistence keeps
   their semantic pane kind.
3. Add durable layout metadata separately from process lifetime so restart
   restore can recreate layout intent without claiming processes are alive.
   Active recipe identity and three fixed saved layout slots now persist as
   workspace metadata, including derived saved-layout display labels, pane-tree
   split axes and flex weights, plus tab title, serializable item kind/id,
   active, preview, dirty, pinned, project-path metadata, and explicit
   restore-planning intent. Restore applies saved pane-tree shape when all
   saved center panes still exist. User-authored fixed-slot labels,
   free-form named layouts, and a saved-layout manager with save-to-slot
   controls plus restore metadata and confirmation-backed clear-all are
   available; bulk import/export and duplication workflows remain future work.
4. Add resize-driven `auto_reflow` using semantic recipe variants rather than
   raw pixel snapshots. Initial recipe-application reflow and resize-triggered
   root-orientation reflow for active recipes are implemented. First ultrawide
   orientation variants are implemented for vertical-first recipes. Four-Agent
   Matrix, Six-Agent Supervisor, plus Worktree Matrix prefer ultrawide
   column-first splits when newly applied, and already-open many-agent layouts
   flatten nested rows into existing horizontal columns on ultrawide resize,
   then reshape the same panes back into nested or narrow variants when the
   workspace leaves ultrawide.
5. Add UI for named saved layouts and layout history once the underlying
   metadata is stable. Three fixed durable saved-layout slots are implemented
   in the Panel Layout menu with derived restore labels plus clear actions, and
   manual structural changes mark the active recipe as custom. Tab-role
   metadata and user-authored slot-label metadata are captured in saved slots,
   project-path-backed and serializable tabs now reopen during restore, and
   built-in fixed-slot plus free-form named-layout UI and an inspectable
   save/restore manager with clear-all are implemented, while bulk
   import/export/duplication workflows and actual process restoration remain
   future work.

## Session Rail {#session-rail}

Refine the existing sidebar direction into Session Rail:

- The rail is for cross-workspace orientation, not fixed content.
- Sections include workspace switcher, favorites, active workspaces, agent
  attention, running tasks, recent sessions, remote hosts, saved layouts, and
  utility actions.
- Metadata is configurable: branch, worktree, pull request, remote, ports,
  running agents, diagnostics, participants, latest attention, and media state.
- Rail entries navigate to the exact WorkspaceItem that produced an indicator.
- Modes: hidden, icon, compact, detailed, overlay, and auto.
- Automatic grouping and sorting must not move interaction targets while the
  user clicks or drags.
- Runtime row rendering now honors `session_rail.metadata` for worktree/branch
  labels, agent-state labels, active Canvas layout labels, and
  latest-attention timestamps/badges while keeping the Ctrl-Tab thread switcher
  independent of rail display density. The default metadata now includes
  `layout`; layout metadata also shows the saved-layout count for the project
  group, and `saved_layout` remains accepted as an alias for saved-layout
  visibility.
- `workspace_bar.show_agent_attention` gates the visual attention layer on top
  of that metadata: workspace-level badges, row notification dots, and
  collapsed-project waiting/notification markers.
- Runtime row ordering now honors `session_rail.sort_by` for
  `attention`, `agent_state`, `creation_time`, `recent_activity`, and
  `project`. Project sorting orders thread and terminal rows by their first
  worktree label or path inside each project group, then falls back to recent
  activity. `manual` now preserves existing thread and terminal row positions
  across rail rebuilds, persists user-authored row order in sidebar state, and
  exposes `Move Selected Entry Up` / `Move Selected Entry Down` commands for
  selected thread or terminal rows within a project group. New rows append by
  recent activity until explicitly reordered.
- Runtime rail hosting now honors `session_rail.visibility = "hidden"` by
  reporting zero sidebar width, rendering an empty rail, and suppressing the
  resize hitbox. `visibility = "icon"`, `"compact"`, or `"detailed"` now acts
  as the concrete rail density override while `auto` and `overlay` keep the
  current rail surface until distinct renderers exist.
- Runtime rail layout now honors `session_rail.mode` for the first concrete
  modes: `hidden` removes the rail, `icon` uses a narrow icon-only rail with
  thread/terminal labels and metadata hidden, `compact` clamps the rail to a
  narrower readable width, `detailed` enforces a wider minimum rail width, and
  `always` keeps the rail open across startup, restore, toggle, and close
  actions unless the rail is explicitly hidden. Concrete `visibility` values
  take precedence over `mode` for rail density.
- Runtime rail placement now honors `session_rail.position` for the effective
  left/right side. The sidebar side menu writes both the legacy `sidebar.side`
  compatibility setting and the Canvas `session_rail.position` setting.

## Agents and terminal sessions {#agents-and-terminal-sessions}

Create one registry model for structured agents and terminal-native agents:

- Structured Agent Thread uses provider events when available.
- Agent Terminal wraps a real PTY with metadata, detection, attention, and
  restoration layers.
- Agent Dashboard, Fleet View, Agent Matrix, Focus Queue, Timeline, Inspector,
  and Fork are regular WorkspaceItems.

Terminal-agent detection:

- Detect known commands such as `claude`, `codex`, `aider`, `agy`, `opencode`,
  `gemini`, `amp`, `crush`, `devin`, `droid`, `goose`, `grok`, `openhands`,
  `pi`, `qwen`, `cursor`, and `copilot`. Keep ambiguous bare commands such as
  `agent` out of standalone title classification until runtime hooks can
  disambiguate them.
- Use provider hooks or protocol events as authoritative state when available.
- Use process title, foreground command, terminal bell, exit status,
  breadcrumbs, OSC markers, and output heuristics only as observed or inferred
  state.
- Store detection confidence and show labels such as `Agent detected`,
  `Possibly waiting`, or `Disconnected`.
- Default `agent_ui.detect_terminal_agents` and
  `agent_ui.show_terminal_agents_in_session_rail` to `true`.
- Implemented first slices: terminal-thread metadata rows classify known agent
  CLI titles and render matching identity in the Session Rail and thread
  switcher. The classifier now covers the same non-ambiguous command families
  as terminal runtime hooks. Standalone `terminal_view::TerminalView` tabs are
  scanned from open workspaces and surfaced when their title or live foreground
  command identifies a known agent CLI; activation and close route back to the
  existing terminal tab. Session Rail terminal rows now label detected agents as
  `Agent detected` and terminal-bell attention as `Possibly waiting`. Runtime
  settings now honor
  `agent_ui.detect_terminal_agents`,
  `agent_ui.show_terminal_agents_in_session_rail`,
  `agent_ui.show_detection_confidence`, and `agent_ui.notify_on_attention`.
- Default `agent_ui.allow_multiple_visible_agents` to `true` so agents can live
  in normal tabs across multiple panes.
- Default `agent_ui.connect_hooks`, `agent_ui.resume_sessions_on_restart`, and
  `agent_ui.notify_on_attention` to `true`; runtime code must still distinguish
  authoritative provider hooks from terminal-observed heuristics.
- Runtime startup now respects `agent_ui.resume_sessions_on_restart` for
  automatic active agent-thread and terminal restoration. Terminal title,
  breadcrumb, program, and bell hooks now respect `agent_ui.connect_hooks`, and
  Agent Panel terminal attention state/popups respect
  `agent_ui.notify_on_attention`.
- Runtime settings now expose the full Canvas agent UI family:
  `presentation`, `event_verbosity`, `group_tool_calls`,
  `keep_failures_expanded`, `keep_permissions_expanded`, `fleet_view`, and the
  terminal/session settings above. Failed, rejected, and canceled tool-call
  cards honor `agent_ui.keep_failures_expanded`; permission prompts keep
  approval actions visible while `agent_ui.keep_permissions_expanded` controls
  whether generic permission detail sections default open. Sandbox and
  confusable-warning permission details stay expanded for safety. When
  `agent_ui.group_tool_calls` is enabled, adjacent visible tool-call cards share
  a subtle transcript rail so bursts of tool activity read as one operation.
  `event_verbosity = "summary"` now hides completed generic tool-call events
  that have no content or raw input, while preserving subagents, terminal tools,
  edits, pending work, permission prompts, failures, cancellations, and rejected
  calls.
  `agent_ui.presentation` now controls transcript density: `compact` uses
  tighter message padding for side-by-side agent lanes, `chat` uses a
  conversational middle density, and `document` preserves the roomy default
  transcript spacing.

Restoration:

- Persist layout and session metadata separately from process lifetime.
- Reconcile process identity before showing a live state.
- If a process cannot be restored, show an in-place Resume Placeholder with
  retry, replace, remove, and open transcript options.
- Closing a tab does not terminate a durable process unless the configured
  policy says it should.

Notifications:

- Group by workspace, session, state, severity, and time.
- Include session title, project, provenance, and safe next action.
- Selecting a notification focuses the exact source tab.
- Do not repeatedly announce streaming output.
- Implemented first attention slice: standalone agent terminal rows reuse the
  terminal bell as a notification signal, label that observed state as
  `Possibly waiting`, and clear it on Session Rail activation.

## Markdown and document surfaces {#markdown-and-document-surfaces}

Markdown should align with Canvas document composition:

- Open `.md` files as rendered preview first by default.
- Keep source editing one action away through Edit Source or split source.
- Default `markdown_preview.default_open_mode` to `preview` and keep
  `markdown_preview.show_edit_source_action` enabled.
- Runtime open routing registers Markdown preview as a Markdown-only project
  item opener. Setting `markdown_preview.default_open_mode` to `source` returns
  `.md` opens to the regular editor path.
- Persist preview tabs and follow-preview mode.
- Use ContentSheet alignment and readable-width modes.
- Use JetBrains Mono for code blocks and prose-friendly UI font/line height for
  rendered text.
- Do not hide raw source or make preview-first irreversible.

## Command systems and automation {#command-systems-and-automation}

Add semantic command surfaces:

- Command Palette groups: recommended, recent, current pane, workspace, layout,
  agent, terminal, collaboration, settings, and media.
- Optional Command Bar with commands like `:split right`, `:layout tiled`,
  `:agent next`, and `:workspace detach`.
- Command chaining for safe semantic actions only. Destructive chains require
  preview and confirmation.
- Optional Prefix Mode that coexists with Vim, VS Code, JetBrains, and existing
  Zed keymaps.
- Runtime prefix settings are now available through `MultiplexerSettings`, and
  the layout menu surfaces the configured prefix/broadcast policy when prefix
  mode is enabled. Default `ctrl-b` prefix key-dispatch capture, timeout
  handling, and the pending-prefix title-bar chip are implemented; custom
  dynamic remapping remains future work.

Automation API:

- Query workspaces, pane trees, layouts, tabs, notifications, agents, and tasks.
- Split, move, resize, focus, zoom, and restore panes.
- Open WorkspaceItems.
- Start terminals and agents.
- Send bounded terminal input and read bounded terminal output.
- Wait for agent or task state transitions.
- Control browser panes only through explicit capability grants.
- Apply named layouts and capture metadata.

Safety:

- Authenticate local clients.
- Scope tokens by workspace and action.
- Redact secrets.
- Bound output size and history.
- Log consequential mutations.
- Prevent background automation from stealing focus unless requested.

## Layout recipes {#layout-recipes}

Ship purpose-based starter layouts:

- Focus.
- Main and Stack.
- Main Top.
- Golden Split.
- Code, Run, Observe.
- Review.
- Debug.
- Documentation Studio.
- Browser Development.
- Agent Operations Center.
- Four-Agent Matrix.
- Six-Agent Supervisor.
- Worktree Matrix.
- Remote Operations.
- Pair Programming.
- Incident Response.
- Portrait Display.

Layouts are semantic recipes. They preserve pane IDs and tab contents where
possible and reflow instead of overwriting saved desktop geometry. Runtime
recipe slices now implement `Full Canvas`, `Agent Control`, `Focus Editor`,
`Main + Stack`, `Main Top`, `Golden Split`, `Code/Run/Observe`,
`Review`, `Debug`, `Documentation Studio`, `Browser Development`,
`Agent Operations Center`, `Four-Agent Matrix`, `Six-Agent Supervisor`,
`Worktree Matrix`, `Remote Operations`, `Pair Programming`,
`Incident Response`, `Portrait Display`, `Even Columns`, `Even Rows`, and
`Cycle Canvas Layout` against the existing pane/panel bridge.
`workspace::ApplyCanvasLayoutRecipe { name }` exposes the same normalized
recipe-name dispatcher to keymaps, command chaining, and automation without
requiring a dedicated action per recipe.
`Cycle Canvas Layout` reads `multiplexer.layout_cycle` so users can choose the
order of named recipes without changing keybindings. When
`multiplexer.prefix_mode = true`, the default `ctrl-b` prefix can trigger the
core Canvas command set from workspace focus, including fixed-step pane
resizing and pane equalization; `multiplexer.prefix_timeout_ms` controls
pending prefix replay and `0` disables it. Custom prefixes can be expressed
through user keymap overrides while dynamic prefix remapping remains future
work. `ctrl-b ctrl-b` sends the default prefix through to the focused item.
The Panel Layout menu exposes the default prefix command set as discoverable
disabled rows while prefix mode is enabled.

## Accessibility and performance requirements {#accessibility-and-performance}

Accessibility:

- Every pointer workflow has a keyboard equivalent.
- Focus remains visible and spatially understandable.
- Pane switching announces the active WorkspaceItem.
- Agent attention is announced once, not repeatedly.
- Runtime agent and terminal-agent notification paths now honor
  `accessibility.announce_agent_attention` for OS/window attention requests;
  in-app notification popups remain controlled by `agent_ui.notify_on_attention`
  and the existing agent waiting notification setting.
- State is not color-only.
- High-contrast and reduced-motion modes are supported.
- Runtime motion now honors `accessibility.reduced_motion`: `reduced` forces
  GPUI reduced motion, `full` disables it, and `system` falls back to the
  existing root `reduce_motion` setting.
- Increased font sizes reflow controls without clipping critical actions.

Performance:

- Do not rerender all panes when one agent state changes.
- Keep tab badge updates localized.
- Batch high-frequency terminal metadata.
- Virtualize large rails, fleets, timelines, file trees, and result lists.
- Avoid blur and layout animation on high-frequency terminal output.
- Typing must remain independent of agent streaming.
- Failed restoration must not block the workspace.

## Delivery sequence {#delivery-sequence}

1. Documentation and planning.
2. Schema-backed Canvas layout defaults.
3. Agent pane tab-bar behavior and Canvas layout naming.
4. Upstream merge and conflict resolution.
5. Visual defaults: Lumin, JetBrains Mono, settings, theme attribution.
6. Pane shell: Pane Frame, Tab Bar, Context Bar, Empty Pane, attention ring,
   close/restore behavior.
7. Panel migration: host existing panels as WorkspaceItems by default while
   keeping compatibility plumbing.
8. Session Rail: metadata, modes, attention navigation, saved layouts.
9. Agent registry: structured and terminal-native sessions, provenance,
   confidence labels, notifications, resume placeholders.
10. Markdown preview-first behavior and ContentSheet refinements.
11. Layout presets, layout cycling, layout history, responsive reflow.
12. Command Bar, optional Prefix Mode, safe Broadcast Groups.
13. Automation API and capability model.
14. Browser WorkspaceItem and agent-visible browser control.
15. Collaboration-aware layouts, shared agents, shared terminal permissions.
16. Accessibility, performance, high-contrast, reduced-motion, and large
    workspace hardening.

## No-build validation before implementation {#no-build-validation}

During the current docs-only phase:

- Keep repository changes limited to `docs/src/`.
- Do not build.
- Run Markdown formatting checks.
- Keep the existing untracked `dist/` app artifact untouched.

When implementation starts, use targeted checks before full builds:

- `git diff --check`.
- Theme JSON validation.
- Font asset discovery.
- Focused Rust tests for pane restoration, agent session metadata, Markdown
  preview, terminal metadata, and notification behavior.

Full build and broader test passes wait for explicit approval.
