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

| Area                   | Evidence now                                                                                                                                                                                                                                                                                       | Completion gap                                                                                                                   |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| Upstream               | Integrated `upstream/main` `9d0ef37a2571` through two-parent merge `2be63cfea347`; eleven conflicts are resolved and classified; consolidated build provenance is recorded                                                                                                                         | Complete runtime regression, installed coexistence, and design-partner proof                                                     |
| Identity               | Dez source guards pass; the corrected arm64 app, helper, and `dev.dez.Dez-Dev`/`dez-dev` ad-hoc bundle are audited; the rebuilt raw CLI exposes `--dez <PATH>` and no visible legacy alias; the launched app held no TCP connection or listener during the recorded soak                           | Official-Zed install coexistence, consolidated rebuild, public signing/notarization, updater, remote, and visual proof           |
| App Session            | Restore barrier, lifecycle state, ordered Workspace registry, explicit ordered viewport records, active selection, unresolved identity retention, live background-viewport attachment, and durable final-project fallback exist in source; all ten focused Session tests pass                      | Shared live entity composition and consolidated runtime proof                                                                    |
| Workspace and Surfaces | Pane/Canvas repair, panel-to-pane work, startup request ordering, and typed path projection exist                                                                                                                                                                                                  | Authoritative EvidenceSet, scoped tools, movement proof, and shared-store isolation                                              |
| Local Host             | Protocol 4 app/helper builds and focused tests pass; an authenticated packaged-runtime Session retained one shell PID, 88 replay chunks, both pre/post-resize dimensions, and explicit Detached state                                                                                              | GUI-exit/same-Session reattach proof and default-backend decision                                                                |
| Terminal recovery      | Host/Session references, attach/detach/terminate, recovery surfaces, honest transport states, and dimension-aware replay exist in source and packaged runtime                                                                                                                                      | Full GUI restart scenario, stale-host cleanup, and rendered UX verification                                                      |
| Agent adapter          | Structured Codex hook path, observation-only capabilities, bounded file targets, objective/context projection, and onboarding exist                                                                                                                                                                | Live hook proof and a second adapter after the PMF gate                                                                          |
| Attention              | Session Rail projection, restart-safe attention, acknowledgement, mute, resolution, priority, and stale handling exist                                                                                                                                                                             | Consolidated runtime and accessibility proof                                                                                     |
| Review                 | Native and terminal review briefs consume observed commands/checks, Git/worktree state, bounded file targets, cwd provenance, risks, and missing-evidence labels                                                                                                                                   | Compiled proof, live navigation, and side-by-side hero-flow validation                                                           |
| UI/UX                  | The rebuilt bundle includes the rail, blank-center, footer, and utility-row corrections; newer source makes empty Dez windows terminal-first, replaces ambiguous zero-session/caught-up and `+ New` copy, hides inert zero-session filters/search, and gives compact chrome 280 px of usable width | Rebuild the newest source, capture it, then complete shell hierarchy, outward polish, onboarding, accessibility, and state audit |
| Release                | Static gates, focused tests, the corrected protocol-4 app/helper build at `679cdc28445c`, exact signed-bundle launch, authenticated runtime Session exercise, and deep-strict ad-hoc bundle audit pass                                                                                             | Full GUI Session restart, visual/a11y, app-facing lint, public signing/install, coexistence, and partner proof                   |

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
Project ready launch surface even if a legacy/restored pane predates the welcome
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
      schemes, channels, or updates.
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
      failed restoration now marks the affected identity unresolved without
      removing its ordered membership or viewport placement, and all 11
      focused Session tests pass. The full startup integration check and
      recovery UI remain open.

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
      directories. User-selected evidence remains to be added to the same
      owner.
- [ ] Recompute evidence on file open/move/close, terminal cwd change,
      Session attach/reconnect, Workspace restore, and explicit user choice.
      Visible root and remote-Host evidence recomputes on worktree/remote
      changes; live terminal cwd changes update stable session-provenanced
      records, and a newly opened idle terminal seeds its initial cwd before
      the first PTY event. Open pane files recompute on add/remove/title-path changes with
      stable IDs, deduplication, a 256-record cap, and truncation. Explicit
      selection and consolidated restore/runtime proof remain.
- [ ] Ensure generic tool, settings, search, Git, and conversation surfaces do
      not attach roots merely by existing.
- [ ] Scope file tree, search, Git, diagnostics, tasks, debugger, terminals,
      environment, and settings to Workspace evidence and explicit tool-local
      selection.
- [ ] Move eligible panel-only tools into ordinary pane Surfaces while keeping
      familiar toggles and dock layouts.
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
      after GUI restart with no false running or completed state.
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

- [ ] Establish one stable shell grammar for App Session, Workspace, Surface,
      Session Rail, command center, status, and transient overlays.
- [ ] Give every primary region a visible purpose, stable placement, clear
      focus treatment, and predictable resize/collapse behavior. The app View
      menu, title-bar/sidebar chrome, and collapsed status control now call the
      supervision region Session Rail consistently. The rail now uses the same
      mode-resolved width for workspace reservation and root painting instead
      of painting compact mode 60 px wider than its allocation; a rebuilt
      rendered resize/collapse audit remains. Commit `1ebb7c79d4` raises the
      compact cap from 240 px to 280 px and the resize floor from 200 px to
      240 px so the visible labels and actions are no longer designed into a
      crushed column. The focused `sidebar` source check passes.
