# Dez Architecture Baseline

This page maps the code that exists now to the ownership model in the
[Fork Notes](./fork-notes.md). It is descriptive, not an alternate source of
truth. Update it when a source change moves an ownership boundary.

## Current ownership

| Concern                                                    | Current owner                                                                      | Persistence and lifetime                                                                                                                                          | Gap from the Dez target                                                                                                             |
| ---------------------------------------------------------- | ---------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Launch identity, restore lifecycle, and durable membership | `session::AppSession`                                                              | Stores launch identity, window stack, ordered Workspace membership, ordered viewport composition, resolution, and active selection in KVP; owns restore lifecycle | SQLite remains Workspace-content authority and windows still own live entities                                                      |
| Live workspace registry                                    | `workspace::WorkspaceStore`                                                        | Holds weak workspace entities paired with window handles for the process lifetime                                                                                 | Registration is window-bound and primarily supports collaboration routing                                                           |
| Window composition                                         | `workspace::MultiWorkspace`                                                        | Owns retained workspaces, active workspace, Session Rail state, and per-window serialization                                                                      | An operating-system window is still the concrete collection owner rather than a viewport over one durable app session               |
| Workspace                                                  | `workspace::Workspace`                                                             | Owns pane graph, items, local UI state, database ID, and one `Entity<Project>`                                                                                    | This is close to the target, but unresolved and empty workspace identity still needs end-to-end verification                        |
| Durable workspace rows                                     | `workspace::WorkspaceDb`                                                           | SQLite stores workspace, pane, item, session, and window bindings; KVP stores `MultiWorkspace` state                                                              | The data can restore a prior launch, but the owner is distributed across DB, `AppSession`, `WorkspaceStore`, and windows            |
| Project scope                                              | `project::Project`                                                                 | Each workspace constructs local or remote worktree, buffer, Git, LSP, task, debugger, environment, and agent-server stores                                        | Stores are project-owned rather than shared backend stores viewed through workspace scope                                           |
| Pane tool Surfaces                                         | `workspace::PanelItem` in Workspace-tools and Agent pane kinds                     | With legacy docks hidden, Files, Git, Outline, Debug, Terminal, and Agent remain Workspace-owned pane tabs; internal Project keys and legacy docks remain compatible | Cross-Workspace Surface movement and rendered serialization/focus proof remain open                                                  |
| Local terminal and process                                 | `dez-terminal-host`, hosted `Terminal`, and `LocalTerminalHost` compatibility path | The opt-in helper owns PTYs across GUI lifetimes; the GUI emulator owns display state; the default path remains in-process pending verification                   | Helper routing is still experimental and task terminals have not moved; consolidated build and restart evidence remain open         |
| Agent-terminal history                                     | `agent_ui::TerminalThreadMetadataStore` and Host/Session snapshots                 | SQLite stores terminal identity, workspace metadata, restart-safe attention, and nullable Host/Session references; the helper retains structured adapter state    | Structured state currently requires explicit Codex hook setup and the opt-in helper                                                 |
| Native agent conversations                                 | `agent_ui::ThreadMetadataStore` and agent thread stores                            | Structured conversation metadata and provider state use their existing stores                                                                                     | Native evidence remains separate from the terminal Codex hook adapter                                                               |
| Remote execution                                           | `remote::RemoteClient` and `remote_server::HeadlessProject`                        | Reconnectable SSH, WSL, Docker, and mock transports can preserve a remote project server                                                                          | The remote project transport is not yet a vendor-neutral Host and terminal-session service                                          |
| Session Rail                                               | `sidebar::Sidebar`                                                                 | Derived projection over agent threads, terminal metadata, live terminal entities, helper snapshots, notifications, and transient review briefs                    | Structured Codex activity is functional in source; file/Git provenance, a second adapter, and live restart verification remain open |

The Session Rail now also constructs a transient `RunReviewBrief` from those
authoritative owners. It does not persist another Run record. The brief opens
as a normal Markdown pane and identifies observed actor, state, host/session,
typed workspace-root and terminal-working-directory evidence, and action-log
diff counts. Missing commands, checks, and terminal file-change evidence are
written as missing; zero tracked lines are not presented as a clean worktree.
The Session Rail can copy the observed terminal working directory or stable
Host/Session reference without claiming either one is a global project root.

