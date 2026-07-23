# Dez Fork Notes

This is the permanent product and architecture source of truth for Dez. Plans,
release checklists, and historical design documents must conform to it.

## Product definition {#product-definition}

Dez is a complete native development environment and durable computing
workspace for terminal-native developers who work directly and supervise
coding agents or long-running sessions across repositories and hosts.

Zed supplies the native editor, GPUI, language support, Git, debugger, remote
infrastructure, collaboration substrate, and agent ecosystem. Dez continuously
inherits those capabilities while changing the product model around them.

The GUI is a client over durable work. The editor is a surface in the
workspace, not the owner of the application. Dez is not "Zed with more
terminals" and not a panel that lists more agents.

The initial promise is:

> You can see what is running, what needs attention, what changed, and what is
> ready for review without reconstructing terminal and editor state.

Direct editing, debugging, testing, Git, search, and navigation remain
first-class product work. Multi-agent supervision is the first customer and
delivery wedge, not a reason to reduce editor quality or fork compatible Zed
capabilities.

## Product primitives {#product-primitives}

- **App Session:** The durable application universe. It owns Workspace
  identity, order, active and unresolved records, shared registries, and
  viewport state without owning window-bound GPUI entities.
- **Workspace:** Durable human context containing panes, surfaces, focus,
  history, and local navigation state. It does not own a permanent project
  root.
- **Surface:** A draggable pane item such as a file, terminal, agent, Git view,
  search, debugger, browser, settings page, or review.
- **Evidence:** Typed, provenance-bearing context or verification contributed
  by path-bearing surfaces, Hosts, processes, structured events, Git, and the
  filesystem. Evidence is labeled observed, derived, agent-reported,
  user-confirmed, or unknown.
- **Host:** A local or remote machine or execution environment that owns
  processes.
- **Environment:** A reproducible description or activation mechanism for
  dependencies and services. It may configure a Host context but is neither a
  Host nor a Workspace.
- **Session:** Durable computation owned by a host rather than by the GUI.
- **Actor:** A human, agent, task, debugger, or process performing work.
- **Run:** The user-facing unit that connects an objective, actor, host,
  session, evidence, repository state, observed commands and checks, attention,
  review, and outcome. A shell can remain a session without becoming a Run.
- **Change Set:** A logical reviewable relationship between intent, one or more
  Runs, repository identity, files or hunks, Evidence, and review status. Git
  remains authoritative for v0.0.1; a Change Set is not a second Git database.
- **Projection:** Navigation or status UI derived from real surfaces and
  sessions. A projection never owns duplicate lifecycle state.

## Interface composition {#interface-composition}

Dez uses one pane grid and one supervision projection. User-facing names
describe purpose, not the inherited dock or panel implementation:

| Region              | Owns                                                               | Does not own                                                        |
| ------------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------- |
| **Session Rail**    | Search, attention scope, Workspace grouping, and navigation        | Terminal processes, Agent Sessions, editor state, or duplicate tabs |
| **Workspace Tools** | Files, Outline, Git, and Debug tabs in a hideable left tool pane   | A second Workspace, root selection, or terminal placement           |
| **Main work area**  | File, terminal, search, diagnostics, settings, and review Surfaces | Global project scope or sidebar-only copies of active work          |
| **Agent**           | Native and ACP conversation Surfaces in a hideable right tool pane | Terminal-agent process ownership                                    |

Every visible **New Terminal** action creates a normal main-area Surface. It can
be tabbed, split, moved, detached from a durable Host Session, or reattached
without introducing a separate Terminal Panel model.

Session Rail rows are projections. Selecting a terminal row focuses its
attached terminal Surface or reattaches the Host-owned Session. Selecting an
Agent Session row focuses its existing conversation Surface. A row may compose
actor, lifecycle, attention, evidence, changes, and recency, but it never
becomes a second owner of those facts.

Terminal title data remains full through local, durable Host, retained-Agent,
Session Rail, and Session Switcher metadata paths. Visual tabs and rows own
space-based truncation so tooltips and restored projections retain useful
identity. Explicit custom names are trimmed, persist across restoration, and
override the live shell title without discarding decorative agent-state
prefixes. The action is named **Rename Terminal…** and double-clicking the tab
invokes the same editor.

