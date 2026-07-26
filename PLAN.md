# Dez v0.0.1 Completion Plan

This is the repository-level execution ledger for completing Dez v0.0.1. It
does not replace the permanent [Fork Notes](docs/src/development/dez/fork-notes.md)
or the living [Roadmap](docs/src/development/dez/roadmap.md). If this file
conflicts with either document, Fork Notes win first and the Roadmap wins for
execution state.

The goal is not a cosmetic Zed rename. Dez must launch as a polished native
development environment where terminal-native developers can work directly,
supervise concurrent agents, recover durable computation, and review observed
results without reconstructing context by hand.

## Authority and evidence

Read these before changing product behavior:

1. [Fork Notes](docs/src/development/dez/fork-notes.md) — permanent product and
   architecture decisions.
2. [Roadmap](docs/src/development/dez/roadmap.md) — detailed progress,
   discoveries, decisions, and verification history.
3. [Product Strategy](docs/src/development/dez/product-strategy.md) — customer,
   wedge, product loop, and product-fit hypotheses.
4. [Architecture Baseline](docs/src/development/dez/architecture-baseline.md) —
   actual current ownership and open seams.
5. [Upstream Synchronization](docs/src/development/dez/upstream-sync.md) and the
   [Upstream Feature Ledger](docs/src/development/dez/upstream-ledger.md) — fork
   sustainability and current merge evidence.
6. [Consolidated Plan Reconciliation](docs/src/development/dez/consolidated-plan-reconciliation.md)
   — adopted, adapted, rejected, and deferred parts of the supplied plan.
7. [Dez v0.0.1](docs/src/development/dez-v0.0.1.md) — launch snapshot and known
   limitations.

A checked item requires authoritative source, test, command, rendered UI, or
runtime evidence. Intent, a plausible implementation, or absence of a known
failure is not completion evidence.

## Product definition of done

Dez v0.0.1 is complete only when all of the following are true:

- A user can distinguish Dez from Zed in every public application, executable,
  scheme, updater, storage, help, onboarding, and packaging surface.
- One durable App Session restores ordered Workspaces, active selection, empty
  Workspaces, unresolved records, viewport associations, panes, surfaces, and
  focus before applying launch requests.
- An eligible local terminal process survives GUI exit through the host-owned
  Session path and returns with the same Host/Session identity and bounded
  replay.
- Closing, detaching, reconnecting, terminating, archiving, and deleting are
  distinct, understandable operations with no silent replacement process.
- A Codex Run in an ordinary terminal produces structured state, low-noise
  attention, observable activity, conservative checks, and a deterministic
  review brief linked to its owning surfaces.
- The UI clearly answers: what is running, what needs attention, what changed,
  what was verified, and what is ready for review.
- Direct editing, search, navigation, Git, tasks, tests, debugging, remote
  workflows, and language intelligence remain coherent Zed-quality work.
- Keyboard and pointer flows have parity; empty, loading, disconnected,
  missing, failed, and recovery states are deliberate and accessible.
- A one-work-area layout cannot retain a duplicate blank split, hide a pane
  containing user work, or use a full-surface accent rectangle as its default
  focus treatment.
- Irrelevant Zed product promotion or account-centric chrome is removed or
  demoted without breaking compatible editor capabilities or the upstream
  merge path.
- The intended Dez binary passes the consolidated compile, focused tests,
  identity, security, restart/recovery, visual, accessibility, packaging, and
  release-provenance gates.

## Current evidence baseline

Status below reflects repository evidence through 2026-07-23. The corrected
app, CLI, helper, and signed bundle existed at the recorded build checkpoint;
their hashes remain historical evidence, but the regenerable local artifacts
were removed before the public source push. Build proof still does not imply
rendered or end-to-end interaction proof.

| Area                   | Evidence now                                                                                                                                                                                                                                                                                                 | Completion gap                                                                                                                   |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------- |
| Upstream               | Integrated `upstream/main` `9d0ef37a2571` through two-parent merge `2be63cfea347`; eleven conflicts are resolved and classified; consolidated build provenance is recorded                                                                                                                                   | Complete runtime regression, installed coexistence, and design-partner proof                                                     |
| Identity               | Dez source guards pass; the corrected arm64 app, helper, and `dev.dez.Dez-Dev`/`dez-dev` ad-hoc bundle are audited; the rebuilt raw CLI exposes `--dez <PATH>` and no visible legacy alias; the launched app held no TCP connection or listener during the recorded soak                                     | Official-Zed install coexistence, consolidated rebuild, public signing/notarization, updater, remote, and visual proof           |
| App Session            | Restore barrier, lifecycle state, ordered Workspace registry, explicit ordered viewport records, active selection, unresolved identity retention, live background-viewport attachment, durable final-project fallback, and distinct restore-failure truth exist in source; all 12 focused Session tests pass | Shared live entity composition and consolidated runtime proof                                                                    |
| Workspace and Surfaces | Pane/Canvas repair, panel-to-pane work, startup request ordering, authoritative bounded EvidenceSet, explicit selection persistence, terminal lifecycle reconciliation, durable terminal ownership, same-path isolation tests, and Workspace-owned Session Rail branch projection exist                      | Complete tool-by-tool scope audit, movement proof, and shared-store isolation                                                    |
| Local Host             | Protocol 4 app/helper builds and focused tests pass; an authenticated packaged-runtime Session retained one shell PID, 88 replay chunks, both pre/post-resize dimensions, and explicit Detached state                                                                                                        | GUI-exit/same-Session reattach proof and default-backend decision                                                                |
| Terminal recovery      | Host/Session references, attach/detach/terminate, recovery surfaces, honest transport states, and dimension-aware replay exist in source and packaged runtime                                                                                                                                                | Full GUI restart scenario, stale-host cleanup, and rendered UX verification                                                      |
| Agent adapter          | Structured Codex hook path, observation-only capabilities, bounded file targets, objective/context projection, and onboarding exist                                                                                                                                                                          | Live hook proof and a second adapter after the PMF gate                                                                          |
| Attention              | Session Rail projection, restart-safe attention, acknowledgement, mute, resolution, priority, and stale handling exist                                                                                                                                                                                       | Consolidated runtime and accessibility proof                                                                                     |
| Review                 | Native and terminal review briefs consume observed commands/checks, Git/worktree state, bounded file targets, cwd provenance, risks, and missing-evidence labels                                                                                                                                             | Compiled proof, live navigation, and side-by-side hero-flow validation                                                           |
| UI/UX                  | The rebuilt bundle includes the rail, blank-center, footer, and utility-row corrections; newer source makes empty Dez windows terminal-first, replaces ambiguous zero-session/caught-up and `+ New` copy, hides inert zero-session filters/search, and gives compact chrome 280 px of usable width           | Rebuild the newest source, capture it, then complete shell hierarchy, outward polish, onboarding, accessibility, and state audit |
| Release                | Static gates, focused tests, the corrected protocol-4 app/helper build at `679cdc28445c`, exact signed-bundle launch, authenticated runtime Session exercise, and deep-strict ad-hoc bundle audit pass                                                                                                       | Full GUI Session restart, visual/a11y, app-facing lint, public signing/install, coexistence, and partner proof                   |

The intended raw executable was used for the first consolidated runtime gate.
No local executable is retained after the requested pre-push cache cleanup; the
next runtime gate must rebuild and identify a fresh exact candidate. The
excluded untracked `dist/Superzed.app` was removed without being opened. The first unlocked
desktop capture exposed a client-decoration bug that stretched the Session Rail
over the entire window. Commit `36d8024280` fixes the geometry; subsequent
commits preserve durable Workspaces, retain terminal dimensions across Host
replay, project ordinary live shells into Session Rail, and repair the footer.
The later screenshot supplied on this date exposed a second width-contract bug:
compact mode reserved 240 px but the root rail still painted at its stored 300
px width, so 60 px of header, rows, and footer were clipped. Commit
`79f69b273c` resolves the width once and uses it for both layout reservation and
painting, with compact, detailed, and icon regression assertions. The exact
same screenshot also showed a blank center despite a loaded worktree. Commit
`4829f6b052` makes any empty tabbed pane with a loaded project render the
Workspace ready launch surface even if a legacy/restored pane predates the welcome
flag; the flag still controls the no-project welcome surface. Commit
`0d8496969f` bounds project, worktree, and branch controls inside the
fixed-height footer so their existing truncation can work, while `abc4f8bedb`
removes the redundant Command Search row. The exact arm64 bundle now contains
all four corrections and passes deep-strict signing verification. macOS is
locked, so a fresh corrected-artifact capture remains open and no broader
visual claim is inferred from source, build, or protocol evidence.

## Execution plan

### 0. Upstream and identity gate

- [x] Establish the canonical documentation hierarchy and reconcile supplied
      plans without resetting real progress.
- [x] Record the current upstream base, stable tag, drift, feature inventory,
      and conflict rehearsal.
- [x] Add local and CI identity guards for executable, bundle, URL scheme,
      updater, and packaged terminal helper.
- [x] Select the next upstream base and merge it through a reversible,
      reviewable branch.
- [x] Resolve presentation and settings conflicts according to Fork Notes;
      retain compatible upstream functionality instead of reimplementing it.
- [x] Audit remote/headless identifiers, logs, crash metadata, help links,
      collaboration copy, telemetry labels, and first-party UI for stale Zed or
      Superzed identity.
- [ ] Prove Dez and official Zed coexist without overwriting binaries, data,
      schemes, channels, or updates. Source now isolates bundle IDs, schemes,
      CLI installation, updater/cloud gates, Linux listener sockets, Windows
      instance IDs, and macOS single-instance ports; the Dez CLI can no longer
      silently autodetect an official Zed executable. Installed side-by-side
      proof remains open because no official Zed app or CLI is present here.
- [x] Record source commit, upstream base, toolchain, dependency lock,
      packaging inputs, and release provenance.

Acceptance: the upstream ledger is current, identity checks pass, coexistence
is demonstrated, and the intended Dez artifact cannot install or update as
official Zed.

### 1. Durable App Session and Workspace recovery

- [x] Restore the prior Workspace collection before launch-time CLI, URL, or
      extension requests.
- [x] Move Pending → Restoring → Ready lifecycle ownership into `AppSession`.
- [x] Register durable Workspace membership by stable identity without keeping
      window-bound GPUI entities alive.
- [x] Persist and restore Workspace identity order in App Session ownership
      rather than losing it in an ID-sorted collection. Consolidated runtime
      proof remains open.
- [x] Persist active Workspace selection per viewport independently of which OS
      window is frontmost. Consolidated runtime proof remains open.
- [x] Preserve empty Workspace membership and explicitly unresolved prior
      Workspace IDs until user removal. Removing the final project or closing
      the last project-backed Workspace now allocates a database identity for
      the empty fallback and makes it active in the same durable viewport
      instead of constructing disposable UI. Their recovery UI remains open.
- [x] Define explicit ordered viewport records so a Workspace can be presented
      in more than one OS window without duplicating global App Session
      membership. Live entity composition and rendered proof remain open.
- [ ] Make every OS window a view over the same App Session rather than an
      independent state universe. Registering a durable Workspace in a
      MultiWorkspace now records that viewport even before activation; shared
      live entity composition remains open.
- [x] Make New Window create another viewport without silently creating a
      separate application universe. Dez no longer inserts an unsolicited
      blank editor over the terminal-first launch surface. Database-backed new
      windows register during Workspace construction and MultiWorkspace root
      registration. The existing headless New Window regression now uses the
      real shared AppState and asserts distinct viewport/Workspace IDs, one App
      Session membership set, and independent active selection. Runtime proof
      remains open.
- [x] Prove durable viewport normalization cannot duplicate, reorder, or
      garbage-collect Workspace membership accidentally. Ten focused Session
      tests cover ordered updates, duplicate viewport replacement, duplicate
      Workspace IDs, invalid active selection, unresolved composition,
      multi-viewport membership, idempotent live attachment, one-copy removal,
      migration, and round trips.
      Live entity composition and consolidated runtime proof remain open.
- [ ] Add focused persistence and startup-order tests for empty, unresolved,
      reordered, multi-viewport, queued-open, and failed-restore cases. The
      first four persistence cases now have focused coverage, and the existing
      last-project removal regression now asserts a database ID, global
      membership, and active viewport ownership. The queued-open path now uses
      one explicit completion barrier and has an authored regression requiring
      pre-barrier retention and post-barrier arrival order, including the
      failure-recovery completion path. Its cold Dez target check reached the
      storage floor before completion, so that regression is not yet claimed;
      failed restoration now marks the affected identity `RestoreFailed`
      without removing its ordered membership or viewport placement. This is
      distinct from a prior identity simply skipped by the active restore
      policy. All 12 focused Session tests pass. A persistent Session Rail
      callout exposes Open Recent and Dismiss-reference actions; its compiled
      and rendered proof remains open. The failure toast also stays visible and
      exposes a direct Open Dez log action instead of dead-end copy.

Acceptance: a mixed set of populated, empty, and unresolved Workspaces returns
in the same order and selection; later launch requests apply only after
restoration; opening or closing a viewport does not change durable ownership.

### 2. Workspace Evidence and universal Surfaces

- [x] Keep one upstream-compatible `Entity<Project>` per Workspace.
- [x] Label Workspace roots and terminal working directories as different
      evidence kinds in review projections.
- [x] Introduce the minimal authoritative Workspace `EvidenceSet` with stable
      identity, provenance, confidence, Host, lifecycle, and truncation for
      visible worktree roots, open pane files, and terminal working
      directories. Explicit user-selected file evidence now lives in the same
      owner with distinct provenance and a bounded 128-path cap.
- [x] Recompute evidence on file open/move/close, terminal cwd change,
      Session attach/reconnect, Workspace restore, and explicit user choice.
      Visible root and remote-Host evidence recomputes on worktree/remote
      changes; live terminal cwd changes update stable session-provenanced
      records, and a newly opened idle terminal seeds its initial cwd before
      the first PTY event. Open pane files recompute on
      add/remove/title-path changes with stable IDs, deduplication, a
      256-record cap, and truncation. Explicit add/remove/clear actions now
      preserve selected paths after their tabs close, and Review Briefs prefer
      the explicit selected-path label over a duplicate passive open-file row.
      The normal Workspace database now serializes only those explicit paths
      and rehydrates them with current Host classification on restore.
      Saved hosted terminals now restore last-known cwd evidence under the
      original Session as Unresolved when attach fails; a later successful
      attach replaces it with Current truth. Live hosted TerminalViews observe
      authoritative Host snapshot revisions: Attached/Starting/Detached map to
      Current, Reconnecting/Missing/Incompatible to Unresolved, and Exited to
      Stale while snapshot cwd changes update the same Session record.
      Consolidated compiled restart/runtime proof remains.
- [x] Ensure generic tool, settings, search, Git, and conversation surfaces do
      not attach roots merely by existing. EvidenceSet mutation is now
      crate-private; downstream tools consume immutable records while only
      Workspace-owned worktree, pane, terminal, and explicit-choice routes can
      mutate authoritative evidence.
- [ ] Scope file tree, search, Git, diagnostics, tasks, debugger, terminals,
      environment, and settings to Workspace evidence and explicit tool-local
      selection. Pending Workspaces now receive distinct stable evidence
      namespaces even when they show the same path. Detached terminal snapshots
      carry an additive durable Workspace ID; TerminalView associates both the
      in-process and helper Host Session, and Session Rail resolves that owner
      before conservative cwd fallback. The broader tool-by-tool scope audit
      remains. Focused live same-path terminal evidence and two-Workspace
      selected-evidence persistence tests now prove one Workspace's mutation or
      clear cannot change the other. Session Rail branch metadata now reads the
      owning open Workspace's repository snapshot; closed historical rows use
      only branch values that agree across every open Workspace and omit an
      ambiguous branch instead of guessing. A Settings window with an
      originating viewport now discovers and mutates project settings only for
      Workspaces in that MultiWorkspace; another OS window cannot silently
      enter its project-settings scope.
- [x] Move eligible panel-only tools into ordinary pane Surfaces while keeping
      familiar toggles and dock layouts where they support the product model.
      With Dez's default legacy docks hidden, Files, Git, Outline, and Debug
      route into Workspace tools while Agent uses its dedicated pane. Terminals
      have one public placement model: New Terminal creates a center Surface
      that can participate in the normal tab and split grid. The inherited
      Terminal Panel remains official-Zed compatibility code but is absent from
      Dez menus, commands, keybindings, and Settings. Focused routing and
      product-visibility assertions freeze the surviving developer-tool map.
- [ ] Prove Surfaces can move across panes and Workspaces without global root,
      repository, Host, or tool-state leakage.
- [ ] Keep discovery lazy; opening a path must not imply recursive indexing,
      LSP startup, diagnostics, or checkers until demanded.
- [ ] Measure and prevent duplicate expensive backend work before extracting
      shared stores from `Project`.

Acceptance: two Workspaces can show related repository data with independent
scope and layout; moving a terminal updates evidence without changing another
Workspace or eagerly scanning a broad directory.

### 3. Local Host and persistent terminal Sessions

- [x] Define stable Host and Session identities separate from panes, terminal
      metadata, windows, and GPUI entities.
- [x] Implement versioned authenticated create, list, attach, detach, input,
      resize, metadata, bounded replay, terminate, and snapshot commands.
- [x] Move opt-in local PTY ownership to `dez-terminal-host` and retain the
      existing terminal renderer as the client-side surface.
- [x] Reject unsafe token/socket paths, oversized frames, identity mismatches,
      uncertain command replay, and silent disposable fallback.
- [x] Provide explicit display-only recovery surfaces for missing,
      incompatible, malformed, and unavailable saved Sessions.
- [x] Compile and run focused protocol, framing, permissions, lifecycle,
      replay, slow-client, reconnection, process-reaping, and failure tests.
- [x] Negotiate additive Host capabilities during the authenticated handshake
      and carry provider-neutral adapter evidence capabilities in structured
      snapshots; missing fields fail closed for older peers.
- [x] Add a negotiated authenticated heartbeat with nonce correlation and host
      observation time so liveness probes cannot be confused with delayed
      responses or mutate Session state.
- [x] Add a bounded cursor-addressed Host event envelope and reconnect resume.
      The GUI establishes one authoritative list baseline, applies only newer
      snapshot events, and falls back to a full resync when retention truncates
      its cursor.
- [x] Replace GUI event polling with a separately authenticated,
      capability-negotiated server-pushed snapshot stream. Cursor resume,
      bounded batches, coalesced notifications, disconnect recovery, and full
      resync after retention loss remain explicit; older helpers use the
      bounded polling fallback.
- [ ] Prove the helper remains alive after GUI exit and reattaches the same
      computation without spawning a replacement shell. Helper PID, Host ID,
      socket, and single-instance reuse are proven. A protocol-4 hosted PTY now
      proves stable shell PID, dimension-aware bounded replay, resize, and
      detach; GUI-driven restart and reattachment remain blocked by the locked
      desktop.
- [ ] Verify detach, close, reconnect, terminate, observed exit, missing,
      incompatible, and stale copy/actions in rendered UI.
- [x] Keep task terminals GUI-owned and non-durable in v0.0.1. Their rerun,
      completion, cancellation, and task-status contracts make automatic
      cross-GUI survival ambiguous; users can choose an ordinary durable shell
      for long-lived supervision instead.
- [ ] Promote the helper to the default ordinary local-shell backend only after
      the recovery gate passes.
- [ ] Ensure helper relaunch, version upgrades, cleanup, crash behavior, and
      explicit termination cannot orphan or kill unrelated processes.

Acceptance: the intended Dez client creates a shell, interacts, resizes,
detaches, exits, restarts, replays bounded output, reattaches the same Session,
terminates explicitly, and reports observed exit truthfully.

### 4. Run, agent, attention, and review loop

- [x] Detect Codex through structured lifecycle hooks when configured and mark
      process-only discovery as lower-confidence Detected state.
- [x] Keep provider Session, lifecycle, resumability, permission, command,
      exit, completion, and bounded activity evidence on the owning terminal
      Session snapshot.
- [x] Project native and terminal agents into Session Rail rows that focus the
      existing owner rather than opening a duplicate conversation or terminal.
- [x] Persist terminal attention and generate deterministic review briefs from
      existing authoritative state.
- [x] Classify checks only when a known validation command has an observed exit
      status; never infer a clean worktree or passing check.
- [x] Add a non-owning Run projection for objective, actor, Workspace, Host,
      Session, evidence, attention, repository state, review state, and outcome
      without duplicating source stores. Review Briefs recompute these
      relationships from the owning thread/session, Workspace evidence, Git
      store, attention condition, and reviewer-owned notes.
- [x] Separate active attention condition, unread/acknowledged presentation,
      mute/snooze, resolution, priority, and stale expiry in the terminal
      attention source model and Session Rail projection. Legacy bits migrate
      as active unread conditions; opening acknowledges without resolving;
      observed bell conditions expire after seven days; permission/failure
      states derive urgent priority from the structured adapter; row actions
      acknowledge, snooze, resume, or resolve explicitly. Consolidated build
      and rendered interaction proof remain part of the final gate.
- [x] Gate permission/input actions behind separate false-by-default adapter
      capabilities and require scope, duration, actor, and audit evidence before
      any consequential approval. Codex hooks v1 are observation-only, so Dez
      deliberately presents no synthetic approve/respond button and directs the
      user to the owning terminal.
- [x] Add file and Git provenance, changed-file links, observed diff state,
      failures, risks, unresolved items, and evidence truncation to Run Brief.
      Open-file provenance and truncation now project from Workspace Evidence;
      native agent action logs now project sorted, deduplicated changed-file
      paths as direct local links alongside observed diff totals. Git-store
      worktree, main-worktree, branch, status, conflict, untracked, bounded
      changed-path, and truncation evidence now projects explicitly without
      attributing whole-repository changes to one Run. Recognized Codex
      write/edit/patch hooks retain bounded direct file targets, explicitly
      labeled as intended scope rather than proof that a mutation succeeded.
- [x] Make review open beside the owning terminal or agent and provide direct
      navigation to diff, file, command, check, and activity evidence. Session
      Rail pointer/context review actions now activate the existing owner and
      open the deterministic brief in a right-hand pane; command-palette review
      now does the same. Briefs include section jumps and local file links;
      changed-file links now come from native action logs, and threads with
      observed changes expose the same direct Review Changes action on hover and
      in the context menu. Structured terminal activity, commands, and checks
      retain their observed working directory as a direct local source link.
      Terminal review now uses one Workspace-owned Git action that reveals the
      bounded Git Changes drawer, selects a changed file, and opens the
      uncommitted diff in the Main Work Area; repeating it cannot accidentally
      close the drawer. The destination now identifies itself as **Diff ·
      filename**, while its tooltip retains the diff base and relative path.
- [x] Add explicit review outcomes without inventing a second Run owner. Every
      editable Review Brief contains Continue, Request changes, and Accept as
      reviewed checkboxes and states that they are reviewer notes, not lifecycle
      mutations. Existing owner-backed archive, remove, detach, and confirmed
      terminate actions remain separate; unsupported discard is not implied.
- [ ] Restore agent state, attention, bounded activity, and review projection
      after GUI restart with no false running or completed state. Persisted
      attention already retains condition/presentation/expiry separately from
      live adapter state; commit `7893762cd5` now renders that condition in the
      textual Saved, Detached, Reconnecting, Missing, Incompatible, and Exited
      state instead of relying on a warning color/icon. Commit `bd36afd3f4`
      carries the Host's bounded-activity truncation flag through
      detach/list/reattach, marks retained rail evidence **partial**, and adds
      the eviction risk to Review Briefs. Full helper-process/GUI restart proof
      remains in the consolidated gate.
- [ ] Complete the Codex hero flow live before adding a second terminal-agent
      adapter; then prove the common contract with one additional adapter.

Acceptance: start Codex in an ordinary terminal, observe structured progress,
receive one actionable attention event, respond in the owning surface, open an
honest review brief beside it, restart Dez, and recover the same state and
evidence.

### 5. Outward UI/UX polish

The UI must feel intentionally designed as Dez, not like unrelated Zed panels
with new labels.

#### Shell and hierarchy

- [x] Establish one stable shell grammar for App Session, Workspace, Surface,
      Session Rail, command center, status, and transient overlays. Commit
      `ff91b34a81` removes the rail's remaining user-facing Project/Workspace
      split across remote, options, focus, new-window, recent, and rules
      controls while preserving internal upstream types. Commit `6f1562847e`
      extends the grammar into the center shell: the multi-tool region is
      **Workspace tools**, its file tree is **Files**, and official Zed retains
      its Project Panel copy through explicit compatibility branches. Commit
      `f6aea3e013` extends that contract through the title bar, Recent
      Workspaces pickers, remote Workspace indicators/deletion, and the Agent
      History picker while preserving official Zed copy. Commit `b749a25619`
      finishes the same pass through recent-work search/actions/errors, Files
      multi-root controls, and Restricted Mode trust copy. Command-center and
      transient-overlay vocabulary continue through `0e2c0dcae3`, which
      carries the contract into developer-tool empty/error states, pane search,
      review evidence, agent rules/checkpoints, skills, scoped Settings, status
      controls, and the complete Files settings section. Commits `e969abda4a`
      and `0607771783` remove misleading inherited command-center actions and
      finish Workspace terminology in retained remote/shared recovery overlays.
      Internal compatibility identifiers and explicit upstream Help references
      remain deliberate. Commit `2092acd453` then replaces the stale screenshot's
      ambiguous **Sessions** heading and repeated absence copy with **Session
      Rail**, a counted **workspace ready** summary, and a single scoped empty-
      Workspace action. Rendered verification is tracked separately.
