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
| **Sessions**        | Search, attention scope, Workspace grouping, and navigation        | Terminal processes, Agent Sessions, editor state, or duplicate tabs |
| **Workspace Tools** | Files, Outline, Git, and Debug tabs in a hideable left tool pane   | A second Workspace, root selection, or terminal placement           |
| **Main work area**  | File, terminal, search, diagnostics, settings, and review Surfaces | Global project scope or sidebar-only copies of active work          |
| **Agent**           | Native and ACP conversation Surfaces in a hideable right tool pane | Terminal-agent process ownership                                    |

Every visible **Start Terminal Session** action creates a normal main-area
Surface. It can be tabbed, split, moved, detached from a durable Host Session,
or reattached without introducing a separate Terminal Panel model. Compact
toolbar affordances may use **Start Terminal** beside the **Sessions** title,
but their tooltip and accessible name must keep the full Main Work Area
destination.

Sessions rows are projections. Selecting a terminal row focuses its
attached terminal Surface or reattaches the Host-owned Session. Selecting an
Agent Session row focuses its existing conversation Surface. A row may compose
actor, lifecycle, attention, evidence, changes, and recency, but it never
becomes a second owner of those facts.

Terminal title data remains full through local, durable Host, retained-Agent,
Sessions, and Session Switcher metadata paths. Visual tabs and rows own
space-based truncation so tooltips and restored projections retain useful
identity. Explicit custom names are trimmed, persist across restoration, and
override the live shell title without discarding decorative agent-state
prefixes. The action is named **Rename Terminal…** and double-clicking the tab
invokes the same editor.

At zero sessions, the overview owns the rail title and **No sessions yet**
status. The compact start block uses one quiet **Start with a Workspace**
heading rather than a decorative card. It explains that a codebase supplies
context to Terminal and Agent Sessions and that their changes return to the IDE
for review, offers one filled **Open Workspace…** action, and keeps **Open
Scratch Terminal** as the outlined pathless alternative. With an active
Workspace but no Session, the primary recovery action is **Start Terminal
Session**. Start, search recovery, attention scope, and Session scope actions
are keyboard tab stops and name their destination in accessibility output.
Once one or more Sessions exist, the overview's compact **Start Terminal**
control becomes an outlined utility because the Session list, not repeat
creation, is the primary content.

Workspace Tools and Agent are ordinary pane-grid regions with stable placement
and normal focus behavior. Hiding one keeps its items available, returns focus
to a visible editor or terminal pane, and persists the layout. Opening a named
tool reveals the correct region and activates its existing tab.

### Everyday routing {#interface-everyday-routing}

Action names describe their destination. They do not expose inherited panel
terminology:

| Intent                                        | Result                                                                 |
| --------------------------------------------- | ---------------------------------------------------------------------- |
| **Start Terminal Session**                    | Opens a terminal tab in the active Workspace's main work area          |
| **New Agent Session**                         | Opens or focuses a conversation in the right Agent pane                |
| **Files**, **Outline**, **Git**, or **Debug** | Opens the named tab in left-side Workspace Tools                       |
| Select a Sessions row                         | Activates its Workspace and focuses or reattaches the existing Surface |
| Hide Workspace Tools or Agent                 | Hides that region and returns focus to an editor or terminal           |
| Split or move a Surface                       | Rearranges the same Workspace; it does not create a second project     |

The active or keyboard-focused Workspace keeps **Start Terminal Session** and
**Workspace Options** visible. Other inactive Workspace actions may reveal on
hover because selecting or focusing the Workspace first makes the same controls
persistent. Every icon-only control must retain an accessible name, tooltip,
and keyboard tab stop. Repeated Workspace-row controls include the visible
Workspace name in both; internal element and hover-group identifiers are
presentation-only and must never enter user-facing copy.

Visible controls must also perform their advertised action: the Agent title
pencil starts editing, worktree closure names its window scope, and the
main-area overflow control is **Switch Surface**, not Open Tab.

