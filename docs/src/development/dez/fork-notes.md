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
an accessible name and tooltip.

The main-area tab-bar plus control is named **Add to Main Work Area** in Dez.
Its menu opens files, Workspace search and symbols, or a terminal in that same
pane grid; it does not add a sidebar panel or create a second terminal model.

Discarding an Agent Session draft from either its Session Rail row or its
main-area tab requires confirmation because unsent prompt text is permanently
removed. Archiving a saved Agent Session remains immediate and reversible from
Agent History.

Visible conversation scope is **Agent Session** throughout the Agent pane,
context picker, search, sandbox status, and Session Switcher. Internal action,
protocol, database, and mention identifiers may retain `thread`; the compatible
mention keyword remains `@thread`. The Agent menu names its destinations as
**Agent Settings** and **Toggle Session Rail**.

Session Switcher previews are reversible. Confirming records and focuses the
selected Session; cancelling restores the original Agent Session, center
terminal Surface, or Host Session through its actual source without changing
ownership or routing a terminal through Agent.

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
  editor, diff, diagnostics, and Git tools.
- Search, settings, diagnostics, and review briefs open as normal main-area
  Surfaces. They can be tabbed or split without becoming permanent sidebars.
- The Session Rail observes these surfaces and durable sessions. It adds
  navigation, attention, evidence, and recency, but never becomes the editor,
  terminal, Agent, or process owner.

This is the IDE integration: Dez retains Zed's editor and project engine, then
organizes its existing capabilities around terminal-native supervision. The
terminal is not embedded inside chat, and the editor is not a separate mode.
Both are first-class Surfaces in one Workspace.

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
13. Closing or disconnecting the GUI does not imply terminating a session.
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