- [ ] Give every primary region a visible purpose, stable placement, clear
      focus treatment, and predictable resize/collapse behavior. The app View
      menu, title-bar/sidebar chrome, and collapsed status control now call the
      supervision region Session Rail consistently. The rail now uses the same
      mode-resolved width for workspace reservation and root painting instead
      of painting compact mode 60 px wider than its allocation; a rebuilt
      rendered resize/collapse audit remains. Commit `1ebb7c79d4` raises the
      compact cap from 240 px to 280 px and the resize floor from 200 px to
      240 px so the visible labels and actions are no longer designed into a
      crushed column. The focused `sidebar` source check passes. Commit
      `56f7c46db6` keeps the persistent terminal header useful without adding
      another status bar: its tooltip now always identifies lifecycle,
      ownership, folder, and available process or Session details, including
      durable terminals that have no local PID. Project Diff tabs now identify
      the active Dez review Surface and file as **Diff · filename**, with base
      and relative path retained in hover detail; official Zed remains
      unchanged.
- [ ] Keep fixed shell chrome bounded under real project names and narrow
      widths. Commit `0d8496969f` gives the project identity and Git controls
      explicit shrinkable, overflow-hidden regions so their one-line labels do
      not collide with footer controls. Formatting and diff checks pass;
      compiled and rendered narrow-width proof remain in the consolidated gate.
      Commit `a9b1a961c0` then removes that duplicate project/branch row from
      Dez's footer entirely because the Session Rail group hierarchy already
      owns it; essential Restricted Mode and embedded application-menu content
      can still reopen the row. The terminal context strip now clips its
      shrinkable lifecycle/repository metadata before it can paint through the
      fixed Files, Review Changes, and Session Details action group; full
      values remain available in Session Details. The action group now
      discloses labels by priority instead of turning three long labels on at
      one breakpoint: below 480 px every action is a named icon; ordinary split
      widths label Review when changes exist, otherwise Files/Open Workspace;
      Files joins Review at 720 px, and the long Session Details label appears
      only at 920 px.
- [ ] Avoid stacked utility chrome that steals space from supervised work.
      Commit `abc4f8bedb` removes Dez's dedicated Command Search footer row,
      keeps the action as a labeled icon in the existing utility bar, hides the
      unowned upstream update surface, and renders the Canvas prefix row only
      while prefix mode is active. Formatting and diff checks pass; compiled
      and rendered proof remain open. Commit `798df9ec04` removes the remaining
      duplicate Command Palette icon from the Session Rail footer and names the
      rail-specific utility group for assistive technology.
- [x] Make Session Rail orientation explicit in source with a named header,
      visible counted All and Attention scopes, truthful action-needed state,
      search, and a clear creation path. Singular/plural and accessibility
      labels derive from the same counts. Rendered verification remains in the
      consolidated gate.
- [ ] Show Workspace, Host, repository/worktree/branch, actor, Session state,
      work state, attention, changes, checks, and recency with consistent
      hierarchy rather than one dense metadata sentence. Terminal rows now
      separate actor, state, Host, scope, changes, and recency into stable
      clusters; native rows use the same actor/state grammar. Structured
      terminal checks now show passed/running/failed summaries, with observed
      command count as the source-backed fallback. Rendered narrow-width proof
      and native check projection remain.
- [ ] Use color as a secondary signal only; icons, labels, shape, and copy must
      preserve meaning in low contrast and for color-vision deficiencies.
      Commit `7893762cd5` closes the restored-attention gap: active attention is
      included in the state label even when no live structured adapter snapshot
      exists. Commit `56f7c46db6` gives terminal panes a dynamic accessible name
      containing their title and textual lifecycle state, so their meaning does
      not depend on the tab icon color. The app-wide audit remains open.
- [ ] Align density, spacing, radii, borders, typography, hover, focus,
      selection, and animation across panes, Canvas, rails, cards, callouts,
      menus, settings, and recovery surfaces. Commit `798df9ec04` replaces
      oversized centered Session Rail absence states with compact, top-anchored
      states using one icon tile, one explanation, and one full-width primary
      recovery action. It also gives the welcome surface a terminal-first
      hierarchy and ordered Start/Watch/Verify orientation. Commit
      `7f0da8c04a` top-aligns the scrollable welcome surface so short windows do
      not center content into inaccessible overflow. Commit `67001bf0ef`
      establishes the first-party visual baseline: new installs follow the
      system with Lumin Blur/Lumin Light, use JetBrains Mono across interface,
      code, terminal, prompt, and review roles, normalize the working typography
      to 14 px, and restore low-contrast borders, focus, active-line, and
      scrollbar hierarchy inside the translucent theme. The Lumin variants now
      also separate rail/drawer, Main Work Area, tab, terminal, and elevated
      surfaces with restrained neutral layers. Static checks measure region
      separation and a 1.5:1 structural-divider floor after transparency is
      composited; focus retains the stronger 3:1 floor. Source, license,
      first-run, and font assets are guarded. Hover, active, selection,
      scrollbar, and active-line states now follow measured differentiation
      floors across every Lumin variant; the opaque fallback no longer makes
      active weaker than hover or hides the active editor line. Rendered
      density, material, contrast, and narrow-height proof remain open.
      Welcome, Onboarding, Session Rail, and terminal-context controls now use
      one icon grammar for Terminal, Workspace/Files, File creation, review,
      Session details, supervision, History, and Settings.
      Existing profiles with the exact old Dez-generated `.ZedSans` signature
      migrate to the new JetBrains Mono UI role without touching arbitrary
      custom fonts or official Zed settings.
      The built-in file/folder icon theme is named **Dez (Default)** in product
      settings while the original Zed name remains a compatibility alias.

#### Interaction quality

- [ ] Provide keyboard-first switching across Workspaces, Surfaces, sessions,
      actors, Hosts, attention items, and recent targets. While Session Rail is
      focused, Shift-A now toggles its Attention projection and Shift-V opens
      the selected Review Brief on every supported desktop keymap; tooltips
      expose both bindings. Commit `57290c27c3` makes the rail's platform and
      Vim creation bindings terminal-first through a dedicated New Session
      action; the separate New Agent Thread command remains available. The
      selected-session handoff now also exposes Shift+Enter to return,
      Shift+F for Files, and Shift+G for Agent or Git change review across the
      three default desktop keymaps. Files now uses a dedicated idempotent
      reveal/focus action, so repeating that route cannot close the destination.
      When the selected Session owns a closed Workspace, Files reopens that
      exact Workspace, restores the existing Session, and only then reveals the
      project tree instead of failing silently.
      Broader Host/actor switching remains open.
- [ ] Preserve selection and focus intentionally when filtering, switching
      scope, opening review, moving a Surface, or returning from an overlay.
      Session Rail rebuilds now preserve keyboard selection by stable session
      identity across reorder/filter updates and choose the nearest actionable
      row if the selected session disappears; the cross-surface audit remains.
- [ ] Give pointer and keyboard users the same actions, descriptions, disabled
      reasons, confirmation semantics, and recovery paths. Commit `8bcd11f4b6`
      gives the empty rail's New File and Open alternatives explicit accessible
      labels plus action-aware shortcut tooltips, and aligns Agent History and
      Command Palette utility names with their actual behavior. Commit
      `633dcc4bec` adds a keyboard-addressable **Close Worktree from Window**
      submenu for the pointer-only hover control and preserves multi-root scope
      in its labels. Commit `e0e8f119e0` makes the repeated **New Terminal** and
      **Workspace Options** controls name their visible Workspace in both
      accessibility labels and tooltips instead of leaking an internal hover
      group identifier; active Workspace controls remain persistently visible.
- [ ] Ensure hover-only actions also exist in context menus or command palette
      and expose accurate accessibility names. Session Rail review, terminal
      lifecycle, attention, hook setup, and evidence-copy actions now have
      pointer/context parity. Files, Review Changes, Session Details, and Return
      are now named Session Rail actions with keyboard routes; the app-wide
      audit remains open.
- [ ] Keep destructive actions visually and spatially separate from focus,
      detach, close, acknowledge, archive, and ordinary navigation. Session
      Rail now renders detached/reconnecting termination as a red Stop action
      with an explicit critical confirmation across hover, context-menu, and
      keyboard paths; live detach and exited/saved cleanup remain non-destructive
      one-step actions. A rejected Host termination remains visible and raises an
      operational toast instead of failing only in logs. Commit `633dcc4bec`
      also renames the broader group action from ambiguous **Remove** to
      **Remove Workspace from Window**, separated from per-worktree closure.
      Commit `56f7c46db6` applies the same distinction inside the terminal:
      closing a hosted surface is labeled **Detach Terminal**, while
      **Terminate Terminal Session…** remains a separate destructive action.
      Commit `dd2459eef9` derives that label from the terminal's backing type
      rather than transient Host registration, preserving detach semantics
      during reconnection. Commit `7664c6e59b` closes the remaining destructive
      path: the selected terminal's controller is now authoritative even when a
      different global Host exists, exited and unavailable terminals do not
      advertise termination, the command is separated from close/detach and
      marked with an ellipsis, and a critical prompt explains the exact
      irreversible effect before dispatch. The app-wide audit remains open.
- [ ] Use progressive disclosure: the default view communicates current work;
      details reveal provenance, capabilities, protocol, and diagnostics only
      when requested. Commit `bd36afd3f4` keeps the default evidence summary
      concise with a **partial** qualifier while the Review Brief explains that
      older structured activity was evicted from bounded Host history.

#### States and copy

- [ ] Design and inspect populated, empty, searching, no-result, caught-up,
      loading, connecting, reconnecting, disconnected, missing, incompatible,
      failed, resumable, exited, archived, and partial-evidence states. Session
      Rail now composes structured work state with non-live transport state so
      Running cannot conceal Detached, Reconnecting, Missing, Incompatible, or
      Exited; Review Briefs prioritize exceptional transport truth and state the
      resulting evidence risk. Commit `56f7c46db6` adds explicit Active,
      Running, Completed, Failed, Exited, Status unknown, and Unavailable
      terminal-header states with unavailable truth taking priority. Rendered
      inspection remains open.
- [ ] Every error states what happened, what Dez did not do, whether work is
      safe, and the next valid action. Center/panel terminal launch failures now
      state that no process started and direct users through settings to New
      Terminal; Host connecting/reconnecting/failed callouts explicitly state
      fallback, process-safety, wait/restart, and next-launch recovery behavior,
      and reconnect/failure states can copy the full helper detail for support.
- [ ] Every empty state teaches one useful next step without funneling all work
      into projects or agents. The welcome surface is now terminal-first and
      the Session Rail has deliberate search/no-session recovery. Empty project
      groups now start a terminal in that exact Workspace (restoring a closed
      group when needed), and an active search reports matching-session counts
      rather than the misleading caught-up state. An empty tabbed pane with a
      loaded worktree now always renders Workspace ready actions, including for a
      legacy/restored pane whose welcome flag is absent, instead of leaving the
      center blank. Its primary action is now New Terminal, followed by Find
      File and New File. New Window and startup fallbacks preserve this surface
      in Dez instead of covering it with an unsolicited blank editor. The full
      Session Rail zero state now says No sessions yet, exposes New Terminal,
      and suppresses the inert All 0 / Attention 0 scopes. Session search is
      now on demand: the overview exposes one compact, named search control
      when multiple Sessions make filtering useful, the shortcut can reveal
      the same inline field at any count, and closing it returns focus to
      Sessions. One unfiltered Session also omits the redundant All/Attention
      scope row; an active Attention projection retains those controls so the
      user can exit it. A non-empty query remains visible until it can be
      cleared. Commit `4e6292ff0a` goes further: the full **Start
      a session** state owns the only creation action instead of stacking below
      a duplicate Sessions overview, and its New File and Open alternatives
      have distinct icons. Commit `4fc53b860f` also removes the global overview
      shortcut when open Workspace groups have zero sessions, leaving each
      group's correctly scoped New Terminal action as the single path. An
      existing query remains visible so it can always be cleared. The full
      state audit and rendered proof remain open. Commit `2092acd453` removes
      the last **No sessions** repetition from an open Workspace group: the
      overview reports ready Workspace scope, the group says **Ready for a
      session**, and the scoped button has the Workspace name in its accessible
      label. Commit `57290c27c3` then keeps creation behavior stable after the
      first session: the Workspace plus control, worktree picker, and default
      keyboard path all create a terminal, while **New Agent Thread** is an
      explicit secondary Workspace option. Commit `798df9ec04` makes the empty,
      no-result, and caught-up panels compact and top anchored. Commit
      `7f0da8c04a` makes the virtual session list and its empty replacement
      mutually exclusive, preventing a full-size empty list from pushing the
      recovery panel outside the visible rail. Commit `dcd38968d3` then makes
      every plain **New Terminal** launch path on the welcome, Session Rail, and
      empty Workspace surfaces open a center terminal; the legacy terminal
      panel remains an explicitly named secondary menu action.
- [ ] Remove dead buttons, unsupported provider actions, duplicate navigation,
      noisy badges, ambiguous icon-only controls, and success copy unsupported
      by observed evidence. The compiled Zed Pro trial-end overlay/reset action
      is removed and provider-limit recovery no longer exposes its upstream
      subscription CTA. Commit `ad59a60926` also hides five Session Rail
      settings whose branch/worktree/project/onboarding surfaces are removed in
      Dez while preserving their compatibility schema; the app-wide audit
      remains open. Commit `dcd38968d3` also stops Dez from scanning or
      advertising inherited Zed release-channel thread migration and replaces
      the upstream native-agent glyph with a neutral agent mark across the
      reachable Dez agent surfaces while preserving official Zed identity.
      Commit `933e3f515f` removes inherited diagnostics/metrics
      opt-ins from Dez onboarding and Settings because the fork has no
      Dez-owned upload endpoint; Anthropic retention remains visible because it
      controls model-request policy instead. Commit `2680937952` also hides the
      inert Auto Update section in Dez because the fork updater is deliberately
      disabled, while preserving the compatibility key and official Zed UI.
      Commit `9239006d4b` removes Collaboration Panel button/dock/width controls
      from the Dez Panels page after the panel, commands, and Collaboration page
      were removed; official Zed and the compatibility schema remain intact.
      Commit `e969abda4a` also removes inherited collaboration,
      feedback, account, docs, status, and merchandise commands from Dez's
      palette while retaining explicit upstream documentation/repository links
      in Help. Commit `0ddf84161e` closes the keyboard/title-bar equivalents:
      every keymap source filters channel, collaboration-panel, collaboration,
      and follow-collaborator actions in Dez, and inherited call chrome is
      official-Zed-only. Commit `2efbf166b7` corrects the remaining Settings
      workflow copy to teach `dez <path>` and describes private skill imports
      without claiming that Zed performs the retry.
- [x] Replace remaining reachable stale Zed/Superzed product copy while
      preserving
      necessary compatibility, file-format, upstream attribution, and
      developer-facing references. Native draft placeholders, component
      previews, settings explanations, GPU diagnostics, Windows IPC errors,
      CLI help, OAuth browser handoff, extension cards, provider setup, remote
      and debugger errors, system diagnostics, and outbound HTTP/API identities
      now identify Dez. Upstream account/model/edit-prediction copy is labeled
      explicitly, and invalid recovery no longer offers to install Zed over a
      Dez incompatibility. Retained Zed strings identify actual upstream
      services/providers/links, tests, formats, or compatibility identifiers;
      the identity guard checks the public boundary. Commit `f89f55868c`
      renders internal `zed::…` actions as `dez: …` in command-facing UI and
      makes copied Settings links use the active Dez release-channel scheme
      while preserving legacy input compatibility. Commit `526218a972` removes
      the upstream Zed Assistant glyph from Codex rows and the session switcher.
      Commit `e969abda4a` closes a missed unsupported-GPU boundary: the dialog
      now identifies Dez, uses `DEZ_ALLOW_EMULATED_GPU`, attributes the linked
      upstream guide, and replaces placeholder copy. Database-load recovery now
      explains that files are untouched, names the state at risk, and opens
      local logs instead of filing an upstream Zed issue.

#### Onboarding, settings, and accessibility

- [ ] Add a short first-run path for opening work, starting a durable terminal,
      installing the Codex hook deliberately, understanding attention, opening
      review, and learning detach versus terminate. Commit `bb0cf408b4` adds a
      deliberate **Copy Codex Hook** action before New Terminal, copies only the
      bundled setup, and explicitly says Dez does not install or modify hooks.
      Source now teaches the
      terminal → Session Rail → review loop, provides New Terminal, and explains
      close/detach/terminate plus Host-dependent persistence; hook installation
      remains deliberately manual, but eligible detected Codex rows now show a
      visible Hook setup state and one-click copy action with context-menu
      parity. Rendered flow verification remains open.
      Commit `4a102fc50e` makes the terminal-first workflow a named region with
      an ordered Start/Watch/Review list, moves the safety explanation above a
      wrapping action row, and uses 28 px targets for Copy Codex Hook and New
      Terminal so zoom and longer copy do not crush the first-run actions.
      Dez also exposes its isolated Install CLI action instead of hiding it
      behind the official Zed product gate; install interaction remains open.
      Once a terminal exists, the guide remains available inside **Session
      Details** as **How Dez Works**, preserving **Run → Supervise → Review**
      without adding another permanent help row. A Scratch Terminal shows
      **Open Workspace** in its context strip; the selected codebase joins the
      same window through a folder-only picker, so the running computation is
      preserved while Files and Git review become available. The same
      disclosure contains a compact
      **Evidence** contract: Terminal/Host owns lifecycle, Git counts remain
      Workspace-owned, Session attribution is not inferred, agent
      confidence/checks require trusted evidence, and terminal prose is not
      proof.
- [ ] Group Dez settings by Workspace, Sessions, Agents, Attention, Evidence,
      Appearance, Privacy, and Advanced compatibility; hide experimental
      internals from the default path. The settings shell now names Workspace
      & Privacy, Sessions & Terminal, Agents, Attention, Evidence, Appearance,
      and Network & Compatibility; Attention and Evidence expose real
      trust/accessibility controls. Advanced instrumentation is absent from
      ordinary Dez navigation and returns only in staff builds with feature
      flag overrides. Session Rail chrome uses product terminology, and dead
      sign-in/user-menu/avatar settings are no longer visible because Dez
      deliberately suppresses that upstream account chrome; compatibility keys
      remain readable. Current local source orders Workspace, Sessions, Agents,
      Attention, and Evidence before inherited IDE customization, moves Sessions
      placement into Sessions & Terminal, and gives Agent configuration a
      concrete Runtime & Providers entry point. The remaining audit covers
      experimental controls outside the graphical Settings root.
- [ ] Provide safe defaults and explain persistence, output retention,
      redaction, adapter trust, and experimental-host limitations. Evidence
      settings now distinguish local process detection, authenticated
      structured evidence, metadata-only restart restoration, and explicitly
      state that Dez never installs hooks or edits provider configuration.
      Commit `b39eedf724` redacts obvious secret-bearing environment assignments
      before bounded command evidence enters Host retention and corrects the
      settings contract: the metadata database stores identity/attention rather
      than transcripts or structured activity, which returns only from the same
      live Host Session. Commit `9323af8008` extends the shared redactor to
      explicit secret-suffixed CLI flags such as `--token` and `--api-key`
      without masking ordinary arguments. Commit `80ff1df75f` covers URL
      userinfo and secret query parameters, and the Evidence setting now states
      that bounded file targets remain verbatim for review navigation. A future
      sensitive-path policy remains open rather than silently breaking evidence
      links. Commit `933e3f515f` also forces upstream diagnostics and metrics
      false for every non-Zed build even if legacy user settings enable them, so
      Dez cannot post fork usage or crash data to the inherited Zed endpoint.
      Commit `2fc5226a51` applies the same non-Zed gate to eager language-model
      provider authentication, so an inherited `auto_connect: true` cannot turn
      ordinary Dez startup into a cloud-provider discovery pass. Commit
      `cc2509e8b8` closes the matching presentation leak: legacy Zed/Mercury edit
      prediction selections normalize to unavailable in Dez, their status and
      setup controls disappear, and explicit local or user-configured providers
      remain available. Commit `b909b31d45` closes a later command-palette
      override that could re-enable Zed Predict onboarding and prediction
      actions after the product filter ran. Commit `aab0e5f2f2` also prevents a
      non-Zed stable build from installing the inherited crash handler merely
      because an upstream endpoint was compiled in. Dez minidumps now require
      explicit `DEZ_GENERATE_MINIDUMPS=1|true`, and recovery metadata/artifacts
      use fork identity.
- [ ] Audit focus order, accessible roles/names/descriptions, key shortcuts,
      minimum hit targets, screen-reader announcements, zoom, reduced motion,
      contrast, truncation, and localization-resistant layouts. Shared Session
      Rail rows now expose list-item/selection semantics plus actor, state,
      Host, unread, remote/archive, and observed diff information without
      duplicating a richer state label. Newly active authenticated Host
      attention now triggers the configured OS window-attention request once
      per condition transition, including when no terminal surface is attached;
      commit `c47637c2ac` adds a named list container and exposes each non-sticky
      Workspace group as a selected/expanded list item whose accessible label
      includes ready, running, and attention state. Sticky visual duplicates are
      deliberately excluded from the accessibility hierarchy. Commit
      `7e91f00b69` names the Session scope and search regions, gives changing
      totals and empty results status semantics, and keeps scope identity stable
      while the shared toggle state reports selection. Commit `e28b78ed57`
      gives the shared Callout primitive status semantics for informational and
      successful state, and alert semantics for warnings and failures, covering
      durable Host and Workspace-recovery callouts. Commit `a90fae5873` raises
      compact Workspace headers and their primary creation, scope, and recovery
      controls to the shared 28 px medium target, eliminating the prior 24/28 px
      header/control mismatch. Commit `f6318ea907` makes the keyboard-selected
      Workspace or Session row the active accessibility descendant; the shared
      animation layer already collapses repeated activity rotation to one
      static frame when Dez resolves reduced motion. Commit `9930e86677`
      retires the unfinished 56 px icon presentation: legacy `icon`
      configuration remains readable but safely resolves to the smallest
      complete compact rail instead of clipping supervision, search, evidence,
      and recovery controls. Commit `33f7ff5893` adds non-visual shortcut
      metadata to shared text/icon buttons and exposes Shift+A and Shift+V on
      the Session scope and Review Brief controls, matching their tooltips. The
      primary focus roots observed in the installed audit now pair their tracked
      focus handles with stable IDs, Region or Alert roles, and specific labels:
      populated/empty Files, Outline, Git Changes/History, Debug, Agent, native
      and external Agent Sessions, Agent History, and terminal failure/panel
      surfaces. The rebuilt accessibility log and broader rendered matrix remain
      open.
- [ ] Capture a visual state matrix at compact, balanced, and spacious density
      in representative light and dark themes and at narrow/normal/wide rail
      widths.

Acceptance: a new target user completes the activation and hero workflows
without source knowledge; no primary action is hidden behind unexplained
chrome; every state is truthful, accessible, and visually coherent.

### 6. Complete native development experience

- [ ] Verify files, navigation, find, symbols, rename, diagnostics, language
      servers, formatters, tasks, tests, debugger, Git, diff, terminal, remote,
      collaboration, settings, themes, keymaps, extensions, and updates remain
      coherent after Dez shell changes.
- [ ] Remove or demote account, promotion, onboarding, collaboration, assistant,
      and cloud surfaces only when they do not serve the target Dez workflow;
      retain useful upstream capability behind appropriately named entry points.
      Dez no longer renders upstream sign-in/user-plan/connection chrome, the
      View menu omits Collab, and Help no longer routes to upstream bug, feature,
      email, social, or hiring flows. Commits `1d5c03d88b` and `9318b270d9`
      also prevent inherited settings from silently auto-connecting the Zed
      cloud, disable inherited Zed/Mercury edit-prediction providers while
      preserving explicit Copilot/Codestral/custom providers, and stop eagerly
      constructing the Collab panel. Upstream docs/repository links remain
      explicitly attributed. Commit `699cbd1bc8` also restricts upstream
      title-bar onboarding promotions and Return to Onboarding to official Zed,
      renames Help's entry to Getting Started, and aligns the optional Dez
      welcome surface with Workspace and supervision language. Post-build
      runtime proof remains open. Commit `a20074de26` removes the inherited
      Calls/Collaboration settings page and GUI controls for Zed auto-connect
      and collaboration-server configuration while retaining compatibility
      parsing and the live proxy control. Commit `2435348289` also removes the
      dead Zed Edit Predictions data-collection control from Dez and frames
      setup around explicit providers. Commit `f40877d4ab` filters inherited
      collaboration actions from the Dez Command Palette without unregistering
      their compatibility namespace.
