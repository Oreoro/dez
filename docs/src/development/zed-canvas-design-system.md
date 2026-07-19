# Zed Canvas design system

Zed Canvas is a content-first, pane-native design system for editor, terminal,
agent, collaboration, browser, media, settings, and developer-tool interfaces.

This page is an internal design-system specification for Superzed workspace and
agentic UI work. Use it with [Zed Canvas](./zed-canvas.md).

## Principles {#principles}

- Content before chrome. Permanent chrome stays small, contextual, and
  discoverable through actions.
- One composable canvas. Every important tool can become a regular
  WorkspaceItem inside a Tab.
- Calm by default. Saturation and motion appear only for meaningful state.
- Dense, not cramped. Use hierarchy, grouping, alignment, and progressive
  disclosure before adding containers.
- Keyboard and pointer parity. Pointer affordances expose semantic command
  equivalents.
- State over decoration. Focus, selection, dirty state, diagnostics, agent
  attention, collaboration, security, and drag targets have priority over
  decorative emphasis.
- Reversible manipulation. Structural changes need undo, restore, or visible
  recovery paths where state still exists.
- Progressive expertise. Beginners and experts use the same architecture with
  different density, hints, and automation.
- Honest agent interfaces. Do not imply certainty where only terminal or output
  heuristics exist.

## Token architecture {#token-architecture}

Use four token layers:

```text
Foundation tokens
→ Semantic tokens
→ Component tokens
→ Instance overrides
```

Application code should consume semantic or component tokens by default.
Instance overrides are allowed only for local, named exceptions.

### Color tokens {#color-tokens}

Use perceptually balanced neutral ramps and accent ramps. The default interface
should mostly use neutral values. Themes may remap the primary accent without
changing semantic state colors.

Surface tokens:

```text
surface.window
surface.workspace
surface.pane
surface.pane_inactive
surface.tab_bar
surface.tab_active
surface.tab_hover
surface.row_hover
surface.row_selected
surface.input
surface.raised
surface.popover
surface.modal
surface.tooltip
surface.scrim
surface.code
surface.terminal
surface.agent
surface.diff_added
surface.diff_removed
surface.diff_modified
```

Text tokens:

```text
text.primary
text.secondary
text.tertiary
text.disabled
text.placeholder
text.inverse
text.link
text.link_hover
text.accent
text.on_accent
text.code
text.terminal
text.selection
```

Border tokens:

```text
border.subtle
border.default
border.strong
border.focused
border.selected
border.drag_target
border.warning
border.error
border.success
border.collaboration
```

State tokens:

```text
state.info
state.info_muted
state.success
state.success_muted
state.warning
state.warning_muted
state.error
state.error_muted
state.neutral
state.neutral_muted
```

Agent tokens:

```text
agent.starting
agent.working
agent.waiting
agent.blocked
agent.paused
agent.idle
agent.completed
agent.failed
agent.disconnected
agent.restoring
agent.unknown
agent.inferred
```

Agent state must use label and icon in addition to color where space permits.

Git, diagnostic, and collaboration state should also use semantic tokens. Do
not reuse diagnostic colors for participant identity without another visual
distinction such as initials, avatar, label, or cursor shape.

### Attention model {#attention-model}

| Level    | Meaning                           | Treatment                           |
| -------- | --------------------------------- | ----------------------------------- |
| Quiet    | Background activity               | Muted icon or no indicator          |
| Active   | Work currently running            | Stable accent indicator             |
| Notice   | Useful new information            | Badge without interruption          |
| Action   | User input required               | Pane ring, Rail badge, notification |
| Critical | Security, loss, or severe failure | Strong warning with explicit action |

Do not animate an entire pane for ordinary activity.

## Typography {#typography}

Font roles:

```text
font.interface
font.editor
font.terminal
font.prose
font.code_inline
font.numeric
```

Recommended defaults:

- Interface: configured UI font or system sans serif.
- Editor: JetBrains Mono in the native Superzed default profile.
- Terminal: JetBrains Mono in the native Superzed default profile.
- Prose: UI font with more generous line height.
- Markdown code blocks and other code-like metadata: JetBrains Mono.
- Numeric: tabular figures where alignment matters.

Restrained type scale:

```text
type.2xs = 10
type.xs  = 11
type.sm  = 12
type.md  = 13
type.lg  = 14
type.xl  = 16
type.2xl = 20
type.3xl = 24
```

