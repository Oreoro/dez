# Superzed Canvas implementation plan

This plan integrates the Superzed upstream merge request with the Zed Canvas
product and design-system specifications:

- [Zed Canvas](./zed-canvas.md)
- [Zed Canvas design system](./zed-canvas-design-system.md)

Current phase: Canvas foundation and native defaults. Do not build until the
user asks.

Implemented in this phase:

- Merged latest `zed-industries/zed:main` into the Superzed work branch.
- Added Canvas settings families:
  `design_system`, `workspace_bar`, `session_rail`, `pane_grid`, `agent_ui`,
  `multiplexer`, and `accessibility`.
- Added the Canvas layout action that syncs dock panels into regular pane tabs
  and closes legacy dock chrome.
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
- Session Rail terminal rows now classify known terminal agent CLIs from title
  metadata, including Claude Code, Codex, Gemini CLI, Aider, OpenCode, Amp,
  Goose, Qwen Code, Cursor Agent, and GitHub Copilot.
- Standalone terminal tabs whose titles identify a known agent CLI now appear
  in the Session Rail. Activating the rail row focuses the existing terminal
  tab, and closing the row closes that tab instead of spawning or restoring an
  Agent Panel terminal.
- Standalone agent terminal bell state now marks the Session Rail row as
  notified and clears when the row is activated.
- Added command-palette/menu actions for Canvas layout recipes:
  `Canvas: Full`, `Canvas: Agent Control`, `Canvas: Focus Editor`, and
  `Cycle Canvas Layout`.
- Added additional named Canvas recipe actions for Main + Stack, Main Top,
  Golden Split, Code/Run/Observe, Agent Operations Center, and Four-Agent
  Matrix.
- `workspace_bar.show_layout` now controls whether Canvas layout commands show
  in the Command Palette and Panel Layout chrome.
- `workspace_bar.show_agent_attention` now controls workspace-level attention
  badges plus Session Rail thread, terminal, and collapsed-project attention
  markers; `session_rail.metadata` still controls latest-attention timestamps.

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
- Runtime pane rendering now honors `pane_grid.auto_hide_single_tab_bar`; the
  Canvas default remains `false` so single-tab panes still advertise their
  draggable tab surface.
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
  `Agent Operations Center`, and `Four-Agent Matrix` without starting
  processes or replacing tab contents.
- `Cycle Canvas Layout` now honors `multiplexer.layout_cycle`, including the
  default `even_columns`, `even_rows`, `main_left`, `main_top`, `tiled`, and
  `agent_control` recipe names, with tolerant matching for label-style names.
- Add explicit close, restore, and "detach process" behavior so closing a tab
  is not confused with killing a process.

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
  labels, agent-state labels, and latest-attention timestamps/badges while
  keeping the Ctrl-Tab thread switcher independent of rail display density.
- `workspace_bar.show_agent_attention` gates the visual attention layer on top
  of that metadata: workspace-level badges, row notification dots, and
  collapsed-project waiting/notification markers.

## Agents and terminal sessions {#agents-and-terminal-sessions}

Create one registry model for structured agents and terminal-native agents:

- Structured Agent Thread uses provider events when available.
- Agent Terminal wraps a real PTY with metadata, detection, attention, and
  restoration layers.
- Agent Dashboard, Fleet View, Agent Matrix, Focus Queue, Timeline, Inspector,
  and Fork are regular WorkspaceItems.

Terminal-agent detection:

- Detect known commands such as `claude`, `codex`, `aider`, `opencode`,
  `gemini`, `amp`, and similar entries already present in the fork.
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
  switcher. Standalone `terminal_view::TerminalView` tabs are scanned from open
  workspaces and surfaced when their title identifies a known agent CLI;
  activation and close route back to the existing terminal tab. Session Rail
  terminal rows now label detected agents as `Agent detected` and terminal-bell
  attention as `Possibly waiting`. Runtime settings now honor
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
`Agent Operations Center`, `Four-Agent Matrix`, and `Cycle Canvas Layout`
against the existing pane/panel bridge.
`Cycle Canvas Layout` reads `multiplexer.layout_cycle` so users can choose the
order of named recipes without changing keybindings.

## Accessibility and performance requirements {#accessibility-and-performance}

Accessibility:

- Every pointer workflow has a keyboard equivalent.
- Focus remains visible and spatially understandable.
- Pane switching announces the active WorkspaceItem.
- Agent attention is announced once, not repeatedly.
- State is not color-only.
- High-contrast and reduced-motion modes are supported.
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