- [ ] Ensure opening a file, folder, repository, URL, remote target, recent
      Workspace, or empty Workspace routes into the existing App Session
      without creating an accidental parallel universe.
- [ ] Verify panel-to-pane and Canvas conversions preserve item serialization,
      focus, actions, menus, drag/drop, split, zoom, close, reopen, and recovery.
- [ ] Measure startup, first interaction, memory, CPU, background work, terminal
      throughput, large-repository behavior, and idle cost against the selected
      upstream baseline.

Acceptance: direct development is not degraded to fund agent supervision; Dez
feels complete for daily work and remains maintainable as a Zed fork.

### 7. Consolidated verification and public preview

- [x] Freeze the source slice and record the exact intended Dez executable and
      bundle paths before launching anything.
- [ ] Run formatting, metadata, identity, documentation, lint, focused unit,
      integration, and migration gates at scope appropriate to the changes.
      Formatting, diff, identity, the nine-test Session slice, the rebuilt CLI,
      and a full `zed --bin dez` source check pass at `e4fbc22a3a`; app-facing
      Clippy and the remaining integration/runtime matrix remain open.
- [x] Compile the intended Dez app and helper once at the consolidated gate;
      do not open a second historical SuperZed/Dez binary.
- [ ] Inspect first launch, normal launch, restored launch, offline launch,
      failed-host launch, and incompatible-host launch.
- [ ] Execute the full persistent-terminal restart scenario and capture Host,
      Session, PID, replay cursor, process-liveness, and no-replacement proof.
- [ ] Execute the full Codex attention/review/restart hero scenario and capture
      structured-event, acknowledgement, evidence, review, and restoration
      proof.
- [ ] Run the visual state matrix, keyboard/pointer parity, accessibility,
      crash behavior, security, permissions, privacy, storage migration,
      coexistence, updater, packaging, signing, and install/uninstall audits.
- [ ] Test with target users on real repositories and document blockers,
      observed recovery rate, false states, attention misses/noise, review use,
      startup/memory regressions, and crashes.
- [ ] Resolve every release blocker or document a narrow honest limitation with
      a safe fallback that does not contradict Fork Notes.
- [x] Produce release notes, known limitations, recovery documentation,
      provenance, checksums, and rollback instructions for v0.0.1 in the
      operator-facing Release Runbook.

Acceptance: all definition-of-done requirements have direct evidence, the
activation and hero workflows pass on the intended artifact, and no required
work remains hidden behind “source present,” “tests authored,” or “looks
compatible.”

## Deferred until after the v0.0.1 vertical gate

Do not move these onto the critical path without product evidence or a new Fork
Notes decision:

- autonomous agent teams and organization administration;
- a custom foundation model or bundled-token business;
- hosted sandboxes, relay, mobile editing, or collaborative terminal control;
- Change Set storage that duplicates Git;
- broad Environment orchestration, DevPod, Dagger, or browser automation;
- provider marketplaces or a large adapter matrix;
- replacing GitHub, issue tracking, CI, or deployment platforms;
- unlimited terminal output retention.

## Working and rollback rules

- Make small reversible source slices, but judge each slice by movement toward
  the complete product rather than ease of testing.
- Preserve unrelated user changes in the dirty worktree.
- Extend compatibility schemas additively and keep migrations reversible.
- Run cheap static gates continuously; compile and launch at the explicit
  consolidated gate unless a newly discovered blocker requires earlier proof.
- Never silently fall back from enabled durable behavior to disposable work.
- Never claim runtime, visual, recovery, or packaging success without direct
  evidence from the intended Dez artifact.
- Update this ledger and the Roadmap when evidence changes a checkbox, gap,
  dependency, decision, or acceptance criterion.

## Progress log

- 2026-07-22: Created this completion ledger from the reconciled authoritative
  documents. Confirmed no Dez process was running; no application binary was
  launched. Added the outward Session Rail orientation and attention-scope
  polish slice in source, removed its duplicate footer control, and made scope
  changes preserve the selected session when it remains visible or choose the
  nearest/first actionable row for immediate keyboard navigation.
  Search and no-session dead ends now explain the state and provide Clear
  Search or New Terminal directly.
- 2026-07-22: Replaced App Session's ID-sorted membership map with an ordered,
  compact durable state. It now retains unresolved prior identities, records
  active Workspace selection per viewport, persists changes outside GPUI
  entities, reconciles database-resolved records in place, and follows live
  activation. True viewport composition and runtime recovery proof remain open.
- 2026-07-22: Added ordered durable viewport records alongside global Workspace
  membership. Legacy membership/window maps migrate additively; restored
  MultiWorkspace groups publish their full ordered composition and active
  selection; one Workspace can appear in multiple viewports; removing one copy
  preserves the other and removes global membership only after the last copy.
  Focused migration, deduplication, reconciliation, and removal tests are
  authored. Live entity composition and consolidated runtime proof remain open.
- 2026-07-22: Audited the supplied historical Superzed screenshots. They show
  generic welcome copy, a project-first start path, repeated untitled chrome,
  unexplained blank windows, dead space, and stale Zed identity. Source now
  makes the welcome path terminal-first with the Dez product promise, prevents
  section-header wrapping, uses dynamic Dez labels in settings/install/update/
  permission surfaces, and replaces active Zed plan onboarding with neutral
  provider-controlled configuration. The screenshots predate the current
  source, so rendered verification remains open until the intended Dez build.
- 2026-07-22: Reframed first-run setup around Dez's activation loop rather than
  upstream account conversion. A terminal-first workflow card teaches Start,
  Watch, and Review; exposes New Terminal; and distinguishes close, detach, and
  terminate without promising continuity beyond the connected Host. Optional
  ACP agents remain available without a bundled Zed subscription surface.
- 2026-07-22: Reworked Session Rail row metadata so terminal actor, work state,
  Host, scope, changes, and recency no longer share one punctuation-heavy
  string. Identity/scope metadata truncates as a left cluster while observed
  activity remains right-aligned; accessibility labels include each field.
- 2026-07-22: Extended the same row grammar to pane-native agents: Dez Agent or
  provider identity and Draft/Running/Waiting/Error/Completed state are now
  explicit metadata rather than icon-only inference.
- 2026-07-22: Added the first Workspace-owned `EvidenceSet`. Visible worktree
  roots have deterministic identity plus provenance, confidence, Host,
  lifecycle, and truncation truth; worktree and remote-context events recompute
  it. Run Briefs use the open Workspace's authoritative evidence and fall back
  to saved row metadata only when the Workspace is closed.
- 2026-07-22: Routed the terminal view's existing cwd-change stream into the
  Workspace EvidenceSet. Root refreshes preserve terminal records, each cwd is
  tied to a stable terminal Session, and Run Briefs include only cwd evidence
  belonging to their owning session so another terminal cannot leak scope.
- 2026-07-22: Added terminal evidence lifecycle truth. Terminal activity marks
  session-provenanced cwd evidence Current, observed process exit marks it
  Stale without deleting review history, and Run Briefs disclose stale
  observations as risk.
- 2026-07-22: Added trustworthy Session Rail totals and stable keyboard
  selection. All and Attention scopes now show live counts with grammatical
  status and accessible labels; rebuilds preserve the selected Thread/Terminal
  by identity and fall back to the nearest actionable row when it vanishes.
  The HTTP client now identifies the fork as Dez rather than advertising Zed.
- 2026-07-22: Prevented the no-project start state from hiding valid standalone
  or restored sessions, search results, and the caught-up attention view. Added
  a compact source-backed evidence indicator to terminal rows: recognized
  checks report passed/running/failed outcomes, while command-capable adapters
  fall back to an observed command count. Labels, icons, color, and assistive
  text carry the same fact.
- 2026-07-22: Preserved missing and protocol-incompatible Host lifecycle truth
  through Session Rail state and deterministic Review Briefs instead of
  presenting both as merely Saved. Added cross-platform Session Rail shortcuts
  for Attention (Shift-A) and Review Brief (Shift-V), binding-aware tooltips,
  and Copy Details actions on Host reconnect/failure callouts.
- 2026-07-22: Separated durable-session termination from ordinary close/detach
  semantics. Detached and reconnecting rows use a red Stop affordance and a
  critical prompt that explains computation will end; hover, context-menu, and
  keyboard routes share the same gate. Live detach and exited/saved cleanup do
  not inherit the destructive confirmation. Host rejection now produces a
  visible “not terminated” toast while leaving authoritative state intact.
- 2026-07-22: Made per-Workspace Session Rail empty states actionable. An empty
  group now offers New Terminal scoped to that exact Workspace and restores a
  closed group before creating it. Search mode reports matching-session totals
  with a search icon instead of claiming the user is caught up merely because
  the query hid all sessions; failed Host guidance names the exact opt-in
  environment switch to omit on the next launch.
- 2026-07-22: Made top-level navigation match the product hierarchy. File now
  leads with New Terminal and names New File explicitly; View exposes Session
  Rail directly; title-bar and collapsed status controls say Open/Hide Session
  Rail rather than reverting to upstream “sidebar” terminology.
- 2026-07-22: Removed three inert upstream-account controls from visible Dez
  settings: Show Sign In, Show User Menu, and Show User Picture. Their schema
  keys remain compatible, while the remaining section, placement option, and
  descriptions consistently name Session Rail and its actual chrome.
- 2026-07-22: Refreshed the permanent upstream gate without touching the dirty
  product worktree. `upstream/main` advanced to `9d0ef37a2571`, 81 commits
  beyond the merge base; stable remains `v1.11.3`. A disposable detached
  worktree rehearsal found ten conflicts (the prior nine plus `workspace.rs`),
  was aborted, and was removed. The feature ledger classifies all nine newly
  observed upstream commits; integration resolution remains an explicit gate.
- 2026-07-22: Closed a stale-agent truth gap. Session Rail now composes
  structured agent and transport state (for example Running · Detached) rather
  than allowing a cached work snapshot to hide Missing, Incompatible,
  Reconnecting, Detached, or Exited ownership. Review Briefs prioritize those
  exceptional transport states and add explicit evidence risks; long state
  chips shrink and truncate without losing their full accessible label.
- 2026-07-22: Completed the honest review-decision surface. Generated Review
  Briefs now include an editable, reviewer-owned Continue / Request changes /
  Accept as reviewed checklist and explicitly state that checking it does not
  mutate, stop, or resolve the authoritative Run. Destructive or archival
  lifecycle actions remain in their existing owner-backed controls.
- 2026-07-22: Extended attention accessibility to detached structured agents.
  Session Rail observes the authoritative Host snapshot revision, compares only
  a transient set of active attention session IDs, and requests OS window
  attention once for each newly raised condition when the accessibility setting
  allows it. Acknowledgement removes the ID so a future condition can announce.
- 2026-07-22: Integrated the selected upstream base, repaired post-merge
  settings/sidebar/lifecycle compatibility, removed stale onboarding keymap
  actions that panicked the final binary, and kept `auto_connect = false`
  genuinely local by preventing eager cloud-provider authentication. Fifteen
  focused terminal tests, eight helper tests, and three Session Rail lifecycle
  tests passed before the consolidated build.
- 2026-07-22: Completed the warning-free consolidated arm64 app/helper build at
  `da562e14bb403af815cbab9802225dda0b2418c8`, then built the intended CLI with
  the same locked low-disk profile. The exact raw Dez executable launched
  without the corrected keymap or provider-auth failures; no historical
  Superzed artifact was opened.
- 2026-07-22: Proved the external Host boundary survives the GUI lifecycle.
  Helper PID `48768` stayed alive and reparented to PID 1 after GUI PID `48519`
  exited; GUI PID `50092` reused that exact helper, socket, and Host ID with one
  helper instance. A live hosted PTY was not created, so same-Session replay and
  child-PID reattachment remain open rather than inferred.
- 2026-07-22: Hardened debug packaging in `ce11c4ed3d`. The macOS script now
  reuses a complete host debug artifact set without materializing a duplicate
  target graph, restores its temporary manifest on failure, uses the pinned
  bundler's plain-output fallback, and omits release-only remote-server work.
  The 1.0G ad-hoc bundle passes deep strict signature verification with
  `dev.dez.Dez-Dev`, version `0.0.1`, scheme `dez-dev`, and arm64 app, CLI,
  helper, and Git executables. Privacy prompts now identify developer-tool
  requests clearly. Public signing, notarization, installation, and official
  Zed coexistence remain separate gates.
- 2026-07-22: Retried live UI inspection exclusively through the approved
  macOS accessibility/computer-control path. The desktop remained locked and
  automatic unlock failed, so the visual state matrix, keyboard/pointer audit,
  accessibility tree, and full hosted-terminal recovery scenario remain
  explicitly unverified.
- 2026-07-22: Ran warning-denied Clippy for every terminal Host target. Current
  Clippy found one behavior-neutral `100 / 100` fallback-theme normalization;
  commit `3ad224dfd6` expresses it as `1.0`, and the Host graph then passes.
  App-facing modified-crate Clippy remains open because its much larger graph
  exceeds the remaining storage budget.
- 2026-07-22: Promoted terminal geometry into protocol-4 replay truth. The Host
  records 80x24 and 132x41 dimensions with the corresponding output fragments;
  focused model, hosted-renderer, and helper lifecycle tests pass. The packaged
  helper then created Session `040b4465-5f0a-416b-9cb3-549da1a2a28b`, retained
  88 replay chunks and both resize markers, and reported explicit Detached
  state without replacing shell PID `53394`.
- 2026-07-22: Made ordinary live shells visible in Session Rail and repaired the
  lower workspace footer. Agent detection now classifies a terminal instead of
  deciding whether it exists; the footer is a single truncating row and hides a
  redundant default worktree label.
- 2026-07-22: Rebuilt, packaged, and audited the intended arm64 `Dez Dev.app`.
  Commit `fcd1d06564` signs nested executables inside-out; the app passes deep
  strict verification, uses `dev.dez.Dez-Dev` and `dez-dev`, and is running via
  its exact bundle path. The locked desktop still blocks fresh visual and
  accessibility evidence; `dist/Superzed.app` remains unopened.
- 2026-07-22: Added the operator-facing v0.0.1 Release Runbook. It consolidates
  release notes, exact bundle identity and executable checksums, terminal
  recovery semantics, state-specific safe actions, known limitations,
  verification rules, rollback, and public-preview promotion gates without
  upgrading local ad-hoc evidence into a public-release claim.
- 2026-07-22: Exposed Session Rail as a named `Complementary` accessibility
  landmark and made its overview and empty-Workspace status copy truncate at
  narrow widths rather than re-enter word-level wrapping. The complete
  `sidebar` dependency graph passes focused `cargo check`; the corrected bundle
  now contains this source slice, while rendered accessibility evidence remains
  open.
- 2026-07-22: Traced the supplied crushed-rail screenshot to a second concrete
  layout contract violation. Compact mode reserved 240 px through
  `WorkspaceSidebar::width`, while both decoration branches still painted the
  root at the stored 300 px width. Commit `79f69b273c` makes the renderer use
  the same resolved compact/icon/detailed width and adds assertions for all
  three modes. Formatting and diff checks pass. The focused test build was
  attempted but intentionally stopped after the volume twice exhausted its
  remaining link space while reconstructing deleted dependency caches; this is
  not recorded as a passing focused test. The complete corrected app and bundle
  build now contain the fix; rendered proof remains open.
- 2026-07-22: Removed the screenshot's blank-center failure in source. Commit
  `4829f6b052` gives a loaded project priority over the legacy welcome-page flag
  when selecting empty-pane content, so an empty restored tabbed pane renders
  Workspace ready with Find File, New File, and New Terminal actions. A focused
  model assertion covers loaded versus no-worktree selection. Formatting and
  diff checks pass. The corrected app and bundle build now contain the fix; the
  storage-bound focused test and rendered interaction remain open.
- 2026-07-23: Closed a hidden local-first launch violation exposed by the stale
  bundle log. Despite hidden collaboration/account chrome and a false default,
  inherited user settings still started the upstream Zed websocket, LiveKit
  reconnection, and Zed-hosted edit prediction. Commit `1d5c03d88b` gates
  automatic cloud authentication and the Collab panel to the official Zed
  product and ignores inherited Zed/Mercury prediction providers in Dez while
  retaining explicit non-Zed providers. Commit `9318b270d9` makes those
  boundaries part of `script/dez-identity-check`, which passes. The rebuilt
  bundle held no established or listening TCP socket during the recorded soak.
- 2026-07-23: Repaired the remaining footer layout contract exposed by the
  supplied screenshot. Commit `0d8496969f` places project identity and
  worktree/branch controls in bounded, shrinkable, overflow-hidden regions,
  allowing their existing one-line truncation and tooltips to work inside the
  fixed-height row. Formatting, diff, and complete app/bundle build checks pass;
  rendered narrow-width evidence remains open.
- 2026-07-23: Removed the screenshot's redundant stacked footer utility row.
  Commit `abc4f8bedb` moves Command Search into the existing icon bar with an
  accessible action tooltip, suppresses Dez's otherwise empty upstream
  workspace/update row, and still shows the Canvas prefix indicator on demand.
  Upstream Zed behavior remains gated to the Zed product. Formatting and diff
  checks pass.
- 2026-07-23: Rebuilt the complete arm64 Dez app and protocol-4 terminal Host
  from source head `679cdc28445c`, including all four screenshot-driven shell
  corrections and the local-first boundary. The ad-hoc `Dez Dev.app` passes
  deep-strict verification with CDHash
  `0dc2e1e872b88cbd6288f1bea5455fbc48271cc5`, and PID `85053` resolves to its
  exact bundle executable. A runtime soak observed no established or listening
  TCP sockets. The approved UI controller still reports a locked desktop, so
  corrected visual and accessibility evidence remains open rather than
  inferred. The excluded `dist/Superzed.app` remains unopened.
- 2026-07-23: Closed the source-level durable viewport regression slice in
  `a91b04809c`. `cargo test -p session --lib` passes all nine tests, including
  duplicate viewport replacement without reordering, Workspace deduplication,
  invalid active-selection clearing, unresolved composition, multi-viewport
  membership, one-copy removal, legacy migration, and serialization round
  trips.
- 2026-07-23: Made the empty Dez workspace deliberately terminal-first in
  `e4fbc22a3a`. Workspace ready now leads with New Terminal; New Window and both
  startup fallback paths no longer cover the launch surface with an
  unsolicited blank editor. Official Zed keeps its upstream blank-editor
  behavior. The same slice removes stale public CLI help and hides the legacy
  `--zed` alias while retaining compatibility. The raw CLI rebuild hashes to
  `cc8d62764f0892da5306aeefb9206732e8f25584f9213e84631184d2ae8d9787`;
  formatting, diff, identity, and full `zed --bin dez` source checks pass. The
  running bundle predates this slice, so rebuild and rendered proof remain
  open.
- 2026-07-23: Replaced the screenshot's misleading zero-session shell copy in
  `d9688490ad`. An empty rail now says No sessions yet instead of caught up,
  visibly labels its primary action New Terminal, and uses Start working rather
  than a project-only frame. Search, singular/plural attention, and genuine
  caught-up states remain distinct in an authored model assertion. The full
  `zed --bin dez` source check, formatting, diff, identity, CLI build, and help
  audit pass; the sidebar test target was not linked because it selected a
  second 1 GiB WebRTC graph on the storage-constrained volume. The current raw
  CLI hash is
  `06f2b4e799b9fc4dcc1178d3095cecea0a0dd2636f77a9d1827b98fc16a5563b`.
- 2026-07-23: Calmed the empty Session Rail in `1ebb7c79d4`. Compact mode now
  has a 280 px cap and 240 px resize floor, zero-session mode omits the
  meaningless All 0 / Attention 0 scope row, and search stays hidden until a
  session exists unless a live query must remain clearable. A focused model
  assertion covers those visibility rules. After clearing only regenerable
  Cargo caches, `cargo check --locked -p sidebar --lib -j1` passes from a clean
  graph in 14m34s; formatting, diff, and identity checks pass. The running
  bundle predates this source, so rendered proof remains open.
- 2026-07-23: Restored the reachable Dez CLI installation flow in
  `704314cc92`. The application menu now exposes Install CLI for Dez, the
  handler installs only `/usr/local/bin/dez`, Linux guidance names the bundled
  `dez` executable and preserves the official `zed` command, and launch
  handshake failures identify Dez. The official-Zed compatibility branch
  refuses to manage upstream CLI ownership. Formatting, diff, identity, CLI
  build/help, and the full `zed --bin dez` source check pass. The current raw
  CLI hash is
  `31ea17a6ddf2adf159cb55adca81c5f10d07c77c66608f6ec36242bc0c411e80`;
  bundle rebuild and interaction proof remain open.
- 2026-07-23: Closed the idle-terminal scope gap in `7a20dc1d19`. Terminal
  views now seed their current working directory into the Workspace-owned
  EvidenceSet during subscription, before the first PTY event, so a quiet new
  shell can produce an honest review brief. Later cwd changes, wakeups, and
  process exit retain their existing Current/Stale lifecycle behavior. The full
  `zed --bin dez` source check, formatting, diff, and identity gates pass;
  consolidated runtime and bundle proof remain open.
- 2026-07-23: Connected live background Workspace registration to durable
  viewport composition in `962b611605`. `MultiWorkspace::register_workspace`
  now records a database-backed Workspace in the current viewport without
  making it active; repeat registration is idempotent, a second viewport keeps
  global membership singular, and later activation updates selection in place.
  All ten low-disk `session --lib` tests pass, the full workspace library graph
  checks, and formatting, diff, and identity gates pass. Shared live entity and
  consolidated restart proof remain open.
- 2026-07-23: Made final-project fallback Workspaces durable in `e9a595fcff`.
  Both close-Workspace and remove-project-group paths now allocate a Workspace
  database ID before constructing an empty replacement, so construction and
  activation register the same App Session membership and viewport selection
  as any populated Workspace. The existing persistence regression now requires
  that identity, membership, and active selection. The production
  `cargo check --locked -p workspace --lib -j1` passes in 5m08s; the broader
  test-metadata check was stopped before a code result when free space fell
  below 1 GiB. Formatting, diff, and identity gates pass; runtime restoration
  proof remains open.
- 2026-07-23: Closed the source-level New Window App Session contract in
  `2334fbdcfc`. The existing headless regression now constructs its original
  window with the same real AppState used by `open_paths`, finds the second
  MultiWorkspace, and requires two viewport IDs, two Workspace IDs, singular
  shared App Session ownership, and the correct active Workspace in each
  viewport. The slice also removes a stale test-only call to Dez's deleted
  bottom dock and retains the supported flexible side-dock clamp assertion.
  `cargo check --locked -p workspace --tests -j1` passes with one unrelated
  dead-code warning. A direct execution attempt was cancelled during prolonged
  codegen/I/O saturation, so runtime test execution and packaged GUI proof are
  still open; formatting, diff, and identity gates pass.
- 2026-07-23: Made queued launch intent an explicit startup-barrier contract in
  `47e769da5d`. The continuing open listener now delegates to one ordered
  dispatcher that cannot release traffic before restoration completion and
  preserves request arrival order afterward. Its focused regression exercises
  pre-barrier retention, ordered release, and continued post-barrier delivery;
  the same completion signal is used after successful restoration or visible
  failure fallback. Formatting, diff, and identity gates pass. A cold
  `cargo check --locked -p zed --bin dez -j1` was stopped without a code result
  at the 3.4 GiB free-space safety floor, and only artifacts generated by that
  attempt were removed; no bundle was built or launched, so compiled-test and
  runtime claims remain open.
- 2026-07-23: Corrected failed-restore ownership truth in `d10d90648d`.
  Startup now marks a database-backed Workspace identity unresolved when its
  window cannot materialize, while deliberately retaining its ordered global
  membership and viewport placement for later retry or explicit removal. The
  state transition is idempotent and does not disturb successfully resolved
  neighbors. `cargo test --locked -p session --lib -j1` passes all 11 tests in
  2m08s; formatting, diff, and identity gates pass. The full Dez integration
  check and user-facing unresolved-Workspace recovery surface remain open, and
  no application bundle was built or launched. Commit `fbf8443359` later
  refines this into the distinct `RestoreFailed` state and persistent recovery
  callout described below.
- 2026-07-23: Made the existing failed-restore notice actionable in
  `31cc1b1205`. The toast now uses concise recovery copy, remains visible, and
  offers **Open Dez log** directly; one stable notification identity prevents
  duplicate failure stacks. The active-window path now reports success only
  when the toast update actually succeeds, so an unusable window still falls
  through to the durable empty recovery Workspace. Formatting, diff, and
  identity gates pass. Full Dez compilation and rendered interaction proof
  remain in the consolidated build gate.