Hierarchy order:

1. Size.
2. Weight.
3. Color.
4. Spacing.
5. Container treatment.

Agent conversations should read like structured documents, not consumer chat.
Use compact turn headers, readable prose, monospaced tool and code sections,
collapsible technical details, small metadata labels, full-width change blocks,
and no oversized colored speech bubbles.

Markdown, documentation, settings explanations, and agent responses need
readable-width modes:

- Narrow: 680 px.
- Comfortable: 760 px.
- Wide: 900 px.
- Full pane.

## Spacing, shape, and elevation {#spacing-shape-and-elevation}

Use a 4 px conceptual spacing base:

```text
0, 2, 4, 6, 8, 10, 12, 16, 20, 24, 32, 40, 48, 64
```

Density profiles:

| Profile  |        Tabs | Controls |        Rows |
| -------- | ----------: | -------: | ----------: |
| Compact  | about 28 px | 26-28 px | about 26 px |
| Balanced | about 32 px | 30-32 px | about 30 px |
| Spacious | about 38 px | 36-40 px | about 38 px |

Radius scale:

```text
radius.none = 0
radius.xs   = 2
radius.sm   = 4
radius.md   = 6
radius.lg   = 8
radius.xl   = 12
radius.full = 999
```

Use radius sparingly. Panes should not become independent rounded cards inside
the workspace. Adjacent panes use hairline separators, not shadows. Use
elevation for popovers, dialogs, and floating palettes.

## Motion {#motion}

Motion should explain spatial change, state transitions, insertion, removal,
drag destination, pane zoom, overlay appearance, or task completion. It should
not decorate typing, terminal output, or agent streaming.

Durations:

```text
instant = 0-50 ms
fast    = 80-120 ms
normal  = 140-180 ms
slow    = 200-260 ms
```

Approved motion:

- Pane zoom transition.
- Tab movement.
- Rail expansion.
- Popover appearance.
- Drag preview.
- Collapsible event expansion.
- Progress transitions.

Prohibited motion:

- Continuous glowing borders.
- Pulsing agent states.
- Animated gradients.
- Bouncing notification badges.
- Repeated pane flashing.
- Layout animation during rapid terminal updates.

Respect reduced-motion settings.

## Pane components {#pane-components}

Pane Frame anatomy:

```text
PaneFrame
├── PaneTabBar
├── PaneContextBar (optional)
├── WorkspaceItem
└── PaneTransientLayer
```

Pane Frame states:

- Focused.
- Unfocused.
- Drag target.
- Zoomed.
- Needs attention.
- Disconnected.
- Empty.
- Loading.
- Error.

Pane Tab Bar rules:

- Hide when one tab exists only if the user enables auto-hide.
- Preserve drag space.
- Support horizontal scrolling.
- Keep active tab visible.
- Use overflow search when tabs exceed available width.
- Do not shrink titles below recognition.
- Support platform-standard middle-click close where available.

Pane Context Bar rules:

- Hosts item-specific actions only.
- Examples: editor breadcrumbs, terminal host and directory, agent model and
  state, browser address, diff navigation, collaboration state, media controls.
- Hide empty sections.
- Collapse low-priority actions into overflow.
- Stay keyboard reachable.
- Do not duplicate Status Strip content.

Pane Attention Ring:

- Indicates information, agent input, permission, failure, or collaboration
  request.
- Never flashes continuously.
- Clears only after relevant acknowledgement.
- Remains visible if the user views another tab in the same pane.
- Works without color.
- Respects notification preferences.

Empty Pane actions:

- Open file.
- Open terminal.
- Start agent.
- Open recent item.
- Open command.
- Restore closed tab.
- Remove pane.

Empty panes should accept drag-and-drop immediately.

## Notion-inspired composition {#notion-inspired-composition}

Document-oriented workspace items may use a `ContentSheet` alignment model:

```text
ContentSheet
├── Breadcrumbs
├── Icon
├── Title
├── Metadata
├── Primary content
└── Optional outline
```

The ContentSheet is not a card. It is a readable alignment model inside a pane.

Use content blocks for paragraphs, headings, code, tool calls, diffs,
diagnostics, tasks, checklists, callouts, tables, file references, terminal
excerpts, approvals, agent results, collaboration comments, and media.

Block handles appear on focus or hover and must not interfere with text
selection.