At zero sessions, the overview owns the rail title and **No sessions yet**
status. The compact start block does not repeat an empty-state headline or
decorative card: it explains that terminals open in the Main Work Area, then
offers one filled **New Terminal** action and one outlined **Open Workspace…**
alternative. Both name their destination in accessibility output.

Workspace Tools and Agent are ordinary pane-grid regions with stable placement
and normal focus behavior. Hiding one keeps its items available, returns focus
to a visible editor or terminal pane, and persists the layout. Opening a named
tool reveals the correct region and activates its existing tab.

### Everyday routing {#interface-everyday-routing}

Action names describe their destination. They do not expose inherited panel
terminology:

| Intent                                        | Result                                                                 |
| --------------------------------------------- | ---------------------------------------------------------------------- |
| **New Terminal**                              | Opens a terminal tab in the active Workspace's main work area          |
| **New Agent Session**                         | Opens or focuses a conversation in the right Agent pane                |
| **Files**, **Outline**, **Git**, or **Debug** | Opens the named tab in left-side Workspace Tools                       |
| Select a Session Rail row                     | Activates its Workspace and focuses or reattaches the existing Surface |
| Hide Workspace Tools or Agent                 | Hides that region and returns focus to an editor or terminal           |
| Split or move a Surface                       | Rearranges the same Workspace; it does not create a second project     |

The active Workspace keeps **New Terminal** and **Workspace Options** visible.
Inactive Workspace actions may reveal on hover because selecting the Workspace
first makes the same controls persistent. Every icon-only control must retain
an accessible name and tooltip. Repeated Workspace-row controls include the
visible Workspace name in both; internal element and hover-group identifiers
are presentation-only and must never enter user-facing copy.

Visible controls must also perform their advertised action: the Agent title
pencil starts editing, worktree closure names its window scope, and the
main-area overflow control is **Switch Surface**, not Open Tab.

Empty primary regions use compact, top-anchored recovery guidance rather than
floating a small prompt in the middle of an empty pane. The heading names the
missing prerequisite, the explanation is specific to Files, Git, or Agent, and
the primary action says **Open Workspace**.

The everyday Canvas Layout menu is a workflow picker, not a diagnostics or
storage dashboard. It exposes Full, Agent Control, Focus Editor,
Code/Run/Observe, Review, and Debug; saved-layout detail belongs in **Manage
Saved Layouts…**. The active Workspace exposes this submenu through its
persistent **Workspace Options** control in the Session Rail. Official Zed's
account and organization chrome remains unchanged compatibility code.

The main-area tab-bar plus control is named **Add to Main Work Area** in Dez.
Its menu opens files, Workspace search and symbols, or a terminal in that same
pane grid; it does not add a sidebar panel or create a second terminal model.
It remains visible when focus moves to another region. Commands that open a
picker or overlay use an ellipsis.

Tab-bar chrome follows region ownership. Main Work Area panes own add, split,
and zoom. Workspace Tools and Agent never inherit those controls: each exposes
one persistent close control named **Hide Workspace Tools** or **Hide Agent**.
Accessibility landmarks use the same visible region names: **Main work area**,
**Workspace Tools**, and **Agent**.

Discarding an Agent Session draft from either its Session Rail row or its
main-area tab requires confirmation because unsent prompt text is permanently
removed. Archiving a saved Agent Session remains immediate and reversible from
Agent History.

Visible conversation scope is **Agent Session** throughout the Agent pane,
context picker, search, sandbox status, and Session Switcher. Internal action,
protocol, database, and mention identifiers may retain `thread`; the compatible
mention keyword remains `@thread`. The Agent menu names its destinations as
**Agent Settings** and **Toggle Session Rail**.

Untitled conversation storage retains the upstream `New thread` sentinel for
database compatibility, but every visible Dez fallback is **New Agent
Session**. Icon-only toolbar controls require accessible names, and a disabled
control must explain why it is unavailable rather than repeating its enabled
label.

Session Switcher previews are reversible. Confirming records and focuses the
selected Session; cancelling restores the original Agent Session, center
terminal Surface, or Host Session through its actual source without changing
ownership or routing a terminal through Agent.

Mixed switcher rows use distinct Agent and terminal icons. Their explicit
**Agent Session** or **Terminal Session** type remains in accessibility output
and the row tooltip instead of repeating as visible metadata under every title.
Hover only reveals row emphasis; it never previews or activates work. Keyboard
cycling previews deliberately, click confirms directly, and the selected row is
the active descendant of the named **Recent sessions** list.