- 2026-07-23: Added persistent failed-Workspace recovery to the Session Rail in
  `fbf8443359` and corrected a false-alarm edge case in the same slice. Durable
  resolution now distinguishes `RestoreFailed` from an identity that was merely
  not selected by the active restore policy; the failure state survives
  reconciliation until a real resolution or explicit removal. Only actual
  failures render a warning callout with **Open Recent** and **Dismiss** actions,
  and Dismiss removes the unresolved App Session reference without directly
  deleting its recent-workspace row. All 12 focused Session tests pass in
  4.89s. The offline lock update adds only Sidebar -> Session. Formatting,
  diff, and identity gates pass. A focused Sidebar library check was stopped
  at the 3.4 GiB storage floor while compiling inherited audio/WebRTC
  dependencies, before the final crate produced a result; only artifacts from
  that attempt were cleared. No bundle was built or launched, so compiled UI
  and rendered interaction claims remain open.
- 2026-07-23: Closed three source-level Zed coexistence collisions in
  `c101fe6a43`. macOS Dez channels now own ports 45737/45837/45937/46037 before
  per-user offsets instead of official Zed's 43737 range; Linux listener and
  CLI endpoints now agree on `dez-{channel}.sock`; and Linux/Windows CLI
  autodetection refuses to fall through to official Zed executables when its
  matching Dez app is absent. Identity guards freeze all three boundaries.
  `cargo check --locked -p cli --bin cli -j1` passes in 1m22s, and formatting,
  diff, and identity gates pass. No other executable was launched. Installed
  side-by-side proof remains open because the inspected app and command
  locations contain no official Zed installation.
- 2026-07-23: Removed the remaining upstream onboarding route from Dez chrome
  in `699cbd1bc8`. Title-bar promotion banners and the welcome page's Return to
  Onboarding action are now official-Zed-only. Dez Help says **Getting
  Started**; its optional welcome surface says Open Workspace, Recent
  Workspaces, and Supervise agent work, with evidence-first explanatory copy.
  Commit `d2e2a3992a` later replaces the promotional supervision and agent cards
  with one concise workflow summary so start actions and recent Workspaces own
  the hierarchy. Identity guards freeze both onboarding gates. Formatting,
  diff, and identity checks pass; compiled and rendered proof remains in the
  consolidated build gate, and no application bundle was built or launched.
- 2026-07-23: Removed the last Zed-branded glyph from Dez getting started in
  `869cddcce0`. The supervision card now uses the neutral Robot icon, and the
  identity suite rejects reintroducing Zed Assistant there. Formatting, diff,
  and identity checks pass; no application bundle was built or launched.
- 2026-07-23: Collapsed the empty Session Rail to one activation hierarchy in
  `4e6292ff0a`. The full **Start a session** state now owns the only New Terminal
  action; the ordinary Sessions overview returns once content exists. New File
  and Open have distinct icons, and a model assertion plus identity guard cover
  the handoff. Formatting, diff, and identity checks pass. Compilation and
  rendered proof remain deferred to the consolidated build; no application
  bundle was built or launched.
- 2026-07-23: Removed duplicate project/branch identity from the Dez Session
  Rail footer in `a9b1a961c0`. Workspace identity remains in the rail's group
  hierarchy rather than wrapping above the footer utilities. Restricted Mode
  and embedded application menus still open the row when essential; official
  Zed is unchanged. Two model tests and an identity guard cover those
  boundaries. Formatting, diff, and identity checks pass; no bundle was built
  or launched.
- 2026-07-23: Unified the Session Rail's visible Workspace vocabulary in
  `ff91b34a81`. Remote, options, focus, new-window, recent, and rules controls
  no longer alternate between Project and Workspace, and the recent entry uses
  a Folder Open glyph. Internal upstream types are unchanged; an identity
  rejection freezes the visible terminology. Formatting, diff, and identity
  checks pass; no bundle was built or launched.
- 2026-07-23: Removed the last zero-session creation duplicate in
  `4fc53b860f`. With open Workspaces but no sessions, each Workspace row now
  owns the single scoped New Terminal action; the global overview shortcut
  returns after a session exists. The no-Workspace activation surface is
  unchanged. An authored model assertion and identity guard cover the
  transition. Formatting, diff, and identity checks pass; no bundle was built
  or launched. The 2026-07-26 populated-state ownership slice below supersedes
  the shortcut-return behavior for Dez while preserving official Zed.
- 2026-07-23: Clarified the remaining Session Rail utility actions in
  `8bcd11f4b6`. The clock now says Agent History, the command icon consistently
  says Command Palette, and New File/Open expose explicit accessible labels and
  action-aware tooltips. An identity rejection freezes those visible names.
  Formatting, diff, and identity checks pass; no bundle was built or launched.
- 2026-07-23: Removed dead Session Rail controls from Dez Settings in
  `ad59a60926`. Branch status/name, worktree name, duplicate project identity,
  and upstream onboarding toggles are hidden because their surfaces no longer
  render. Compatibility schema remains readable; live Files Pane, menus, and
  window-button controls remain exposed. A model test and identity guard cover
  the filter. Formatting, diff, and identity checks pass; no bundle was built
  or launched.
- 2026-07-23: Removed inherited cloud/call surfaces from Dez Settings in
  `a20074de26`. Calls/Collaboration is no longer a Settings page, Network keeps
  only the live proxy control, and GUI paths for Zed auto-connect/server URL are
  absent. Compatibility keys remain readable and official Zed is unchanged.
  Attention copy no longer names the removed workspace bar. A model test and
  identity guards cover the boundary; no bundle was built or launched.
- 2026-07-23: Removed upstream prediction-data controls from Dez Settings in
  `2435348289`. Zed Edit Predictions data collection is absent because the fork
  disables its Zed/Mercury providers; provider setup now describes explicit
  providers honestly. Compatibility parsing and official Zed behavior remain.
  A model test and identity guard cover the boundary; no bundle was built or
  launched.
- 2026-07-23: Productized command and settings-link identity in `f89f55868c`.
  Internal `zed::…` actions display as `dez: …` without changing keymap/action
  identities. Settings copy links and URL registration now share the active
  channel's canonical Dez scheme; legacy `zed://` remains input-only
  compatibility. Focused assertions, formatting, diff, lockfile, and identity
  checks pass; no bundle was built or launched.
- 2026-07-23: Removed inherited collaboration actions from the Dez Command
  Palette in `f40877d4ab`. The namespace stays registered for keymap/action
  compatibility and official Zed remains unchanged; only dead Dez presentation
  is filtered. A model assertion and identity guard pass; no bundle was built
  or launched.
- 2026-07-23: Removed Zed Assistant glyphs from Codex session surfaces in
  `526218a972`. The Session Rail and keyboard switcher now use a neutral Robot
  glyph while preserving explicit Codex labels and metadata. A directory-wide
  identity rejection covers both paths; no bundle was built or launched.
- 2026-07-23: Added deliberate Codex setup to first-run onboarding in
  `bb0cf408b4`. **Copy Codex Hook** copies the bundled JSON without installing
  or modifying anything, precedes New Terminal in keyboard order, and sits with
  lifecycle and Host-persistence limitations. An identity guard covers the
  action; no bundle was built or launched.
- 2026-07-23: Added explicit user-selected Workspace review evidence in
  `a8ce563373`. Command Palette actions add, remove, or clear selected paths;
  file-tab context actions reach the same Workspace owner. Selections survive
  passive open-file recomputation and tab closure, use stable user-selection
  provenance, stop at 128 paths with visible feedback, and project as Selected
  path in Review Briefs without a duplicate Open file row. Focused model tests
  are authored; formatting, diff, and identity checks pass. No bundle was
  built or launched.
- 2026-07-23: Made explicit Workspace review evidence restart-durable in
  `e101b63e43`. A new additive Workspace database column stores only selected
  paths; passive roots, open tabs, and terminal observations still recompute
  from their owners. Add/remove/clear schedules the established Workspace save,
  and restore rehydrates selections with the current Host classification. A
  focused database round-trip test is authored; formatting, diff, and identity
  checks pass. No bundle was built or launched.
- 2026-07-23: Made the file-tab evidence menu state-aware in `f535c5e6ae`.
  Tabs now expose Add or Remove according to the active Workspace EvidenceSet,
  never both simultaneously; keyboard users retain separate add/remove/clear
  commands with truthful no-op feedback. Formatting, diff, and identity checks
  pass. No bundle was built or launched.
- 2026-07-23: Reconciled saved terminal cwd evidence on restore in
  `0e6507756e`. Failed hosted-session attach retains the last-known cwd under
  its original Session ID as Unresolved, Review Briefs disclose that risk, and
  successful reattach replaces the same record with Current Host truth. A
  focused model test is authored; formatting, diff, and identity checks pass.
  No bundle was built or launched.
- 2026-07-23: Added live hosted-terminal evidence lifecycle reconciliation in
  `ea2bb18453`. Each hosted TerminalView observes Host snapshot revisions and
  updates the same Session-owned cwd record across Current, Unresolved, and
  Stale states without treating transport loss as process exit. A focused
  state-mapping test is authored; formatting, diff, and identity checks pass.
  No bundle was built or launched.
- 2026-07-23: Restricted authoritative EvidenceSet mutation to the Workspace
  crate in `0f8740b1a1`. Downstream search, Git, settings, conversation, and
  review consumers retain immutable record access but cannot attach roots or
  invent selections through the evidence model. Formatting, diff, and identity
  checks pass. No bundle was built or launched.
- 2026-07-23: Isolated provisional Workspace evidence identity in
  `af232402f5`. Each not-yet-persisted Workspace now owns a stable UUID-backed
  evidence namespace, so two Workspace views of the same path cannot emit
  colliding record IDs. A focused same-path isolation test is authored;
  formatting, diff, and identity checks pass. No bundle was built or launched.
- 2026-07-23: Bound retained terminal Sessions to durable Workspace ownership
  in `a4047d95c0`. Host snapshots carry an additive optional Workspace ID;
  TerminalView associates in-process and helper Sessions, and Session Rail
  prefers that exact owner over cwd prefix matching. Older snapshots default
  to unknown ownership and retain the conservative fallback. Model,
  compatibility, and integrated same-cwd/two-Workspace tests are authored;
  formatting, diff, and identity checks pass. No bundle was built or launched.
- 2026-07-23: Added focused live and persisted Workspace evidence isolation
  proof in `6f2061d2c7`. Two same-path Workspace entities retain different
  Session cwd records and identities; two durable Workspaces retain independent
  selected paths, and clearing one does not mutate the other. Tests are
  authored; formatting, diff, and identity checks pass. No bundle was built or
  launched.
- 2026-07-23: Scoped Session Rail Git branch projection to Workspace ownership
  in `27279ca542`. Open terminal and agent rows read branch metadata only from
  their owning Workspace repository snapshot. Closed historical rows use an
  unambiguous cross-Workspace fallback; disagreement for the same worktree path
  suppresses the branch rather than allowing iteration order to lie. A focused
  fallback regression is authored; formatting, diff, and identity checks pass.
  No Rust test, bundle build, or application launch was performed.
- 2026-07-23: Scoped graphical project settings to their originating viewport
  in `a2d733eea3`. Settings opened from one MultiWorkspace no longer aggregates
  every live Project in the application or switches to unrelated Workspaces if
  the originating window closes. Global settings remain global; project-file
  discovery, restricted-mode lookup, file opening, and updates share the same
  viewport-scoped project resolver. Two cross-window regressions are updated;
  formatting, diff, and identity checks pass. No Rust test, bundle build, or
  application launch was performed.
- 2026-07-23: Closed the Settings-window reuse gap in `498f94a525`. Reopening
  the single Settings window from a different MultiWorkspace now refreshes its
  project-file inventory before applying a requested target, so stale files
  from the prior viewport cannot receive the action. The cross-window test now
  proves scope replacement in both directions. Formatting, diff, and identity
  checks pass; compilation remains deferred.
- 2026-07-23: Completed the registered-panel reachability map in
  `5efa0398ad` and `1f186f9b8c`. Debug joined Project, Git, and Outline as an
  ordinary Project-tool pane tab whenever Dez hides legacy docks; Agent retained
  its dedicated pane. The same map covered the inherited compatibility Terminal
  Panel, which `ad2fdcf766` later removed from Dez's public interaction model in
  favor of center-only terminals. The mapping test still covers every registered
  developer-tool key plus unknown-key rejection.
- 2026-07-23: Unified the center launch and tool-recovery copy on Workspace
  vocabulary in `e4f1e341c9`. The loaded-worktree launch surface now says
  **Workspace ready**, and an unattached tool pane reports **Workspace tools
  unavailable**. Internal upstream Project and PaneKind names remain unchanged.
  The identity guard rejects the former mixed vocabulary; formatting, Bash
  syntax, diff, and identity checks pass. Rendered proof remains deferred.
- 2026-07-23: Named the outward tool hierarchy deliberately in `6f1562847e`.
  Dez now exposes **Workspace tools** for the multi-tool pane and **Files** for
  the file tree in tabs, empty state, tooltip, reveal action, and Settings;
  legacy dock settings state their compatibility-only effect. Internal panel
  keys and the official-Zed copy remain stable behind product branches. Cargo
  metadata, formatting, Bash syntax, diff, and identity checks pass; no compile,
  bundle build, or rendered interaction claim is made.
- 2026-07-23: Distinguished the supplied crushed-shell screenshot, the running
  bundle, and current source instead of treating them as one artifact. The
  screenshot is timestamped 20:10 and visibly contains `0 sessions · caught
up`; a read-only string fingerprint confirms that copy is absent from PID
  `85053`'s 01:10 executable. That executable contains the intermediate
  **Project ready** state but not the later **Start a session** or **Workspace
  ready** source. The screenshot remains valid defect evidence, but neither it
  nor the stale bundle is current-source render proof.
- 2026-07-23: Extended outward Workspace vocabulary in `f6aea3e013`. Dez now
  says **Open Recent Workspace**, **Recent Workspaces**, **Remote Workspace**,
  **Remove from Recent Workspaces**, and **Delete Remote Workspace** across the
  title bar, recent-work picker, remote controls, and Agent History. Explicit
  product branches retain official Zed copy. Formatting, Bash syntax, diff,
  and identity checks pass; compilation and rendered proof remain deferred.
- 2026-07-23: Closed the recent-work and trust-language follow-up in
  `b749a25619`. Dez's search, empty states, open/move/remove, multi-root,
  Dev Container, and failure copy now consistently names Workspaces. Restricted
  Mode says what Workspace settings and configured tools it blocks, and trust
  scope validation reports a Workspace boundary. Official Zed retains Project
  labels through explicit branches. Formatting, Bash syntax, diff, and identity
  checks pass; compile and rendered verification remain deferred.
- 2026-07-23: Extended the Workspace/Files contract across developer tools in
  `0e2c0dcae3`. Remote search and disconnection, Agent History, pane search,
  review-evidence prompts, agent rules/checkpoints, skill scope, MCP/external
  agent empty states, trust/status settings, and every visible Files setting now
  use the product's outward language. Official Zed retains Project copy through
  explicit branches; internal actions and schema remain compatible. Formatting,
  Bash syntax, diff, and identity checks pass; compile/render proof is deferred.
- 2026-07-23: Removed the remaining reachable upstream recovery/promotional
  traps in `e969abda4a`. Dez's unsupported-GPU dialog uses Dez identity and
  override environment, explicitly attributes the upstream rendering guide,
  and avoids placeholder copy. The Command Palette hides inherited cloud,
  feedback, account, docs, status, and merchandise actions; Help retains
  explicit upstream references. Database failure now states file safety and
  Session/Workspace risk and opens local logs. Static gates pass; compile and
  interaction proof remain deferred.
- 2026-07-23: Completed the source-level shell grammar in `0607771783`.
  Retained disconnected, follow/join, shared-agent, and multi-root collaboration
  recovery states now say Workspace in Dez while official Zed keeps Project.
  Together with the command-center cleanup in `e969abda4a`, no reachable Dez
  shell state relies on Project as the outward container term. Identity,
  formatting, Bash syntax, and diff gates pass; rendered proof remains deferred.
- 2026-07-23: Reworked the screenshot's zero-session hierarchy in
  `2092acd453`. The rail is now named **Session Rail**, reports ready Workspace
  count instead of repeating session absence, and gives each empty Workspace a
  concise **Ready for a session** state with one scoped New Terminal action.
  The action's accessible name includes the Workspace. Focused assertions and
  the identity guard pass; compilation and rendered proof remain deferred.
- 2026-07-23: Made Session Rail creation terminal-first in `57290c27c3`.
  Workspace plus controls and multi-worktree pickers now create terminals;
  macOS, Linux, Windows, and Vim default rail bindings dispatch a dedicated New
  Session action. Pane-native agent creation remains an explicit **New Agent
  Thread** Workspace option instead of the silent default. A focused target
  assertion and identity guards pass; compilation and rendered proof remain in
  the consolidated gate.
- 2026-07-23: Closed a Workspace-options pointer/keyboard parity gap in
  `633dcc4bec`. Every non-active worktree exposed through hover can now also be
  closed from a keyboard-addressable **Close Worktree from Window** submenu,
  with multi-root names preserved. Whole-group removal now says **Remove
  Workspace from Window** so its broader scope cannot be confused with closing
  one worktree. Focused label assertions and identity guards pass; compiled and
  rendered interaction proof remains deferred.
- 2026-07-23: Made restored attention textual in `7893762cd5`. A persisted
  active condition now composes **Needs attention** with Saved, Detached,
  Reconnecting, Missing, Incompatible, and Exited transport truth even when no
  live adapter snapshot exists. Focused state assertions and the identity guard
  pass; durable activity/review recovery and rendered accessibility proof remain
  open.
- 2026-07-23: Made bounded Host evidence gaps durable and visible in
  `bd36afd3f4`. Structured activity now records when older events were evicted,
  preserves that fact across detach/list/reattach, qualifies rail summaries as
  **partial**, and explains the limitation in Review Brief risks. A focused
  40-event Host lifecycle assertion and identity guards pass; actual helper
  process/GUI restart and rendered review proof remain deferred.
- 2026-07-23: Hardened retained terminal evidence and its settings contract in
  `b39eedf724`. Secret-looking environment assignments are redacted before
  command evidence is bounded or retained. Evidence settings now describe
  local lifecycle observation, authenticated adapters, the no-transcript
  boundary, independent identity/attention metadata, and Host-owned structured
  activity. Focused redaction and identity assertions pass; compiled hook flow
  and broader redaction audit remain deferred.
- 2026-07-23: Extended retained-command privacy in `9323af8008`. Shared
  redaction now covers secret-suffixed CLI flags in both `--token value` and
  `--api-key=value` forms while preserving ordinary options such as `--mode`
  and `--license-file`. Focused utility and Host-ingestion assertions plus the
  identity guard pass; compiled hook-flow proof remains consolidated.
- 2026-07-23: Extended retained-command privacy to URLs in `80ff1df75f`.
  Userinfo passwords and secret-suffixed query parameters are redacted while
  host, path, and ordinary query structure remain reviewable. Evidence settings
  explicitly disclose that bounded file targets remain verbatim for navigation.
  Focused utility/Host assertions and identity guards pass; compiled hook-flow
  proof remains consolidated.
- 2026-07-23: Exposed the Session Rail group hierarchy to assistive technology
  in `c47637c2ac`. The virtual rows now live in a named accessible list;
  Workspace headers report selection, expansion, ready/running work, and
  attention counts without relying on color. Sticky visual copies are excluded
  to avoid duplicate announcements. Focused copy assertions and identity guards
  pass; platform accessibility-tree inspection remains deferred.
- 2026-07-23: Made Session Rail scope and result changes announceable in
  `7e91f00b69`. All/Attention controls now belong to a named scope group with
  stable control names, separate toggle state, and action descriptions; search
  is a named region, while totals, no matches, and caught-up results use status
  semantics. Focused copy assertions and identity guards pass; platform
  announcement timing remains deferred.
- 2026-07-23: Exposed callout urgency to assistive technology in `e28b78ed57`.
  The shared component now reports informational/success messages as status and
  warning/error messages as alerts, so durable Host startup failures and failed
  Workspace restoration are not conveyed only through icon, color, and copy.
  Authored role assertions and identity guards pass; platform announcement
  timing remains deferred.
- 2026-07-23: Normalized high-frequency Session Rail pointer targets in
  `a90fae5873`. Compact Workspace headers now match their 28 px hover controls;
  New Terminal, All/Attention scope, Host-detail, and failed-Workspace recovery
  controls use the same medium target instead of 18 px compact buttons. Static
  identity guards pass; rendered density and zoom proof remains deferred.
- 2026-07-23: Connected Session Rail keyboard navigation to accessibility focus
  in `f6318ea907`. The focused Workspace header or Session row now claims
  active-descendant focus while the rail retains actual keyboard focus, so row
  labels and state can follow Up/Down navigation. The audit also confirmed that
  GPUI renders one static frame for repeating activity animations when Dez's
  reduced-motion setting resolves true. Identity guards pass; platform focus
  and motion inspection remains deferred.
- 2026-07-23: Retired the incomplete 56 px Session Rail icon presentation in
  `9930e86677`. The compatibility value is still parsed, but now resolves to
  compact—the smallest v0.0.1 layout that can present Workspace hierarchy,
  search, evidence, inline actions, and recovery without clipping. The width
  regression assertion and identity guard cover the fallback; rendered compact
  proof remains deferred.
- 2026-07-23: Polished the terminal-first onboarding workflow in `4a102fc50e`.
  Start/Watch/Review now form a named accessible list inside a named region; the
  detach/terminate/Host/hook safety explanation sits above a wrapping action row
  instead of competing for horizontal space, and both actions use 28 px targets.
  Identity guards pass; rendered zoom and narrow-window proof remains deferred.
- 2026-07-23: Closed the inherited telemetry boundary in `933e3f515f`. Non-Zed
  builds now force diagnostics and metrics off before the telemetry client can
  queue or upload them; Dez onboarding and Settings no longer imply that opting
  in improves Dez through an upstream endpoint. Anthropic retention remains
  separately configurable because it governs model-request eligibility.
  Focused source assertions and identity guards pass; compiled network proof
  remains consolidated.
- 2026-07-23: Removed the inert Dez Auto Update section in `2680937952`. The
  fork updater already returns false and defaults off, so presenting a switch
  implied behavior that cannot occur. The compatibility key remains readable
  and official Zed retains its setting. Focused source assertions and identity
  guards pass; Settings rendering remains consolidated.
- 2026-07-23: Removed the last Collaboration Panel settings leak in
  `9239006d4b`. Dez no longer shows button, dock, or width controls for a panel
  whose page and commands are already absent. Compatibility parsing and official
  Zed UI remain intact. Identity guards pass; Settings rendering remains
  consolidated.
- 2026-07-23: Exposed Session Rail shortcuts to assistive technology in
  `33f7ff5893`. Shared text and icon buttons can now report a shortcut without
  drawing a key label; All/Attention announces Shift+A and Review Brief buttons
  announce Shift+V, matching the visible action-aware tooltips. Identity guards
  pass; platform announcement proof remains consolidated.
- 2026-07-23: Closed the legacy `auto_connect` provider-authentication bypass in
  `2fc5226a51`. Eager authentication now requires both official Zed identity and
  explicit auto-connect; a stale true value cannot make Dez contact every model
  provider at startup. The existing source test now derives its expectation
  from the product gate, and identity guards pass; compiled network proof
  remains consolidated.
- 2026-07-23: Removed unavailable upstream edit-prediction presentation in
  `cc2509e8b8`. Legacy Zed and Mercury selections now resolve to no provider in
  Dez before the status item renders; neither provider appears in the picker,
  and the Mercury setup card is absent. Official Zed behavior and explicit
  Copilot, Codestral, Ollama, and compatible API paths remain intact. Focused
  source assertions and identity guards pass; Settings/status rendering remains
  consolidated.
- 2026-07-23: Prevented the Agent UI from restoring upstream prediction
  commands in `b909b31d45`. Dez now keeps the Zed Predict onboarding namespace
  and action hidden after every Agent/settings filter refresh, and stale
  Zed/Mercury providers behave like no provider when deciding whether edit
  prediction commands exist. Explicit supported providers and official Zed are
  unchanged. Focused source assertions and identity guards pass; rendered
  command-palette proof remains consolidated.
- 2026-07-23: Isolated crash recovery and duplicate-instance identity in
  `aab0e5f2f2`. Ordinary Dez launches no longer install the inherited
  stable-channel crash handler based on an upstream endpoint; local minidump
  generation requires explicit `DEZ_GENERATE_MINIDUMPS=1|true`. Startup logs,
  duplicate-instance output, crash metadata, and temporary artifacts identify
  Dez. Official Zed retains its existing policy. Focused source assertions and
  identity guards pass; compiled crash-path proof remains consolidated.
- 2026-07-23: Removed inherited collaboration shortcuts and call chrome in
  `0ddf84161e`. Dez filters channel/collaboration namespaces plus the
  follow-collaborator action from every loaded keymap source, including user
  bindings, so removed handlers cannot shadow useful editor commands. Title-bar
  call controls are official-Zed-only. AI opt-out filtering and official Zed
  behavior remain intact. Focused source assertions and identity guards pass;
  compiled key-routing proof remains consolidated.
- 2026-07-23: Corrected remaining CLI/import guidance in `2efbf166b7`. Dez
  Settings now teaches `dez <path>` for default-open behavior, while the skill
  URL importer describes its `GITHUB_TOKEN` retry without attributing the
  behavior to Zed. Official Zed retains its CLI copy. Focused source assertions
  and identity guards pass; rendered Settings/skill proof remains consolidated.