Slash menus may appear in agent composers, notes, Markdown, task plans,
collaboration messages, and terminal rich-input mode. They should expose
developer actions without breaking normal text or shell input.

Suggested reference triggers:

```text
@ file, symbol, agent, participant, skill
# issue, task, pull request, diagnostic
/ command or block
> action
! task or terminal command
```

Triggers must be configurable and disabled where they conflict with programming
or shell input.

## Multiplexer patterns {#multiplexer-patterns}

Use a tmux-inspired hierarchy without terminal-only terminology:

| Zed Canvas    | tmux-like concept | Purpose                                |
| ------------- | ----------------- | -------------------------------------- |
| Workspace     | Session           | Durable project or task context        |
| Layout Page   | Window            | Alternate pane-grid surface            |
| Pane          | Pane              | Simultaneously visible region          |
| WorkspaceItem | Running surface   | Editor, terminal, agent, browser, etc. |

Layout Pages are named alternative pane grids inside one workspace. Switching
pages must not close workspace items.

Detach Workspace removes a workspace from the visible window while preserving
compatible processes and sessions. Attach Workspace reconnects through native
restoration, tmux, Herdr, SSH, remote Zed, container runtime, or collaboration
host where supported.

Prefix Mode is optional and must coexist with Zed, Vim, VS Code, and JetBrains
keymaps.

Layout cycling should preserve pane IDs and tab contents while cycling through
even rows, even columns, main left, main right, main top, main bottom, tiled,
focus and stack, golden ratio, agent control, review, and terminal matrix.

Broadcast Groups synchronize input across selected compatible panes. Safety
requirements:

- Explicit opt-in.
- Persistent visible indicator.
- Preview target list.
- Exclude password and secret fields.
- Immediate cancel action.
- Confirm destructive commands.
- Never broadcast editor keystrokes accidentally.

Copy Mode applies to terminal output, logs, agent events, and read-only streams.
It supports movement, search, selection, copying, opening file references and
URLs, adding selection to agent context, creating notes, and pinning output.

## Agentic TUI patterns {#agentic-tui-patterns}

The terminal remains the source of visual truth for terminal-native agents.
Zed must not capture keys unexpectedly, draw controls over terminal cells, alter
terminal colors, assume mouse support, parse TUI screens as authoritative state,
or replace the TUI with a lossy transcript.

TUI workspace item layers:

```text
Terminal Surface
Agent Detection Adapter
Session Metadata
Zed Attention and Restoration Layer
```

Agent Command Deck is an optional compact layer for terminal agents. It can
send instructions, interrupt, pause or resume where supported, open changes,
open logs, open worktree, duplicate session, fork task, detach, or stop. Hide
it by default and expose it through the Context Bar, tab menu, or Command
Palette.

Agent Registry is a session collection. It may group agents into:

```text
Inbox
Working
Waiting
Review
Done
Failed
Archived
```

Fleet View is a regular workspace item for many agents. It should show filters,
grouping, sorting, total sessions, sessions needing attention, lanes, and
primary next actions.

Agent Tile contains state icon, title, provider, project or worktree, task
summary, elapsed or idle time, authoritative progress if available, and primary
next action. Avoid fake percentage progress.

Agent Matrix is a TUI-inspired grid of active sessions. Compact terminal tails
must be bounded, redacted where configured, local-only by default, and
non-authoritative.

Focus Queue is a keyboard-first sequence of sessions requiring attention.
Supported actions include next, previous, approve, deny, reply, open terminal,
open diff, snooze, and stop.

Agent Timeline should group repeated tools, keep approvals and failures
expanded, and collapse successful low-level work by default.

Agent Inspector may show provider, model, session ID, origin, process, current
directory, worktree, Git branch, context sources, permissions, hooks, resume
capability, resource usage, and event log.

Agent Fork creates a new session from existing context. It must state whether
conversation, file context, uncommitted changes, and permissions are inherited.

## Session Rail and command systems {#session-rail-and-command-systems}

Session Rail hierarchy:

```text
SessionRail
├── Workspace Switcher
├── Favorites
├── Active Workspaces
├── Agent Attention
├── Running Tasks
├── Recent
└── Utility Actions
```

Sections with no items disappear. Metadata is configurable. Indicators navigate
to the exact workspace item that produced the state.

Command Palette result anatomy:

```text
Icon | Action title | Context | Shortcut | Safety/state
```

Command Bar is a lightweight semantic command line:

```text
:split right
:layout tiled
:agent next
:agent open blocked
:workspace detach
:broadcast select working
:collab share local
```

Command Bar entries map to semantic actions, not raw shell interpolation.
Chains require preview before destructive actions, stop on failure by default,
show results, can be saved, and respect permissions and context.

## Layout recipes {#layout-recipes}

Important built-in layouts:

- Main and Stack.
- Main Top.
- Golden Split.
- Agent Operations Center.
- Four-Agent Matrix.
- Six-Agent Supervisor.
- Code, Run, Observe.
- Worktree Matrix.
- Remote Operations.
- Browser Development.
- Data and Notebook.
- Documentation Studio.
- Pair Programming.
- Incident Response.
- Portrait Display.
- Presentation.

Each layout is a semantic recipe. It must preserve item identity and should
reflow instead of destroying saved geometry.

## Responsive layout engine {#responsive-layout-engine}

Layout classes:

```text
micro
compact
standard
wide
ultrawide
portrait
```

Classes derive from usable pane-grid dimensions, not only total window width.

Minimum pane behavior:

1. Collapse Context Bar.
2. Collapse tab metadata.
3. Move actions into overflow.
4. Reduce gutters.
5. Use compact views.
6. Offer stacking into tabs.
7. Never render unusably narrow content.

WorkspaceItems may declare:

```text
minimum_width
minimum_height
preferred_aspect
can_compact
can_stack
can_overlay
priority
```

If panes cannot fit, place lower-priority panes into an overflow tab stack
instead of shrinking everything below usability.

## UI and UX fixes {#ui-and-ux-fixes}

Canvas work should explicitly fix:

- Panel fragmentation. Convert tools to WorkspaceItems with shared Pane Frame,
  Tab, Context Bar, focus, drag, restore, and lifecycle contracts.
- Hidden background agents. Register detected sessions globally and navigate
  from rail indicators to exact source tabs.
- One-agent bottleneck. Model agents as many sessions visible across arbitrary
  panes and windows.
- Notification ambiguity. Include session, project, state, and safe next action.
- Tab overload. Add pinned compact tabs, searchable overflow, grouping,
  switchers, duplicate-title disambiguation, and task-based saved groups.
- Layout loss. Add continuous snapshots, named layouts, history, per-project
  restoration, and crash-safe persistence.
- Focus uncertainty. Add stable active-pane indicators, focus flash, directional
  navigation, pane numbering, Pane Map, focus history, and announcements.
- Excessive chrome. Use one tab bar, one optional context bar, contextual
  controls, semantic separators, and overflow actions.
- Agent transcript noise. Group repeated tools, summarize low-risk success,
  keep failures and approvals expanded, and preserve exact terminal output.
- Beginner overload. Provide starter layouts, empty-pane actions, drag hints,
  searchable commands, preset explanations, and undo.
- Expert friction. Add Prefix Mode, Command Bar, layout cycling, semantic action
  chaining, custom keymaps, automation, socket API, and templates.
- Environment confusion. Label local, remote, container, staging, and production
  surfaces with text, icon, and optional border treatment.
- Inferred-state overconfidence. Store and show detection confidence.
- Accidental destructive broadcasting. Require explicit activation, target
  indication, previews, environment warnings, and cancellation.
- Stale restored sessions. Reconcile process identity before showing live state;
  otherwise show a disconnected placeholder with manual resume options.

## Component state contract {#component-state-contract}

Every interactive component must define:

- Default.
- Hovered.
- Focused.
- Pressed.
- Selected.
- Disabled.
- Loading.
- Empty.
- Error.
- Warning.
- Dragging.
- Drop target.
- High contrast.
- Reduced motion.
- Large text.
- Screen-reader behavior.

Every component specification should include purpose, anatomy, variants, states,
keyboard interaction, pointer interaction, focus behavior, accessibility
semantics, token mapping, responsive behavior, performance constraints,
examples, anti-patterns, and tests.

## Accessibility and performance {#accessibility-and-performance}

Expose navigation regions for workspace navigation, pane grid, active pane, tab
list, workspace item, status, notifications, and modal layer.

Provide skip actions for active content, workspace bar, session rail, tab bar,
status strip, next pane, and next attention item.

Do not announce every streamed token, terminal line, progress animation, or
collaborator cursor movement. Announce agent attention, permission requests,
task completion, failures, collaboration changes, active pane changes, layout
changes, broadcast activation, and destructive action results.