The Agent region is named **Agent** in user-facing controls; inherited Panel
terminology remains an implementation detail. File actions name **Files** as
their destination, and layout actions remain **Canvas Layout** even when
compatibility settings still use a dock-backed implementation.

There is no Dez **Terminal Thread** destination. The inherited action remains
only as an official-Zed compatibility implementation. Dez hides it from Agent
menus and the command palette, redirects compatibility dispatches to **New
Terminal**, and does not restore the inherited Agent-terminal surface after a
restart.

### How IDE features integrate {#interface-ide-integration}

Each Workspace owns one upstream-compatible `Project`. Editors, terminals,
language servers, search, diagnostics, Git, debugger state, tasks, and Agent
context all resolve through that same Workspace and Project:

- A file opens as a main-area Surface. Language intelligence and diagnostics
  come from the Workspace's Project.
- A terminal opens beside files in the same pane grid and starts with that
  Workspace's working-directory context.
- Files, Outline, Git, and Debug are alternate views of the same Project. They
  do not create a second root or copy state into the Session Rail.
- The Agent pane uses the active Workspace's Project context. Agent edits land
  in ordinary buffers and Git changes, so they remain reviewable with the same
  editor, diagnostics, and Git tools. **Agent Review** is the interactive
  change Surface for Keep/Reject decisions; a **Review Brief** is the separate
  evidence summary for a Run.
- Search, settings, diagnostics, and review briefs open as normal main-area
  Surfaces. They can be tabbed or split without becoming permanent sidebars.
- The Session Rail observes these surfaces and durable sessions. It adds
  navigation, attention, evidence, and recency, but never becomes the editor,
  terminal, Agent, or process owner.
- Empty and recovery states must state this ownership boundary in their visible
  copy. A Session Rail action may open a terminal in the main work area, but it
  must never imply that the terminal runs inside the rail.
- Workspace readiness uses Workspace vocabulary and accessible status
  semantics. Automatic trust names the newly opened folder scope and the
  language servers, Workspace settings, and configured tools that it enables.

This is the IDE integration: Dez retains Zed's editor and project engine, then
organizes its existing capabilities around terminal-native supervision. The
terminal is not embedded inside chat, and the editor is not a separate mode.
Both are first-class Surfaces in one Workspace.

## Visual identity and typography {#visual-identity-and-typography}

Dez follows the operating system appearance and ships with the attributed
Lumin pair: **Lumin Blur** for dark mode and **Lumin Light** for light mode.
Lumin remains derived from Daksh Sharma's MIT-licensed source; the theme asset,
standalone license, and aggregate source attribution travel with every build.

Blur is a window-shell material, not an excuse to erase hierarchy. The root
surface may be translucent, while low-contrast dividers, selected tabs,
scrollbars, active lines, and a restrained peach focus accent keep panes and
controls legible. Elevated menus and overlays remain visually solid enough for
text. Editor and terminal regions reuse the single shell material instead of
stacking independent blur effects over continuously updating content.

Typography uses explicit roles:

- **JetBrains Mono** is bundled under the SIL Open Font License and is the
  default for buffers, terminals, Agent prompt/code content, Markdown code,
  and Git commit input.
- **`.ZedSans`**, currently the bundled IBM Plex Sans family, is the default
  for navigation, labels, prose, menus, settings, and other interface chrome.
- The balanced v0.0.1 baseline is 14 px for UI, editor, Agent, and terminal
  text, with a 1.5 editor line height and a slightly smaller 13 px Git commit
  input.

First-run settings must select the same Lumin and font profile as product
defaults. They must not pin a stale upstream theme or oversized typography that
makes a fresh install look different from the intended Dez experience. Users
remain free to override every role through normal settings.

## Locked identity {#locked-identity}

- Product and stable application name: `Dez`
- Development application name: `Dez Dev`
- Executable: `dez`
- Public version for the first preview: `0.0.1`
- Canonical public URL scheme: `dez://`
- Bundle IDs, update channels, configuration, and mutable data remain isolated
  from official Zed.
- Automatic upstream code synchronization is required.
- Installing an official Zed binary over Dez is prohibited.

The first preview may continue using the existing `Superzed` storage location
as an explicit compatibility boundary. It must not silently strand, delete, or
overwrite legacy data. Replace that boundary only with a transactional,
reversible migration.