- 2026-07-23: Reconciled the frozen workspace lock graph in `c3dfb7aa79`.
  Full locked metadata exposed two previously omitted direct dependency edges:
  `util` for `dez_terminal_host` and `paths` for `project_panel`. No package
  version or source changed. Full locked metadata now resolves; the recorded
  lockfile hash is updated in Release Evidence.
- 2026-07-23: The consolidated compile exposed a missing GPUI `Role` import and
  stateful-element boundary in the shared Callout accessibility slice. Commits
  `5a25a72f92` and `42ff77e99c` import the role and give each Callout a stable
  call-site ID before applying it. The focused `ui` build passes in 2m33s; both
  failed attempts stopped before replacing an executable.
- 2026-07-23: The resumed consolidated compile exposed the terminal view
  importing Host snapshot globals from their parent module instead of the
  public `session_host::transport` module that defines them. Commit
  `1ed2ff814a` aligns the terminal view with the already-working Session Rail
  import without changing snapshot behavior. Formatting, diff, and identity
  checks pass; the failed build stopped before replacing an executable.
- 2026-07-23: The next compile exposed one ownership conflict between terminal
  event handling and hosted snapshot observation. Commit `4369bb1f3b` gives
  the event closure its own weak Workspace clone, preserving navigation,
  lifecycle evidence, and Host reconciliation. The focused `terminal_view`
  build passes in 7m36s.
- 2026-07-23: Closed the remaining consolidated compile boundaries. Commit
  `af8ecad961` exports the existing tested Dez prediction-provider
  normalization for Settings UI. Commits `9ed0e1aaaf` and `2452d3b4ec` give
  the Session Rail and terminal-first onboarding landmarks stable GPUI IDs
  before applying accessibility roles. The clean locked arm64 app/helper build,
  separate CLI build, and `Dez Dev.app` bundle all pass through
  `2452d3b4ec`. The 1.0G bundle identifies as Dez v0.0.1 with scheme `dez-dev`,
  contains only the expected arm64 app, CLI, Host, and Git executables, and
  passes deep strict ad-hoc signature verification. Runtime launch and fresh
  visual/accessibility evidence remain open.
- 2026-07-23: Reduced compact Session Rail rows to one legible decision
  hierarchy in `059656999e`. Narrow rows retain identity, priority evidence,
  Review Brief, and the contextual lifecycle action; rename and raw-diff
  controls remain in the context menu and return inline only in the detailed
  layout. Stronger evidence displaces redundant recency at compact widths, and
  the full title plus state metadata remains available in a wrapping tooltip.
  Formatting, static identity checks, and diff checks pass; rendered proof
  remains deferred by the active no-build gate.
- 2026-07-23: Removed the false second terminal model in `ad2fdcf766`. Dez no
  longer advertises the unloaded compatibility Terminal Panel in the View menu,
  New Item menus, terminal context menu, Command Palette, inherited keybindings,
  status-bar Settings, or dock-size Settings. The duplicate internal
  **New Center Terminal** action is hidden from the palette while remaining
  available to source-level UI dispatch. **New Terminal** now names the one
  public center-tab/split behavior everywhere; official Zed keeps its existing
  panel presentation. Focused pure assertions, formatting, identity, and diff
  checks pass; rendered proof remains deferred by the no-build gate.
- 2026-07-23: Removed implementation-placement language from the Dez View menu
  in `622acd1a61`. The structural toggle is **Workspace Tools** and its
  destinations are **Files**, **Outline**, **Debug**, **Agent**, and **Git**;
  users no longer have to interpret Project Tab, Project Panel, or pane-versus-
  dock implementation details. Official Zed retains its existing labels.
  Focused pure assertions and the static identity guard pass; rendered proof
  remains deferred.
- 2026-07-23: Removed promotional card stacking from the welcome surface in
  `d2e2a3992a`. The three-row supervision explainer and separate agent card no
  longer compete with start actions and recent Workspaces; the header now says
  **Start in a terminal. Track attention. Review evidence.** The change also
  removes an Open Agent shortcut that could accidentally close an already-open
  Session Rail before focusing Agent. Formatting, focused pure assertions,
  identity, and diff checks pass; rendered proof remains deferred.
- 2026-07-23: Tightened center absence-state density in `594296efa8`. Welcome
  content now uses a restrained 640 px measure, 24 px inset, and one spacing
  rhythm; empty Workspace and unavailable-tool recovery states use a 384 px
  measure rather than broad card-like canvases. Static identity, formatting,
  and diff checks pass; rendered narrow-window proof remains deferred.
- 2026-07-23: Made the Session Rail utility strip describe its real actions in
  `9e14ca2db7`. The configuration popover now uses a Settings glyph and
  **Agent Tools and Settings** identity instead of a Robot glyph that implied it
  opened Agent; its entries name MCP servers, Agent Context, Agent Profiles,
  Open Settings, and Hide Session Rail explicitly. The folder utility now says
  **Open Recent Workspaces** instead of the broader Open Workspace. Formatting,
  static identity, and diff checks pass; rendered tooltip/menu proof remains
  deferred.
- 2026-07-23: Polished terminal editing, lifecycle, and failed-reconnect actions
  in `72cec1f285`. Copy appears only when a selection exists; Paste Clipboard,
  Paste Text Only, Select All, and Clear Screen state their scope. A hosted
  terminal offers Detach Terminal versus Terminate Session, while a local tab
  offers Close Terminal Tab versus Terminate Terminal Session. Failed restore
  now says **This terminal cannot reconnect** and **Start New Terminal** without
  implying a replacement shell exists. The permanently disabled terminal
  Inline Assist branch is removed. Source inspection also confirms the
  right-aligned checkmark and clock in the supplied screenshot are shell-owned
  PTY prompt content, not a TerminalView overlay; Dez must not restyle them.
  Focused pure assertions, formatting, identity, and diff checks pass; rendered
  menu/callout proof remains deferred.
- 2026-07-23: Normalized top-level Dez menus on user-facing Workspace language
  in `218e346ede`. Settings now opens Workspace Settings, File adds folders to
  and closes a Workspace, and Edit finds in a Workspace while official Zed
  retains Project labels. Help leads with Getting Started and Release Notes,
  calls the retained local event viewer **Open Local Diagnostics Log**, calls
  dependency notices **Open Source Licenses**, and consolidates attribution
  links under one **Upstream Zed** submenu. Focused label assertions, static
  identity, formatting, and diff checks pass; rendered native-menu proof
  remains deferred.
- 2026-07-23: Productized retained Command Palette names in `35c516a5bb`.
  Dez now presents **Session Rail**, **Files**, **Workspace Search**,
  **Workspace Symbols**, **Workspace Tools**, and **Dez** instead of inherited
  implementation namespaces such as sidebar, project panel, and zed actions.
  Project-category badges read **workspace** in Dez, while official Zed labels,
  action IDs, keymaps, telemetry, and dispatch remain unchanged. Formatting,
  static identity, and diff checks pass; rendered palette proof remains
  deferred.
- 2026-07-23: Made the center-terminal/Session-Rail lifecycle truthful in
  `be1a20dae1`. An attached center tab remains the interactive terminal and
  closes by detaching its view; a Host-only running row now says
  **Terminate Running Session**, uses destructive presentation, and requires
  confirmation before stopping computation. Exited, missing, incompatible, and
  saved rows use close/remove language, and ownership reads **Durable Host**,
  **Remote Workspace**, or **Workspace process**. Commit `ddd7f25f4e` also
  gives empty PTY titles the same **Terminal** fallback in the Session Rail
  that terminal tabs already used, preventing valid Sessions from rendering as
  blank rows. Focused pure assertions, formatting, identity, and diff checks
  pass; rendered row/action proof remains deferred.
- 2026-07-23: Replaced ambiguous bottom chrome in `91f738f83b`. At normal
  Session Rail widths, the persistent footer now names **Agent Tools**,
  **History**, and **Workspaces**; it collapses to the same accessible,
  tooltip-backed icons only below the existing compact breakpoint. The center
  status bar now calls its retained tools **Search Workspace Files** and
  **Workspace Diagnostics** in Dez, and a terminal-focused Workspace with no
  diagnostics shows the restrained text **No diagnostics** instead of an
  unexplained checkmark. Official Zed retains Project labels and compact
  zero-state presentation. The locked metadata graph was reconciled for the
  new direct `paths` dependencies and an existing omitted `terminal_view`
  edge. Formatting, locked no-dependency metadata, identity, and diff checks
  pass; rendered responsive proof remains deferred.
- 2026-07-23: Separated Agent Session vocabulary from terminal identity in
  `92e0591811`. Dez now says **New Agent Session**, **Rename Agent Session**,
  **Archive Agent Session**, **Regenerate Agent Session Title**, and
  **Open Agent Session as Markdown** throughout the Session Rail. Empty drafts,
  Agent History search/count/filter/import/empty states, review-risk copy,
  title-generation toasts, and Command Palette actions follow the same model.
  **Terminal Session** remains reserved for terminal-backed computation.
  Official Zed retains Thread terminology and underlying action IDs remain
  unchanged. Agent History's icon-only row and toolbar actions have explicit
  accessible names, with their regression guards completed in `1f3432a211`.
  Focused pure assertions, formatting, locked offline metadata, identity, and
  diff checks pass; rendered rail/history proof remains deferred.
- 2026-07-23: Aligned Workspace tool promises with their center destinations
  in `ce770c5eee`. Workspace Search now retains **Workspace Search** as its
  fallback tab/tooltip title, says **Loading workspace…**, and reports
  no-results scope as the Workspace rather than reverting to Project after the
  status-bar action opens it. Workspace Diagnostics uses the same
  **Workspace Diagnostics** tooltip as its launcher, and its custom tab content
  now always includes **Diagnostics** before the check/error/warning status
  instead of rendering an unexplained status-only tab. Official Zed retains
  Project labels. Focused pure assertions, formatting, locked offline metadata,
  identity, and diff checks pass; rendered destination proof remains deferred.
- 2026-07-23: Removed the clipped shortcut-badge failure mode from the empty
  Workspace launcher in `281b6e22c9`. **New Terminal**, **Find File**, and
  **New File** remain full-width actions with unchanged dispatch and keymaps,
  but shortcut discovery now lives in each action tooltip instead of competing
  with the label inside the constrained button width. Explicit accessible names
  preserve keyboard and assistive-technology clarity. Formatting, locked
  offline metadata, identity, and diff checks pass; rendered narrow-width proof
  remains deferred.
- 2026-07-23: Rebalanced terminal and Agent Session rows in `be2a8d6ec6`.
  Compact and minimum-detailed rails now prioritize title, live state, attention,
  evidence, and recency instead of also forcing actor, Host ownership, and
  worktree context into the same narrow metadata line. Those supplemental
  labels return at 440 px; actor and Host identity remain in the row's
  accessibility name and tooltip while visually hidden. Plain terminal rows
  also omit the redundant **Terminal Session** actor label below that threshold.
  Focused source assertions, formatting, locked offline metadata, identity, and
  diff checks pass; rendered width/theme proof remains deferred.
- 2026-07-23: Made panel toggles honor the pane-tab model in `39c8379f05`.
  Files, Outline, Git, Debug, and Agent already open as tabs in dedicated
  Workspace Tools or Agent panes when legacy docks are disabled, but their
  generic close path still targeted docks only. Closing or re-toggling a focused
  tool now hides its center tool pane, preserves a visible editor or terminal
  pane, restores focus there when needed, and serializes the resulting layout.
  A focused source regression and the static identity guard cover the contract.
  Formatting, locked offline metadata, identity, and diff checks pass; rendered
  toggle/focus proof remains deferred.
- 2026-07-23: Added a permanent-deletion boundary to Agent History in
  `9615b513d4`. The archived-row trash control and the keyboard remove action
  now share one critical confirmation that names the Agent Session, states that
  deletion removes it from Agent History, and says it cannot be undone.
  Archive and restore remain immediate reversible actions. Product-specific
  prompt assertions, formatting, locked offline metadata, identity, and diff
  checks pass; rendered modal and focus-return proof remain deferred.
- 2026-07-23: Made the tool hierarchy legible before interaction. Commit
  `799147c525` keeps Workspace expand/collapse disclosure icons visible instead
  of hiding the only structural affordance until hover. Commit `5a64bc9af1`
  records the permanent interface contract: Session Rail is a non-owning
  projection, Workspace Tools and Agent are hideable pane-grid regions, and
  terminals remain ordinary main-area Surfaces. Formatting, locked offline
  metadata, identity, diff, and documentation-format checks pass; rendered
  hierarchy proof remains deferred.
- 2026-07-23: Closed two pane-tab lifecycle seams. Commit `6cbdda5405`
  removes a retained Workspace Tool item when its backing panel is
  unregistered and verifies focused-tool re-toggle behavior. Commit
  `a95e0a4bb4` excludes the inherited `TerminalPanel` from Dez's Workspace
  Tools routing so terminals have one visible home: main-work-area tabs and
  splits. Official Zed retains Terminal Panel behavior. Formatting, locked
  offline metadata, identity, diff, and documentation-format checks pass;
  compilation and rendered remove/re-register and terminal-migration proof
  remain deferred.
- 2026-07-23: Clarified where terminal-backed work starts and lives. Commit
  `f863b3e45c` keeps the active Workspace's terminal creation action visible,
  terminal-shaped, and Workspace-named. Commit `d1ea4a914f` makes external
  Agent Session import copy consistent, while `037eebaf02` renames Dez's
  terminal startup setting to **Terminal Session Startup Command**. Commit
  `d490735631` restricts Workspace Tools to Files, Outline, Git, and Debug in
  Dez; Terminal and Collaboration remain official-Zed-only compatibility
  panels. Formatting, locked offline metadata, identity, and diff checks pass;
  compilation and rendered multi-Workspace, modal, settings, and panel-routing
  proof remain deferred.
- 2026-07-23: Restored Agent History search as a real interaction in
  `5cf88897ba`. The existing editor and update subscription no longer feed a
  hard-coded empty query; a visible Search region now filters Agent Session
  titles and Workspace path names, exposes a clear action, and distinguishes
  **No matching Agent Sessions** from genuinely empty history. Empty history
  provides a full-width **Start New Agent Session** action. Product-copy,
  search-binding, and empty-state assertions plus static guards pass;
  compilation and rendered search/focus proof remain deferred.
- 2026-07-23: Removed the last scoped terminal-creation route into the inherited
  Agent panel in `d1b2b640de`. A Workspace's visible Session Rail terminal
  action now activates that Workspace and dispatches `NewCenterTerminal`,
  matching the welcome, empty Workspace, menu, and shortcut behavior. The
  action therefore creates a normal main-work-area terminal Surface instead of
  an Agent terminal thread. Formatting, locked offline metadata, identity, and
  diff checks pass; compilation and rendered multi-Workspace placement proof
  remain deferred.
- 2026-07-23: Closed the corresponding Agent-side routes in `263553d036`.
  Terminal is no longer offered as an Agent type in Dez, a remembered
  terminal-thread selection cannot replace **New Agent Session**, and
  Agent-focused compatibility actions dispatch `NewCenterTerminal` instead.
  Compatibility action IDs and official Zed behavior remain intact. Focused
  source assertions, formatting, locked offline metadata, identity, and diff
  checks pass; compilation and rendered selector/shortcut proof remain
  deferred.
- 2026-07-23: Made the upgrade boundary truthful in `16d1bd16b8` and
  `d1f5b2a15a`. Dez now projects stored terminal metadata only when a real live
  Agent terminal or Host Session backs it; Host-backed records attach in the
  main work area, stale metadata cannot shadow live center/Host terminals, and
  the retired Agent-terminal surface no longer auto-restores or appears in the
  Command Palette. Official Zed retains its compatibility behavior. Focused
  assertions, formatting, locked offline metadata, identity, and diff checks
  pass; compilation and rendered upgrade proof remain deferred.
- 2026-07-23: Recorded the complete Workspace surface model and kept critical
  Workspace controls discoverable in `86775eb7bc`. Fork Notes now specifies
  where each everyday action lands and how Zed's editor, language, search,
  diagnostics, Git, debug, terminal, and Agent capabilities share one
  Workspace Project. The active Workspace's options control remains visible
  with an explicit tooltip; inactive rows retain the quieter hover treatment.
  Formatting and static checks pass; compilation and rendered narrow-width
  proof remain deferred.
- 2026-07-23: Clarified the Agent title edit affordance in `c625c1f3b4`. The
  icon-only pencil now has an accessible name and tooltip, says **Edit Agent
  Session Title** in Dez, preserves official Zed's **Edit Thread Title**, and
  calls the retained compatibility terminal surface **Edit Terminal Title**.
  Pure label assertions and the static identity guard pass; rendered hover and
  accessibility-tree proof remain deferred.
- 2026-07-23: Clarified unsupported Agent Session mentions in `a0f51cbc3a`.
  Dez now explains that this capability requires the built-in Dez Agent instead
  of exposing upstream Thread/native-agent terminology. Official Zed retains
  its existing error. Product-copy assertions, formatting, locked offline
  metadata, identity, and diff checks pass; rendered notice proof remains
  deferred.
- 2026-07-23: Protected unsent Agent Session drafts in `94b27cb2db`. Discarding
  a draft through its Session Rail row, keyboard action, hover control, or
  main-area tab now requires a warning that unsent prompt text will be
  permanently removed. Saved Agent Sessions still archive immediately and can
  be restored from Agent History. Pure behavior assertions, formatting, locked
  offline metadata, identity, and diff checks pass; rendered confirmation and
  focus-return proof remain deferred.
- 2026-07-23: Clarified the final pane-boundary controls in `9ef23f920a`.
  Dez's center-tab plus button now says **Add to Main Work Area** and explains
  that it adds a file, search, or terminal. The Agent pane now consistently
  uses **Agent Session** for its current-session header, new-session tooltip,
  title actions, Markdown export, and empty state, while official Zed retains
  Thread vocabulary. Pure copy assertions, formatting, identity, and diff
  checks pass; rendered tooltip/menu proof remains deferred.
- 2026-07-23: Completed the visible Agent context-picker rename in
  `bf76ab9b63`. Dez now shows **Agent Sessions** instead of **Threads** while
  preserving the compatible internal `@thread` keyword and official Zed's
  existing label. Pure vocabulary assertions, formatting, identity, and diff
  checks pass; rendered completion-menu proof remains deferred.
- 2026-07-23: Made Agent Session sandbox scope explicit in `e064c91b53`.
  Per-session policy, disabled-sandbox status, failed-sandbox fallback, and
  unsandboxed execution warnings now say **Agent Session** in Dez while
  internal protocol identifiers and official Zed wording remain stable. Pure
  copy assertions, formatting, locked offline metadata, identity, and diff
  checks pass; rendered tooltip/warning proof remains deferred.
- 2026-07-23: Named the Agent menu's real destinations in `70dd7e12c6`.
  **Agent Settings** opens Agent configuration and **Toggle Session Rail**
  controls the global Session Rail; official Zed retains Settings/Sidebar
  wording. Pure copy assertions and static identity guards pass; rendered menu
  proof remains deferred.
- 2026-07-23: Polished Agent Session switching and search in `4229be7201`.
  The switcher is now a named responsive dialog with recent-session count,
  explicit open/cancel guidance, and preserved modifier-release behavior.
  In-session search says **Search this Agent Session** and exposes a Search
  landmark. Commit `ff1d03b8d3` aligns the deferred completion expectation with
  the **Agent Sessions** context label. Formatting, locked offline metadata,
  identity, and diff checks pass; rendered keyboard/search proof remains
  deferred.
- 2026-07-23: Clarified Agent diagnostic and clipboard notices in
  `07024bf5c0`. Copy/load results, missing-session errors, metadata buffers, and
  Workspace prerequisites now use Agent Session, Workspace, and Session Rail
  vocabulary in Dez while official Zed retains its existing diagnostics.
  Static identity, formatting, and diff checks pass; rendered toast proof
  remains deferred.
- 2026-07-23: Made Session Switcher cancellation source-preserving in
  `67a8152db7`. The original switcher selection now retains whether a terminal
  came from a center Workspace item, durable Host Session, or retained
  compatibility surface; cancelling a preview restores through that same
  route instead of forcing every terminal through Agent. Commit `23ae297ef0`
  adds focused center-terminal source coverage. Formatting, identity, and diff
  checks pass; compilation and live Escape/focus proof remain deferred.
- 2026-07-23: Removed the last ambiguous Agent controls and mixed switcher rows
  in `745792e28e`. Session Switcher rows gained explicit **Agent Session** and
  **Terminal Session** identity; `79e87f2351` later retained that distinction
  in accessibility and tooltips while removing its repetitive visible second
  line. The retained Agent toggle says **Agent** rather than Agent Panel, and
  Session Rail's Agent Tools menu says **Agent Settings**. Product-copy
  assertions, formatting, identity, and diff checks pass; rendered density and
  accessibility-tree proof remain deferred.
- 2026-07-23: Removed the visible upstream default title and finished Agent
  toolbar labeling in `a3cf18ce8e`. The database-compatible `New thread`
  sentinel remains unchanged, while tabs, Session Rail, Agent History,
  mentions, Markdown, completion, and title editing show **New Agent Session**
  in Dez. Title retry now uses a retry-shaped icon; retry, Agent Options,
  full-screen, and new-session controls have explicit accessible names. Pure
  assertions, formatting, locked offline metadata, identity, and diff checks
  pass; rendered title/toolbar proof remains deferred.
- 2026-07-23: Explained unavailable pane splitting in `b42be6e0b2`. Supported
  Surfaces retain the standard Split Pane control; an unsupported Dez Surface
  now exposes **Split Pane Unavailable** and explains that it cannot be split
  or moved into a new pane. Pure copy assertions, formatting, locked offline
  metadata, identity, and diff checks pass; rendered disabled-control proof
  remains deferred.
- 2026-07-23: Clarified two main-work-area controls. Commit `a6ff92643e`
  names the worktree action **Close Worktree from Window**, making its scope
  explicit. Commit `eb603481d5` names the tab-overflow affordance **Switch
  Surface** and its menu **Surfaces**, matching Dez's model of terminals,
  files, search results, and diagnostics as peer Surfaces. Pure copy
  assertions and static identity guards pass; rendered menu proof remains
  deferred.
- 2026-07-23: Made the visible Agent title pencil perform its advertised
  action in `e09bb2d73b`, guarded against regression by `b92e001bb3`. The
  control now starts title editing for both Agent Sessions and terminal
  Surfaces instead of presenting an inert hover affordance. Existing focused
  tests cover both routes; compilation and rendered pointer/focus proof remain
  deferred.
- 2026-07-23: Removed four remaining hard-coded upstream Assistant glyphs from
  Agent controls and notifications in `ab3f8e0408`. The Agent surface toggle,
  Agent Session notifications, Agent diff Surface, and profile controls now
  resolve through the app-aware native Agent icon, preserving Zed identity
  upstream and using Dez's Robot identity in the fork. Static identity guards
  cover all four call sites; rendered icon proof remains deferred.
- 2026-07-23: Productized the Agent Session's exceptional recovery states in
  `8da9d0d694`. Context-too-large and token-limit callouts now describe Agent
  Sessions consistently, their recovery buttons create a new Agent Session,
  and cross-channel import feedback reports Agent Sessions instead of Threads.
  Official Zed retains its upstream vocabulary. Pure copy assertions and
  static identity guards pass; rendered callout/toast proof remains deferred.
- 2026-07-23: Replaced the inherited sparse project-recovery composition in
  `f622dad03d`, with upstream-layout isolation tightened in `b1a20d9b17`.
  Empty Dez Files, Git, and Agent regions now use compact, top-anchored,
  region-specific Workspace guidance, a visible heading, primary/secondary
  action hierarchy, icons, and an accessible region name. Agent zoom now says
  **Expand Agent** and **Restore Agent** rather than implying application
  full-screen mode. Official Zed retains its original copy and centered layout;
  `22893a6491` declares the Git UI's product-identity dependency explicitly.
  Formatting, locked offline metadata, identity, and diff checks pass; rendered
  narrow-pane, zoom, and focus proof remain deferred.
- 2026-07-23: Reduced the Canvas Layout menu from an implementation dashboard
  to an everyday workflow picker. Commit `cdedb6a23a` removes disabled design,
  hosting, history, and prefix-key diagnostics and strips repeated `Canvas:`
  prefixes. Commit `75fb4bc5c9` keeps the six v0.0.1 workflows—Full, Agent
  Control, Focus Editor, Code/Run/Observe, Review, and Debug—while removing the
  uncurated matrix, operations, specialty, and generic geometry catalogue from
  the public menu. Commit `ed5750c0ce` consolidates saved-layout storage behind
  **Save Layout As…** and **Manage Saved Layouts…**, retaining only Cycle and
  Restore Previous as immediate actions. Commit `df8864833d` places that
  curated submenu on the active Workspace's existing, persistent **Workspace
  Options** control in the Session Rail. This avoids another global chrome row
  and leaves official Zed account, organization, sign-in, and sign-out behavior
  untouched. The underlying advanced actions remain available to deliberate
  workflows without dominating routine UI.
  Formatting, locked offline metadata, identity, and diff checks pass; rendered
  menu height, selection, and saved-layout-manager proof remain deferred.