Rendering rules:

- Virtualize large rail, fleet, timeline, result, and file-tree collections.
- Do not rerender all panes when one agent state changes.
- Keep tab badge updates localized.
- Batch high-frequency terminal metadata.
- Throttle elapsed-time labels.
- Avoid blur on large continuously updating surfaces.
- Do not animate during rapid terminal output.
- Cache measured static labels.
- Release hidden heavyweight renderers where safe.

Large-workspace target: hundreds of tabs, dozens of panes across layout pages,
dozens of agent sessions, large scrollback, large repositories, multiple remote
hosts, and long collaboration sessions.

## Design settings {#design-settings}

Initial settings shape:

```json [settings]
{
  "design_system": {
    "family": "zed_canvas",
    "density": "balanced",
    "radius": "subtle",
    "motion": "system",
    "contrast": "standard",
    "content_width": "comfortable",
    "icon_style": "outline",
    "show_labels": "contextual"
  },
  "workspace_bar": {
    "visibility": "always",
    "height": "compact",
    "center_command_search": true,
    "show_layout": true,
    "show_agent_attention": true
  },
  "session_rail": {
    "visibility": "auto",
    "mode": "compact",
    "position": "left",
    "group_by": "project",
    "sort_by": "manual",
    "metadata": ["branch", "worktree", "agent_state", "layout", "latest_attention"]
  },
  "pane_grid": {
    "auto_reflow": true,
    "layout_history": true,
    "focus_indicator": "border_and_title",
    "attention_ring": true,
    "tab_overflow": "searchable",
    "auto_hide_single_tab_bar": false
  },
  "agent_ui": {
    "presentation": "document",
    "event_verbosity": "normal",
    "group_tool_calls": true,
    "keep_failures_expanded": true,
    "keep_permissions_expanded": true,
    "show_detection_confidence": true,
    "fleet_view": "lanes"
  },
  "multiplexer": {
    "prefix_mode": false,
    "prefix": "ctrl-b",
    "layout_cycle": [
      "even_columns",
      "even_rows",
      "main_left",
      "main_top",
      "tiled",
      "agent_control"
    ],
    "broadcast_confirmation": "risky"
  },
  "accessibility": {
    "visible_focus": "strong",
    "reduced_motion": "system",
    "announce_agent_streaming": false,
    "announce_agent_attention": true,
    "minimum_target_size": "system"
  }
}
```

## Implementation order {#implementation-order}

1. Foundation: semantic tokens, density, accessibility mappings, typography,
   icon sizes, component-state contracts, visual fixtures.
2. Pane shell: Pane Frame, Tab Bar, Context Bar, attention rings, Empty Pane,
   Pane Map, layout history, snapshots.
3. Multiplexer: automatic layouts, layout cycling, optional Prefix Mode,
   detach and attach semantics, Layout Pages, safe Broadcast Groups.
4. Agent surfaces: Agent Registry, Fleet and lanes, document-style Agent Tabs,
   terminal-agent detection, hook-based authoritative states, Attention Queue,
   restoration placeholders, and Session Inspector.
5. Notion-like composition: ContentSheet, content blocks, block handles, slash
   menus, inline references, readable-width settings, structured settings and
   documentation surfaces.
6. Responsive behavior: WorkspaceItem layout hints, adaptive reflow, tab
   overflow, rail overlay mode, compact item variants, laptop, portrait,
   ultrawide, and large-text tests.

## Acceptance criteria {#acceptance-criteria}

- Permanent colors derive from semantic tokens.
- Every component supports light, dark, and high-contrast themes.
- Every primary pointer workflow has a keyboard equivalent.
- Panes support named automatic layouts.
- Layout changes can be undone.
- Any WorkspaceItem can occupy any pane.
- Session Rail metadata is configurable.
- Agents requiring attention identify their exact source tab.
- Agent state indicates whether it is authoritative or inferred.
- Agent prose uses document composition rather than oversized chat bubbles.
- Tool activity can be summarized or expanded.
- Terminal TUIs retain unmodified terminal behavior.
- Prefix Mode is optional.
- Broadcast Mode has persistent warning and target list.
- Restored layouts do not falsely imply restored processes.
- Compact windows reflow without destroying saved layout.
- Increased font size does not clip critical controls.
- Notification motion respects reduced-motion settings.
- Large session lists and timelines are virtualized.
- Beginners can start from purpose-based layouts.