The first structured Codex adapter now accepts official lifecycle-hook JSON
through an authenticated `dez-terminal-host agent-event` client. Stable Codex
session identity, lifecycle, attention, resumability, and bounded command/exit
evidence live with the terminal Host/Session snapshot. The Session Rail uses
that state ahead of process-name detection, and a review brief consumes its
observed command events. Known validation commands count as checks only with an
observed exit status; other commands and missing outcomes remain unclassified.
The same bounded event journal supplies the review brief's recent activity and
the terminal row's last-meaningful-event recency. Hook installation remains
explicit and documented in [Codex Terminal Adapter](./codex-adapter.md).

## Revised-plan gap audit {#revised-plan-gap-audit}

The revised consolidated Product and Execution Plan adds useful target
requirements that are not yet current-state claims:

- `AppSession` owns restore lifecycle, ordered Workspace identity membership,
  explicit ordered viewport records, active selection, and unresolved records,
  but not live viewport composition, shared stores, Host, Run, or Change Set
  registries.
- Workspace owns a typed `EvidenceSet` for visible roots, open files, explicit
  user-selected paths, and terminal working directories, with deterministic
  identity, provenance, confidence, Host, lifecycle, persistence, and
  truncation truth. Root refresh preserves Session-provenanced cwd records;
  restore and live Host transitions reconcile Current, Unresolved, and Stale
  lifecycle without deleting review history. Broader tool-local selection and
  compiled restart proof remain open.
- `Project::local` still constructs substantial per-Workspace store graphs.
  No shared-store/scoped-view refactor has proved isolation or lazy demand.
- Change Set and Environment are vocabulary only. Git remains authoritative;
  no duplicate persistence registry should be added merely to match a
  conceptual type.
- Protocol version 3 negotiates protocol, Host identity, and additive Host
  capabilities, then carries provider-neutral adapter evidence capabilities in
  structured snapshots. Missing capability fields default to false, so mixed
  versions fail closed per feature. A negotiated heartbeat command echoes a
  caller nonce with Host observation time without touching Session state. A
  bounded event envelope assigns monotonic Host cursors. Clients establish an
  authoritative list baseline, then use a separately authenticated subscription
  socket for pushed batches. Reconnect resumes newer events and full-resyncs
  when a cursor predates retention; older peers keep bounded polling. A
  cross-platform named-pipe transport remains open.
- Adapter capabilities separately declare file targets, permission responses,
  and input responses. Codex hooks v1 provide bounded file targets for
  recognized write/edit/patch events but no consequential response contract.
  File targets describe intended scope, not observed mutation success; approval
  and input controls remain absent until an adapter supplies scoped audit
  evidence.
- Session transport truth and terminal-agent work truth are separate enums in
  source. Run review derives a presentation from them. This is intentional and
  should not be replaced by the consolidated plan's mixed Run-state list.
  Missing and protocol-incompatible transport states remain distinct through
  the Session Rail and Review Brief rather than degrading to Saved history.
  The compact rail composes work and non-live transport labels; Review Brief
  gives exceptional transport ownership priority and records the associated
  evidence risk, so a cached Running snapshot cannot conceal a missing Host
  session.
- Terminal attention now stores an active or resolved condition separately
  from unread or acknowledged presentation, priority, a mute deadline,
  resolution time, update time, and optional stale expiry. The legacy bit is a
  migration input and compatibility projection, not the new source of truth.
  Opening an owner acknowledges its presentation without claiming resolution;
  the Session Rail offers explicit acknowledge, one-hour snooze, resume, and
  resolve actions. Structured permission and failure states derive urgent
  priority from their owning adapter instead of being copied into SQLite.

Workspace Evidence also projects files currently represented by pane surfaces.
The set is sorted, deduplicated, capped at 256 records, and recomputed on pane
item add, removal, or path/title change. Each file carries stable
Workspace-and-path identity, open-surface provenance, authoritative confidence,
Host identity, and current lifecycle. Truncation survives unrelated root
refreshes and is disclosed by Review Briefs; this is open-surface evidence, not
yet a changed-file or Git-status claim.

These gaps refine the next slices; they do not invalidate the existing helper
or justify broad Host, Run, Change Set, Environment, or browser frameworks
before the restart-and-review vertical gate.

## Observed restore path