- 2026-07-23: Reframed the interactive agent diff as **Agent Review** in
  `c7f73fb0fe`, with narrow-pane padding and toolbar guards tightened in
  `c782f2fc63`. The Surface and tooltip now name their review purpose; the
  empty state is compact and top-aligned with an explicit **Return to Agent**
  path. User actions say Keep/Reject Change and Keep/Reject All Changes instead
  of exposing hunk jargon. Previous, next, and review-all icons have matching
  accessible names; generation is a visible status, and the unavailable
  per-change reject path for a newly created file explains its limitation.
  Official Zed retains upstream Diff/Hunk copy. Pure copy assertions,
  formatting, locked offline metadata, identity, and diff checks pass;
  rendered hunk navigation, narrow-pane, disabled-tooltip, and focus-return
  proof remain deferred.
- 2026-07-23: Removed remaining tool-implementation terminology in
  `6223c05368`. Dez tool controls and tooltips now say **Files**, **Outline**,
  **Git**, and **Debug** consistently instead of exposing Outline Panel, Git
  Panel, or Debug Panel. Official Zed retains its inherited labels, and the
  Outline crate declares its product-identity dependency explicitly. Static
  identity, formatting, locked offline metadata, and diff checks pass; rendered
  tool-tooltip proof remains deferred.
- 2026-07-23: Made first-use Workspace state explain the real interface model in
  `38524c9c01`. The empty Session Rail now says terminals open in the main work
  area and that only live status and attention appear in the rail. Workspace
  scan progress is an accessible status with product-appropriate vocabulary,
  and automatic trust names the newly opened folder scope, Workspace settings,
  language servers, and configured tools it enables. Official Zed retains its
  Project copy. Formatting, locked offline metadata, identity, and diff checks
  pass; compilation and rendered first-use proof remain deferred.
- 2026-07-23: Removed two remaining leaks of panel implementation terminology
  in `ff63d573ba`. Editor file actions now say **Reveal in Files**, and the Dez
  layout menu remains **Canvas Layout** even if compatibility settings route
  tools through dock-backed panels. Official Zed retains **Reveal In Project
  Panel** and **Panel Layout** where appropriate. Pure copy assertions,
  formatting, locked offline metadata, identity, and diff checks pass;
  compilation and rendered menu proof remain deferred.
- 2026-07-23: Made the intended Dez visual profile survive first launch in
  `67001bf0ef`. Lumin Blur/Lumin Light now follows system appearance in both
  product defaults and the generated user settings file; the stale One
  Dark/One Light and 16/15 px overrides are gone. JetBrains Mono is the bundled
  editor, terminal, prompt/code, Markdown-code, commit-input, and interface
  face. Lumin now preserves
  translucent material without losing pane, focus, selection, active-line, or
  scrollbar hierarchy, and its light variant uses real alpha instead of an
  opaque blur declaration. Static guards cover theme selection, first-run
  settings, every bundled font face, and both upstream licenses. Prettier
  parsing, documentation formatting, Bash syntax, locked offline metadata,
  identity, and diff checks pass; no build or visual launch was performed.
- 2026-07-23: Made terminal termination deliberate and ownership-correct in
  `7664c6e59b`. The action no longer infers ownership from a global local Host,
  which could coexist with an ordinary GUI-owned terminal; the selected
  terminal's own controller now performs the operation. Exited and unavailable
  terminals omit the destructive item, close/detach and terminate occupy
  separate context-menu groups, the ellipsis signals a follow-up, and a
  critical confirmation distinguishes durable Host termination from stopping a
  local shell and foreground process. The terminal emits its single canonical
  close event after termination rather than also emitting a duplicate item
  close. Pure lifecycle assertions and static guards cover availability,
  wording, separation, confirmation, and controller routing. Formatting,
  locked offline metadata, identity, Bash syntax, and diff checks pass; no
  build, test binary, or visual launch was performed.
- 2026-07-23: Corrected repeated Workspace header control identity in
  `e0e8f119e0`. **New Terminal** and **Workspace Options** now include the
  visible Workspace name in their accessibility labels and matching tooltips;
  private hover-group IDs remain presentation-only. Focused pure label
  assertions and the identity gate prevent that implementation detail from
  returning. Formatting, locked offline metadata, Bash syntax, identity, and
  diff checks pass; no build, test binary, or visual launch was performed.
- 2026-07-23: Made pane chrome follow the product's three-region model in
  `1c87220109`. Editor and terminal Surfaces now expose **Main work area** as
  their accessibility landmark instead of the inaccurate **Editor pane**.
  **Add to Main Work Area** remains available when focus moves elsewhere and
  its follow-up commands use ellipses. Workspace Tools and Agent no longer
  inherit main-area creation, split, and zoom controls; each keeps one
  persistent, specifically named hide control. Official Zed retains its
  inherited pane labels and focused-only chrome. Pure copy assertions,
  formatting, Bash syntax, identity, locked offline metadata, and diff checks
  pass; no build, test binary, or visual launch was performed.
- 2026-07-23: Quieted and stabilized the mixed Session Switcher in
  `79e87f2351`. The earlier visible type labels remain in each row's
  accessibility name and tooltip but no longer repeat as visual metadata under
  every Dez title. The list now has a **Recent sessions** landmark and exposes
  its selected row as the active descendant. Pointer hover is visual-only and
  cannot preview or activate a different Workspace; deliberate keyboard cycles
  still preview, and click confirms directly. Official Zed retains visible
  Thread/Terminal labels. Pure copy assertions, formatting, Bash syntax,
  identity, locked offline metadata, and diff checks pass; no build, test
  binary, or visual launch was performed.
- 2026-07-23: Collapsed the remaining first-use Session Rail duplication in
  `d53cd5d656`. The overview now owns the **No sessions yet** status while the
  top-anchored start block only explains the Main Work Area handoff and offers
  one filled **New Terminal** action plus one outlined **Open Workspace…**
  alternative. The repeated **No running sessions / Start a terminal** heading,
  decorative icon card, and folder-scoped label are gone. Both actions expose
  destination-specific accessibility names. Pure copy assertions, formatting,
  Bash syntax, identity, locked offline metadata, and diff checks pass; no
  build, test binary, or visual launch was performed.
- 2026-07-23: Preserved terminal title fidelity through the entire supervision
  path in `e28314a893`. Session Rail, Session Switcher, local Host, durable
  transport, and retained-Agent metadata now receive the full terminal title
  instead of a value already truncated to 25 characters. Tabs and rows still
  fit titles to their own width, leaving tooltips and restored projections
  useful. Custom names trim surrounding whitespace, compare against the full
  live title, and use **Rename Terminal…** while tab double-click continues to
  open the same editor. Pure assertions, formatting, Bash syntax, identity,
  locked offline metadata, and diff checks pass; no build, test binary, or
  visual launch was performed.
- 2026-07-23: Separated compact Session Rail utilities from terminal-focused
  Workspace status in `05df05d282`. When a terminal or other non-editor Surface
  is active, Dez now shows **Search files** beside the existing **No
  diagnostics** state instead of leaving two unexplained glyphs. Editor
  Surfaces retain the compact search icon. The global strip exposes itself as
  **Workspace status and navigation**, while Agent Tools, History, and recent
  Workspaces remain owned by the Session Rail. Official Zed retains its
  upstream icon-only status-bar behavior. Pure copy assertions, formatting,
  Bash syntax, identity, locked offline metadata, and diff checks pass; no
  build, test binary, or visual launch was performed.
- 2026-07-23: Unified terminal lifecycle safety across pointer, context-menu,
  and selected-row keyboard paths in `c83b56b5aa`. Every route now derives its
  label and confirmation requirement from the same terminal source/runtime
  policy, closing the shortcut path that could terminate a live Host-owned
  Session without confirmation. Destructive actions use ellipses, the critical
  prompt says **Terminate Terminal Session?**, names the shell and foreground
  process, and no longer exposes internal “durable session” terminology. The
  mixed underlying compatibility action appears in Dez's Command Palette as
  **Session Rail: Remove Selected Session**, while official Zed retains its
  upstream Agent archive name. The public terminal guide now explains the real
  Main Work Area, Session Rail, Workspace Tools, and Agent ownership model
  instead of the removed Terminal-Thread-in-Agent-Panel flow. Pure assertions,
  static guards, formatting, Bash syntax, identity, locked offline metadata,
  and diff checks pass; no build, test binary, or visual launch was performed.
- 2026-07-23: Removed a polished-looking but dead terminal control in
  `222c658123`. Dez no longer exposes the inherited
  `agent.terminal_init_command` setting because Main Work Area terminals do not
  consume the Agent Panel's Terminal Thread startup path. The stored key and
  official-Zed setting remain intact for migration and upstream synchronization.
  A pure visibility assertion and identity guard prevent the unavailable
  control from returning. Formatting, Bash syntax, identity, locked offline
  metadata, and diff checks pass; no build, test binary, or visual launch was
  performed.
- 2026-07-23: Closed a Session Rail pointer/keyboard parity gap in
  `f0e817669a`. The keyboard-selected active-descendant row now reveals the same
  contextual controls as pointer hover, and Agent/terminal rename, review,
  stop, discard, archive, setup, and close controls enter the tab order while
  visible. Rename mode still suppresses competing row actions. The shared row
  component honors focused action slots, so the behavior cannot be defeated
  after the Sidebar supplies the controls. Pure visibility assertions, static
  guards, formatting, Bash syntax, identity, locked offline metadata, and diff
  checks pass; no build, test binary, or visual launch was performed.
- 2026-07-23: Unified the terminal Surface with Session Rail lifecycle language
  in `7f8fd167c2`. The context menu and critical prompt now say **Terminate
  Terminal Session…** / **Terminate Terminal Session?**, name the shell and
  foreground-process effect, and avoid internal “durable process” vocabulary.
  The confirmation button says **Terminate Session**. The Surface landmark now
  reads **Terminal Session: _title_. Status: _state_**; the tab tooltip correctly
  labels **Working directory**; the task rerun icon has an explicit name; and
  unavailable-session recovery names its **Main Work Area** destination in the
  tab order and tooltip. Pure lifecycle/accessibility assertions, static guards,
  formatting, Bash syntax, identity, locked offline metadata, and diff checks
  pass; no build, test binary, or visual launch was performed.
- 2026-07-23: Made terminal tab metadata read like an inspectable Session in
  `88948b869a`. Ownership now distinguishes **Persistent Terminal Session**,
  **Saved Terminal Session**, and **Workspace Terminal Session**. Tooltip values
  are labeled **Working directory**, **Process ID**, and **Session ID**; the
  rerun icon's accessible name matches its tooltip exactly. Static guards,
  formatting, Bash syntax, identity, locked offline metadata, and diff checks
  pass; no build, test binary, or visual launch was performed.
- 2026-07-23: Decoupled footer legibility from detailed-row density in
  `b43503e3fe`. Dez's default 280 px compact Session Rail now visibly labels
  **Agent Tools**, **History**, and **Workspaces** instead of guaranteeing an
  unexplained three-glyph strip on first launch. Rails narrower than 280 px
  retain icon mode with their existing accessible names and tooltips. A focused
  breakpoint assertion and identity guard keep the compact maximum and utility
  label threshold aligned. Formatting, Bash syntax, identity, locked offline
  metadata, and diff checks pass; no build, test binary, or visual launch was
  performed.
- 2026-07-23: Made the labeled compact footer fit its own breakpoint in
  `ef35bf40a7`. Agent Tools, History, and Workspaces now use compact control
  padding rather than three medium standalone buttons inside the 280 px utility
  strip. Their labels, small typography, icons, accessible names, tooltips, and
  actions remain intact. Static guards cover all three controls. Formatting,
  Bash syntax, identity, locked offline metadata, and diff checks pass; no
  build, test binary, or visual launch was performed.
- 2026-07-23: Made global Session Rail terminal creation destination-explicit
  in `04fd20133b`. The visible action stays the concise **New Terminal**, while
  its accessible name and tooltip say **New Terminal in Main Work Area of
  Active Workspace**. The zero-session explanation names the same active
  Workspace handoff and live Session Rail projection, distinguishing this
  global action from a row-scoped Workspace action and the removed Agent
  terminal path. A source assertion and identity guard preserve that contract.
  Formatting, Bash syntax, identity, locked offline metadata, and diff checks
  pass; no build, test binary, or visual launch was performed.
- 2026-07-23: Replaced the remaining inherited Panels hierarchy in Dez Settings
  with **Workspace Tools & Agent** in `2877de2c3a`. Files, Outline, and Git are
  presented as Workspace Tool surfaces; Agent is presented as its own region;
  Agent font controls are **Agent Typography**. Functional content and
  Workspace-status controls remain available, while dock position,
  dock-specific sizing, and flexible-dock controls are hidden because Dez does
  not expose legacy docks. Official Zed retains every upstream label and
  setting. A pure visibility test and identity guards preserve both product
  branches. Formatting, Bash syntax, identity, locked offline metadata, and
  diff checks pass; no build, test binary, or visual launch was performed.
- 2026-07-23: Finished the reachable Agent Session content terminology pass in
  `5e6c925fb5`. Add Context now offers **Agent Sessions** while retaining the
  compatible `@thread` insertion key. Edit-restart, feedback disclosure, and
  Markdown export copy all name the Agent Session in Dez; official Zed retains
  Thread language. The feedback tooltip explicitly states that rating a
  response sends the current Agent Session to the upstream agent service. An
  identity guard covers all four surfaces. Formatting, Bash syntax, identity,
  locked offline metadata, and diff checks pass; no build, test binary, or
  visual launch was performed.
- 2026-07-23: Verified that the screenshot's check/time prompt and blue mascot
  are PTY output, not Dez overlays: the terminal Surface renders the terminal
  grid and only adds an unavailable-session recovery callout. Current source
  already keeps a terminal tab header visible even when single-tab auto-hide is
  enabled (`fb2b90e193`), preventing shell output from being mistaken for
  application chrome. Commit `7fbad3934e` adds identity guards across the Item
  contract, pane policy, and Terminal override. Bash syntax, identity, locked
  offline metadata, formatting, and diff checks pass; no build, test binary, or
  visual launch was performed.
- 2026-07-23: Finished the reachable Agents settings vocabulary and disclosure
  pass in `83b1c59e07`. Dez now describes a subagent's parent **Agent Session**,
  focused-review **Surfaces**, Agent edit and terminal cards, and **Workspace
  status** without exposing Thread, buffer, Panel, or generic status-bar
  implementation terms. Feedback settings now disclose that rating sends the
  current Agent Session to the upstream agent service. Official Zed retains its
  upstream wording. A pure product-copy test and identity guards cover the
  boundary. Formatting, Bash syntax, identity, locked offline metadata, and
  diff checks pass; no build, test binary, or visual launch was performed.
- 2026-07-23: Made the Session Switcher globally discoverable without stealing
  the editor's conventional `Ctrl-Tab` tab-switching behavior in
  `2690f62f75`. Agent and Session Rail contexts retain direct `Ctrl-Tab` /
  `Ctrl-Shift-Tab` session cycling. Everywhere else, the Command Palette now
  presents the compatibility action as **Session Rail: Switch Sessions**
  instead of **Toggle Thread Switcher**. Its action documentation names the
  real open-or-cycle behavior, and a product-copy assertion plus identity guard
  preserve the mapping. Formatting, Bash syntax, identity, locked offline
  metadata, and diff checks pass; no build, test binary, or visual launch was
  performed.
- 2026-07-23: Finished the Session Switcher's interaction guidance and
  collection semantics in `d08a9697a9`. Held-shortcut mode now says to continue
  cycling and release to open; direct command mode says to repeat, press Enter
  to open, or Escape to return. The same explanation is attached to the dialog
  for assistive technology, the footer is an explicit status, and every mixed
  Terminal Session / Agent Session row reports its one-based position and total
  collection size while preserving the existing active-descendant selection.
  Reusable `ThreadItem` semantics and identity guards protect the contract.
  Formatting, Bash syntax, identity, locked offline metadata, and diff checks
  pass; no build, test binary, or visual launch was performed.
- 2026-07-23: Removed the remaining reachable compatibility-Panel vocabulary
  from Workspace Tool actions in `738df46829`. Dez's Command Palette now
  presents Outline, Git, Debug, and Agent namespaces directly; official Zed
  keeps its inherited `*_panel` names. Outline's no-content guidance says
  **Toggle Outline With**, and Git review hands comments to **Agent**, not an
  invisible Agent panel. Product-branch assertions and identity guards cover
  all three paths. Formatting, Bash syntax, identity, locked offline metadata,
  and diff checks pass; no build, test binary, or visual launch was performed.
- 2026-07-23: Rebuilt the Dez Outline empty/search state in `03daa99d53`.
  Instead of one centered **No outlines available** placeholder, it now
  distinguishes no filter matches, no Workspace search results, no symbols in
  the active file, and no file to outline. The compact state follows the
  top-aligned Workspace Tool hierarchy, exposes an explicit accessibility
  status, labels the current filter, and provides **Clear Filter** as a direct
  recovery action. Official Zed retains its upstream presentation. Pure copy
  assertions and an identity guard protect the complete state matrix.
  Formatting, Bash syntax, identity, locked offline metadata, and diff checks
  pass; no build, test binary, or visual launch was performed.
- 2026-07-23: Replaced Git's centered **No changes to commit** placeholder with
  an inspectable clean-repository state in `f883f2f8a0`. Dez now presents a
  success-marked **Working tree clean** status, names the current branch in its
  explanation, and only offers **Review Branch Changes** on non-main branches.
  That full-width action explains that it compares committed work with the base
  branch. The state is top-aligned and accessible; official Zed retains its
  upstream presentation. Pure branch-copy assertions and an identity guard
  protect the contract. Formatting, Bash syntax, identity, locked offline
  metadata, and diff checks pass; no build, test binary, or visual launch was
  performed.
- 2026-07-23: Rebuilt Git's opened-Workspace/no-repository state in
  `579e7448f7`. Dez now says **No repository in this Workspace**, explains that
  Git initialization starts tracking the open folder, and presents one
  full-width primary **Initialize Repository** action with Git-specific
  identity and tooltip. The setup region follows the same top-aligned compact
  hierarchy as other Workspace Tools and has an explicit accessibility name.
  Official Zed retains its centered upstream state. A pure copy assertion and
  identity guard protect the fork boundary. Formatting, Bash syntax, identity,
  locked offline metadata, and diff checks pass; no build, test binary, or
  visual launch was performed.
- 2026-07-23: Reworked Git's dubious-ownership recovery into an explicit
  security decision in `9c71b58d64`. Dez now presents a top-aligned
  **Repository ownership needs review** alert, explains why Git blocked the
  repository, shows the exact path in a labeled inspectable chip, and states
  that approval changes global Git configuration. **Trust This Directory** uses
  warning styling, an accessible effect description, and an exact command
  tooltip; **Learn About Safe Directories** remains a separate secondary path.
  Official Zed retains its upstream state. Pure copy assertions and an identity
  guard protect the boundary. Formatting, Bash syntax, identity, locked offline
  metadata, and diff checks pass; no build, test binary, or visual launch was
  performed.
- 2026-07-23: Rebuilt the idle Debug experience in `e2b237025d`. Dez now names
  the region and pane tab **Debug**, presents a top-aligned **Start debugging**
  hierarchy, explains that controls, variables, call stack, console, and
  breakpoints stay together in the Workspace, and makes **Start Debug Session**
  the primary action. **Configure debug.json**, **Documentation**, and **Debug
  Adapters** have explicit supporting roles. The empty breakpoint area now says
  **No breakpoints yet** and points to the editor gutter. Both states have named
  accessibility regions/statuses; official Zed retains its upstream Debugger
  presentation. Pure copy assertions and identity guards protect the product
  branch. Formatting, Bash syntax, identity, locked offline metadata, and diff
  checks pass; no build, test binary, or visual launch was performed.
- 2026-07-23: Removed the duplicated fake-terminal recovery presentation for
  unavailable Terminal Sessions in `8dc53f4077`. A failed reconnect now
  preserves the original custom terminal title, hides the inert cursor, leaves
  the PTY grid free of synthetic recovery output, and reports the actual Host,
  process, or saved-reference failure in one **Terminal Session unavailable**
  warning. Dez still starts no replacement process automatically. The recovery
  action is now **Start Fresh Terminal**, names its Main Work Area destination,
  and explains that it creates separate computation rather than reattaching.
  Pure description assertions and identity guards cover title, reason, cursor,
  blank recovery grid, and action semantics. Formatting, Bash syntax, identity,
  locked offline metadata, and diff checks pass; no build, test binary, or
  visual launch was performed.
- 2026-07-24: Closed the remaining Terminal Session rename parity gap in source.
  Session Rail terminal rows now respond to the selected-row rename command,
  expose a focusable pencil at normal widths, retain context-menu access at
  every width, and edit inline without replacing live shell or agent status
  prefixes. The shared Command Palette entry now says **Session Rail: Rename
  Selected Session** instead of describing only Agent Sessions. The custom title
  updates Workspace- and Agent-owned terminal Surfaces immediately and persists
  for detached Host-owned Sessions. Blank input or the unchanged live title
  resets the override. Compact rows omit the extra pencil so close, detach,
  review, and setup actions keep their space. Focused behavior assertions and
  identity guards cover the contract. This is a source-only slice; no build,
  test binary, alternate binary, or visual launch was performed.
- 2026-07-24: Started the installed-runtime UX recovery in source. The Dez
  welcome screen now explains one release-defining loop—**Run**, **Supervise**,
  then **Review**—with **Open Workspace** as the primary entry and **Open
  Scratch Terminal** as the explicit projectless path. Git tabs now say **Git
  Changes** and **Git History**, removing the collision with Agent History.
  Terminal ownership wording is now evidence-based: **Persistent** appears only
  when the active external terminal Host owns the exact saved snapshot;
  GUI-owned computation is a **Workspace Terminal Session**. Official Zed keeps
  its upstream presentation. Identity guards cover all three product
  boundaries. This remains source-only and is not present in the installed
  `91a1514` artifact.
- 2026-07-24: Repaired two P0 lifecycle contradictions found in the installed
  runtime. Session Rail now synthesizes a transient empty-Workspace group for
  a Scratch Terminal, so the visible live shell is projected even before a
  project is opened. Application shutdown now permits two seconds for
  concurrent PTY cleanup and session-state database writes; the former 200 ms
  aggregate deadline produced normal `app_will_quit` timeouts and risked
  incomplete restore state. Identity guards protect both contracts. These are
  source-only repairs and still require a new artifact plus close/reopen smoke
  verification.
- 2026-07-24: Made the Session Rail zero state restore-aware. While Workspace
  restoration and saved Terminal Session reconciliation are pending, the rail
  now reports **Loading sessions** instead of **No sessions yet** or **No active
  sessions**. It withholds the true-empty **New Terminal** action until
  restoration reaches ready, avoiding duplicate computation during startup.
  Pure state-copy assertions and identity guards protect the contract. This is
  source-only and is not present in the installed artifact.
- 2026-07-24: Replaced the opaque `missing serialized agent thread item`
  startup error with typed stale-tab recovery. When an Agent Session tab points
  to metadata that no longer exists, Dez omits only that tab and shows one
  persistent notice directing the user to Agent History for any recoverable
  Session. The next Workspace serialization heals the saved layout; unrelated
  deserialization errors remain visible as errors. Identity guards cover the
  typed boundary and notice. This is source-only pending a new artifact.
- 2026-07-25: Connected the active terminal to the IDE in source. Every
  standalone Dez terminal now owns a compact, named toolbar that combines
  authoritative lifecycle, Workspace/branch, and Git changed-file count with
  direct **Files**, **Review Changes**, and **Session Details** actions. Git
  state refreshes from structured Git-store events; arbitrary terminal text is
  never treated as evidence. Session Rail terminal rows expose the same review
  and details vocabulary. Workspace Tools and Agent panes now start at the
  smaller of 360 px or 22% of available width instead of splitting the canvas
  equally. Pointer and keyboard resizing, visibility changes, layout recipes,
  and persisted restoration enforce a combined 40% contextual-region ceiling,
  preserving at least 60% for the Main Work Area. Restored Agent panes also
  retain Agent ownership instead of decoding as ordinary tabs. Focused
  assertions and identity guards cover these contracts. This remains
  source-only; no build or alternate binary was launched.
- 2026-07-25: Replaced the competing-column shell with responsive drawers in
  source. Revealing Workspace Tools closes Agent and revealing Agent closes
  Workspace Tools at every window size; ultrawide space belongs to the Main
  Work Area instead of a second persistent contextual column. Window
  restoration and resizing apply the same policy. The terminal handoff is a
  32 px tab-bar-aligned header without a duplicate visible actor title, and the
  supervisor's visible heading is **Sessions** instead of internal Session Rail
  jargon. Static guards and focused source assertions cover the contract. No
  build, test binary, alternate binary, or visual launch was performed.