Action hierarchy follows the next useful transition. Dez Welcome emphasizes
exactly one recommended first action: **Open Workspace** without a codebase, or
**Start Terminal Session** in an active Workspace. Secondary creation and
navigation actions remain available without competing for the same visual
weight. Critical controls must not depend on pointer hover in Dez. Icon-only
toolbar controls must be keyboard-focusable, expose a specific accessible
name, and use the same wording in their tooltip. Official Zed compatibility
branches may retain upstream hover and icon behavior.

Welcome's primary section is deliberately limited to three transitions. With
no Workspace it offers **Open Workspace**, **Clone Repository**, and **Open
Scratch Terminal**. With a Workspace it offers **Start Terminal Session**,
**Open Files**, and **New File**. Generic utilities such as the command palette
and replacing the active Workspace remain available through normal chrome, but
do not compete with the release-defining start loop.

The empty Main Work Area uses the same **Start Terminal Session** vocabulary.
It describes live terminal and Agent state without calling the default
GUI-owned terminal durable; durability is shown only when an external Host
actually owns the exact Session.

Empty primary regions use compact, top-anchored recovery guidance rather than
floating a small prompt in the middle of an empty pane. The heading names the
missing prerequisite, the explanation is specific to Files, Git, or Agent, and
the primary action says **Open Workspace**. That action always accepts folders
and keeps the current Dez window: a file cannot accidentally satisfy a
Workspace prerequisite, and recovery cannot strand the current Sessions in a
different window. Shared Open Workspace and Clone Repository controls are
keyboard tab stops.

Popover triggers expose their current state as well as their destination.
Agent Options and New Agent Session use selected treatment while open and
report expanded state to assistive technology; closing the popover clears both
signals.

Dez Agent onboarding is provider setup, not Zed AI subscription onboarding.
The reachable empty state is a named **Agent provider setup** region that says
**Agent Session**, offers **Configure Agent Providers**, and exposes **Start
Agent Session** only after a non-Zed-cloud provider is authenticated. Both
actions are keyboard tab stops. Inherited Zed plan/trial components may remain
for upstream compatibility, but the Dez Agent entry path must not render them.

The Agent composer control row has one interaction contract. Expand or
minimize, Add Context, Follow, Fast Mode, Thinking Mode, thinking effort,
Send/Queue, Stop Agent Run, and Sandbox Settings are keyboard tab stops with
specific accessible names. Toggle controls report their current state, popup
controls report expanded state and current value where applicable, and active
state uses selected treatment rather than icon color alone. These controls
configure or operate the current Agent Session; they do not create hidden
terminal ownership or a second Project.

Conversation follow-up controls remain visible at rest instead of lowering the
entire action row to decorative contrast. Copy Response, return-to-prompt,
return-to-top, feedback, and feedback submission are named keyboard tab stops;
feedback reports its selected state.

Queued messages form a named ordered list. Every row reports its position and
whether it is next, uses a row-specific status identifier, and keeps Remove,
Edit, Steer, and Send Now visibly available to pointer and keyboard users.
Steer changes when that queued message interrupts the current Agent Run; Send
Now operates on the exact queued-message identity.

Agent permission prompts keep Allow, Deny, retry, and provider-supplied choices
in the keyboard tab order. Permission Scope reports its current value and
expanded state, uses selected treatment while open, and applies the chosen
scope to the exact pending tool call. A warning may disable Allow, but it must
not hide Deny or Retry.

Agent Review is an IDE workflow, not hover decoration. Every changed-file row
keeps Review, Reject, and Keep visible at rest, assigns row-specific element
identity, and preserves the exact Buffer target. Pending edits disable Keep and
Reject with an explicit reason. Review Changes, Reject All, and Keep All are
keyboard tab stops.