```text
session::AppSession previous launch ID and window stack
-> WorkspaceDb::last_session_workspace_locations
-> read_serialized_multi_workspaces
-> zed::restore_or_create_workspace
-> MultiWorkspace and Workspace entities
-> pane and item deserializers
```

The path restores ordering, active workspace selection, window grouping,
scratch workspaces, pane data, and workspace-local UI state. Tests in
`workspace::persistence` cover several of these pieces. A consolidated build
and live restart scenario have not run for the current dirty product slice.

Startup now treats restoration as a barrier rather than an alternative to an
initial CLI or URL request. `AppSession` is the source of truth for the ordered
Pending -> Restoring -> Ready lifecycle and emits state changes for future
viewport consumers. Its compact KVP state preserves Workspace identity order,
explicit ordered viewport composition, active selection per viewport, and
unresolved records without retaining paths or GPUI entities. Legacy
membership/window hints migrate into viewport records. Database-resolved
records reconcile in place; live creation, activation, and viewport-aware
removal update the same state. A Workspace identity can be presented in
multiple viewports without duplicating global membership, and removing one
presentation preserves the others. Dez restores the prior multi-workspace collection,
dispatches the first queued request, and only then marks the app session Ready,
processes later listener traffic, or garbage-collects prior workspace rows.
Invalid or duplicate lifecycle transitions are rejected without panicking.
SQLite still owns Workspace content serialization and `MultiWorkspace` still
composes live entities per window, so completing the live viewport/entity seam
and proving the compact state through restart remain open.

## Terminal truth boundary

Two terminal identities now have different intended meanings:

- `agent_ui::TerminalId` identifies a terminal entry or surface for agent UI
  and saved metadata.
- `terminal::session_host::TerminalSessionId` identifies host-owned
  computation independently of panes, entities, workspaces, and windows.

The second identity now drives an in-process local host. `terminal::session_host`
defines versioned snapshots, bounded replay positions, lifecycle states, and
explicit list, create, attach, detach, input, resize, and terminate commands.
`LocalTerminalHost` is initialized from the persisted installation identity,
retains local terminal entities after a view closes, and distinguishes observed
process exit from view closure.

Terminal items and `TerminalThreadMetadataStore` now persist nullable
Host/Session pairs alongside their existing compatibility metadata. A valid
pair reattaches the same retained entity when the in-process host still owns
it. A stale, foreign, partial, or malformed reference does not create a shell;
it opens a display-only recovery surface explaining that Dez started no
replacement computation. Agent recovery also skips the initialization command
in that state. Rows with both new columns null remain legacy rows and preserve
their previous behavior.

Terminal attention is also stored as an additive SQLite field. Bell-derived
attention is persisted immediately when raised and when acknowledged, so a GUI
restart cannot silently erase it. Structured Codex attention remains owned by
the helper snapshot and is acknowledged when the owning terminal is opened.

The Session Rail projects detached local sessions, uses the same stable session
identity before and after reattachment, and reattaches the existing terminal
entity into a normal pane. Closing a live row detaches it; removing a detached
row terminates it explicitly. The terminal context menu also exposes Terminate
Terminal Process as a separate destructive command. Selecting a saved hosted
row no longer fails invisibly when reconciliation fails: Dez opens a
display-only recovery surface with the reason and an explicit New Terminal
action, while starting no replacement process.

The in-process compatibility path is not cross-process durable. Exiting Dez
drops that host and terminates its PTYs. It remains the default until the
helper path passes the consolidated verification gate.

The additive `dez-terminal-host` binary now establishes the next ownership
boundary in source. It authenticates a client using a private token file,
checks stable host and protocol identity, rejects oversized frames before
payload allocation, and owns raw local PTYs independently of GPUI. Each PTY has
a poll-driven command/output loop; output enters the same byte-bounded,
sequence-addressed replay model used by the in-process contract. A sequential
transport client can handshake, negotiate supported operations, and issue
lifecycle commands.