- 2026-07-25: Bounded transient feedback in source. Workspace notifications no
  longer occupy a fixed 448 px, full-height overlay; one named shelf derives
  width from the actual Main Work Area, stays within 280–420 px when space
  permits, caps height at 42%, and scrolls overflow. Toasts no longer allocate
  an invisible full-screen layer and bound visible content to 90% width and
  42% height. Welcome now teaches Run, Supervise, and Review in one coherent
  panel, empty drawers return to the Main Work Area with inward arrows, and New
  File uses its object icon. Static guards and source assertions cover the
  contract. No build, test binary, alternate binary, or visual launch was
  performed.
- 2026-07-25: Made first-run guidance and terminal surface naming contextual.
  With no project, Welcome leads with **Open Workspace** and **Open Scratch
  Terminal**. Once a Workspace exists, it stops repeating that request and
  instead leads with **Start Terminal Session** and **Open Files**, with
  Workspace-specific Run copy. Meaningfully titled Dez terminal tabs now use
  **Terminal · title** so the Main Work Area identifies the surface before its
  content; official Zed keeps its inherited titles. Focused assertions and
  identity guards protect both product branches. This is source-only; no build
  or visual launch was performed.
- 2026-07-25: Unified the selected Session Rail handoff in source. Every
  supported desktop keymap now exposes Shift+Enter to return to the existing
  selected Session, Shift+F to open its Workspace files, Shift+G to open the
  ownership-appropriate Agent or Git change review, and Shift+V to open Session
  details. The Command Palette uses the same Session Rail vocabulary. The
  implementation activates the existing Terminal or Agent Session and its
  Workspace before opening IDE context, so the workflow creates no duplicate
  terminal or project. This remains source-only; no build or visual launch was
  performed.
- 2026-07-25: Removed release-version drift from the active product story. The
  application manifest, README, public product guide, roadmap objective, and
  active plan now agree that the current source train is v0.0.2. v0.0.1
  documents are explicitly historical evidence. The stale task list that still
  requested the already-complete upstream merge now records the actual
  sequence: finish source interaction work, freeze a checkpoint, build that
  exact source, prove the hero/restart/recovery/visual matrices, verify
  coexistence, and only then sign and publish. This is documentation and guard
  work only; no build or visual launch was performed.
- 2026-07-25: Completed the source-side terminal-to-diff transition. Every
  Terminal Session **Review Changes** entry point now dispatches one
  Workspace-owned Git action. It reveals Git Changes, selects a concrete
  changed file when available, and opens the uncommitted diff in the Main Work
  Area; a clean repository stays on the truthful clean state. This replaces the
  previous pair of toggles that could close an already-focused drawer or show a
  changes list without an actual review surface. Command Palette vocabulary and
  identity guards cover the new contract. No build or visual launch was
  performed.
- 2026-07-25: Repaired the primary missing-accessibility-node paths recorded by
  the installed runtime audit. Populated and empty Files, Outline, Git
  Changes/History, Debug, Agent, native and external Agent Sessions, Agent
  History, and terminal failure/panel roots now expose an element ID, semantic
  role, product-specific label, and the same focus handle they track. Static
  guards protect every owner. This is source evidence only; the rebuilt
  artifact must still prove that its accessibility log remains clean through
  the keyboard workflow.
- 2026-07-25: Preserved product orientation after first activation without
  adding more shell chrome. The terminal's existing **Session Details**
  disclosure now contains **How Dez Works**: run computation in the Terminal
  Session, supervise live state and attention in Session Rail, then review the
  same Workspace through Files and Git. The trigger names both purposes for
  assistive technology and pointer users. This is source-only; no build or
  visual launch was performed.
- 2026-07-25: Removed the Scratch Terminal's post-activation dead end. When no
  codebase is attached, its context strip now offers **Open Workspace** instead
  of hiding Files/Git with no direct next step. The action forces a same-window
  open, preserving the running Terminal Session while the selected codebase
  gains Files and Git review. The collapsed guide says **Connect** until that
  context exists. This is source-only; no build or visual launch was performed.
- 2026-07-25: Made the terminal-to-review destination self-identifying. Dez
  Project Diff tabs now render **Diff · filename** for the active reviewed file
  instead of the generic inherited **Uncommitted Diff**. Hover detail preserves
  the diff base and relative path, and official Zed keeps its upstream
  vocabulary. Focused source assertions and an identity guard cover both
  product branches. This is source-only; no build or visual launch was
  performed.
- 2026-07-25: Made Terminal Session provenance visible without adding permanent
  chrome. **Session Details** now includes a compact **Evidence** section that
  distinguishes Terminal/Host lifecycle, Workspace-owned Git counts,
  non-inferred Session attribution, adapter-gated agent confidence/checks, and
  untrusted terminal prose. The public product and terminal guides use the same
  contract, and the identity check protects the exact disclosure. This is
  source-only; no build or visual launch was performed.
- 2026-07-25: Removed toggle semantics from the terminal-to-Files handoff.
  Terminal context controls, Session Rail selected-row keyboard actions, and
  terminal context menus now dispatch one Project Panel **Reveal** action that
  opens and focuses Files idempotently. Repeating **Files** no longer closes an
  already-visible destination. The Command Palette names the new action
  **files: open**, and guards reject the old toggle path. This is source-only;
  no build or visual launch was performed.
- 2026-07-25: Rebuilt Lumin's region hierarchy instead of tweaking isolated
  colors. Lumin Blur, its opaque fallback, and Lumin Light now distinguish the
  rail/drawer, Main Work Area, tab strip and active tab, terminal, and elevated
  overlay layers. Neutral structural dividers clear a measured 1.5:1 floor
  after translucent compositing, while focus keeps the stronger 3:1 floor.
  Theme checks now reject collapsed surfaces and faint boundaries. This is
  source-only; no build or visual launch was performed.
- 2026-07-25: Normalized Lumin interaction feedback across blurred dark,
  opaque dark, and light modes. Hover is visibly distinct; active and selected
  are stronger; scrollbars progress from idle to hover to drag; and the active
  editor line is never transparent. The theme guard measures each state on its
  actual composited panel/editor surface and checks the ordering as well as the
  floor. This is source-only; no build or visual launch was performed.
- 2026-07-25: Hardened the terminal context strip for compact panes. Its
  lifecycle/repository cluster is now explicitly overflow-hidden inside the
  shrinkable region, while Files, Review Changes, Open Workspace, and Session
  Details remain fixed-priority actions. At narrow widths their labels now
  yield to the same semantic icons with full accessible names and matching
  tooltips. Hidden metadata remains available in Session Details and in the
  toolbar's accessible name. The identity guard protects this layout order.
  This is source-only; no build or visual launch was performed.
- 2026-07-25: Made the Scratch Terminal's **Open Workspace** contract exact.
  It now dispatches a dedicated folder-only action rather than the inherited
  file-or-directory picker, allows multiple Workspace folders, and forces them
  into the current window so the running Terminal Session survives. Static
  guards cover the action definition, picker policy, and terminal route. This
  is source-only; no build or visual launch was performed.
- 2026-07-25: Unified start-state navigation around truthful destinations.
  **Open Files** is now a Workspace-owned action shared by Welcome, Terminal,
  and Session Rail; its Project Panel implementation always reveals and focuses
  Files, so repeated activation cannot close Workspace Tools or expose a
  different retained tool. Welcome and the zero-session rail use the
  folder-only, same-window **Open Workspace** route. The true-empty terminal
  action is explicitly **Open Scratch Terminal**, and all start-fresh actions
  remain withheld while App Session restoration is pending. Focused assertions
  and identity guards cover the new contracts. This is source-only; no build or
  visual launch was performed.
- 2026-07-25: Completed a first-party product-icon isolation pass. A shared
  resolver now preserves Zed's Agent and Assistant marks only for official Zed,
  while Dez uses Robot for Agent identity and Sparkle for Inline Assist.
  Editor, terminal, diagnostics, Git branch/diff/conflict, settings, and setup
  controls use the neutral grammar. Agent creation uses Robot and its registry
  uses Blocks instead of a generic plus. Static guards reject direct inherited
  Assistant marks in those production surfaces. This is source-only; no build
  or visual launch was performed.
- 2026-07-25: Aligned visual hierarchy with keyboard and accessibility
  hierarchy. Dez Welcome now has one filled recommended transition determined
  by Workspace state; the remaining actions stay secondary. Git, Debug,
  Outline, and Agent toolbar controls are keyboard-focusable and explicitly
  named. Debug uses Debug/Stop instead of generic Plus/Power, and Agent title
  editing stays visible in Dez rather than hiding behind pointer hover.
  Official Zed compatibility behavior remains unchanged. This is source-only;
  no build or visual launch was performed.
- 2026-07-25: Removed the empty-app pathless-terminal trap. Session Rail now
  matches Welcome by making **Open Workspace** its filled first action and
  keeping **Open Scratch Terminal** secondary. When a Workspace is ready but
  has no Sessions, **Start Terminal Session** becomes the filled recovery.
  Restoration status now overrides stale attention color/icon state, so
  **Loading sessions** cannot present as a warning. This is source-only; no
  build or visual launch was performed.
- 2026-07-25: Closed a Session Rail keyboard/pointer parity gap. Keyboard focus
  now keeps a Workspace's New Terminal and Options controls visible, and both
  controls are explicit tab stops. An already-open Workspace menu keeps its
  scoped close controls visible instead of requiring a second hover. Session
  search clearing and onboarding-banner dismissal also enter the tab order.
  Static guards cover the focus, visibility, naming, and tab-order contract.
  This is source-only; no build or visual launch was performed.
- 2026-07-25: Normalized the Main Work Area's pane-control row. Back, Forward,
  Add, Switch Surface, Split, Zoom, Hide Workspace Tools, and Hide Agent are now
  explicit tab stops with their existing accessible destination names. The
  active unpinned Dez Surface keeps Close visible and keyboard-focusable under
  the hover-close preference; inactive tabs remain quiet and pinned dirty tabs
  retain their status indicator. Official Zed retains upstream tab-close
  behavior. This is source-only; no build or visual launch was performed.
- 2026-07-25: Found the concrete reason the installed screenshots did not show
  the promised typography or glass theme: the machine's older generated Dez
  profile pinned `.ZedSans`, `"mode": "light"`, and One Light. Expanded the
  narrowly scoped migration to recognize that exact generated signature and
  upgrade UI font, appearance mode, and light theme together while preserving
  all unrelated settings. Official Zed and arbitrary custom profiles remain
  excluded. The local generated profile was aligned directly to JetBrains Mono
  and system-selected Lumin Light/Lumin Blur without a build or app launch.
- 2026-07-25: Corrected Session Rail creation priority. A ready Workspace with
  no Session now gets a filled, explicitly scoped **Start Terminal Session**
  action. Once Sessions exist, the overview's **New Terminal** control remains
  visible but becomes outlined so repeat creation does not compete with the
  Session list, attention, and review. This is source-only; no build or visual
  launch was performed. The 2026-07-26 populated-state ownership slice below
  removes that remaining duplicate from Dez.
- 2026-07-25: Removed two persistent healthy-state labels from terminal-focused
  Workspace status. Search remains a keyboard-focusable icon named **Search
  Workspace Files**; zero diagnostics remains a check icon announced as
  **Workspace diagnostics: no problems**. Real errors, warnings, counts, and
  messages remain visible. This reduces the bottom-bar prose visible in the
  installed screenshot without sacrificing discoverability or accessibility.
  This is source-only; no build or visual launch was performed.
- 2026-07-25: Verified that Lumin Blur/Light already reaches the native macOS
  whole-window blur path, then added a discoverable **Restore Dez Visual
  Profile** action to Settings and the command palette. It restores the
  system-selected Lumin pair, bundled JetBrains Mono roles, and built-in Dez
  icons; preserves font sizes and non-visual settings; awaits persistence
  before showing success; and has a focused settings-mutation test. First-run
  settings now explicitly name the Dez icons and Markdown code font too. This
  is source-only; no build or visual launch was performed.
- 2026-07-25: Closed the remaining Main Work Area width-budget escape hatch.
  **Reset Pane Sizes** previously equalized Workspace Tools/Git and a terminal
  to 50/50 after every other reveal, resize, restore, and recipe path had been
  constrained. Reset now reapplies the Dez auxiliary-pane budget, and a focused
  two-pane assertion protects the exact Git-plus-terminal screenshot shape.
  This is source-only; no build or visual launch was performed.
- 2026-07-25: Disambiguated the Session Rail's responsive footer navigation.
  Detailed rails now name **Agent History** and **Recent Workspaces**, while
  the default compact rail retains the shorter **History** and **Workspaces**
  labels and genuinely narrow rails retain named, tooltip-backed icons. This
  separates agent-run history from Git history without widening compact chrome
  or changing any destination behavior. Focused helper assertions and static
  identity guards protect all three width states. This is source-only; no
  build or visual launch was performed.
- 2026-07-25: Replaced the Session Rail's generic provider glyphs with the
  bundled Codex/OpenAI, Claude, Gemini, OpenCode, Grok, Copilot, and Cursor
  marks. Providers without a specific bundled mark retain the neutral Robot
  fallback. The Session Rail now owns one icon mapping shared by the keyboard
  session switcher, preventing the two navigation surfaces from drifting.
  Focused assertions and identity guards cover the provider and fallback
  contracts. The screenshot mascot remains verified PTY content, not Dez
  decoration. This is source-only; no build or visual launch was performed.
- 2026-07-25: Repaired the multi-repository terminal-to-review contract. The
  terminal context count already aggregated the whole Workspace, but Review
  Changes inspected only the active repository and could therefore advertise
  work before opening no diff. Review now keeps an active dirty repository or
  deterministically activates the first dirty repository, synchronizes Git
  Changes, and opens an actual changed-file diff. A pure routing assertion and
  static ownership guard cover active-dirty, active-clean, no-active, and
  all-clean cases. This is source-only; no build or visual launch was
  performed.
- 2026-07-25: Rebalanced Git Changes around review instead of permanent commit
  composition. The collapsed commit editor now reserves four lines rather than
  six, while the existing full-height and modal expansions remain explicit.
  View Diff, Stage/Unstage All, Commit, remote actions, and every split-menu
  chevron are now tab stops with specific accessible names; chevrons expose
  matching tooltips and expanded state. Static guards cover both the density
  and interaction contracts. This is source-only; no build or visual launch
  was performed.
- 2026-07-25: Made Agent recovery match the product promise. The no-Workspace
  Agent state now opens folders in the current Dez window instead of invoking
  an inherited file-or-folder picker. Shared Open Workspace and Clone
  Repository recovery buttons now enter the keyboard tab order. Agent Options
  and New Agent Session triggers expose expanded state and keep selected
  treatment while their popovers are open. Static guards cover routing,
  keyboard reachability, and popup state. This is source-only; no build or
  visual launch was performed.
- 2026-07-25: Unified the Agent composer's primary controls. Expand/Minimize,
  Add Context, Follow, Fast Mode, Thinking Mode, thinking effort, Send/Queue,
  Stop Agent Run, and Sandbox Settings now enter the keyboard tab order and
  expose specific accessible names. Toggles report state, popovers report
  expanded state and values, and selected treatment supplements icon color.
  Static guards cover the complete control row. This is source-only; no build
  or visual launch was performed.
- 2026-07-25: Repaired Agent follow-up interaction hierarchy. Response,
  navigation, and feedback actions are no longer dimmed as a decorative row
  and now expose keyboard names and selected feedback state. Queued prompts are
  a named ordered list with visible, keyboard-reachable Remove, Edit, Steer,
  and Send Now actions and row-specific status identifiers. Permission
  decisions enter the tab order; Permission Scope reports value, expanded
  state, and open treatment. Static guards cover all three contracts. This is
  source-only; no build or visual launch was performed.
- 2026-07-25: Made Agent Review a persistent IDE workflow. Per-file Review,
  Reject, and Keep actions remain visible, use row-specific identity, enter the
  tab order, and explain pending-edit disablement. Review Changes, Reject All,
  Keep All, Stop Subagent, and Return to Parent Agent Session are specifically
  named keyboard controls. Restore Checkpoint now warns that it replaces
  Workspace files and requires Restore Files confirmation before targeting the
  exact checkpoint. Message-edit cancel/restart controls are named tab stops.
  Static guards cover visibility, ownership, focus, and confirmation. This is
  source-only; no build or visual launch was performed.
- 2026-07-25: Unified Agent errors and recovery around explicit actions.
  Provider retry copy now identifies Dez instead of Zed. Retry,
  authentication, configuration, model selection, plan clearing, security
  settings, skill files, environment recovery, updates, dismissals, and the
  shared Copy control are named keyboard tab stops. Skill rows name their file.
  Provider data-retention Accept now requires a warning that discloses the
  persistent setting, Anthropic log retention, and request retry before
  **Accept and Retry** can run. Static guards cover identity, focus, naming,
  and consent sequencing. This is source-only; no build or visual launch was
  performed.
- 2026-07-25: Made Agent tool cards self-explanatory without hover discovery.
  Tool-card disclosures are explicit keyboard tab stops with
  context-specific names and expanded state. The shared Disclosure component
  now supports that opt-in contract without turning inherited hover-only
  disclosures into invisible tab stops. Copy Code, Copy Command, thinking,
  terminal-output, and general tool-output controls remain visible at rest.
  Running-command Stop, sandbox help, interrupted-edit discard, and Open File
  are named keyboard actions. Truncation and command failure are status rather
  than inert IconButtons. Subagent preview, Stop, and the labeled Open Subagent
  Session action retain distinct exact-Session semantics. Static guards cover
  visibility, roles, naming, focus, and ownership. This is source-only; no
  build or visual launch was performed.
- 2026-07-25: Made the true-empty Session Rail explain the product integration
  directly. **Start with a Workspace** now states that a codebase feeds
  Terminal or Agent Sessions and that their changes return to the IDE for
  review. Open Workspace remains the filled same-window, folder-only action;
  Open Scratch Terminal remains secondary. Start, search recovery, attention,
  and All/Attention scope controls now enter the keyboard tab order. A stale
  embedded assertion for the superseded terminal-first empty state was aligned
  with the canonical copy. Static guards cover hierarchy, routing, focus, and
  exact Main Work Area destinations. This is source-only; no build, test
  binary, or visual launch was performed.
- 2026-07-25: Polished Session Rail recovery notices without weakening their
  truth boundary. Terminal Session connecting, reconnecting, and unavailable
  notices retain explicit no-shell/no-replacement/process-untouched semantics;
  Open Local Log and Copy Details/Error are named tab stops. Workspace restore
  actions are also tab stops, and ambiguous Remove Entry copy is now **Remove
  Recovery Entry**, with both notice and tooltip stating that only the rail
  record is removed while recent Workspace data remains. Static guards cover
  wording, focus, diagnostics, and exact removal scope. This is source-only; no
  build, test binary, or visual launch was performed.
- 2026-07-25: Closed the remaining secondary-window visual-identity bypasses.
  About and Copilot verification now request the active Lumin native window
  material instead of GPUI's opaque default; both also initialize the
  configured UI font, as Audio Test now does. Intentionally shaped Agent and
  collaboration notification popups retain transparent outer windows and
  their existing configured-font setup. Static guards protect the stable
  window versus shaped-popup distinction. This is source-only; no build, test
  binary, alternate binary, or visual launch was performed. The 2026-07-26
  attention-overlay slice below keeps the Agent shape available but makes it an
  explicit Dez opt-in.
- 2026-07-25: Tightened the first-work hierarchy at both Main Work Area entry
  points. Welcome now exposes exactly three primary-section transitions per
  state: Open Workspace, Clone Repository, and Open Scratch Terminal without a
  codebase; Start Terminal Session, Open Files, and New File inside one.
  Command-palette and Workspace-replacement utilities remain in normal chrome
  instead of competing with startup. The empty Main Work Area now uses Start
  Terminal Session consistently and no longer describes the default GUI-owned
  terminal as durable. Static guards cover count, order, routing, wording, and
  ownership truth. This is source-only; no build, test binary, alternate
  binary, or visual launch was performed.
- 2026-07-25: Separated reachable Dez Agent-provider onboarding from dormant
  upstream Zed AI subscription components. The setup card is now a named
  accessibility region, consistently says Agent Session, and exposes
  **Configure Agent Providers** or **Start Agent Session** as specifically
  named keyboard tab stops. A guard prevents the Agent entry path from
  importing Zed AI trial/plan onboarding. This is source-only; no build, test
  binary, alternate binary, or visual launch was performed.
- 2026-07-25: Rebuilt the native Dez shell material hierarchy around Lumin
  glass. The macOS stable window now uses the native under-window backdrop,
  behind-window blending, and active/inactive system-state tracking. Lumin Light
  controls are translucent interaction layers guarded after compositing. Sessions
  and Workspace Tools use panel material; the Main Work Area keeps
  editor/terminal material; empty work cards use surface material; feedback uses
  elevated surface material. Workspace notifications are bounded to a top Main
  Work Area shelf, toasts clear the status bar without a full-screen invisible
  layer, and glass feedback drops modal-grade shadows. Public copy now centers
  **Sessions** and **Start Terminal Session** while preserving official-Zed
  compatibility branches. Static identity and theme guards cover the contract.
  This is source-only; no build, test binary, alternate binary, or visual launch
  was performed.
- 2026-07-25: Made the empty Main Work Area explain the Dez route without adding
  another overlay. The existing **Run. Supervise. Review.** launch panel now has
  compact chips for **Run -> Main Work Area**, **Supervise -> Sessions**, and
  **Review -> Files + Git**. The row is a named accessibility list, uses
  ordinary element material, and remains inside the bounded surface card. Static
  guards protect the route. This is source-only; no build, test binary,
  alternate binary, or visual launch was performed.
- 2026-07-25: Gave the true-empty Sessions rail the same concrete route in a
  narrow vertical list: **Run -> Main Work Area**, **Supervise -> Sessions**,
  and **Review -> Files + Git**. This keeps the rail honest about its job before
  any Terminal or Agent Session exists, preserves **Open Workspace…** as the
  primary action, and keeps **Open Scratch Terminal** visibly transient. Static
  guards and source assertions protect the copy and material. This is
  source-only; no build, test binary, alternate binary, or visual launch was
  performed.
- 2026-07-25: Quieted the default focus chrome that matched the installed
  screenshots' orange full-pane outlines. Dez now ships `active_pane_modifiers`
  with `border_size: 0.0` and `inactive_opacity: 1.0`, so focus defaults to the
  title/selected-tab cue without dimming sibling panes. Full-pane focus borders
  and inactive-pane dimming remain opt-in user settings. Static guards pin this
  default. This is source-only; no build, test binary, alternate binary, or
  visual launch was performed.
- 2026-07-26: Removed more public **Session Rail** vocabulary from Dez-facing
  UI paths. Command Palette selected-session actions now display under
  `sessions: ...`, the Agent metadata debug buffer says **All Sessions Agent
  Session Metadata**, and bundled settings/schema comments describe **Sessions**
  while retaining compatible `session_rail` keys internally. Static guards pin
  the public/internal boundary. This is source-only; no build, test binary,
  alternate binary, or visual launch was performed.
- 2026-07-26: Made Terminal Session handoff labels survive normal split-pane
  widths. The terminal context strip now keeps **Files**, **Review Changes**,
  **Open Workspace**, and **Session Details** labels visible down to 560 px
  instead of collapsing at 700 px, so the terminal reads as an IDE-integrated
  Surface instead of a raw shell with mystery icons. Static guards pin the
  threshold. This is source-only; no build, test binary, alternate binary, or
  visual launch was performed.
- 2026-07-26: Removed the Sessions-row action mask and terminal icon cluster.
  Shared Session rows now truncate titles within layout instead of painting
  gradient fades over their content. A normal-width Terminal Session exposes
  one readable handoff: **Review** when its Workspace has changes, otherwise
  **Details**. Rename, Codex hook setup, and other secondary utilities remain
  in the existing Terminal Session context menu; terminate/close remains the
  only adjacent lifecycle control. Static guards pin the 280 px label
  threshold, the single-handoff priority, and the no-gradient row contract.
  This is source-only; no build, test binary, alternate binary, or visual
  launch was performed.
- 2026-07-26: Reduced Agent Session rows to the same one-action hierarchy.
  Runtime state now decides the only inline action: **Stop** for a running or
  permission-blocked run, **Discard** for a nonempty draft, **Review** for
  reviewable Workspace changes, or **Brief** for observed evidence. Running and
  destructive decisions take priority over navigation. Rename and Archive stay
  available through selected-Session commands and the context menu rather than
  returning as hover icons. Labels remain visible at 280 px and above; narrower
  rails retain named, tooltip-backed icons. Focused source assertions and
  identity guards pin the priority order and prevent the secondary icon cluster
  from returning. This is source-only; no build, test binary, alternate binary,
  or visual launch was performed.