A subagent title bar names **Stop Subagent** and **Return to Parent Agent
Session** as keyboard actions. Returning changes presentation only; it
navigates to the existing parent conversation and does not cancel or recreate
either Session.

Restoring an Agent checkpoint is a consequential Workspace-wide file
replacement. Dez names the scope, requires a warning confirmation with
**Restore Files** and **Cancel**, and only invokes the exact message checkpoint
after confirmation. Cancelling or restarting an edited message is separately
named and keyboard-reachable.

Agent errors and warnings separate state from recovery. Retry, Authenticate,
Configure Provider, Select Model, Open Skill, environment recovery, updates,
and dismiss controls are keyboard tab stops with specific names. Dismiss only
hides the named notice; it never claims to retry, repair, configure, or clear
the underlying condition. Provider retry copy names the running product, never
inherited Zed.

The shared Copy control is keyboard-addressable when rendered and exposes the
same name as its tooltip, including its temporary copied state. Skill-warning
rows name the exact file they open rather than presenting an anonymous row.

Provider data-retention consent is a trust boundary. Learn More and the
retention-safe fallback remain available; **Accept** opens a warning that names
the persistent Dez setting, Anthropic log retention, and the current-request
retry. Only **Accept and Retry** changes the setting and resends; **Cancel**
leaves both untouched.

Sessions recovery notices preserve ownership truth. Terminal Session
startup and reconnection copy states whether a shell was started and whether
running processes were touched; raw transport details stay behind
keyboard-reachable **Open Local Log** and **Copy Details/Error** actions.
Workspace restoration offers **Open Recent Workspaces** to retry and **Remove
Recovery Entry** to remove only the rail's recovery record. Removal does not
delete recent Workspace data and never presents itself as a successful reopen.

Agent tool cards teach their behavior without requiring pointer discovery.
Copy Code, Copy Command, and expandable-output controls remain visible at
rest. Disclosure controls are keyboard tab stops, announce the exact content
they expand, and expose expanded state. **Stop This Command** acts on the exact
running terminal tool call; truncation and exit icons are status, not fake
buttons. **Discard Interrupted Edit** targets only that partial edit.
Subagents expose distinct **Stop Subagent**, **Expand Subagent Preview**, and
**Open Subagent Session** actions; opening a Session navigates to that existing
conversation and does not create or restart work.

The everyday Canvas Layout menu is a workflow picker, not a diagnostics or
storage dashboard. It exposes Full, Agent Control, Focus Editor,
Code/Run/Observe, Review, and Debug; saved-layout detail belongs in **Manage
Saved Layouts…**. The active Workspace exposes this submenu through its
persistent **Workspace Options** control in Sessions. Official Zed's
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

Every visible pane-chrome control is a keyboard tab stop: Back, Forward, Add to
Main Work Area, Switch Surface, Split, Zoom, Hide Workspace Tools, and Hide
Agent. In Dez, the active unpinned Surface keeps its close control visible and
keyboard-focusable even when the user preference otherwise reveals tab close
buttons on hover. Inactive tabs remain quiet, and pinned tabs preserve their
dirty/status indicator until hover reveals Unpin. Official Zed retains its
upstream tab-close presentation.

The global Workspace status strip stays compact in healthy states. Search uses
one conventional icon with the accessible name **Search Workspace Files**;
zero diagnostics uses a check icon announced as **Workspace diagnostics: no
problems**. Errors, warnings, counts, and diagnostic messages remain visible.
The strip does not spell out healthy utility labels merely because a terminal
has focus; orientation belongs to Sessions and the terminal context strip.

Discarding an Agent Session draft from either its Sessions row or its
main-area tab requires confirmation because unsent prompt text is permanently
removed. Archiving a saved Agent Session remains immediate and reversible from
Agent History.