## Architecture invariants {#architecture-invariants}

These invariants win when an upstream merge exposes a product difference:

1. One durable app session owns the workspace collection.
2. An operating-system window is a viewport over that session, not an
   independent state universe.
3. Workspaces own panes, surfaces, layout, focus, and workspace-local UI state.
4. Empty and unresolved workspaces are valid and persist until explicitly
   removed.
5. Each workspace owns one upstream-compatible `Entity<Project>`.
6. That `Project` scopes UI-facing context over reusable shared backend stores.
7. Repository, worktree, path, and host selection is never global.
8. Tool-specific selection remains local to the tool.
9. Files and terminal working directories provide evidence; generic tool,
   search, Git, settings, and conversation surfaces do not attach roots merely
   by existing.
10. Worktree discovery alone does not start recursive indexing, language
    servers, heavy diagnostics, or checkers.
11. Panes and tabs are the universal composition model. The sidebar is a
    projection and navigation surface, not a second pane system.
12. Host-owned sessions distinguish close, detach, reconnect, and terminate.
    A terminal's own backing controller is authoritative; the existence of a
    global Host never changes which process an action owns.
13. Closing or disconnecting the GUI does not imply terminating a session.
    Destructive terminal termination is separated from close or detach,
    unavailable after exit or failed restore, and requires an explicit critical
    confirmation that names its effect.
14. Agent state belongs to the actual terminal session or conversation that
    owns it. Sidebar rows focus that existing owner.
15. A Run relates authoritative state; it does not duplicate terminal,
    conversation, Git, or workspace ownership.
16. Critical agent state uses structured events when available, not terminal
    text scraping alone.
17. Summaries distinguish observed facts from generated interpretation and link
    back to diffs, commands, checks, and events.
18. Evidence provenance remains explicit: observed, derived, agent-reported,
    user-confirmed, and unknown facts are never silently flattened.
19. An active attention condition, its unread or acknowledged presentation,
    muting, and final resolution are separate state. Visiting a surface does
    not claim that a permission request, failure, or conflict was resolved.
20. Adapters declare capabilities. UI actions for permissions, resumption,
    patches, checks, context injection, or cost appear only when supported.
21. Session transport state, agent work state, attention state, and Run review
    state remain separate authoritative facts even when one projection presents
    them together.
22. A Change Set relates Git and Evidence; it does not duplicate repository or
    worktree ownership.
23. Credentials, secrets, unbounded terminal output, live language-server
    processes, and diagnostic results are not workspace persistence data.
24. Upstream Zed is merged regularly. Compatible upstream functionality is
    adapted instead of manually recreated.

## Target ownership {#target-ownership}

```text
DezSession
|-- shared backend stores
|   |-- worktrees, Git, buffers, languages, debugger, tasks, search
|   `-- hosts, sessions, Runs, Change Sets, agents, and reusable caches
|-- durable workspace collection
`-- viewport state

Workspace
|-- workspace-scoped Entity<Project>
|   |-- visible evidence
|   |-- visible worktrees and repositories
|   `-- active host, path, worktree, and repository
`-- pane graph
    |-- file surfaces
    |-- terminal surfaces
    |-- agent surfaces
    `-- tool and review surfaces