- 2026-07-26: Closed the stale-empty-split restoration gap. Dez now reapplies
  its one-Main-Work-Area cleanup immediately after restoring the default layout
  or a saved Full, Agent Control, or Editor Focus layout. Only surplus empty
  tab panes are hidden; files, terminals, diffs, and other user Surfaces remain
  visible, and explicit multi-pane recipes keep their requested empty work
  areas. Source assertions and an identity guard pin the restoration boundary.
  This is source-only; no build, test binary, alternate binary, or visual
  launch was performed.
- 2026-07-26: Removed the remaining Workspace-header action overlays from
  Sessions. Workspace names, layout metadata, disclosure, and status now share
  one flexible truncated region; Start Terminal Session and Options occupy one
  fixed inline action region. The opaque-window gradient masks and duplicate
  action-side padding are gone, so Lumin glass and opaque themes use identical
  non-overlapping geometry. A source guard prevents gradient header layers from
  returning. This is source-only; no build, test binary, alternate binary, or
  visual launch was performed.
- 2026-07-26: Removed the duplicated terminal launcher inside an expanded empty
  Workspace group. Its full labeled **Start Terminal Session** action now owns
  the transition; the compact header terminal control appears only when the
  group is collapsed or already contains Sessions. Options remains available
  in the header. Pure source assertions and identity guards protect each state.
  This is source-only; no build, test binary, alternate binary, or visual
  launch was performed.
- 2026-07-26: Quieted the expanded empty Workspace body in Sessions. The
  overview already reports ready Workspace count and the header's accessible
  name reports readiness, so the decorative dot plus **Ready for a session**
  caption no longer repeats that state. The scoped **Start Terminal Session**
  action remains the sole visible body content. The identity guard now rejects
  the redundant caption. This is source-only; no build, test binary, alternate
  binary, or visual launch was performed.
- 2026-07-26: Rebalanced the Sessions footer around information value. At
  compact/default widths the familiar Settings gear remains icon-only with its
  full **Agent Tools and Settings** tooltip and accessible name, freeing space
  for the ambiguous **History** and **Workspaces** labels. Detailed widths
  expand all three destinations; genuinely narrow rails keep named icons.
  Pure assertions and identity guards cover the three width states. This is
  source-only; no build, test binary, alternate binary, or visual launch was
  performed.
- 2026-07-26: Removed the populated-state terminal creation duplicate. In Dez,
  the Sessions overview now stays focused on status and All/Attention scope,
  while each Workspace header owns the launcher for its exact destination.
  Keyboard shortcuts and command-palette routes continue to create a Main Work
  Area terminal, and official Zed retains its compatibility overview action.
  Focused source assertions and an identity guard pin both product branches.
  This is source-only; no build, test binary, alternate binary, or visual
  launch was performed.
- 2026-07-26: Removed floating Agent attention windows from Dez's default
  interaction model. Sessions already owns unread and action-needed state, so
  background Agent work no longer places a shaped popup over the Main Work
  Area. **Floating Attention Popups** provides an explicit opt-in; sound and
  accessible window-attention policy remain independent. Official Zed keeps
  its inherited behavior, and opt-in popups retain Lumin material plus the
  configured UI font. Source assertions and identity guards cover each branch.
  This is source-only; no build, test binary, alternate binary, or visual
  launch was performed.
- 2026-07-26: Replaced the terminal context strip's single all-label breakpoint
  with a progressive handoff hierarchy. Below 480 px, every action remains a
  named, tooltip-backed icon. At ordinary split widths, one primary transition
  is readable: **Review Changes** when Git reports changes, otherwise **Files**
  or **Open Workspace**. Files can join Review at 720 px; the long **Session
  Details** label appears at 920 px. This avoids the previous three-label width
  jump while preserving every destination, the 32 px chrome boundary, and the
  full evidence disclosure. Pure source assertions and identity guards cover
  each state. This is source-only; no build, test binary, alternate binary, or
  visual launch was performed.
- 2026-07-26: Made the empty Main Work Area a native Workspace surface instead
  of a bordered card floating over an already-empty canvas. Its heading now
  says **Run, supervise, and review in this Workspace**; the passive route
  names **Terminal in Main Work Area**, **Live state in Sessions**, and
  **Files, Git, and diffs**. Those route items no longer use borders or element
  backgrounds that make them resemble clickable controls, while the actual
  Start Terminal Session, Find File, and New File actions retain their clear
  hierarchy and keyboard routes. Static guards reject both the floating card
  shell and button-like passive guidance. This is source-only; no build, test
  binary, alternate binary, or visual launch was performed.
- 2026-07-26: Applied the same passive orientation grammar to the true-empty
  Sessions rail. **Run**, **Supervise**, and **Review** are now individually
  named icon-and-copy list items targeting **Terminal in Main Work Area**,
  **Live state in Sessions**, and **Files, Git, and diffs**. Their old rounded
  borders and control backgrounds are gone, so they no longer compete with the
  actual **Open Workspace…** and **Open Scratch Terminal** buttons. Open
  Workspace remains the only filled action. Source assertions and a rejection
  guard protect the distinction between explanation and interaction. This is
  source-only; no build, test binary, alternate binary, or visual launch was
  performed.
- 2026-07-26: Unified Welcome with the native first-use grammar. Its headline
  now states the actual promise—Workspace-connected Terminal and Agent work
  remains reviewable through the IDE—instead of repeating Run/Supervise/Review.
  **How Dez Works** is now a named passive list using Terminal, Sessions, and
  Diff icons. The bordered panel, dividers, numbered selected-looking pills,
  and decorative bordered header icon tile are removed. The state-aware
  three-action entry list and its single filled recommended action remain.
  Static guards reject the old card treatment and protect the accessible list
  structure. This is source-only; no build, test binary, alternate binary, or
  visual launch was performed.
- 2026-07-26: Anchored empty Workspace Tools and Agent recovery to the top of
  their drawers. The recovery region now owns the full available height, uses
  a zero minimum height plus internal vertical scrolling, and keeps deliberate
  top/bottom padding. It can no longer inherit the pane placeholder's centered
  alignment and resemble a floating prompt in a blank column. The bounded
  measure, named region, inward arrow, keyboard route, and single **Return to
  Main Work Area** action remain. A static geometry guard protects the
  contract. This is source-only; no build, test binary, alternate binary, or
  visual launch was performed.
- 2026-07-26: Removed the inherited whole-pane drag overlay from Dez. The
  top-center visible pill and its larger invisible hit target previously
  covered pane tab/header chrome and suggested that the stable Sessions,
  Workspace Tools, Main Work Area, and Agent regions were arbitrary columns.
  Explicit split/move actions, ordinary Surface tab drag and drop, and Canvas
  layout recipes remain available; official Zed retains its upstream handle.
  A pure product-identity assertion and identity guards reject an
  unconditional overlay. This is source-only; no build, test binary, alternate
  binary, or visual launch was performed.
- 2026-07-26: Repaired Agent title/action width ownership. Dez's visible title
  pencil was still absolutely positioned as an inherited Zed hover overlay, so
  it reserved no width and could cover a long Agent Session or terminal title.
  The title now occupies a flexible, truncating slot and the pencil occupies a
  fixed inline slot; the Dez path renders no gradient mask or absolute action
  layer. The named keyboard action and edit routing remain, while official Zed
  retains its upstream hover presentation. Product assertions and an identity
  guard protect the split. This is source-only; no build, test binary,
  alternate binary, or visual launch was performed.
- 2026-07-26: Removed inherited gradient overlays from Dez Agent plans. The
  collapsed current task now owns flexible one-line space while its remaining
  count owns a fixed inline slot; expanded rows use constrained overflow and
  preserve their full-text tooltips. Counts and row fades can no longer paint
  over task text or create opaque Lumin Blur patches. Official Zed retains the
  upstream masks. A pure product assertion and static summary/row guard protect
  the split. This is source-only; no build, test binary, alternate binary, or
  visual launch was performed.
- 2026-07-26: Simplified Dez to one optional auxiliary drawer. Sessions remains
  the stable rail, the Main Work Area remains dominant, and Workspace Tools
  and Agent can no longer remain visible together—even at 2400 px ultrawide.
  Revealing one closes the other; restored double-drawer state keeps the active
  or recipe-appropriate region. Official Zed keeps its upstream coexistence
  behavior. A pure product assertion, focused ultrawide source test, and static
  guard protect the invariant. This is source-only; no build, test binary,
  alternate binary, or visual launch was performed.
- 2026-07-26: Tightened the launch profile around the minimalist shell instead
  of adding another layout mode. Main Work Area tab bars no longer show
  persistent Back and Forward buttons by default; editor toolbars no longer
  show generic quick-action or selection menus by default; and Sessions
  defaults to the two supervision signals that matter in the rail—live Agent
  state and latest attention. Navigation, selection, and editor actions remain
  available through shortcuts and commands. Canvas Layout remains contextual
  in each Workspace's **Workspace Options**, while add, split, Surface
  switching, breadcrumbs, diagnostics, and Agent review remain visible. Static
  guards protect this hierarchy. This is source-only; no build, test binary,
  alternate binary, or visual launch was performed.
- 2026-07-26: Made command search obey the same opinionated Canvas Layout
  contract as **Workspace Options**. Dez now exposes the six workflow recipes
  (Full, Agent Control, Focus Editor, Code/Run/Observe, Review, and Debug) plus
  Cycle, Save Layout As, Manage Saved Layouts, and Restore Previous. It hides
  the generic centered-editor toggle, legacy Classic/Canvas toggles,
  experimental matrices and studios, numeric slot internals, and
  clipboard/storage actions from command search. Those
  action types remain implemented for saved-layout internals and official Zed
  compatibility; official Zed retains the complete upstream inventory. A pure
  product-policy assertion and static action-set guard protect the boundary.
  This is source-only; no build, test binary, alternate binary, or visual
  launch was performed.
- 2026-07-26: Repaired the Main Work Area terminal-to-Sessions handoff and
  made the rail viewport-responsive. Starting Codex, Claude, or another
  recognized foreground process inside an existing terminal now emits one
  explicit process-info change after the asynchronous process refresh.
  Sessions subscribes to both existing and newly added Main Work Area
  terminals and rebuilds only for semantic state changes—not every PTY output
  frame. Shell → Codex → shell remains one terminal and one Session identity.
  The rail now reserves and paints the same window-derived width: 240 px in an
  800 px window, 200 px at the narrow floor, and 280 px at the compact wide
  default. This is source-only; no build, test binary, alternate binary, or
  visual launch was performed. Runtime proof of the process transitions,
  identity stability, output responsiveness, and width matrix remains open.
- 2026-07-26: Completed the narrow Sessions and optional-drawer allocation
  pass. At the 200 px rail floor, the two scope controls use **All** and
  **Needs** with their counts, compact targets, full accessibility names, and
  a tooltip for truncated status; **Attention** returns at 240 px. Workspace
  Tools or Agent now targets 240 px when space permits, remains capped at
  360 px, and may grow only to 40% on smaller workspaces so the Main Work Area
  retains at least 60%. At 2000/1000/560/400 px of available workspace, the
  initial drawer resolves to 360/240/224/160 px. This is source-only; no build,
  test binary, alternate binary, or visual launch was performed.
- 2026-07-26: Made Sessions activate the terminal that is actually on screen.
  When durable metadata or a Host snapshot matches a live Main Work Area
  terminal, the live `TerminalView` now owns the row, its Workspace, and its
  focus route. A stale stored path cannot capture that row in another
  Workspace, and selecting it no longer attempts a duplicate Host attachment.
  Dez also rejects the inherited full-pane focus overlay at render time, so
  imported border settings cannot restore the screenshot-class orange frame;
  official Zed keeps the upstream option. Pure source assertions and static
  guards cover both policies. This is source-only; no build, test binary,
  alternate binary, or visual launch was performed.
- 2026-07-26: Removed the remaining absolute geometry from the Dez Sessions
  shell. The client-decorated rail is now a normal-flow child of the exact
  width reserved by `MultiWorkspace`, so layout, painting, and pointer hit
  testing have one owner rather than a hidden second surface. Its header
  divider now belongs to the header container instead of an absolute
  full-header decorative child. Official Zed keeps its inherited
  client-decoration path. A pure source assertion and identity guards cover
  both decisions. This is source-only; no build, test binary, alternate binary,
  or visual launch was performed.
- 2026-07-26: Closed the Session Switcher's invisible blocked-window path.
  Its intentional full-window interaction boundary now dispatches Cancel on an
  outside click, restoring the previewed origin through the existing
  source-preserving dismissal flow. The bounded dialog stops propagation, so
  row selection and controls remain interactive. Escape, Enter, modifier
  release, and focus-loss behavior are unchanged. A static guard protects both
  halves of the interaction. This is source-only; no build, test binary,
  alternate binary, or visual launch was performed.
- 2026-07-26: Made a foreground agent visible without adding more UI. The
  semantic process-info transition now invalidates the existing terminal
  Surface and refreshes its tab. Recognized direct agent commands reuse the
  terminal tab icon and compact context strip for a concise state such as
  **Codex running**; terminal details identify this as observed-process
  evidence. Shells and generic runtimes remain quiet, official Zed keeps its
  upstream chrome, and no Agent panel, overlay, replacement terminal, or
  duplicate Session is created. A pure product-policy assertion and static
  guard protect the contract. This is source-only; no build, test binary,
  alternate binary, or visual launch was performed.
- 2026-07-26: Removed adjacent terminal-status duplication before it could
  become new chrome debt. The terminal tab alone owns the provider icon; the
  32 px context strip may show concise meaningful state such as **Codex
  running**, but does not repeat the glyph. An ordinary active shell omits the
  generic **Active** segment and begins directly with Workspace/Git context;
  failures and other meaningful task states remain visible. Below 360 px the
  activity text yields while the tab icon and full accessible/details state
  remain. This is source-only; no build, test binary, alternate binary, or
  visual launch was performed.
- 2026-07-26: Aligned the native Dez window floor with its actual responsive
  layout contract. Sessions cannot render readably below 200 px, so the
  inherited 360 px window minimum could leave only 160 px for the Main Work
  Area. Dez now uses a 600 × 400 px minimum, preserving 400 px for primary work
  at the narrow checkpoint before an optional drawer; official Zed retains
  360 × 240 px. A pure product assertion and static identity guard protect the
  split. This is source-only; no build, test binary, alternate binary, or
  visual launch was performed.
- 2026-07-26: Made the terminal-to-Sessions response visible in both responsive
  states. A compact terminal row now says **Codex · Running**, **Claude Code ·
  Running**, or the equivalent provider state instead of hiding the actor and
  exposing the implementation phrase **Detected · Live**. If its Workspace
  group is collapsed, the header shows **Codex running** or a count of running
  terminal Agent Sessions; attention counts now include terminal agents as
  well as native Agent Sessions. The group does not auto-expand, focus stays in
  the Main Work Area terminal, and no panel or overlay is created. Pure
  assertions plus the identity gate protect the compact and collapsed
  responses. This is source-only; no build, test binary, alternate binary, or
  visual launch was performed.
- 2026-07-26: Made failed terminal termination a durable recovery state in the
  UI instead of a disappearing implementation error. If an external terminal
  owner rejects or cannot complete termination, Dez now keeps a
  terminal-specific notification stating that the named process may still be
  running and offers **Return to Session**. That action restores the exact
  owning Workspace and terminal route so the user can retry or stop it there.
  The notification does not auto-hide, raw transport errors remain in logs,
  and public copy no longer says **Durable session** or exposes Host internals.
  Pure copy assertions and the identity gate protect the failure/recovery
  contract. This is source-only; no build, test binary, alternate binary, or
  visual launch was performed.
- 2026-07-26: Removed Dez's last Sessions-owned full-window interaction layer.
  While Agent or Sessions has focus, the existing switch-session action now
  activates the next or previous visible Agent or terminal Session directly
  through the same source-preserving route as its row. It no longer creates a
  preview dialog, scrim, outside-click boundary, temporary focus owner, or
  confirmation step. Main Work Area `Ctrl-Tab` remains conventional Surface
  switching, command search remains available, and official Zed retains its
  inherited modal Thread Switcher. A pure product-policy assertion and static
  guard protect the split. This is source-only; no build, test binary,
  alternate binary, or visual launch was performed.
- 2026-07-26: Removed idle filtering furniture from the default Sessions
  hierarchy. One unfiltered Session now flows directly from the overview to its
  Workspace and row instead of rendering redundant **All 1 / Attention 0**
  scope controls or an empty search field. An active Attention projection keeps
  the scope controls so it always has an exit. When multiple Sessions make
  filtering useful, Dez exposes one compact **Search Sessions** control in the
  overview; the existing filter action remains available at every count. Either
  path reveals the inline search and moves focus into it. While search is
  active the overview control yields, so the action is not duplicated. Closing
  clears the query, returns focus to Sessions, and removes the row; a non-empty
  query remains visible until recovery is possible. Official Zed retains its
  inherited scope and populated-rail search presentation. Pure visibility
  assertions and the identity gate protect this behavior. This is source-only;
  no build, test binary, alternate binary, or visual launch was performed.
- 2026-07-26: Closed the hosted-terminal foreground-agent gap. The helper now
  observes the PTY foreground process group for a bounded window around input,
  publishes the normalized command in the durable Session snapshot, and clears
  it on exit. Attached terminals emit the same semantic process-info event as
  ordinary terminals, so Sessions can change the existing row to **Codex ·
  Running** without parsing TUI output or requiring a structured hook.
  Detached Session activation now shows **Opening…** immediately and
  coalesces repeat clicks until the single restored Surface is focused.
  Structured attention and review evidence remain adapter-only. Static guards,
  formatting, locked offline metadata, theme validation, and diff checks cover
  this source slice; no build, test binary, alternate binary, or visual launch
  was performed. Runtime proof remains open for the rebuilt app/helper pair.
- 2026-07-26: Consolidated Sessions visibility into one owner. Fresh Dez
  windows still start with Sessions closed. Under **Auto**, a restored-open rail
  waits for Workspace and Terminal Host truth, keeps Sessions, attention,
  Agent History, and recovery state visible, and closes only when genuinely
  empty. Explicit open/focus cancels the one-shot close. While the rail is
  open, its overview exposes one explicit **Hide Sessions** action and generic
  window chrome no longer duplicates that toggle; when closed, existing
  title/status chrome continues to expose **Open Sessions**. One named Sessions
  Menu now owns Agent History, global Recent Workspaces, and Agent
  tooling/settings at every width; Dez renders no persistent footer. Official
  Zed retains its inherited toggle and footer behavior. The canonical shell
  wireframe now pins the same implementation: a terminal-native agent remains
  interactive in its Main Work Area terminal, Sessions projects lifecycle and
  attention, and selecting the row returns to that existing Surface rather
  than creating a transcript or overlay. Formatting and static identity checks
  cover this source slice; no build, test binary, alternate binary, or visual
  launch was performed.
- 2026-07-26: Made the fresh Workspace honor the Main Work Area hierarchy.
  Workspace Tools no longer opens by default merely because a folder is
  present; Sessions and Agent already begin closed. Files, Outline, Git, and
  Debug remain one click or command away, and an explicit
  `project_panel.starts_open` user override remains supported. Restored windows
  continue to preserve intentional layout state. Together with the existing
  one-drawer exclusivity and 60% Main Work Area budget, the default shell is
  now one primary surface instead of an unsolicited two-column layout. Static
  identity checks cover the default; no build, test binary, alternate binary,
  or visual launch was performed.
- 2026-07-26: Hardened the remaining transient toast against the clipping shown
  in earlier screenshots. Status copy now owns a shrinkable one-line region,
  truncates inside the bounded toast, and exposes the complete text by tooltip
  instead of pushing actions beyond the surface. Action and dismiss controls
  are fixed-width compact targets with explicit keyboard tab stops and
  accessible names. The toast keeps non-modal elevation and only its visible
  content intercepts input. Static identity checks protect the layout and
  interaction contract; no build, test binary, alternate binary, or visual
  launch was performed.
- 2026-07-26: Gave the closed-Sessions recovery action fixed priority in the
  status bar. **Open Sessions** now occupies its own non-shrinking allocation;
  Search, language-server, diagnostics, activity, and other optional status
  items share a separate shrinkable overflow region. Right-positioned Sessions
  receives the same fixed wrapper. Narrow Main Work Areas therefore discard
  low-priority status content before they clip the navigation needed to recover
  supervision. Static identity checks protect the allocation; no build, test
  binary, alternate binary, or visual launch was performed.
- 2026-07-26: Removed fake work from Sessions. Idle Workspaces no longer render
  **Ready for a session** groups, and an empty Agent composer no longer creates
  a **New Agent Session · Completed** row. Workspace navigation remains in
  Workspace Tools and Recent Workspaces; Agent composition remains in Agent
  Tools. The first real terminal, contentful draft, saved session, or active
  Agent run creates the first supervision row. A Main Work Area terminal still
  keeps one stable row while foreground-process events change it from an
  ordinary terminal to **Codex · Running** and back without opening another
  panel. The no-Workspace state is now a concise open-workspace/scratch-terminal
  choice instead of a permanent three-step tutorial. Hiding a focused auxiliary
  pane now restores keyboard focus even when layout reconciliation had already
  advanced the active-pane pointer. Static checks and source assertions cover
  the contract; the inspected installed app predates this source and runtime
  proof remains deferred until the requested later build.
- 2026-07-26: Closed the last ordinary-terminal observation race. PTY process
  inspection still coalesces output bursts behind one asynchronous refresh,
  but a wakeup received while that refresh is running now records one trailing
  inspection. A shell therefore cannot remain stranded in Sessions when Codex
  takes over during the earlier inspection and then becomes quiet; the same
  terminal identity reaches **Codex · Running** without rebuilding on every
  output frame. Direct npm-installed Codex script paths are covered by the
  foreground-command assertion. This is source-only; runtime proof remains
  deferred until the requested later build.
- 2026-07-26: Removed dead-session placeholder terminals from Dez. A failed
  saved-Session activation now leaves the current Main Work Area untouched and
  marks the existing Sessions row **Missing**; retrying that row can still
  reattach if its Host returns, and removing it uses the existing row action.
  Startup restoration drops an unavailable or invalid terminal item rather
  than reopening a full-size **Session unavailable** Surface beside real work.
  Official Zed retains its inherited placeholder behavior. Static product
  assertions and identity guards cover both paths; no build, test binary,
  alternate binary, or visual launch was performed.
- 2026-07-26: Stopped explicit split layouts from repeating first-use guidance
  across every empty pane. Only the active empty Main Work Area now carries the
  passive Run/Supervise/Review route under **Start with a terminal or file**.
  Inactive empty panes keep the same three working actions under compact **Open
  something here** copy and regain orientation when focused. Sticky Sessions
  Workspace headers also drop their small shadow in Dez, staying within the
  flat list material and divider hierarchy; official Zed retains its inherited
  presentation. Static product assertions, formatting, identity, theme,
  metadata, and diff checks cover this source slice; no build, test binary,
  alternate binary, or visual launch was performed.
- 2026-07-26: Reordered Dez Settings around the product workflow. Workspace &
  Privacy, Sessions & Terminal, Agents, Attention, and Evidence now lead the
  root navigation, with retained IDE customization and compatibility pages
  following. Sessions placement moved out of Agent provider configuration and
  into Sessions & Terminal; Agents now starts with Agent Runtime & Providers
  and exposes its compatibility kill switch as Disable Agent Features.
  Official Zed keeps its upstream order and labels. Source assertions and
  static guards cover the hierarchy; no build, test binary, alternate binary,
  or visual launch was performed.
- 2026-07-26: Audited the running installed window against the current Lumin
  source. The installed artifact still shows pre-fix idle Workspace rows and an
  unsolicited Workspace Tools drawer; current source already removes both.
  Lumin Light/Blur correctly reaches the one native macOS under-window material
  with bounded translucent semantic layers and activation-aware behavior.
  Fresh and restored profiles now use compact Canvas density while retaining
  14 px typography and existing control targets. Theme and identity guards pin
  the default, first-run, and recovery paths. No build, test binary, or
  alternate binary was launched.
- 2026-07-26: Completed a final source-level public-shell audit before
  publishing. Ordinary Dez Settings no longer lists the staff-only Advanced
  instrumentation page; official Zed and staff override builds retain it.
  Lumin Light and Lumin Blur now use a tighter translucency ceiling for the
  root, title/status chrome, rail/drawers, editor, terminal, toolbar, and tab
  strip. The native macOS under-window material remains the only blur owner,
  while denser elevated surfaces retain legibility. The theme gate now rejects
  structural layers that flatten the backdrop and still proves text,
  interaction, region, scrollbar, and focus contrast across representative
  backdrops. This is source-only; the installed artifact remains stale and
  rendered proof waits for the requested later build.
- 2026-07-26: Made the application View menu match the one-drawer shell.
  Sessions and Agent remain separate regions. Files, Outline, Git, and Debug
  now live under one **Workspace Tools** submenu with an explicit
  **Show or Hide Workspace Tools** action instead of appearing beside their
  container as unrelated top-level panels. Editor Layout and Diagnostics
  remain distinct. Official Zed keeps the upstream Project Tab/Panel and
  Terminal Panel hierarchy. A pure product-policy assertion and identity guard
  cover the split. This is source-only; menu rendering and keyboard navigation
  remain part of the later consolidated artifact.