Visible conversation scope is **Agent Session** throughout the Agent pane,
context picker, search, sandbox status, and Session Switcher. Internal action,
protocol, database, and mention identifiers may retain `thread`; the compatible
mention keyword remains `@thread`. The Agent menu names its destinations as
**Agent Settings** and **Toggle Sessions**.

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
menus and the command palette, redirects compatibility dispatches to **Start
Terminal Session**, and does not restore the inherited Agent-terminal surface
after a restart.

### How IDE features integrate {#interface-ide-integration}

Each Workspace owns one upstream-compatible `Project`. Editors, terminals,
language servers, search, diagnostics, Git, debugger state, tasks, and Agent
context all resolve through that same Workspace and Project:

- A file opens as a main-area Surface. Language intelligence and diagnostics
  come from the Workspace's Project.
- A terminal opens beside files in the same pane grid and starts with that
  Workspace's working-directory context.
- Files, Outline, Git, and Debug are alternate views of the same Project. They
  do not create a second root or copy state into Sessions.
- The Agent pane uses the active Workspace's Project context. Agent edits land
  in ordinary buffers and Git changes, so they remain reviewable with the same
  editor, diagnostics, and Git tools. **Agent Review** is the interactive
  change Surface for Keep/Reject decisions; a **Review Brief** is the separate
  evidence summary for a Run.
- Search, settings, diagnostics, and review briefs open as normal main-area
  Surfaces. They can be tabbed or split without becoming permanent sidebars.
- Sessions observes these surfaces and durable sessions. It adds
  navigation, attention, evidence, and recency, but never becomes the editor,
  terminal, Agent, or process owner.
- Empty and recovery states must state this ownership boundary in their visible
  copy. A Sessions action may open a terminal in the main work area, but it
  must never imply that the terminal runs inside the rail.
- Workspace change counts and **Review Changes** share the same repository
  scope. In a multi-repository Workspace, review keeps the active repository
  when it is dirty; otherwise it deterministically selects the first dirty
  repository and opens a real changed-file diff. It never advertises aggregate
  changes and then reviews an unrelated clean repository.
- Git Changes reserves the Workspace Tools column for changed-file navigation.
  Its collapsed commit composer is four lines; longer work uses the explicit
  full-height or modal expansion controls. View Diff, stage/unstage, commit,
  remote, and split-menu controls all enter the keyboard tab order, expose
  specific accessible names, and report whether their popup is open.
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

That contract applies to stable secondary windows as well as the main
Workspace. About, sign-in/verification, Settings, Audio Test, and profiling
windows must request the active theme's native window material and initialize
the configured UI font before rendering. Intentionally shaped notification
popups remain transparent so their rounded shell is preserved, but still
initialize the same UI font. A secondary window must never silently fall back
to an opaque native background or GPUI's default text face.

Typography uses one explicit Dez identity:

- **JetBrains Mono** is bundled under the SIL Open Font License and is the
  default for navigation, labels, menus, buffers, terminals, Agent content,
  Markdown, review, settings, and Git commit input.
- Users can independently override UI, buffer, terminal, Agent, and Markdown
  roles through normal settings when they prefer proportional prose.
- The balanced v0.0.1 baseline is 14 px for UI, editor, Agent, and terminal
  text, with a 1.5 editor line height and a slightly smaller 13 px Git commit
  input.

First-run settings must select the same Lumin and font profile as product
defaults. They must not pin a stale upstream theme or oversized typography that
makes a fresh install look different from the intended Dez experience. Users
remain free to override every role through normal settings.

The app menu and command palette expose **Restore Dez Visual Profile** as an
explicit recovery path. It writes only the system-selected Lumin Light/Lumin
Blur pair, **Dez (Default)** icons, and JetBrains Mono for interface, buffer,
terminal, and Markdown code roles. It preserves sizes and unrelated settings,
waits for persistence, and shows success only after the write completes.

