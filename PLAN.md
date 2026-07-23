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
- Irrelevant Zed product promotion or account-centric chrome is removed or
  demoted without breaking compatible editor capabilities or the upstream
  merge path.
- The intended Dez binary passes the consolidated compile, focused tests,
  identity, security, restart/recovery, visual, accessibility, packaging, and
  release-provenance gates.

## Current evidence baseline

Status below reflects repository evidence through 2026-07-23. The corrected
app, CLI, helper, and signed bundle now exist; build proof still does not imply
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

The intended raw executable was used for the first consolidated runtime gate;
the current gate launches only
`/Users/test/Documents/zed 3.0/target/debug/bundle/osx/Dez Dev.app`. The
excluded untracked `dist/Superzed.app` has never been opened. The first unlocked
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
      durable terminals that have no local PID.
- [ ] Keep fixed shell chrome bounded under real project names and narrow
      widths. Commit `0d8496969f` gives the project identity and Git controls
      explicit shrinkable, overflow-hidden regions so their one-line labels do
      not collide with footer controls. Formatting and diff checks pass;
      compiled and rendered narrow-width proof remain in the consolidated gate.
      Commit `a9b1a961c0` then removes that duplicate project/branch row from
      Dez's footer entirely because the Session Rail group hierarchy already
      owns it; essential Restricted Mode and embedded application-menu content
      can still reopen the row.
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
      finishes the first-party visual baseline: new installs follow the system
      with Lumin Blur/Lumin Light, use JetBrains Mono for code and terminals,
      retain a readable sans-serif UI role, normalize the working typography to
      14 px, and restore low-contrast borders, focus, active-line, and
      scrollbar hierarchy inside the translucent theme. Source, license,
      first-run, and font assets are guarded. Rendered density, material,
      contrast, and narrow-height proof remain open.

#### Interaction quality

- [ ] Provide keyboard-first switching across Workspaces, Surfaces, sessions,
      actors, Hosts, attention items, and recent targets. While Session Rail is
      focused, Shift-A now toggles its Attention projection and Shift-V opens
      the selected Review Brief on every supported desktop keymap; tooltips
      expose both bindings. Commit `57290c27c3` makes the rail's platform and
      Vim creation bindings terminal-first through a dedicated New Session
      action; the separate New Agent Thread command remains available. Broader
      Host/actor switching remains open.
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
      pointer/context parity; the app-wide audit remains open.
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
      and suppresses the inert All 0 / Attention 0 scopes and search field until
      a real session exists. Commit `4e6292ff0a` goes further: the full **Start
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
- [ ] Group Dez settings by Workspace, Sessions, Agents, Attention, Evidence,
      Appearance, Privacy, and Advanced compatibility; hide experimental
      internals from the default path. The settings shell now names Workspace
      & Privacy, Sessions & Terminal, Agents, Attention, Evidence, Appearance,
      Network & Compatibility, and Advanced; Attention and Evidence expose
      real trust/accessibility controls. Session Rail chrome uses product
      terminology, and dead sign-in/user-menu/avatar settings are no longer
      visible because Dez deliberately suppresses that upstream account chrome;
      compatibility keys remain readable. Experimental-internal audit remains.
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
      rendered matrix remains open.
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
  or launched.
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
  editor, terminal, prompt/code, Markdown-code, and commit-input face, while
  `.ZedSans` remains the legible interface role. Lumin now preserves
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