- [ ] Keep fixed shell chrome bounded under real project names and narrow
      widths. Commit `0d8496969f` gives the project identity and Git controls
      explicit shrinkable, overflow-hidden regions so their one-line labels do
      not collide with footer controls. Formatting and diff checks pass;
      compiled and rendered narrow-width proof remain in the consolidated gate.
- [ ] Avoid stacked utility chrome that steals space from supervised work.
      Commit `abc4f8bedb` removes Dez's dedicated Command Search footer row,
      keeps the action as a labeled icon in the existing utility bar, hides the
      unowned upstream update surface, and renders the Canvas prefix row only
      while prefix mode is active. Formatting and diff checks pass; compiled
      and rendered proof remain open.
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
- [ ] Align density, spacing, radii, borders, typography, hover, focus,
      selection, and animation across panes, Canvas, rails, cards, callouts,
      menus, settings, and recovery surfaces.

#### Interaction quality

- [ ] Provide keyboard-first switching across Workspaces, Surfaces, sessions,
      actors, Hosts, attention items, and recent targets. While Session Rail is
      focused, Shift-A now toggles its Attention projection and Shift-V opens
      the selected Review Brief on every supported desktop keymap; tooltips
      expose both bindings. Broader Host/actor switching remains open.
- [ ] Preserve selection and focus intentionally when filtering, switching
      scope, opening review, moving a Surface, or returning from an overlay.
      Session Rail rebuilds now preserve keyboard selection by stable session
      identity across reorder/filter updates and choose the nearest actionable
      row if the selected session disappears; the cross-surface audit remains.
- [ ] Give pointer and keyboard users the same actions, descriptions, disabled
      reasons, confirmation semantics, and recovery paths.
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
      operational toast instead of failing only in logs. The app-wide audit remains
      open.
- [ ] Use progressive disclosure: the default view communicates current work;
      details reveal provenance, capabilities, protocol, and diagnostics only
      when requested.

#### States and copy

- [ ] Design and inspect populated, empty, searching, no-result, caught-up,
      loading, connecting, reconnecting, disconnected, missing, incompatible,
      failed, resumable, exited, archived, and partial-evidence states. Session
      Rail now composes structured work state with non-live transport state so
      Running cannot conceal Detached, Reconnecting, Missing, Incompatible, or
      Exited; Review Briefs prioritize exceptional transport truth and state the
      resulting evidence risk. Rendered inspection remains open.
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
      loaded worktree now always renders Project ready actions, including for a
      legacy/restored pane whose welcome flag is absent, instead of leaving the
      center blank. Its primary action is now New Terminal, followed by Find
      File and New File. New Window and startup fallbacks preserve this surface
      in Dez instead of covering it with an unsolicited blank editor. The full
      Session Rail zero state now says No sessions yet, exposes New Terminal,
      starts with Start working instead of implying finished work or requiring
      a project, and suppresses the inert All 0 / Attention 0 scopes and search
      field until a real session exists. An existing query remains visible so
      it can always be cleared. The full state audit and rendered proof remain
      open.
- [ ] Remove dead buttons, unsupported provider actions, duplicate navigation,
      noisy badges, ambiguous icon-only controls, and success copy unsupported
      by observed evidence. The compiled Zed Pro trial-end overlay/reset action
      is removed and provider-limit recovery no longer exposes its upstream
      subscription CTA; the app-wide audit remains open.
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
      the identity guard checks the public boundary.

#### Onboarding, settings, and accessibility

- [ ] Add a short first-run path for opening work, starting a durable terminal,
      installing the Codex hook deliberately, understanding attention, opening
      review, and learning detach versus terminate. Source now teaches the
      terminal → Session Rail → review loop, provides New Terminal, and explains
      close/detach/terminate plus Host-dependent persistence; hook installation
      remains deliberately manual, but eligible detected Codex rows now show a
      visible Hook setup state and one-click copy action with context-menu
      parity. Rendered flow verification remains open.
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
      state that Dez never installs hooks or edits provider configuration; the
      complete retention/redaction audit remains open.
- [ ] Audit focus order, accessible roles/names/descriptions, key shortcuts,
      minimum hit targets, screen-reader announcements, zoom, reduced motion,
      contrast, truncation, and localization-resistant layouts. Shared Session
      Rail rows now expose list-item/selection semantics plus actor, state,
      Host, unread, remote/archive, and observed diff information without
      duplicating a richer state label. Newly active authenticated Host
      attention now triggers the configured OS window-attention request once
      per condition transition, including when no terminal surface is attached;
      the rendered matrix remains open.
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
      explicitly attributed; post-build runtime proof remains open.
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
  Project ready with Find File, New File, and New Terminal actions. A focused
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
  `e4fbc22a3a`. Project ready now leads with New Terminal; New Window and both
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
  no application bundle was built or launched.