The upgrade path recognizes only known exact Dez-generated profile signatures.
The first used `.ZedSans` beside Lumin; the earlier installed profile also
pinned light mode to One Light despite claiming to follow the system. Dez
upgrades those generated values in memory to JetBrains Mono, system appearance,
Lumin Light, and Lumin Blur through the normal backup-and-update flow. It never
rewrites official Zed settings or an arbitrary custom font/theme profile.

Primary icon roles are semantic and stable: Terminal means terminal
computation, Folder Open means Workspace/Files, File means file creation, Diff
means change review, Info means Session details, List Tree means supervision,
Clock means Agent History, and Settings means configuration. Visible labels
remain authoritative; icons reinforce them instead of replacing them.
Recognized terminal-agent providers use their bundled provider icon in both the
Sessions list and session switcher; only providers without a specific bundled
mark fall back to the neutral Robot icon. One shared mapping owns both surfaces.
The selector exposes the built-in file/folder set as **Dez (Default)**.
**Zed (Default)** remains registered only as a compatibility alias for
upstream behavior and existing settings.

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
Opening the owner changes presentation only. Sessions actions acknowledge,
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

## Reference lessons: Synara {#reference-lessons-synara}

Synara is a useful reference, not a blueprint. Its
[README](https://github.com/Emanuele-web04/synara/blob/main/README.md)
describes a local-first desktop app that brings chats, terminals, browser
previews, diffs, branches, provider sessions, and handoffs into one workspace.
Its
[External MCP document](https://github.com/Emanuele-web04/synara/blob/main/docs/external-mcp.md)
describes a user-approved bridge that lets local MCP clients create, wait for,
and read scoped tasks through narrow permissions.

Dez should learn these specific lessons:

- The first-run surface should make the next action obvious and state current
  context. Synara's centered composer shows project, runtime, branch, access
  level, model, and temporary-worktree state in one place. Dez should apply that
  clarity to Welcome, empty Main Work Area, terminal context, and Agent
  composer chrome, while keeping files and terminals as the primary work
  surfaces once a Workspace exists.
- Handoff is a first-class action. A provider or Agent Session should be able to
  hand work to another provider only through an explicit transition that carries
  evidence and review context. Handoff must not create hidden terminal or
  Workspace ownership.
- External automation needs a scoped local integration, not ambient access. A
  future Dez MCP bridge should expose overview, capabilities, allowed
  Workspaces, create Run, wait for Run, and read owned Run tools before any
  broad read-project capability.
- Safe defaults matter more than broad power. External integrations should
  default to managed worktrees, approval-required execution, task ownership, rate
  limits, revocation, audit rows, and idempotent request IDs. Full-access and
  local-checkout execution require separate explicit scopes.
- Setup should not ask the user for internal IDs. A generated setup prompt or
  copy-ready MCP configuration must use the exact running executable, data
  directory, and integration identity; if multiple identities exist, Dez should
  fail clearly instead of guessing.
- Local-first privacy must be stated at the boundary. Dez can coordinate
  provider sessions locally, but prompts, snippets, diffs, terminal output, and
  tool results still go to the selected provider when required for a Session.

Dez should not copy Synara's model wholesale. Dez remains a complete native IDE:
files, terminals, Git, diagnostics, debugger, search, and review stay
first-class. Kanban, broad automations, and a permanent centered chat composer
are future options only if they strengthen the terminal-to-IDE review loop.

## Permanent decisions {#permanent-decisions}

- **2026-07-24: Upstream terminal cleanup must preserve hosted-session
  ownership.** Ordinary terminal close inherits upstream process-group
  termination. Closing the Dez GUI detaches from hosted Terminal Sessions and
  does not terminate their process groups; explicit stop remains the deliberate
  termination path. Merge conflict resolution and identity checks must preserve
  this distinction.
- **2026-07-24: A disabled upstream sandbox is not a Dez security claim.**
  Upstream temporarily feature-gated agent terminal sandboxing in the v0.0.2
  parity train. Dez will document the inherited state, retain confirmation and
  trust-boundary UX, and make no sandbox-protection claim until the withdrawal
  is understood and runtime enforcement is verified.
- **2026-07-25: The Main Work Area owns the horizontal budget.** Workspace
  Tools and Agent are contextual regions, not equal peers of the file,
  terminal, and review canvas. Each starts at no more than 360 px or 22% of
  visible horizontal space, and their combined width cannot silently reduce
  the Main Work Area below 60%. This invariant applies after pointer or
  keyboard resizing, explicit pane-size reset, visibility changes, layout
  recipes, and persisted-layout restoration. **Reset Pane Sizes** returns to
  the Dez hierarchy rather than equalizing contextual tools with active work.
  Persistence must retain Agent, Workspace Tools, and Main Work Area region
  identity. On shells below 1800 px or narrower than a 1.6:1 aspect ratio,
  Workspace Tools and Agent are mutually exclusive drawers: revealing one
  hides the other, restored double-drawer layouts collapse deterministically,
  and both may coexist only on an ultrawide canvas.
- **2026-07-25: One-work-area layouts remove only empty leftovers.** Full,
  Agent Control, and Editor Focus recipes select one authoritative Main Work
  Area and hide surplus empty tab panes left by earlier split recipes. A pane
  containing a file, terminal, diff, or any other user Surface is never hidden
  by this cleanup. The default pane focus indicator lives in the title/selected
  tab rather than painting a saturated rectangle around the full work surface;
  optional border focus remains user-configurable and defaults to one pixel.
  The remaining empty Main Work Area is one bounded launch panel headed **Run.
  Supervise. Review.**, with only the three immediate Workspace actions and a
  compact route row that names **Run -> Main Work Area**, **Supervise ->
  Sessions**, and **Review -> Files + Git**.
- **2026-07-25: Terminal context is chrome, not another panel.** The standalone
  terminal handoff is one 32 px tab-aligned header with lifecycle, repository,
  Files, Review Changes, and Session Details. It uses the tab-bar surface,
  removes the redundant visible **Terminal Session** actor title, and keeps the
  complete actor identity in its accessible name and details disclosure. The
  supervisor region is visibly titled and named **Sessions**, and its true-empty
  state repeats the concrete route: **Run -> Main Work Area**, **Supervise ->
  Sessions**, and **Review -> Files + Git**. **Session Rail**
  remains an implementation and historical documentation term, not unexplained
  primary UI copy.
- **2026-07-25: Transient feedback cannot become a fifth region.** Workspace
  notifications are one named shelf bounded to the actual Main Work Area,
  between 280 and 400 px when space permits, never wider than the available
  surface, and capped at 36% of its height with internal scrolling. The shelf is
  top anchored below the toolbar/tab area. Toasts no longer allocate an invisible
  full-screen layer; only their visible content occludes input, within 90% of
  viewport width, 560 px, and 30% of viewport height, with bottom clearance for
  the status bar. Modal scrims remain full-screen only when they intentionally
  block the application.
- **2026-07-25: Lumin glass is a native material hierarchy.** On macOS the stable
  Dez window uses the native under-window material, blends behind the window, and
  follows active/inactive system state. Lumin then layers semantic surfaces in
  this order: OS backdrop, root background, Sessions and auxiliary drawers, Main
  Work Area editor or terminal, bounded surface cards, then elevated feedback.
  Workspace Tools, Git, Outline, Debug, and Sessions use panel material; the Main
  Work Area uses editor/terminal material; empty work cards use surface material.
  Lumin Light controls are translucent interaction layers, not opaque beige
  blocks. Nonblocking feedback on glass uses lighter elevation and no modal
  shadow.
- **2026-07-25: Recovery surfaces name their real destination.** Empty
  Workspace Tools and Agent drawers are named regions whose single recovery
  closes the drawer and returns to the **Main Work Area**, not generically to
  an editor. Directional arrows point inward. New File consistently uses the
  File object icon. Welcome explains Run, Supervise, and Review inside one
  coherent panel rather than three competing cards.
- **2026-07-25: Destination labels imply idempotent navigation.** **Open
  Workspace** accepts folders, not standalone files, and keeps the current
  window when the flow promises to preserve Sessions. **Open Files** reveals
  and focuses Files; repeating it never toggles Workspace Tools closed or
  exposes whichever tool happened to be active before. Start-fresh controls
  remain unavailable while App Session restoration is pending.
- **2026-07-25: Product marks are not generic object icons.** Official Zed may
  retain its branded Agent and Assistant marks. Dez uses Robot for Agent
  Sessions, Sparkle for Inline Assist, and Blocks for the external-Agent
  registry across editor, terminal, diagnostics, Git review, conflicts, setup,
  and Agent controls. Provider-supplied icons remain provider-owned.
- **2026-07-25: Visual priority and input priority must agree.** Welcome has
  one filled recommended transition rather than a grid of equally weighted
  choices. Debug uses object-specific Debug and Stop icons in Dez instead of
  generic creation and power symbols. Git, Debug, Outline, and Agent toolbar
  controls enter the keyboard tab order and carry explicit accessible names.
  The Agent title-edit control remains visible in Dez instead of existing only
  inside a pointer-hover group. Official Zed retains its upstream presentation.
- **2026-07-25: Establish IDE context before pathless computation.** In a truly
  empty app, both Welcome and Sessions make **Open Workspace** the primary
  transition. **Open Scratch Terminal** remains available but secondary because
  it has no Files or Git context. Once a Workspace exists, **Start Terminal
  Session** becomes the primary zero-session recovery. Restoration status takes
  precedence over stale attention styling so **Loading sessions** cannot show
  a contradictory warning icon.
- **2026-07-25: Keyboard focus reveals the same Workspace controls as
  pointer hover.** A focused Sessions Workspace keeps its named **Start Terminal
  Session** and Options controls visible and keyboard-focusable. Inside an
  already-open Workspace menu, per-worktree close controls remain visible and
  enter the tab order rather than requiring a second pointer hover. Search
  clearing and import-banner dismissal follow the same rule.
- **2026-07-25: Main Work Area chrome uses one keyboard contract.** Back,
  Forward, Add, Switch Surface, Split, Zoom, and auxiliary Hide controls all
  enter the tab order and retain explicit destination names. The active
  unpinned Dez tab keeps Close visible without hover; inactive tabs remain
  visually subordinate, and pinned dirty tabs keep their status indicator.
- **2026-07-25: Generated visual profiles migrate as a unit.** The older
  installed Dez profile pinned `.ZedSans`, light mode, and One Light while its
  generated comment promised Lumin system behavior. The migration recognizes
  that exact signature and upgrades UI font, appearance mode, and light theme
  together. Deliberate custom settings and official Zed remain untouched.
- **2026-07-25: Session creation emphasis follows state.** A ready Workspace
  with no Session gives **Start Terminal Session** the filled treatment and
  names the target Workspace. After Sessions exist, the overview's compact
  **Start Terminal** remains visible but becomes an outlined utility so it does
  not compete with supervision and review.
- **2026-07-25: Healthy Workspace status is compact.** Terminal focus no longer
  expands Search and zero-diagnostics into persistent text labels. Their
  keyboard stops, tooltips, and complete accessible names remain, while real
  errors, warnings, counts, and diagnostic messages stay visible.
- **2026-07-25: Visual identity has an explicit safe recovery action.**
  **Restore Dez Visual Profile** is available from Settings and the command
  palette. It restores only Lumin, JetBrains Mono, and built-in Dez icons,
  preserves sizes and non-visual preferences, and confirms only after the
  settings write succeeds.
- **2026-07-23: Public source must explain the product and its evidence
  boundary.** The canonical repository front page names Dez, explains how the
  Sessions, Workspace Tools, Main Work Area, Agent, and one
  Workspace-scoped Project fit together, credits Zed and third-party assets,
  and distinguishes the current source candidate from a signed binary release.
  Historical artifact hashes remain evidence after generated targets are
  removed, but stale local paths must never be presented as runnable
  candidates.
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
  labels into named, tooltip-backed icons. At detailed widths, History expands
  to Agent History and Workspaces expands to Recent Workspaces so neither
  destination is confused with Git history or the active Workspace.
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
- **2026-07-25: Terminal context actions yield before they clip.** At normal
  Main Work Area widths, Files/Open Workspace, Review Changes, and Session
  Details retain their visible labels. In narrow panes they become the same
  icon-backed controls with complete accessible names and matching tooltips;
  lifecycle and Workspace metadata remain available through Session Details.
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
- **2026-07-23: Public tool names describe regions, not compatibility types.**
  Command Palette namespaces, empty-state guidance, and cross-tool handoffs use
  Files, Outline, Git, Debug, Agent, Workspace Tools, and Session Rail. Internal
  `*_panel` action namespaces, keys, persistence records, and upstream APIs may
  remain, but they do not define Dez's public shell. Official Zed retains its
  inherited Panel vocabulary.
- **2026-07-23: Workspace Tool empty states explain state and recovery.** A
  retained tool does not collapse every empty result into one generic message.
  It distinguishes missing input, valid empty content, filtered-out content,
  and search with no results. Recovery stays inside the state when it is
  immediate—for example **Clear Filter**—and the state is a named accessibility
  status near the top of the region rather than decoration floating in dead
  space.
- **2026-07-24: Main-area utility states follow the same quiet hierarchy.**
  Workspace Search distinguishes ready, indexing, searching, and no-match
  states instead of centering a generic headline and a stack of feature
  buttons. The query field remains the next action, progress uses one semantic
  status, and continuous progress rotation stops when Canvas motion is reduced.
  Search results remain ordinary Main Work Area content rather than a new
  panel or dashboard.
- **2026-07-23: A clean Git repository is a positive state.** Git presents
  **Working tree clean** with the active branch, not an error-like “nothing”
  sentence. On a feature branch it offers **Review Branch Changes** and names
  the base-branch comparison; on `main` or `master` it does not advertise an
  irrelevant comparison. The state follows the same compact top-aligned
  Workspace Tool hierarchy.
- **2026-07-23: Git setup names its Workspace effect.** When a Workspace has an
  open folder but no repository, Git explains that initialization starts
  tracking that folder and offers one primary **Initialize Repository** action.
  The setup path is visually distinct from clean, unsafe, and unopened
  Workspace states; those conditions must not collapse into one generic Git
  placeholder.
- **2026-07-23: Git safe-directory approval is a security decision.** The
  warning names the affected repository path and the global configuration
  effect before approval. The trust action uses warning treatment and exposes
  the exact `git config --global --add safe.directory …` command in its
  tooltip. Documentation remains a separate action. Familiarity with a folder,
  not convenience, is the decision criterion.
- **2026-07-23: Debug is a Workspace Tool and Debug Session launcher.** The
  region, tab, and command language use **Debug**. Its idle state explains the
  debugging workspace, then prioritizes **Start Debug Session** over
  configuration, documentation, and adapter discovery. Breakpoints remain
  visible before a session and their empty state teaches the editor-gutter
  entry point. Internal Debugger and DebugPanel types remain compatible.
- **2026-07-23: Unavailable Terminal Sessions have one recovery surface.** The
  PTY grid never carries synthetic application copy or an inert cursor. The
  warning outside the grid preserves the terminal's original title, names the
  exact failure, and confirms that no replacement shell started. **Start Fresh
  Terminal** creates separate computation in the Main Work Area; it is not a
  reconnect or replay action.