When `DEZ_EXPERIMENTAL_TERMINAL_HOST=1`, ordinary local shells now use that
helper. The runtime connects before launching the packaged sibling, keeps its
token and socket inside a private state directory, and refuses unsafe
stale-path deletion. `TerminalBuilder::new_hosted` creates the display-side
grid with stable session identity; its controller delegates input, resize,
detach, and terminate without giving GPUI process ownership. The GUI establishes
one authoritative snapshot baseline, then opens a separate authenticated
subscription socket. The helper pushes at most eight retained envelopes per
batch, coalesces wakeups, and sleeps until Host truth changes. Reconnect opens a
new subscription after the last delivered cursor; retention loss triggers a
full list resync. Peers without event-stream capability retain the 250 ms to one
second polling fallback. Attach polling applies only unseen sequence-addressed
replay, and a bounded startup wait
prevents persisted items from racing the helper handshake. Display-derived
title and working-directory changes return to the host, while snapshots carry
the owning PID and publish revision notifications so detached and reconnecting
rows cannot become silently invisible.

The experimental flag is a truth boundary: while it is enabled, local shell
creation waits for the authenticated host and reports failure rather than
silently falling back to a GUI-owned PTY. The Session Rail projects Connecting,
Failed, and Reconnecting host health and allows full reconnect/failure details
to be copied without permanently expanding the rail. After transport loss, later ordered
commands wait for a fresh handshake; the command whose response was lost is
never replayed because its outcome is uncertain. Large input is split into
frame-safe chunks, and the raw replay budget is deliberately below the JSON
frame ceiling.

Task terminals intentionally remain GUI-owned and non-durable in v0.0.1. Their
rerun, completion, cancellation, and task-status contracts do not define what
cross-GUI survival should mean; silently retaining them could leave a build or
test alive after the UI reports cancellation. Users who need long-lived
supervision can choose an ordinary durable shell. The default-off ordinary
shell path still uses `LocalTerminalHost` and dies with the GUI process.
Transport loss becomes a Reconnecting snapshot and never
starts replacement computation. Ordered commands now make a bounded series of
reconnection attempts and return an honest error instead of blocking the whole
queue forever; authentication, host, and protocol failures stop immediately.
The helper also releases exited PTY handles on the next command while retaining
the bounded exited snapshot for review. Helper relaunch policy remains later
hardening work; pushed observations deliberately use a second connection
instead of multiplexing mutating commands.

The UI must therefore keep using honest labels:

- **Live** only when a current terminal entity exposes a process.
- **Saved** for metadata without a current process.
- **Detached** only after a host confirms it still owns the session.
- **Missing** when a recorded session cannot be found.
- **Exited** only when an observed host event supplies the terminal state.

Restoring metadata by spawning a new shell must never be presented as
reattachment.

## Safe implementation seams

1. Keep `TerminalSessionId` separate from pane and `TerminalId` values.
2. Persist one stable local `TerminalHostId` from installation-scoped material.
3. Store host/session references in terminal item and agent-terminal metadata,
   with a reversible schema migration and legacy fallback.
4. Define authenticated local IPC, protocol negotiation, ownership locking,
   crash recovery, and a bounded output stream.
5. Extract PTY ownership into a helper process that survives GUI exit while
   preserving the current `TerminalView` input and rendering contract.
6. Reconcile helper sessions at startup before restoring terminal surfaces.
7. Adapt remote/headless terminals to the same generic contract without
   treating the local helper as their transport.

This ordering keeps each step testable and avoids combining a new protocol,
process supervisor, storage migration, and terminal renderer rewrite.

## Known architectural risks

- `Project::local` creates its own worktree, buffer, Git, LSP, task, debugger,
  and environment graph. Sharing those stores prematurely can leak scope
  between workspaces or start expensive work eagerly.
- Remote reconnect currently preserves a headless project connection, not an
  arbitrary local terminal process. Treat it as reusable substrate, not proof
  that Host semantics already exist.
- `WorkspaceStore` removes weak entities on window release. Moving durable
  ownership into it without separating viewport lifetime would preserve the
  current problem under a new name.
- Existing terminal metadata tables are valuable compatibility data. Extend
  them transactionally; do not reinterpret their UUID as a live session ID.
- Output replay must be bounded by bytes and sequence range, redacted before
  persistence, and excluded from normal workspace snapshots.

## Next acceptance slice

The next acceptance slice is complete when the opt-in source path is compiled
and exercised through create, input, resize, close/detach, GUI exit, relaunch,
replay, reattach, explicit terminate, and observed exit. It must verify the
state-specific recovery copy and prove the helper remains alive after GUI exit.
Only after that evidence should the helper become the default local-shell
backend. Legacy rows retain Saved behavior, task terminals remain explicit,
and remote transport is a later adapter.