```

Do not introduce a broad duplicate services owner and synchronize it with
`Project` afterward. Reuse upstream names where their semantics remain correct.

## Terminal and agent model {#terminal-and-agent-model}

```text
terminal surface
-> terminal client
-> host connection
-> durable terminal session
-> PTY and child process
```

The host owns process lifetime, session identity, current working directory,
bounded replay, metadata, attachments, and exit state. The GUI owns rendering,
input focus, pane placement, evidence projection, and user commands.

Dez supports native conversational agents, ACP agents, and agents detected in
ordinary terminals. They appear as peer surfaces. An adapter translates
provider events into generic states such as running, waiting for permission,
waiting for input, completed, failed, disconnected, resumable, and exited.
Codex is the first terminal-agent reference adapter, not an exclusive runtime.

Transport/lifecycle state such as Detached, Reconnecting, Exited, Missing, or
Incompatible belongs to Session. Work state such as running, waiting, checking,
or ready for review belongs to the agent or Run projection. UI may compose
these facts but must not collapse them into one mutable lifecycle enum.

A terminal or conversation can project one Run, but it does not create a second
copy of that Run. Attention items and review briefs link back to the owning
session, surfaces, Git state, and observed evidence.

Terminal attention persists a typed record: active or resolved condition,
unread or acknowledged presentation, normal or urgent priority, optional mute
deadline, explicit resolution/update timestamps, and optional stale expiry.
Opening the owner changes presentation only. Session Rail actions acknowledge,
snooze for one hour, resume, or resolve deliberately. Observed bell events
expire after seven days; structured permission and failure conditions remain
owned and resolved by their adapter. The old SQLite bit remains solely as an
additive migration input and compatibility projection.

## Trust rules {#trust-rules}

1. Never report a check as passing without observing a successful result.
2. Show which actor requested and performed consequential actions.
3. Make permission scope and duration visible.
4. Keep destructive actions explicit and human controlled.
5. Persist minimal structured history and provide a private mode.
6. Never upload terminal output merely to provide persistence.
7. Redact secrets before persistence or model submission.
8. Require confirmation before sharing context between actors.
9. Keep host, provider, and model boundaries visible.
10. Preserve an audit trail for consequential approvals.
11. Make Evidence provenance and truncation visible.
12. Do not expose an adapter action that the owning provider or Session cannot
    actually perform.

## Product boundaries {#product-boundaries}

Before product-market fit, do not prioritize autonomous agent teams, a custom
foundation model, hosted coding sandboxes, organization administration, a new
issue tracker, a replacement for GitHub, collaborative terminal control, full
mobile editing, or unlimited terminal replay.

A proposed feature must reduce lost context, attention cost, review risk, or
session loss, or strengthen the upstream-compatible architecture. Otherwise,
defer it.

## Permanent decisions {#permanent-decisions}

- **2026-07-22: Keep permanent decisions separate from execution state.** The
  Fork Notes remain authoritative while the roadmap changes continuously. A
  single giant plan would mix product invariants with temporary implementation
  detail.
- **2026-07-22: Upstream synchronization is Milestone 0 and a permanent loop.**
  It is release work, not a one-time import.
- **2026-07-22: Validate one vertical product loop before broad platform work.**
  The first complete slice joins workspace recovery, a persistent local
  terminal, agent detection, attention routing, review, and restart recovery.
- **2026-07-22: Preserve the legacy Superzed storage boundary for v0.0.1.** A
  cosmetic rename is less important than retaining user settings and history.
- **2026-07-22: Builds follow source slices, not every edit.** Cheap formatting
  and static checks run continuously. The consolidated build and manual smoke
  run occur at an explicit verification gate.
- **2026-07-22: Add Run as the user-facing unit of active work.** Session remains
  the durable computation primitive. Run connects that computation to intent,
  attention, evidence, review, and outcome without becoming another ownership
  database.
- **2026-07-22: Never silently weaken an explicitly enabled durability mode.**
  If the local host cannot authenticate, connect, or create a shell, expose the
  failure and start no GUI-owned replacement. Reconnection may reconcile later
  commands, but an uncertain command is never replayed automatically.
- **2026-07-22: Reconcile the revised consolidated plan without replacing the
  document hierarchy.** Its complete-product positioning, Evidence provenance,
  adapter capabilities, protocol requirements, and long-range integration map
  are adopted in their owning documents. Its single-file authority, blank
  progress reset, per-slice build mandate, and flattened Run/Session state are
  rejected. The treatment is recorded in
  [Consolidated Plan Reconciliation](./consolidated-plan-reconciliation.md).
- **2026-07-23: Keep Session Rail utilities and Workspace status semantically
  separate.** Agent Tools, Agent History, and recent Workspace navigation
  belong to the Session Rail. Search, diagnostics, language services, file
  state, and editor state belong to the bottom Workspace status/navigation
  toolbar. Terminal-focused status must name useful Workspace-wide actions and
  health states instead of presenting editor-shaped glyphs without context.
  This boundary prevents the terminal-first shell from becoming an
  undifferentiated bottom icon row.
- **2026-07-23: Terminal lifecycle policy is interaction-path invariant.**
  Pointer controls, context menus, and keyboard removal must derive their
  detach, close, remove, or terminate presentation and confirmation requirement
  from the same terminal source/runtime policy. A compatibility action name may
  remain internal, but Dez must present its mixed Session Rail scope truthfully.
  No shortcut may bypass a confirmation required by the visible control.
- **2026-07-23: Capability gates precede product copy.** Renaming an inherited
  setting or action does not make it a Dez feature. A control is visible only
  when the public Dez path consumes it and the claimed effect is implemented.
  Compatibility storage may remain hidden for migrations and upstream sync.
- **2026-07-23: Contextual Session actions follow selection as well as hover.**
  Pointer hover and keyboard active-descendant selection reveal the same row
  controls. Controls that are visually present participate in the tab order,
  retain explicit accessible names, and keep the destructive action last.
  Editing modes may suppress competing actions until editing ends.
- **2026-07-23: Terminal lifecycle language is shared across projections.**
  Main Work Area Surface controls, Session Rail controls, context menus,
  tooltips, and critical prompts use Terminal Session for the user-facing
  computation. Process detail appears only when explaining the concrete shell
  and foreground-process effect. Internal durability terminology never appears
  as product copy.
- **2026-07-23: Terminal tooltip metadata names type and identity.** Ownership
  uses Terminal Session vocabulary. Paths, process identifiers, and Session
  identifiers are labeled precisely; generic Folder, Process, and Session
  prefixes are not sufficient for an inspectable terminal Surface.
- **2026-07-23: Responsive breakpoints follow the control they govern.** The
  Session Rail footer does not inherit the wider breakpoint used for
  supplemental row metadata. The default compact width must keep Agent Tools,
  History, and Workspaces visible; only genuinely narrower rails collapse those
  labels into named, tooltip-backed icons.
- **2026-07-23: Responsive labels reserve space before they appear.** Controls
  made visible at a compact breakpoint use compact padding and typography. A
  breakpoint is incomplete if its newly revealed labels can only fit by
  clipping, wrapping, or stealing the primary content measure.
- **2026-07-23: Global and row-scoped actions name different destinations.**
  A global Session Rail action names the active Workspace and the Main Work
  Area it will change. An action attached to a Workspace row names that visible
  Workspace. Concise visible labels may omit repeated context only when the
  accessible name, tooltip, and nearby explanatory copy retain the full
  destination. Agent-owned terminal language must not return to Workspace
  terminal creation.
- **2026-07-23: Settings describe visible regions, not compatibility
  containers.** Files, Outline, and Git are Workspace Tools; Agent is a
  dedicated region; terminals are Main Work Area Surfaces. Internal panel keys
  remain compatible with upstream storage and APIs, but Dez does not expose
  dock position or dock-only sizing controls while legacy docks are hidden.
  Settings keep only controls whose effect is reachable in the public shell.
- **2026-07-23: Agent Session is the visible conversation unit.** Context
  menus, restart effects, feedback disclosures, export actions, recovery, and
  history name Agent Sessions throughout Dez. Compatible internal Thread types,
  persistence keys, telemetry events, and `@thread` insertion syntax may
  remain. Official Zed keeps its upstream Thread vocabulary.
- **2026-07-23: Terminal output is not application chrome.** Dez does not edit,
  suppress, or reinterpret a shell prompt's escape-sequence output, including
  prompt status, clocks, or artwork. Terminal Session identity and lifecycle
  remain visible outside the PTY grid through the Surface tab header, even when
  the general single-tab auto-hide preference is enabled.
- **2026-07-23: Settings disclose consequential Agent behavior where it is
  configured.** Agent settings use Agent Session, Surface, Agent card, and
  Workspace status vocabulary. A feedback toggle names its upstream
  data-sharing effect instead of relying on a later hover tooltip. Official Zed
  may retain upstream Thread, buffer, Panel, and status-bar copy.
- **2026-07-23: Session switching complements Surface switching.** `Ctrl-Tab`
  retains conventional Surface/tab switching in the Main Work Area. While
  Agent or the Session Rail has focus, the same chord cycles Sessions. The
  global Command Palette exposes **Session Rail: Switch Sessions** so the
  supervision action remains keyboard-reachable without overriding editor
  muscle memory.
- **2026-07-23: Session Switcher guidance follows its invocation mode.** When a
  held shortcut opens the switcher, the footer and accessible description tell
  the user to continue cycling and release to open. When a direct command opens
  it, they tell the user to repeat the command, press Enter to open, or Escape
  to return. Mixed Terminal Session and Agent Session rows retain quiet visual
  metadata but expose type, selection, position, and collection size to
  assistive technology. The switcher previews work; hovering never does.
