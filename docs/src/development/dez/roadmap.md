# Dez Roadmap

This is the living execution plan. [Fork Notes](./fork-notes.md) win whenever
the two conflict.

Update Progress, Discoveries, Decisions, and Verification while work is active.
Do not erase completed evidence.

## Objective {#objective}

Ship Dez v0.0.1 as a credible public preview for terminal-native developers
supervising concurrent coding agents. Prove one complete recovery and review
loop before expanding into a general orchestration platform.

## Dependency order {#dependency-order}

```text
upstream and identity gate
-> durable workspace owner
-> local terminal host and session
-> terminal-agent adapter
-> attention projection
-> Run and review brief
-> restart recovery demonstration
-> remote continuity and conflict awareness
```

Visual polish continues in small slices, but it must support truthful state and
the product loop rather than outrun ownership foundations.

## Revised consolidated-plan intake {#revised-plan-intake}

The 1,756-line revised Product and Execution Plan supplied on 2026-07-22 is
reconciled in
[Consolidated Plan Reconciliation](./consolidated-plan-reconciliation.md). It
does not reset this Roadmap's evidence or replace the Fork Notes.

Its twelve delivery milestones map onto the active dependency plan as follows:

| Supplied plan                                                      | Active Roadmap treatment                                                                    |
| ------------------------------------------------------------------ | ------------------------------------------------------------------------------------------- |
| Baseline, upstream, and identity (0–1)                             | Permanent Milestone 0 gate; partially implemented, build and next-merge evidence open       |
| App Session, scoped Project, Evidence, opening, and Surfaces (2–4) | Milestones 1–2; source work is partial and precedes broad store refactors                   |
| Local Host, persistent terminals, and PMF Run (5–7)                | Current vertical slice; broad source exists, consolidated restart proof open                |
| ACP and power operations (8–9)                                     | Preserve upstream substrate now; project into the common model only after the vertical gate |
| Remote Hosts and environments (10)                                 | Milestone 3 after local lifecycle truth is proven                                           |
| Platform and commercial coordination (11–12)                       | Post-PMF hypotheses, not v0.0.1 scope                                                       |

Change Set storage, Environment orchestration, browser automation, provider
platforms, relay, mobile, and team policy do not enter the critical path merely
because the consolidated plan describes them.

## Milestone 0: Upstream and identity gate {#milestone-0}

This milestone repeats before major work and every release.

- Refresh and record upstream refs and the latest stable tag.
- Rehearse the merge in an isolated branch or worktree.
- Audit and adapt new Zed features instead of reimplementing them.
- Verify executable, app, bundle, URL, storage, update, and signing identities.
- Keep legacy Superzed data available through an explicit compatibility
  boundary until transactional migration exists.
- Add automated upstream drift, merge rehearsal, identity, and updater guards.

**Acceptance:** The selected upstream base and feature audit are recorded; Dez
and Zed coexist; no official Zed update can overwrite Dez; the `dez` build gate
passes when compilation is scheduled.

## Milestone 1: Durable vertical slice {#milestone-1}

### Workspace recovery {#workspace-recovery}

- Establish one app-session owner for durable workspaces.
- Restore every workspace before applying CLI or URL requests.
- Preserve ordering, active workspace, empty workspaces, and unresolved IDs.
- Treat future windows as viewports over the same session.

### Persistent local terminal {#persistent-local-terminal}

- Define host and session identities independent of pane IDs.
- Implement create, list, attach, detach, input, resize, bounded replay,
  metadata, working-directory updates, exit, and explicit termination.
- Move PTY and process lifetime outside the GUI lifecycle.
- Expose reconnecting, detached, exited, missing, incompatible, and stale
  states with a safe path to a new terminal.

### First agent loop {#first-agent-loop}

- Detect Codex in an ordinary terminal through a structured adapter where
  available.
- Attach running, waiting, attention, completed, failed, resumable, and exited
  state to the owning session.
- Associate objective, actor, session, evidence, attention, and review as one
  Run without duplicating their authoritative stores.
- Project attention to one low-noise inbox that focuses the existing surface.
- Generate a deterministic review brief from observed intent, diff, commands,
  checks, failures, permissions, and unresolved risks.
- Restore the workspace, live terminal, agent state, attention, and review after
  a GUI restart.

**Acceptance:** Start Codex in a terminal, receive an attention event, review an
observed result, close Dez without killing the process, reopen it, and recover
the same truthful state.

## Milestone 2: Workspace intelligence {#milestone-2}

- Give each workspace a scoped `Entity<Project>` over shared backend stores.
- Derive roots and repositories from file and terminal evidence.
- Scope file tree, Git, search, diagnostics, tasks, debugger, and settings.
- Convert appropriate dock-only tools into ordinary pane surfaces.
- Add session cards, bounded activity events, and the command center.
- Keep file discovery, Git, diagnostics, and language work lazy and measurable.

**Acceptance:** Two workspaces can share cached repository data while retaining
independent evidence and tool scope. Moving a terminal moves its context without
global leakage or eager home-folder scanning.

Source progress: Project, Git, Outline, Debug, and the compatibility Terminal
Panel now route to the Project-tool pane when legacy docks are hidden; Agent
uses its dedicated pane. New Terminal remains a center Surface. Commits
`5efa0398ad` and `1f186f9b8c` complete the registered-panel reachability map and
add a focused routing regression; compiled and rendered toggle proof remains in
the consolidated gate.

Follow-up `e4f1e341c9` normalizes the loaded-worktree launch and empty tool-pane
recovery surfaces on Workspace vocabulary while leaving compatible internal
Project types unchanged. The identity gate rejects the old mixed copy;
rendered proof remains deferred.

Commit `6f1562847e` names the region **Workspace tools** and its file-tree tab
**Files** across toggle, accessibility, empty, reveal, tooltip, preview, and
Settings surfaces. Official Zed retains Project Panel wording through explicit
product branches, and persistence/action identifiers remain compatible. Cargo
metadata and static gates pass; compilation and rendered proof remain deferred.

Commit `f6aea3e013` carries the same outward contract through the title bar,
recent-work pickers, remote Workspace indicators and deletion, and the Agent
History picker. Official Zed retains its Project vocabulary through explicit
product branches. Formatting and static identity gates pass; compilation and
rendered proof remain deferred.

Commit `b749a25619` completes the adjacent recent-work and Restricted Mode pass:
search, empty, open/move/remove, multi-root, Dev Container, failure, trust, and
trust-validation copy all use Workspace language in Dez. Official Zed and
internal action/type identifiers remain compatible. Formatting and static
identity gates pass; compilation and rendered proof remain deferred.

Commit `0e2c0dcae3` carries the same contract through remote search/errors,
Agent History, pane search, review evidence, agent rules/checkpoints, skills,
MCP/external-agent empty states, scoped Settings, status controls, and the full
Files settings section. Official Zed branches and internal schema/action names
remain compatible. Static gates pass; compilation and render proof are deferred.

Commit `e969abda4a` removes reachable upstream recovery and promotion traps:
the GPU dialog uses Dez identity and `DEZ_ALLOW_EMULATED_GPU`, the palette hides
inherited collaboration/feedback/account/docs/status/merchandise commands, and
database failure explains local safety and opens logs instead of an upstream
issue form. Explicit upstream Help links remain. Static gates pass; compilation
and interaction proof are deferred.

Commit `0607771783` completes the source-level shell grammar in retained
disconnected, follow/join, shared-agent, and multi-root collaboration recovery
states. Dez says Workspace, official Zed retains Project, and compatibility
types remain unchanged. Static gates pass; rendered proof remains deferred.

Commit `2092acd453` removes the supplied screenshot's repeated zero-session
hierarchy from current source. The region is explicitly **Session Rail**, its
summary reports the number of ready Workspaces, and each empty Workspace owns
one scoped New Terminal action with a Workspace-specific accessible name.
Focused assertions and static gates pass; rendered proof remains deferred.

Commit `57290c27c3` makes creation terminal-first after activation as well. The
Workspace plus control, worktree picker, and default desktop/Vim rail bindings
create terminal sessions; **New Agent Thread** remains an explicit secondary
Workspace option. A focused target assertion and static identity gates pass;
compiled interaction proof remains deferred.

Commit `633dcc4bec` closes the Workspace-options hover parity gap. Non-active
worktrees can be closed through a keyboard-addressable **Close Worktree from
Window** submenu with multi-root scope preserved, while whole-group removal is
explicitly **Remove Workspace from Window**. Focused label and identity checks
pass; compiled interaction proof remains deferred.

Commit `7893762cd5` makes restored attention a textual secondary signal. Active
persisted attention composes with Saved, Detached, Reconnecting, Missing,
Incompatible, and Exited transport labels even without a live adapter snapshot.
Focused state assertions and identity checks pass; durable activity/review
recovery and rendered accessibility proof remain open.

Commit `bd36afd3f4` makes bounded Host evidence truncation durable and visible.
The Host preserves an eviction flag through detach/list/reattach, the Session
Rail labels retained evidence **partial**, and Review Briefs explain that older
structured activity was evicted. A focused 40-event lifecycle assertion and
identity checks pass; helper-process/GUI restart proof remains deferred.

Commit `b39eedf724` redacts obvious secret-bearing environment assignments
before structured command evidence enters bounded Host retention. Evidence
settings now accurately separate the identity/attention metadata database from
Host-owned activity, state that transcripts are not stored there, and explain
that structured activity returns only from the same live Host Session. Focused
redaction and identity checks pass; compiled hook-flow proof remains deferred.

Commit `9323af8008` extends shared retained-command redaction to secret-suffixed
CLI flags in spaced and equals forms while preserving ordinary arguments.
Focused utility and Host-ingestion assertions plus identity checks pass;
compiled hook-flow proof remains deferred.

Commit `80ff1df75f` extends retained-command redaction to URL userinfo passwords
and secret-suffixed query parameters while preserving host/path structure. The
Evidence page also discloses that bounded file targets remain verbatim for
review navigation. Focused utility/Host assertions and identity checks pass;
compiled hook-flow proof remains deferred.

Commit `c47637c2ac` exposes the Session Rail hierarchy as a named accessible
list. Non-sticky Workspace headers report selected/expanded state plus
ready/running work and attention counts without relying on color; sticky visual
duplicates are excluded from accessibility output. Focused copy and identity
checks pass; platform-tree inspection remains deferred.

Commit `7e91f00b69` exposes All/Attention as a named Session scope group with
stable control names, shared toggle state, and explicit action descriptions.
Session search is a named region, while changing totals and empty/caught-up
results use status semantics. Focused copy and identity checks pass;
platform-announcement timing remains deferred.

Commit `e28b78ed57` gives the shared Callout primitive urgency semantics:
informational/success messages are status updates, while warning/error messages
are alerts. Durable Host and Workspace-recovery callouts no longer rely on
visual severity alone. Authored role and identity checks pass; platform
announcement timing remains deferred.

Commit `a90fae5873` aligns compact Workspace headers with their 28 px hover
controls and moves high-frequency rail creation, scope, Host-detail, and
Workspace-recovery actions from 18 px compact buttons to the shared medium
target. Identity checks pass; rendered density and zoom proof remains deferred.

Commit `f6318ea907` makes the keyboard-selected Workspace header or Session row
the active accessibility descendant while the rail retains keyboard focus. The
same audit confirmed that GPUI collapses repeating activity animations to a
single static frame when Dez resolves reduced motion. Identity checks pass;
platform focus and motion inspection remains deferred.

Commit `9930e86677` retires the unfinished 56 px icon presentation for Dez
v0.0.1. Legacy `icon` values remain readable but resolve to compact, the
smallest layout that presents the full supervision, search, evidence, action,
and recovery hierarchy without clipping. Width and identity checks pass;
rendered compact proof remains deferred.

Commit `4a102fc50e` makes the terminal-first onboarding workflow a named region
and Start/Watch/Review a named accessible list. Its safety copy now sits above a
wrapping action row, and Copy Codex Hook/New Terminal use 28 px targets. Identity
checks pass; rendered zoom and narrow-window proof remains deferred.

Commit `933e3f515f` forces diagnostics and metrics off for every non-Zed build
before the inherited telemetry client can queue or upload them. Dez onboarding
and Settings no longer offer those misleading opt-ins; Anthropic retention
remains visible as a separate model-request policy. Source and identity checks
pass; compiled network proof remains deferred.

Commit `2680937952` hides the Auto Update section in Dez because the fork
updater is deliberately disabled and defaults off. The compatibility key stays
readable and official Zed keeps the control. Source and identity checks pass;
rendered Settings proof remains deferred.

Commit `9239006d4b` removes Collaboration Panel button/dock/width controls from
the Dez Panels page now that the page, panel entry points, and commands are
removed. Compatibility parsing and official Zed UI remain intact. Identity
checks pass; rendered Settings proof remains deferred.

Commit `33f7ff5893` adds non-visual shortcut metadata to shared text and icon
buttons. Session scope controls now announce Shift+A and Review Brief controls
announce Shift+V, matching their visible action-aware tooltips. Identity checks
pass; platform announcement proof remains deferred.

Commit `2fc5226a51` requires official Zed identity as well as explicit
auto-connect before the language-model registry eagerly authenticates all
providers. A legacy `auto_connect: true` can no longer make Dez contact cloud
providers during ordinary startup. Source and identity checks pass; compiled
network proof remains deferred.

## Milestone 3: Serious multi-actor work {#milestone-3}

- Evaluate and extend upstream remote/headless infrastructure for SSH session
  continuity.
- Add another terminal-agent adapter after the generic contract is proven.
- Warn about overlapping modified files, shared worktrees, risky branch
  operations, and conflicts with unsaved human edits.
- Add explicit worktree brokering and reusable task recipes.
- Add workspace return briefs and adapter-approved agent resumption.

**Acceptance:** Local and remote sessions coexist in one workspace; a host can
disconnect safely; two agents remain visible; conflict warnings are advisory,
traceable, and low-noise.

## Milestone 4: v0.0.1 release {#milestone-4}

- Complete first-run guidance, recovery copy, accessibility, keyboard and
  pointer parity, crash behavior, packaging, migration, and release provenance.
- Validate the hero workflow with design partners on real repositories.
- Measure session recovery, agent-state truthfulness, attention routing, review
  use, startup, memory, and crashes.
- Document close, detach, reconnect, resume, and terminate semantics.

**Acceptance:** A new user completes the activation loop without reading source
code, all launch gates pass, and known limitations are documented.

## Progress {#progress}

- [x] 2026-07-22: Merge the locally recorded upstream base through
      `f14fea9bf3c93797d5161f7440ed418655bc6c57`.
- [x] 2026-07-22: Rename public executable and application identity to Dez
      v0.0.1 while preserving the legacy storage boundary.
- [x] 2026-07-22: Repair dead canvas states, panel-to-pane visibility, focus
      treatment, pane accessibility, offline startup, and truthful terminal-agent
      process status in the current source slice.
- [x] 2026-07-22: Reconcile the supplied product notes into canonical Fork
      Notes, product strategy, upstream policy, and this roadmap.
- [x] 2026-07-22: Reconcile the revised consolidated Product and Execution Plan
      without resetting source progress. Adopt complete-product positioning,
      Evidence provenance, adapter capabilities, and long-range requirements;
      reject its competing authority, per-slice build mandate, and flattened
      Run/Session lifecycle.
- [x] 2026-07-22: Refresh `upstream/main` to `cf61b7ccc5d5`, confirming 72
      upstream commits after the current merge base and stable tag `v1.11.3`.
- [x] 2026-07-22: Add the first derived attention view to the Session Rail.
      Waiting, failed, completed-unread, and notified terminal sessions surface
      without creating duplicate ownership; an empty inbox offers an explicit
      return to all sessions.
- [x] 2026-07-22: Replace the empty Session Rail's project-first actions with
      New Terminal, New File, and Open actions, and remove remaining hard-coded
      Zed branding from shell collaboration and agent-history messages.
- [x] 2026-07-22: Restore Session Rail search using its existing upstream
      editor and fuzzy matching path. Search now renders visibly, focuses from
      the existing action, clears explicitly, composes with attention mode, and
      reports deliberate empty results.
- [x] 2026-07-22: Rehearse the next merge in an isolated worktree and record
      nine presentation/settings conflicts in the Upstream Feature Ledger.
- [x] 2026-07-22: Refresh the permanent upstream gate again. `upstream/main`
      advanced to `9d0ef37a2571` (81 commits after the merge base), stable
      remains `v1.11.3`, and a disposable detached-worktree rehearsal was
      aborted and removed after recording ten conflicts. The new conflict is
      `workspace.rs`; the active dirty product worktree was not modified.
- [x] 2026-07-22: Checkpoint the completed supervision source slice as
      `c2335969f994`, then integrate `upstream/main` `9d0ef37a2571` with the
      reversible two-parent merge `2be63cfea347`. Resolve all eleven actual
      conflicts according to Fork Notes, preserve the Dez sidebar-native shell,
      retain compatible Markdown/workspace/task behavior, and pass formatting,
      metadata, diff, and identity static gates.
- [x] 2026-07-22: Add a locally runnable fork identity checker and GitHub
      workflow for pull-request identity guards, scheduled upstream drift,
      no-commit merge rehearsal, conflict artifacts, and one maintained status
      issue.
- [x] 2026-07-22: Audit current app, workspace, project, terminal, agent, and
      remote ownership and record the source-backed Architecture Baseline.
- [x] 2026-07-22: Define the first versioned terminal Host/Session protocol
      seam with independent host and computation IDs, bounded replay positions,
      truthful lifecycle states, and distinct detach and terminate commands.
- [x] 2026-07-22: Implement an entity-independent in-process Host/Session
      registry with explicit Missing and Incompatible results, bounded
      sequence-addressed replay, and tested create, detach, reattach, and
      terminate transitions. It does not yet own a PTY or survive GUI exit.
- [x] 2026-07-22: Adapt real local terminal entities to the in-process host.
      Host identity derives from persisted installation identity; closing a
      view detaches, the Session Rail shows and reattaches the same entity, an
      explicit action terminates it, and observed process exit is distinct from
      view closure. Remote terminals remain outside this local adapter.
- [x] 2026-07-22: Extend terminal-item and agent-terminal persistence with
      nullable Host/Session references. New rows retain computation identity,
      legacy rows keep their prior shell-restoration behavior, live in-process
      sessions reattach to the same entity, and stale or malformed references
      render a display-only unavailable surface without starting a replacement
      process or rerunning an agent init command.
- [x] 2026-07-22: Scaffold the `dez-terminal-host` helper boundary. It binds a
      user-private local socket, reads a private token file, fails closed on
      authentication, host, protocol, malformed-frame, and frame-size errors,
      and exposes a sequential client transport. The helper now owns raw local
      PTYs on poll-driven I/O threads, routes output into bounded replay, and
      accepts create, list, attach, detach, input, resize, and terminate. Dez
      launch wiring remains open; the terminal crate now has a controller-backed
      hosted-emulator seam for replay output and delegated input, resize,
      detach, and terminate.
- [x] 2026-07-22: Add an opt-in GUI runtime supervisor and release packaging
      seam. With `DEZ_EXPERIMENTAL_TERMINAL_HOST=1`, Dez reuses a private
      persistent token, connects before spawning, removes only a validated
      stale socket, launches the sibling helper without shell interpolation,
      and retains one ordered background connection. The macOS bundle builds,
      copies, and signs the helper, and identity guards require it.
- [x] 2026-07-22: Route ordinary local shells through the helper when the
      experimental host is enabled. Hosted terminals keep the client-side
      emulator and delegate create, replay, input, resize, metadata, detach,
      reattach, and terminate to the helper. Saved terminal and agent surfaces
      briefly await startup reconciliation instead of failing a healthy
      session immediately. The Session Rail observes host snapshot revisions,
      preserves host/session row identity, title, working directory, and PID,
      and distinguishes Detached, Reconnecting, and Exited. Task terminals and
      the default-off path retain their existing backend until the consolidated
      verification gate.
- [x] 2026-07-22: Harden the experimental client boundary. Enabling the host
      now waits for readiness and fails visibly instead of silently creating a
      GUI-owned shell. The Session Rail shows connecting, failed, and
      reconnecting callouts; the ordered transport reconnects before later
      commands without replaying an uncertain command. Input is chunked and
      raw replay is capped below the JSON frame ceiling. Runtime, token, and
      socket paths reject symlink substitution, and cleanup verifies the bound
      socket's device and inode before removal.
- [x] 2026-07-22: Turn stale terminal references into a deliberate recovery
      surface. Missing, foreign, malformed, or unavailable sessions preserve
      their evidence, explain that Dez started no replacement process, and
      expose a single New Terminal action instead of leaving a dead emulator.
- [x] 2026-07-22: Add the first deterministic Run review projection. Native
      agent rows and terminal-agent rows expose Open Review Brief from the
      Session Rail and open it as an ordinary Markdown pane. The brief derives
      actor, lifecycle, host/session identity, workspace evidence, and observed
      diff counts from existing owners; it explicitly reports absent command,
      check, and file-change evidence and never infers a clean worktree or a
      passing check. This is a value projection, not a duplicate Run database.
      Hover actions, the thread context menu, and the command-palette action
      `sidebar::OpenSelectedReviewBrief` reach the same pane-based surface.
- [x] 2026-07-22: Make terminal attention restart-safe. Agent-terminal metadata
      now stores an additive attention bit, writes it immediately when a bell
      is raised or acknowledged, restores it into the owning surface, and
      projects it into the Session Rail without requiring a live entity.
- [x] 2026-07-22: Replace the compatibility attention bit as product truth with
      a typed condition/presentation model. Active versus resolved,
      unread versus acknowledged, normal versus urgent, one-hour snooze,
      resume, explicit resolution, and seven-day expiry for observed bell
      events are independent. Opening an owner acknowledges but does not
      resolve it; the Session Rail keeps acknowledged active work in Attention,
      hides muted work until its deadline, labels snoozed rows, and orders
      structured permission/failure conditions first. The old column remains
      only for additive migration and older-reader compatibility.
- [x] 2026-07-22: Remove the compiled Zed Pro trial-end overlay and its reset
      action instead of hiding it behind a false predicate. Provider-limit
      errors now offer truthful provider/model recovery without an upstream
      subscription CTA; native-agent, authentication, collaboration, update,
      REPL, and recovery copy identify Dez or clearly label an upstream link.
      Terminal hover actions now have context-menu parity, with destructive
      detach/terminate/remove kept in a separated final menu group.
- [x] 2026-07-22: Add open pane files to Workspace Evidence. Add/remove/path
      changes recompute stable authoritative open-surface records, sort and
      deduplicate paths, cap retention at 256, preserve truncation across root
      refreshes, and project the result into deterministic Review Briefs
      without claiming those files changed. Add first-class Attention and
      Evidence settings pages with actual notification, accessibility,
      confidence, adapter, and bounded-restore controls; rename the surrounding
      settings groups around Dez's Workspace/Session/Agent mental model.
- [x] 2026-07-23: Add explicit user-selected Workspace review evidence.
      Commit `a8ce563373` adds Command Palette actions to add, remove, or clear
      selected paths and matching file-tab context actions. User selections
      have stable, authoritative provenance, survive passive open-file
      recomputation and tab closure, and stop at 128 paths with visible
      feedback. Review Briefs label them Selected path and suppress the
      duplicate Open file projection. Focused model tests are authored;
      formatting, diff, and identity checks pass. Commit `e101b63e43` adds an
      additive Workspace database column, saves only explicit selections, and
      rehydrates them with current Host classification. A focused database
      round-trip test is authored. Commit `f535c5e6ae` then makes the tab menu
      expose only the state-appropriate Add or Remove action. Compiled
      interaction and restart proof remain in the consolidated gate.
- [x] 2026-07-22: Make Session Rail review actions preserve owner context.
      Pointer and context-menu actions first activate the existing terminal or
      agent owner, then open the generated Markdown brief in an ordinary
      right-hand pane. Review remains pane-native and no duplicate terminal,
      conversation, or Run owner is created. The command-palette action now
      follows the same owner-first behavior; briefs expose section jumps and
      clickable local paths for Workspace, open-file, and terminal-CWD evidence.
- [x] 2026-07-22: Add the first structured Codex terminal-adapter path. Codex
      lifecycle hooks can send authenticated events to the helper using the
      terminal's stable Host/Session environment. The helper retains bounded
      provider session, lifecycle, resumability, attention, command, and exit
      evidence. The Session Rail prefers this truth over process detection,
      shows exact permission/review states, and acknowledges attention by
      focusing the existing terminal. Hook installation remains explicit and
      the build/live-restart gate remains pending.
- [x] 2026-07-22: Project the helper's bounded event journal into review and
      recency UI. Review briefs now include ordered recent activity, observed
      commands, and conservative checks; structured terminal rows use the last
      meaningful adapter event for recency instead of their creation time.
- [x] 2026-07-22: Add honest adapter onboarding. Lower-confidence process
      detection is labeled **Detected**, and detected Codex rows expose a
      context action that copies the reviewed hook JSON. The action disappears
      after structured Host/Session evidence arrives; Dez never edits Codex
      configuration automatically.
- [x] 2026-07-22: Close the startup request-ordering hole in workspace
      recovery. A launch-time CLI path, URL, or extension request no longer
      bypasses last-session restore. Dez restores the durable workspace set,
      applies the initial request, then releases additional queued open
      requests; workspace garbage collection observes the same boundary.
- [x] 2026-07-22: Move the restore lifecycle boundary into `AppSession`.
      Ordered Pending, Restoring, and Ready state is now observable and rejects
      duplicate or out-of-order transitions without panicking. Live workspace
      entities remain window-owned so this change does not pin GPUI resources;
      durable membership consolidation remains open.
- [x] 2026-07-22: Give `AppSession` the identity-keyed durable workspace
      registry. Persisted IDs and prior viewport associations are adopted before
      entity restoration, live workspaces update their viewport association,
      and explicit removal unregisters membership. SQLite remains the durable
      serializer and the registry deliberately owns no GPUI entities.
- [x] 2026-07-22: Harden hosted-session recovery UX and transport liveness.
      Session Rail reconciliation failures now open a visible display-only
      recovery surface instead of doing nothing, bounded reconnect attempts
      keep one dead helper from blocking the ordered command queue forever,
      permanent identity failures stop immediately, and exited PTY handles are
      reaped without discarding their review snapshot.
- [x] 2026-07-22: Make the Host boundary self-describing and reduce idle work.
      The authenticated handshake now negotiates additive lifecycle, replay,
      metadata, structured-agent, and attention capabilities with false-by-
      default compatibility. Adapter snapshots separately declare structured
      state, attention, activity, command, check, and resumability evidence.
      Stable snapshot polling backs off to one second, remains responsive at
      250 ms after changes, and retries errors at 500 ms; pushed events and
      heartbeats remain explicit follow-up work.
- [x] 2026-07-22: Add source-backed changed-file navigation to Review Briefs.
      Native agent action logs now provide sorted, deduplicated absolute paths
      for unreviewed buffers, rendered as local file links beside conservative
      diff totals. Terminal review projections honor adapter command, check,
      and activity capability flags instead of inferring support from provider
      identity; Git ownership and terminal file provenance remain open.
- [x] 2026-07-22: Give changed native runs a one-step diff path. Session Rail
      rows with observed action-log changes now expose Review Changes through
      both hover controls and the context menu, activate the existing owning
      thread, and open the ordinary agent diff surface without duplicating the
      Run. The action remains absent when no source-backed changes exist.
- [x] 2026-07-22: Strengthen Session Rail row semantics for assistive
      technology. Shared rows now announce remote/archive state and observed
      added/removed line counts in addition to title, actor, state, Host,
      selection, and unread activity. Permission wording is consistent between
      visible tooltips and accessible labels, and duplicate state announcements
      are suppressed when a richer state label is already present.
- [x] 2026-07-22: Complete another outward identity sweep. Native empty drafts
      now say New Dez Agent Thread; component previews, task/edit-prediction
      settings, GPU diagnostics, Windows IPC failures, and CLI help no longer
      present the fork as Zed. Remaining visible Zed labels are restricted to
      truthful upstream services/providers/links or compatibility identifiers.
- [x] 2026-07-22: Add an explicit Host liveness primitive. The negotiated
      heartbeat capability echoes a caller-supplied nonce and Host observation
      time without mutating or creating Sessions, allowing future supervisors
      to reject delayed responses.
- [x] 2026-07-22: Add reconnectable bounded Host event cursors. The helper
      retains at most 512 monotonic snapshot envelopes and returns at most eight
      per response; the GUI begins from one authoritative list, resumes after
      its last delivered cursor, applies typed metadata/state events, and
      requests a full resync when retention truncates history.
- [x] 2026-07-22: Replace GUI snapshot polling with negotiated Host push. A
      second authenticated socket becomes a cursor-resumable subscription,
      preserving the simple ordered command channel while the helper broadcasts
      coalesced state changes to every subscriber. Batches remain bounded,
      reconnects resume without command replay, retention loss forces a list
      resync, and peers without the capability retain the bounded polling
      fallback. Framing and notifier tests are source-authored; live recovery
      verification remains in the consolidated gate.
- [x] 2026-07-22: Make Review Brief repository ownership explicit without
      inventing another Run database. The projection now names its objective
      and reads Git-store worktree, main-worktree, branch, changed-path count,
      conflicts, untracked state, bounded path links, and truncation. A terminal
      resolves the most specific repository containing its working directory;
      whole-repository changes are labeled as observations and never attributed
      to that agent. Structured activity, command, and check events also retain
      their observed cwd as a local navigation target.
- [x] 2026-07-22: Fail closed on consequential agent responses. Permission and
      input response capabilities are separate, false by default, and not
      implied by attention detection. Codex hooks v1 only observe lifecycle, so
      Dez omits synthetic approval/input actions and tells the reviewer to use
      the owning terminal. A future adapter must supply scoped, time-bounded,
      actor-attributed audit evidence before these controls can appear.
- [x] 2026-07-22: Add bounded terminal-adapter file provenance without parsing
      terminal output. Recognized completed Codex write, edit, and patch hooks
      resolve direct and patch-declared paths against the hook cwd, deduplicate
      them, retain at most 64 per event, and surface truncation. Review labels
      these paths as adapter targets rather than successful changes; repository
      status and native action logs remain the sources for observed changes.
- [x] 2026-07-22: Complete another reachable-product identity sweep. OAuth
      browser pages, extension cards, Copilot and local-provider setup, remote
      and debugger errors, system diagnostics, DAP/OAuth client names, and
      outbound OpenRouter titles now identify Dez. Actual upstream Zed accounts,
      hosted models, and Edit Predictions are labeled as upstream. An
      incompatible upstream prediction service can be dismissed but no longer
      offers an “Update Zed” action that would install the wrong product. The
      identity script now guards these boundaries.
- [x] 2026-07-22: Make the trusted Codex hook path discoverable without
      auto-modifying configuration. Eligible durable Codex rows now label Hook
      setup and expose Copy Codex Hook Setup through both hover and context
      actions; the affordance disappears after structured Host/Session evidence
      arrives. Installation remains an explicit user-reviewed Codex action.
- [x] 2026-07-22: Make terminal failure copy operational. Center and panel
      launch failures now say no process started and point from settings to an
      explicit New Terminal retry. Durable-Host connecting, reconnecting, and
      failed callouts distinguish untouched existing work from absent fallback
      computation and provide wait, restart, or next-launch recovery guidance.
- [x] 2026-07-22: Resolve task-terminal eligibility for v0.0.1. Task terminals
      remain GUI-owned because rerun, completion, cancellation, and task-status
      semantics do not yet define safe cross-GUI survival; retaining one could
      contradict a visible cancellation. Ordinary durable shells remain the
      explicit path for long-lived supervision.
- [x] 2026-07-22: Demote upstream account and collaboration chrome from the Dez
      shell. Sign-in, plan/user, and collaboration-connection controls no longer
      render in the Dez workspace bar; the View menu omits Collab; Help removes
      upstream bug, feature, email, social, and hiring routes while retaining
      clearly attributed upstream documentation and repository links.
- [x] 2026-07-22: Add the first typed workspace-evidence projection. Review
      briefs distinguish workspace roots from observed terminal working
      directories rather than flattening both into unlabeled paths, while the
      Session Rail exposes copy actions for the working directory and stable
      Host/Session reference. Evidence remains a projection, not a new owner.
- [x] 2026-07-22: Establish the first outward Session Rail orientation slice.
      The rail now names its purpose, communicates whether action is needed,
      exposes All and Attention as visible scopes, provides a clear New Terminal
      Session action, removes the duplicate footer filter, and preserves the
      selected session across scope changes when it remains visible; otherwise
      it chooses the nearest or first actionable row. Search and
      no-session dead ends now explain the state and provide a direct Clear
      Search or New Terminal action. No Dez process was running, so rendered
      verification remains in the consolidated gate.
- [x] 2026-07-22: Make Session Rail orientation measurable and keyboard-stable.
      All and Attention scopes expose live totals, status copy handles singular
      and plural counts, and assistive labels report the same facts. Rebuilds
      retain selection by Thread/Terminal identity across reorder and filtering
      rather than letting an old numeric index focus unrelated work; removal
      falls back to the nearest actionable row.
- [x] 2026-07-22: Make terminal-native empty and evidence states truthful. The
      no-project onboarding gate no longer hides standalone/restored sessions,
      search results, or the caught-up attention view. Structured terminal rows
      now summarize recognized checks as passed, running, or failed and fall
      back to observed command counts when no classified check exists; the
      visible icon/text/color and accessibility label derive from one fact.
- [x] 2026-07-22: Preserve exceptional Host lifecycle states through the
      product projection. Missing and protocol-incompatible sessions now remain
      distinct in Session Rail and Review Brief state rather than degrading to
      Saved. Host reconnect/failure callouts can copy their complete diagnostic
      detail without exposing it permanently in the compact rail.
- [x] 2026-07-22: Add explicit keyboard paths for the supervision loop. With
      Session Rail focused, Shift-A toggles Attention and Shift-V opens the
      selected Review Brief on macOS, Linux, and Windows; scope and review
      tooltips expose the bindings through the existing keymap authority.
- [x] 2026-07-22: Separate destructive terminal termination from ordinary
      lifecycle controls. Detached and reconnecting Host rows render a red Stop
      action and require a critical confirmation that names the durable session
      and the effect on computation. Hover, context-menu, and keyboard routes
      share the gate; live detach and exited/saved cleanup remain one step. If
      the Host rejects termination, the row remains authoritative and Dez shows
      a visible failure toast rather than relying on logs.
- [x] 2026-07-22: Complete the per-Workspace no-session recovery path. Empty
      project groups provide an inline New Terminal action scoped to that
      Workspace and restore a closed group when necessary. While filtering,
      Session Rail reports matching-session counts with search semantics rather
      than presenting filtered absence as a caught-up state.
- [x] 2026-07-22: Align top-level navigation with Dez's terminal-native shell.
      File leads with New Terminal and disambiguates New File; View exposes the
      Session Rail directly; title-bar and collapsed status controls use Session
      Rail terminology instead of the generic upstream sidebar label.
- [x] 2026-07-22: Remove inert account-chrome settings from the Dez path. Show
      Sign In, Show User Menu, and Show User Picture no longer appear because
      their controls are intentionally absent from Dez. Compatibility schema
      fields remain, while the surviving placement and chrome settings use
      Session Rail terminology and describe behavior that actually renders.
- [x] 2026-07-22: Prevent cached structured work state from hiding transport
      failure. Session Rail composes work and non-live transport labels, while
      Review Brief state gives Missing, Incompatible, Reconnecting, and
      Detached ownership priority and records the matching evidence risk.
      Bounded state chips shrink at narrow rail widths but preserve the full
      accessible label.
- [x] 2026-07-22: Add explicit honest review outcomes without a duplicate Run
      store. Each editable Review Brief includes reviewer-owned Continue,
      Request changes, and Accept as reviewed checkboxes plus a warning that
      notes do not mutate lifecycle truth. Archive, remove, detach, and
      confirmed terminate remain separate owner-backed actions; unsupported
      discard is absent.
- [x] 2026-07-22: Route structured Host attention through the accessible window
      announcement setting even while its terminal surface is detached. The
      rail keeps only transient transition IDs, requests attention once for a
      newly active authoritative condition, and allows a later condition to
      announce after acknowledgement clears the prior ID.
- [x] 2026-07-22: Stop advertising the fork as Zed in outbound HTTP identity.
      The process user agent now derives from the configured Dez application
      name while preserving version, operating-system, and architecture
      diagnostics.
- [x] 2026-07-22: Extend App Session from identity membership to ordered durable
      state. A compact KVP record preserves Workspace order, active selection by
      viewport, and explicitly unresolved prior IDs; database-resolved records
      reconcile without reordering, and live creation, activation, and removal
      update the same state without storing GPUI entities. True viewport
      composition and consolidated restart proof remain open.
- [x] 2026-07-22: Separate global Workspace membership from viewport
      composition. Ordered viewport records now contain ordered Workspace IDs
      and active selection, migrate the earlier compact state, accept the same
      Workspace in multiple viewports, restore complete MultiWorkspace groups,
      and make removal viewport-aware so closing one copy cannot erase another.
      Focused migration, deduplication, reconciliation, and removal tests are
      authored; live entity composition and runtime proof remain open.
- [x] 2026-07-22: Convert the historical visual audit into first-party source
      fixes. The welcome path is terminal-first, carries the Dez product
      promise, and resists narrow header wrapping; settings, installation,
      update, and permission copy use product identity; active AI onboarding
      no longer markets Zed subscriptions and instead explains provider-owned
      configuration. Historical screenshots predate this source, so the
      intended Dez artifact still needs rendered verification.
- [x] 2026-07-22: Reframe onboarding around Dez's activation loop instead of
      upstream account conversion. The first-run page now teaches Start, Watch,
      and Review; provides New Terminal; explains close versus detach versus
      terminate and Host-dependent persistence; and keeps optional ACP agents
      without a bundled Zed plan, sign-in funnel, or trial action. Codex hook
      installation and rendered verification remain open.
- [x] 2026-07-22: Give terminal Session Rail rows an explicit information
      hierarchy. Actor, state, Host, scope, changes, and recency are separate
      accessible fields; identity and scope truncate as one left cluster while
      observed activity remains stable on the right. Native-thread checks and
      rendered narrow-width verification remain open.
- [x] 2026-07-22: Apply the same metadata grammar to native agent rows. Actor
      identity and Draft, Running, Waiting for permission, Error, or Completed
      state are explicit and accessible instead of being inferred only from an
      icon. Check evidence and rendered narrow-width verification remain open.
- [x] 2026-07-22: Establish the first Workspace-owned `EvidenceSet`. Visible
      worktree roots now have deterministic identity plus provenance,
      confidence, Host, lifecycle, and truncation truth; worktree and remote
      context events recompute it. Open Run Briefs consume that authority and
      closed rows fall back explicitly to saved metadata. File and explicit
      choice evidence are now implemented. Commit `0e6507756e` retains a saved
      hosted terminal's last-known cwd as Unresolved when attach fails and
      replaces it with Current truth on successful reattach. Commit
      `ea2bb18453` observes live Host snapshot revisions so transport
      reconnection, missing/incompatible truth, exit, and snapshot cwd changes
      reconcile the same Session-owned record. Compiled runtime proof remains.
- [x] 2026-07-23: Make authoritative evidence mutation an internal Workspace
      boundary. Commit `0f8740b1a1` makes all EvidenceSet mutation methods and
      selection outcomes crate-private; downstream search, Git, settings,
      conversation, and review code can read records but cannot attach roots or
      invent explicit choices merely by existing. Formatting, diff, and
      identity checks pass.
- [x] 2026-07-23: Isolate provisional Workspace evidence identity. Commit
      `af232402f5` replaces the shared `workspace:pending` prefix with one
      stable UUID-backed namespace per EvidenceSet. Same-path provisional
      Workspaces therefore cannot collide in downstream identity maps. A
      focused isolation test is authored; formatting, diff, and identity checks
      pass.
- [x] 2026-07-23: Bind retained terminal Sessions to durable Workspace
      ownership. Commit `a4047d95c0` adds an optional backward-compatible
      Workspace ID to Host snapshots, associates both in-process and helper
      Sessions from TerminalView, and makes Session Rail prefer exact ownership
      over cwd prefix inference. Compatibility, Host-model, and integrated
      same-cwd/two-Workspace tests are authored; compiled detach/reattach proof
      remains in the consolidated gate.
- [x] 2026-07-23: Add direct Workspace evidence isolation proof. Commit
      `6f2061d2c7` covers live terminal cwd evidence in two same-path Workspace
      entities and selected-evidence persistence in two durable Workspaces.
      Records remain distinct, and clearing one Workspace leaves the other
      unchanged. Formatting, diff, and identity checks pass; focused tests are
      authored and await the consolidated compile gate.
- [x] 2026-07-23: Scope Session Rail branch metadata to the owning Workspace.
      Commit `27279ca542` replaces the global last-writer-wins path map with
      per-Workspace repository snapshots for open terminal and agent rows.
      Closed historical rows show a branch only when every open Workspace that
      knows the path agrees; conflicting values are omitted rather than
      guessed. A focused fallback regression is authored; formatting, diff,
      and identity checks pass, while compilation remains deferred.
- [x] 2026-07-23: Scope graphical project settings to the originating
      MultiWorkspace. Commit `a2d733eea3` stops a Settings window from merging
      project files from unrelated OS windows, including after its source
      window becomes unavailable. Project-file discovery, restricted-mode
      lookup, opening, and mutation now share the scoped resolver; user settings
      retain application-global semantics. Two cross-window regressions are
      updated and await the consolidated compile gate. Follow-up `498f94a525`
      refreshes that scoped inventory before a reused Settings window applies
      its new target, preventing stale project files from the prior viewport
      from receiving the action.
- [x] 2026-07-22: Feed live terminal cwd changes into Workspace evidence. Each
      record carries stable terminal-Session provenance, root refreshes preserve
      it, and Run Briefs include only cwd evidence owned by their terminal so
      unrelated sessions cannot leak scope. File, explicit-choice, lifecycle,
      and reconnect evidence remain open.
- [x] 2026-07-22: Add terminal evidence lifecycle truth. Activity marks an
      owning Session's cwd evidence Current; observed process exit marks it
      Stale without deleting review history; and Run Briefs disclose the stale
      observation as a risk. Explicit removal and reconnect reconciliation
      remain open. Commit `0e6507756e` adds restore/reattach reconciliation;
      `ea2bb18453` adds live Host snapshot lifecycle reconciliation. Compiled
      runtime proof remains open.
- [x] 2026-07-22: Complete the upstream-integration compatibility repair and
      focused runtime-facing test slice. Fifteen terminal tests, eight helper
      tests, and three Session Rail terminal lifecycle tests pass. Stale
      onboarding actions no longer panic built-in keymap loading, draft agent
      rows remain visible, and local startup no longer authenticates cloud
      providers when `auto_connect` is false.
- [x] 2026-07-22: Complete the warning-free consolidated arm64 app and helper
      build at `da562e14bb403af815cbab9802225dda0b2418c8`, then build the CLI
      with the same locked low-disk profile. Launch only the exact
      `target/debug/dez`; exclude and do not open the historical untracked
      `dist/Superzed.app`.
- [x] 2026-07-22: Prove the external helper's process-level lifetime. Helper
      PID `48768` survives GUI PID `48519`, reparents to PID 1, and is reused by
      relaunched GUI PID `50092` with one instance, the same socket, and Host ID
      `d9670db8-e498-5537-a9d8-f99ad098f4aa`. Same hosted-Session replay and
      child-PID reattachment remain unproven because the desktop is locked.
- [x] 2026-07-22: Repair the blocking macOS client-decoration layout exposed by
      the first unlocked screenshot. The Session Rail no longer pins both
      horizontal edges or covers the editor/welcome surface; it uses its
      configured width, anchors only its active edge, and presents an empty
      project as a vertical explanation plus full-width New Terminal action.
      The exact corrected raw executable builds and runs while reusing the
      existing helper. A fresh rendered capture remains open because the bare
      Mach-O process is not targetable by the approved accessibility surface.
- [x] 2026-07-22: Harden and audit local debug packaging at `ce11c4ed3d`. The
      bundle script reuses the consolidated host artifacts, restores its
      manifest after failures, avoids the pinned bundler's broken colour path,
      and skips release-only remote-server work. The ad-hoc arm64 bundle passes
      deep strict signature verification with identifier `dev.dez.Dez-Dev`,
      version `0.0.1`, scheme `dez-dev`, and app, CLI, helper, and bundled Git.
      Permission prompts now describe developer-tool requests truthfully.
- [x] 2026-07-22: Run warning-denied Clippy across all terminal Host targets.
      Current Clippy exposed one equal-operands normalization in the fallback
      theme dependency; `3ad224dfd6` replaces `100 / 100` with the identical
      `1.0`, after which the complete Host lint graph passes.
- [x] 2026-07-22: Preserve replay geometry as protocol truth. Protocol 4 records
      terminal dimensions in snapshots and replay fragments; resize updates
      them at the Host boundary, and the renderer applies the matching size
      while replaying. Focused model, hosted-renderer, and helper lifecycle
      tests pass.
- [x] 2026-07-22: Make ordinary live shells first-class Session Rail rows and
      repair the workspace footer. Detection now classifies an agent instead of
      deciding whether a terminal exists, active shells select correctly, the
      footer stays on one line, dynamic labels truncate, and a redundant
      default `main` worktree label is suppressed. A focused visual-model test
      now creates a plain workspace shell and proves its live, unclassified,
      selected Session Rail projection.
- [x] 2026-07-22: Exercise the packaged protocol 4 boundary with a live PTY.
      Host `d9670db8-e498-5537-a9d8-f99ad098f4aa` created Session
      `040b4465-5f0a-416b-9cb3-549da1a2a28b` with shell PID `53394`; 88 replay
      chunks retained 80x24 and 132x41 geometry plus output markers on both
      sides of resize, then reported explicit Detached state. GUI-driven
      restart and same-process reattachment remain open while macOS is locked.
- [x] 2026-07-22: Make local macOS packaging deterministic. Commit
      `fcd1d06564` signs the CLI, terminal Host, Git, and main executable
      inside-out before sealing the app. The resulting arm64 `Dez Dev.app`
      passes deep strict verification with ad-hoc CDHash
      `4aff38edc9de37515e5488a216c2d648a53c0b01` and launches through the exact
      audited bundle path; `dist/Superzed.app` remains unopened.
- [x] 2026-07-22: Give Session Rail an explicit macOS accessibility landmark
      and harden narrow-width copy. Commit `2dd523b6e9` exposes the rail as a
      named `Complementary` region and truncates overview and empty-Workspace
      status instead of permitting word-level wrapping. The focused `sidebar`
      graph compiles; bundle rebuild and accessibility-tree inspection remain
      separate gates.
- [x] 2026-07-22: Repair the compact Session Rail width contract exposed by the
      later running-bundle screenshot. Commit `79f69b273c` makes both server-
      and client-decoration roots paint at the same mode-resolved width the
      workspace reserves: 240 px compact, 56 px icon, and at least 380 px
      detailed. Regression assertions cover all three modes. Formatting and
      diff checks pass. The complete post-fix app and signed bundle now build;
      the focused test and rendered proof remain open, so neither is inferred
      from compilation.
- [x] 2026-07-23: Give compact and zero-session chrome enough room to breathe.
      Commit `1ebb7c79d4` raises the compact rail cap from 240 px to 280 px and
      the resize floor from 200 px to 240 px, hides All 0 / Attention 0 until a
      session exists, and suppresses the inert search row at zero while keeping
      an existing query visible for recovery. The visibility rules have a
      focused model assertion. A clean
      `cargo check --locked -p sidebar --lib -j1` passes in 14m34s after only
      regenerable Cargo caches were cleared; rendered proof remains open.
- [x] 2026-07-22: Keep an empty loaded project actionable after restoration.
      Commit `4829f6b052` makes the visible-worktree launch surface outrank a
      missing legacy welcome flag, preventing the blank center shown beside the
      crushed rail. Find File, New File, and New Terminal remain the deliberate
      recovery actions. The focused assertion is authored; the shared
      storage-bound test target and rendered proof remain open; the complete
      corrected app and signed bundle build pass.
- [x] 2026-07-23: Enforce a quiet local-first launch in source. The stale
      bundle log showed inherited settings starting Zed websocket, LiveKit, and
      Zed-hosted prediction activity behind removed chrome. Commit
      `1d5c03d88b` gates automatic upstream authentication and eager Collab
      panel construction to official Zed, and ignores inherited Zed/Mercury
      prediction providers while preserving explicit non-Zed providers. Commit
      `9318b270d9` guards the policy statically. The corrected bundle rebuild
      passes and the running app held no established or listening TCP socket
      during the recorded soak.
- [x] 2026-07-23: Bound the lower workspace footer under narrow widths and real
      repository names. Commit `0d8496969f` gives project identity and
      worktree/branch controls independent shrinkable, overflow-hidden regions,
      allowing their existing one-line truncation and tooltips to work without
      colliding in the fixed-height row. Formatting and diff checks pass;
      compile and bundle rebuild pass; rendered narrow-width proof remains open.
- [x] 2026-07-23: Consolidate the lower utility chrome. Commit `abc4f8bedb`
      removes the standalone Dez Command Search row, preserves the action and
      accessible tooltip in the existing icon bar, suppresses the unowned
      upstream update surface, and keeps the Canvas prefix row on demand.
      Official Zed behavior is unchanged. Formatting and diff checks pass;
      compile and bundle rebuild pass; rendered proof remains open.
- [x] 2026-07-23: Rebuild and audit the corrected source head
      `679cdc28445c`. The arm64 app and protocol-4 terminal Host complete, the
      inside-out ad-hoc bundle passes deep-strict verification with CDHash
      `0dc2e1e872b88cbd6288f1bea5455fbc48271cc5`, and live PID `85053` resolves
      to the exact intended bundle executable. The Mac remains locked, so the
      corrected visual and accessibility matrix is still an explicit gate.
- [x] 2026-07-23: Prove durable viewport normalization in the focused Session
      slice. Commit `a91b04809c` fixes an uncovered round-trip type annotation
      and adds duplicate-viewport replacement coverage. All nine
      `cargo test -p session --lib` cases pass: ordering, deduplication,
      unresolved composition, multi-viewport membership, one-copy removal,
      legacy migration, reconciliation, and serialization are covered. Live
      viewport/entity composition and restart proof remain open.
- [x] 2026-07-23: Preserve a terminal-first empty Workspace instead of opening
      an unsolicited draft. Commit `e4fbc22a3a` makes New Terminal the primary
      Workspace ready action and gates New Window and startup blank-file seeding
      to official Zed. The same slice removes stale Zed wording from public CLI
      help, hides the legacy compatibility alias, and names `--dez` input
      `<PATH>`. The rebuilt CLI hash is
      `cc8d62764f0892da5306aeefb9206732e8f25584f9213e84631184d2ae8d9787`.
      Formatting, diff, identity, CLI build/help, and full `zed --bin dez`
      source checks pass. Consolidated app rebuild and rendered proof remain
      open.
- [x] 2026-07-23: Make the Session Rail zero state honest and explicit. Commit
      `d9688490ad` replaces `0 sessions · caught up` with `No sessions yet`,
      labels the ambiguous `+ New` action `New Terminal`, and changes the
      project-centric `Start a workspace` frame to `Start working`. A model
      assertion covers empty, search, singular/plural attention, and genuine
      caught-up copy. Formatting, diff, identity, CLI build/help, and the full
      `zed --bin dez` source check pass. The focused sidebar test target is not
      claimed because its separate WebRTC graph exceeded safe local storage
      headroom. Consolidated rebuild and rendered proof remain open.
- [x] 2026-07-23: Make the isolated CLI installer reachable from Dez. Commit
      `704314cc92` moves Install CLI out of the official-Zed-only menu branch,
      keeps the target at `/usr/local/bin/dez`, replaces upstream Linux package
      guidance, and makes handshake failures identify Dez. The compatibility
      branch refuses to manage the official `zed` command. Formatting, diff,
      identity, CLI build/help, and the full `zed --bin dez` source check pass.
      Install interaction and consolidated bundle proof remain open.
- [x] 2026-07-23: Seed terminal scope before the first PTY event. Commit
      `7a20dc1d19` records a new terminal's initial cwd in its Workspace-owned
      EvidenceSet during event subscription, so an idle shell no longer has to
      emit output before review can name its scope. Existing cwd, wakeup, and
      exit events still update Current/Stale lifecycle truth. Formatting, diff,
      identity, and full `zed --bin dez` source checks pass; runtime restoration
      proof remains open.
- [x] 2026-07-23: Connect live background Workspace registration to App
      Session viewport composition. Commit `962b611605` records a durable
      Workspace in the current viewport from `MultiWorkspace::register_workspace`
      without changing active selection. Repeated registration is idempotent,
      a second viewport preserves singular global membership, and activation
      updates selection in place. All ten focused Session tests and the
      workspace library compile gate pass with the low-disk profiles. Shared
      live entity composition and restart proof remain open.
- [x] 2026-07-23: Keep the empty replacement after final-project removal in the
      durable App Session. Commit `e9a595fcff` centralizes both close-Workspace
      and remove-project-group fallbacks behind an allocator-backed constructor.
      The empty Workspace now has a database ID before construction, enters
      global membership, and becomes active in the current viewport. The
      existing persistence regression asserts all three conditions. The
      production workspace library check passes in 5m08s; its expanded test
      target is authored but not claimed because the all-tests metadata graph
      reached the local storage floor before producing a code result.
- [x] 2026-07-23: Prove New Window is another viewport over the shared App
      Session in source. Commit `2334fbdcfc` upgrades the existing headless
      regression from checking only the old sidebar to constructing both
      windows with one AppState and asserting distinct viewport and Workspace
      IDs, shared durable membership, and correct per-viewport active
      selection. It also updates a stale bottom-dock test to Dez's supported
      side-dock model. The full workspace test configuration compiles with one
      unrelated dead-code warning; direct test execution was cancelled during
      sustained codegen/I/O pressure, so packaged runtime proof remains open.
- [x] 2026-07-23: Extract queued-open startup ordering into one explicit
      completion barrier. Commit `47e769da5d` routes the continuing listener
      through an ordered dispatcher and authors a focused regression for
      pre-barrier retention, ordered release, and continued delivery after
      restoration completes. Failure fallback releases the same completion
      signal, so queued user intent is not discarded when restoration reports
      an error. Formatting, diff, and identity gates pass; a cold Dez source
      check was stopped at the 3.4 GiB storage floor before returning a code
      result, so compiled-test and packaged runtime proof remain open.
- [x] 2026-07-23: Preserve truthful ownership when a Workspace window fails to
      restore. Commit `d10d90648d` changes the affected durable identity from
      resolved to unresolved without deleting or reordering its App Session
      membership or viewport placement. The transition is idempotent and all
      11 focused Session tests pass in 2m08s. The full startup integration check
      and polished retry/remove recovery UI remain open; no bundle was built or
      launched. Commit `fbf8443359` later refines this into the distinct
      `RestoreFailed` state and persistent recovery callout described below.
- [x] 2026-07-23: Replace the failed-restore dead end with an actionable notice.
      Commit `31cc1b1205` keeps one stable, non-autohiding toast with concise
      copy and an **Open Dez log** action, and only treats an active-window
      update as successful when the toast was actually installed. A failed
      update therefore still opens the durable empty recovery Workspace.
      Formatting, diff, and identity gates pass; compiled and rendered proof
      remains deferred to the consolidated build gate.
- [x] 2026-07-23: Give failed Workspace restoration a durable Session Rail
      recovery surface without false alarms. Commit `fbf8443359` introduces a
      distinct `RestoreFailed` resolution so identities merely skipped by the
      active restore policy remain ordinary unresolved records. Actual failures
      persist in order and render a compact warning callout with **Open Recent**
      and **Dismiss**; Dismiss removes the App Session reference but does not
      directly delete recent-workspace storage. All 12 focused Session tests
      pass in 4.89s, and the offline lock change adds only Sidebar -> Session.
      The Sidebar library check was stopped at the 3.4 GiB storage floor while
      compiling inherited audio/WebRTC dependencies, before a final code
      result. Formatting, diff, and identity gates pass; compiled UI and
      rendered proof remain open.
- [x] 2026-07-23: Isolate Dez launch endpoints from official Zed in source.
      Commit `c101fe6a43` moves macOS single-instance channels off upstream
      Zed's 43737 range to Dez-owned 45737/45837/45937/46037 bases, renames the
      Linux app/CLI listener to `dez-{channel}.sock`, and removes Linux/Windows
      CLI autodetection fallbacks that could silently launch an official Zed
      executable. Identity guards freeze each boundary. The standalone CLI
      check passes in 1m22s; formatting, diff, and identity gates pass. Actual
      installed coexistence remains open because this machine has no official
      Zed app or CLI to inspect side by side.
- [x] 2026-07-23: Remove the final upstream onboarding route from Dez chrome.
      Commit `699cbd1bc8` makes title-bar promotion banners and Return to
      Onboarding official-Zed-only, renames the Help entry to **Getting
      Started**, and rewrites the optional welcome surface around Workspaces,
      supervision, worktree isolation, and review evidence. Identity guards
      freeze the onboarding gates. Formatting, diff, and identity checks pass;
      compiled and rendered proof remains deferred to the consolidated build.
- [x] 2026-07-23: Remove the last Zed-branded glyph from Dez getting started.
      Commit `869cddcce0` replaces the Zed Assistant icon with the neutral Robot
      glyph and extends the identity guard so it cannot silently return.
      Formatting, diff, and identity checks pass; rendered proof remains in the
      consolidated build gate.
- [x] 2026-07-23: Collapse the empty Session Rail to one activation hierarchy.
      Commit `4e6292ff0a` suppresses the ordinary overview while the full start
      state is visible, so New Terminal appears once instead of competing with
      itself. The state is now titled **Start a session**, explains that live
      state returns to the rail, and icons its New File and Open alternatives.
      An authored model assertion and identity guard cover the handoff.
      Formatting, diff, and identity checks pass; compile and rendered proof
      remain in the consolidated gate.
- [x] 2026-07-23: Remove duplicate project identity from the Dez rail footer.
      Commit `a9b1a961c0` leaves Workspace/project identity in the Session Rail
      group hierarchy instead of repeating and wrapping it above the utility
      icons. Restricted Mode and embedded cross-platform application menus can
      still open the row, and official Zed retains the upstream behavior. Two
      model tests and an identity guard cover those boundaries. Formatting,
      diff, and identity checks pass; compile and rendered proof remain in the
      consolidated gate.
- [x] 2026-07-23: Normalize the Session Rail on Workspace vocabulary. Commit
      `ff91b34a81` changes its remaining visible Remote Project, Project
      Options, focus/open-new-window, Add Project, and Project Rules labels to
      Workspace equivalents. The bottom entry uses a Folder Open glyph.
      Internal upstream types remain intact, and an identity rejection freezes
      the visible language. Formatting, diff, and identity checks pass;
      rendered proof remains in the consolidated gate.
- [x] 2026-07-23: Normalize recent and remote work on Workspace vocabulary.
      Commit `f6aea3e013` updates the title bar, recent-work picker, remote
      controls, and Agent History picker without renaming internal upstream
      types or official Zed copy. Focused label assertions and identity guards
      cover the product branch. Formatting and static checks pass; compiled and
      rendered proof remain in the consolidated gate.
- [x] 2026-07-23: Normalize recent-work actions and Restricted Mode trust copy.
      Commit `b749a25619` carries Workspace language through search, empty,
      open/move/remove, multi-root, failure, and trust states while preserving
      official Zed branches and internal identifiers. Focused trust-label
      assertions and identity guards are authored; static gates pass, and
      compilation/rendered proof remain in the consolidated gate.
- [x] 2026-07-23: Normalize developer-tool and Settings surfaces. Commit
      `0e2c0dcae3` applies Workspace/Files language to remote, agent, pane,
      evidence, skills, scoped Settings, status, and Files configuration UI.
      Official Zed and internal compatibility names remain intact. Identity and
      formatting gates pass; compile/render proof remains consolidated.
- [x] 2026-07-23: Remove stale upstream recovery and promotion actions. Commit
      `e969abda4a` fixes the GPU warning identity/override, hides inherited
      palette actions, and replaces upstream issue filing in database recovery
      with truthful local diagnostics. Authored assertions and identity guards
      cover the product boundary; compiled interaction proof remains deferred.
- [x] 2026-07-23: Finish Workspace grammar in retained recovery states. Commit
      `0607771783` normalizes disconnected, follow/join, shared-agent, and
      multi-root error copy while preserving official Zed branches. Identity
      and formatting gates pass; rendered proof remains consolidated.
- [x] 2026-07-23: Clarify the zero-session hierarchy. Commit `2092acd453`
      replaces the ambiguous Sessions heading and repeated absence copy with a
      named Session Rail, ready-Workspace count, and one accessible scoped
      creation action per empty Workspace. Focused assertions and static gates
      pass; rendered proof remains consolidated.
- [x] 2026-07-23: Make Session Rail creation terminal-first. Commit
      `57290c27c3` routes the Workspace plus control, worktree chooser, and
      desktop/Vim defaults through New Session while preserving an explicit New
      Agent Thread option. Focused and identity assertions pass; compiled and
      rendered proof remains consolidated.
- [x] 2026-07-23: Make Workspace closure scope and input parity explicit.
      Commit `633dcc4bec` adds a keyboard-addressable worktree-close submenu,
      preserves multi-root names, and distinguishes per-worktree closure from
      whole-Workspace removal. Focused and identity assertions pass; compiled
      and rendered proof remains consolidated.
- [x] 2026-07-23: Expose restored attention without relying on color. Commit
      `7893762cd5` composes persisted active attention into every non-live
      transport label while preserving Saved/Detached/Missing truth. Focused
      and identity assertions pass; rendered proof remains consolidated.
- [x] 2026-07-23: Disclose bounded Host evidence gaps. Commit `bd36afd3f4`
      persists structured-event eviction through detach/list/reattach, marks
      rail evidence partial, and adds the precise Review Brief risk. Focused
      lifecycle and identity assertions pass; process-restart/rendered proof
      remains consolidated.
- [x] 2026-07-23: Redact retained terminal evidence and correct its settings
      contract. Commit `b39eedf724` filters secret environment assignments
      before Host retention and distinguishes local metadata from Host-owned
      structured activity and transcripts. Focused and identity assertions
      pass; compiled hook-flow proof remains consolidated.
- [x] 2026-07-23: Redact explicit secret CLI flags from retained commands.
      Commit `9323af8008` covers spaced and equals forms without masking
      ordinary flags. Focused utility, Host-ingestion, and identity assertions
      pass; compiled proof remains consolidated.
- [x] 2026-07-23: Redact URL credentials from retained commands. Commit
      `80ff1df75f` covers userinfo and secret query parameters, preserves useful
      URL structure, and discloses verbatim file-target retention. Focused and
      identity assertions pass; compiled proof remains consolidated.
- [x] 2026-07-23: Expose the Session Rail Workspace hierarchy. Commit
      `c47637c2ac` adds named list semantics and complete selected, expanded,
      ready/running, and attention labels while suppressing sticky duplicates.
      Focused and identity assertions pass; platform inspection remains
      consolidated.
- [x] 2026-07-23: Announce Session Rail scope and result changes. Commit
      `7e91f00b69` names the scope group and search region, gives totals and
      empty results status semantics, and separates stable scope names from
      toggle state and action descriptions. Focused and identity assertions
      pass; platform announcement timing remains consolidated.
- [x] 2026-07-23: Expose callout urgency to assistive technology. Commit
      `e28b78ed57` maps informational/success callouts to status and
      warning/error callouts to alert, covering durable Host and Workspace
      recovery. Authored role and identity assertions pass; platform
      announcement timing remains consolidated.
- [x] 2026-07-23: Normalize high-frequency Session Rail hit targets. Commit
      `a90fae5873` aligns compact headers with their 28 px hover controls and
      raises primary creation, scope, Host-detail, and Workspace-recovery
      actions from compact to medium size. Identity checks pass; rendered
      density and zoom proof remains consolidated.
- [x] 2026-07-23: Connect keyboard selection to accessibility focus. Commit
      `f6318ea907` marks the focused Workspace header or Session row as the
      active descendant while the Session Rail keeps real keyboard focus. The
      audit also confirms the existing static-frame reduced-motion fallback.
      Identity checks pass; platform focus/motion proof remains consolidated.
- [x] 2026-07-23: Retire the clipped 56 px Session Rail icon presentation.
      Commit `9930e86677` keeps legacy configuration readable but resolves it to
      the smallest complete compact layout. Width and identity checks pass;
      rendered compact proof remains consolidated.
- [x] 2026-07-23: Make terminal-first onboarding responsive and semantic.
      Commit `4a102fc50e` names the workflow and its ordered steps, separates
      safety copy from a wrapping action row, and normalizes its primary hit
      targets. Identity checks pass; rendered zoom/narrow proof remains
      consolidated.
- [x] 2026-07-23: Disable inherited upstream telemetry for Dez. Commit
      `933e3f515f` forces diagnostics/metrics false in non-Zed builds and removes
      their fork UI while preserving separate Anthropic retention consent.
      Source and identity checks pass; compiled network proof remains
      consolidated.
- [x] 2026-07-23: Hide the disabled fork updater setting. Commit `2680937952`
      removes the inert Dez Auto Update section while retaining compatibility
      parsing and official Zed UI. Source and identity checks pass; rendered
      Settings proof remains consolidated.
- [x] 2026-07-23: Remove dead Collaboration Panel settings. Commit
      `9239006d4b` hides its button/dock/width section in Dez while preserving
      compatibility and official Zed UI. Identity checks pass; rendered
      Settings proof remains consolidated.
- [x] 2026-07-23: Expose high-value Session Rail shortcuts accessibly. Commit
      `33f7ff5893` reports Shift+A on scope controls and Shift+V on Review Brief
      buttons without adding cramped visual labels. Identity checks pass;
      platform announcement proof remains consolidated.
- [x] 2026-07-23: Keep provider authentication opt-in in Dez. Commit
      `2fc5226a51` gates eager provider authentication on official Zed identity
      plus auto-connect, covering stale fork settings. Source and identity
      checks pass; compiled network proof remains consolidated.
- [x] 2026-07-23: Hide unavailable upstream edit-prediction UI in Dez. Commit
      `cc2509e8b8` normalizes legacy Zed/Mercury selections before rendering,
      removes them from the provider picker, and hides Mercury setup while
      preserving official Zed and explicit local/user-configured providers.
      Focused source and identity checks pass; rendered Settings/status proof
      remains consolidated.
- [x] 2026-07-23: Keep upstream prediction commands hidden after Agent UI
      refreshes. Commit `b909b31d45` prevents the later Agent filter from
      restoring Zed Predict onboarding in Dez and treats legacy Zed/Mercury
      selections as unavailable when filtering prediction actions. Supported
      explicit providers and official Zed remain unchanged. Source and identity
      checks pass; rendered command-palette proof remains consolidated.
- [x] 2026-07-23: Isolate crash recovery identity and opt-in. Commit
      `aab0e5f2f2` prevents an inherited upstream endpoint from enabling the
      crash handler in Dez; fork minidumps require explicit
      `DEZ_GENERATE_MINIDUMPS=1|true`. Startup/duplicate-instance messages,
      crash metadata, and temporary artifacts use Dez identity while official
      Zed remains unchanged. Source and identity checks pass; compiled recovery
      proof remains consolidated.
- [x] 2026-07-23: Remove inherited collaboration keybindings and call chrome.
      Commit `0ddf84161e` filters channel/collaboration namespaces and the
      follow-collaborator action from every keymap source in Dez, preventing
      removed handlers from consuming useful chords. Title-bar call controls
      are official-Zed-only; AI opt-out filtering and official Zed behavior are
      preserved. Source and identity checks pass; compiled key-routing proof
      remains consolidated.
- [x] 2026-07-23: Correct remaining CLI and skill-import guidance. Commit
      `2efbf166b7` teaches `dez <path>` in Dez Settings and removes the false
      attribution that Zed performs private skill URL retries. Official Zed
      keeps its CLI copy. Source and identity checks pass; rendered
      Settings/skill proof remains consolidated.
- [x] 2026-07-23: Reconcile the frozen workspace lock graph. Commit
      `c3dfb7aa79` adds the already-declared `util` and `paths` direct edges for
      `dez_terminal_host` and `project_panel`; no version or source changes.
      Full locked metadata resolves and the candidate hash is updated in
      Release Evidence.
- [x] 2026-07-23: Repair the Callout accessibility compile boundary. The
      consolidated build exposed a missing GPUI `Role` import; commit
      `5a25a72f92` adds it without changing semantics. Formatting and identity
      checks pass; the locked candidate build resumes from its completed graph.
- [x] 2026-07-23: Scope zero-session creation to the exact Workspace. Commit
      `4fc53b860f` hides the overview's global New Terminal shortcut while no
      sessions exist, leaving each open Workspace group's scoped action as the
      single path. The shortcut returns after the first session, and the
      no-Workspace activation surface is unchanged. An authored model
      assertion and identity guard cover the transition. Formatting, diff, and
      identity checks pass; compile and rendered proof remain in the
      consolidated gate.
- [x] 2026-07-23: Clarify the remaining Session Rail utilities. Commit
      `8bcd11f4b6` calls the clock Agent History, uses Command Palette
      consistently, and adds explicit accessible labels plus action-aware
      tooltips to the New File and Open alternatives. An identity rejection
      freezes the visible names. Formatting, diff, and identity checks pass;
      compiled interaction proof remains in the consolidated gate.
- [x] 2026-07-23: Remove dead Session Rail settings from Dez. Commit
      `ad59a60926` hides branch status/name, worktree name, duplicate project
      identity, and upstream onboarding controls whose surfaces no longer
      render in Dez. Compatibility schema remains readable; Files Pane, menus,
      and platform button-layout controls remain because they are live. A
      focused model test and identity guard cover the product filter.
      Formatting, diff, and identity checks pass; compiled Settings UI proof
      remains in the consolidated gate.
- [x] 2026-07-23: Remove inherited cloud and call settings from Dez. Commit
      `a20074de26` omits the Calls/Collaboration page, leaves Network with its
      live proxy control, and removes the graphical paths for inherited Zed
      auto-connect and collaboration-server configuration. Compatibility keys
      remain readable and official Zed keeps the full UI. Attention copy no
      longer names the removed workspace bar. A product-boundary model test and
      identity guards pass; compiled Settings UI proof remains in the
      consolidated gate.
- [x] 2026-07-23: Remove upstream prediction-data controls from Dez. Commit
      `2435348289` hides Zed Edit Predictions data collection because Dez
      disables its Zed/Mercury providers, and rewrites provider setup around
      explicit providers. Compatibility parsing and official Zed behavior are
      retained. A product-boundary model test and identity guard pass; compiled
      Settings UI proof remains in the consolidated gate.
- [x] 2026-07-23: Productize command names and copied Settings links. Commit
      `f89f55868c` displays the internal `zed::…` action namespace as `dez: …`
      across the Command Palette, keymap editor, and which-key UI while leaving
      action identities compatible. Release channels now own their canonical
      Dez URL scheme, which both registration and Settings copy links consume;
      legacy `zed://` remains input-only compatibility. Focused assertions plus
      formatting, diff, lockfile, and identity gates pass; compiled interaction
      proof remains in the consolidated gate.
- [x] 2026-07-23: Remove inherited collaboration commands from Dez's palette.
      Commit `f40877d4ab` filters the registered `collab` namespace from Dez
      presentation while preserving action/keymap compatibility and official
      Zed behavior. A product-boundary assertion and identity guard pass;
      compiled palette proof remains in the consolidated gate.
- [x] 2026-07-23: Remove Zed Assistant glyphs from Codex session surfaces.
      Commit `526218a972` uses the neutral Robot glyph in the Session Rail and
      keyboard switcher while retaining explicit Codex labels and metadata. A
      directory-wide identity rejection covers both paths. Formatting, diff,
      and identity checks pass; rendered proof remains consolidated-gate work.
- [x] 2026-07-23: Add deliberate Codex setup to first-run onboarding. Commit
      `bb0cf408b4` exposes **Copy Codex Hook** before New Terminal, copies the
      bundled setup, states that Dez installs or modifies nothing, and retains
      lifecycle/Host persistence limitations. An identity guard covers the
      action. Formatting, diff, and identity checks pass; rendered onboarding
      proof remains in the consolidated gate.
- [x] 2026-07-23: Add explicit user-selected review evidence. Commit
      `a8ce563373` keeps the selected path in the Workspace-owned EvidenceSet
      after its tab closes, distinguishes user selection from passive open
      surfaces, exposes add/remove/clear commands plus file-tab actions, and
      projects one Selected path row into Review Briefs. Commit `e101b63e43`
      persists only those selections through the normal Workspace database and
      restores them without persisting passive evidence. Formatting, diff, and
      identity checks pass; compiled restart proof remains deferred.
- [x] 2026-07-23: Reconcile saved terminal evidence across failed restore and
      successful reattach. Commit `0e6507756e` retains last-known cwd evidence
      under the original Session as Unresolved, surfaces the uncertainty in
      Review Brief risk copy, and lets a successful attach replace the record
      with Current Host truth. A focused model test is authored; formatting,
      diff, and identity checks pass. Commit `ea2bb18453` then maps live
      Attached/Starting/Detached to Current, Reconnecting/Missing/Incompatible
      to Unresolved, and Exited to Stale while consuming snapshot cwd changes.
      Compiled restart proof remains open.
- [x] 2026-07-23: Restrict EvidenceSet mutation to Workspace ownership. Commit
      `0f8740b1a1` keeps immutable evidence projection public while root,
      open-file, terminal, and selection mutation stays inside the Workspace
      crate. Formatting, diff, and identity checks pass.
- [x] 2026-07-23: Prevent provisional evidence and detached Session identity
      leakage across Workspaces. Commits `af232402f5` and `a4047d95c0` give
      pending evidence sets distinct stable namespaces and retained Host
      Sessions an optional durable Workspace owner, with legacy cwd fallback
      only when ownership is absent. Focused tests are authored; compiled
      runtime proof remains deferred.
- [x] 2026-07-23: Prove live and persisted evidence isolation between two
      Workspaces. Commit `6f2061d2c7` asserts same-path Session records remain
      distinct and database-backed selections cannot be cleared across
      Workspace IDs. Formatting, diff, and identity checks pass.
- [ ] Complete durable app-session ownership.
- [x] Persist Host/Session references in terminal items and metadata.
- [x] Persist local terminal Host/Session references and implement authenticated
      replay/reattachment in source; full process-restart proof remains in the
      release gate.
- [x] Implement the structured Codex adapter and attention inbox in source;
      hook onboarding and live verification remain release-gate work.
- [x] Implement the deterministic review-brief projection with structured
      terminal command/check evidence, conservative classification, Git and
      worktree state, bounded file targets, cwd provenance, and explicit
      non-ownership language. Broader checker adapters remain open.
- [ ] Demonstrate the complete restart-and-review vertical slice.
- [ ] Complete workspace-scoped `Project` and evidence behavior.
- [ ] Add remote continuity, conflict radar, recipes, and worktree brokering.
- [ ] Complete the v0.0.1 release gate.

## Discoveries {#discoveries}

- **Observation:** The checkout had no committed Fork Notes or `PLAN.md`.
  **Evidence:** A working-tree and supplied-attachment filename/content search
  on 2026-07-22 found neither. **Consequence:** The canonical set was created
  under `docs/src/development/dez/` without deleting historical Canvas
  documents.
- **Observation:** The root `AGENTS.md` is a symlink to `.rules`, whose policy
  explicitly rejects architectural maps and drive-by rule additions.
  **Consequence:** Dez architecture lives in documentation rather than
  replacing the shared Rust rules file.
- **Observation:** The local upstream ref is already the merge base and the
  branch is 240 commits ahead and 0 behind that local ref. **Consequence:** The
  next Milestone 0 action is a fresh fetch and isolated rehearsal, not an
  assumption that the remote has not moved.
- **Observation:** The current worktree contains a large uncommitted product
  slice and no current `dez` artifact. **Consequence:** Do not mix a new
  upstream merge into it; finish static checks, then build and smoke-test at the
  explicit consolidated gate.
- **Observation:** Fetching upstream tags reported local collisions for
  `nightly` and `collab-staging`. **Consequence:** Upstream automation must
  separate branch refresh from collision-prone unqualified tag updates and
  report partial fetch outcomes precisely.
- **Observation:** The later consolidated plan introduces Run as a useful
  user-facing relationship that was implicit in the earlier documents.
  **Consequence:** Add Run to the permanent model, but keep Session as the owner
  of computation and existing stores as the owners of facts.
- **Observation:** Existing Session Rail notifications represented completed or
  bell-notified work but did not treat waiting-for-confirmation and failed agent
  states as global attention. **Consequence:** Derive one attention predicate
  from current row state and use it for filtering, sorting, group indicators,
  and workspace chrome.
- **Observation:** Codex exposes trusted lifecycle hooks with stable session,
  working-directory, event, and turn fields. **Consequence:** Use that
  structured feed for critical agent state and retain process-name detection
  only as a visibly lower-confidence fallback.
- **Observation:** Session Rail search had an allocated editor and action, but
  the production query was hard-coded empty and the editor was not rendered.
  **Consequence:** Restore the current upstream search path before inventing a
  separate command-center index.
- **Observation:** The refreshed 81-commit rehearsal produced ten conflicts in
  settings, preview styling, title-bar presentation, task dependency wiring,
  pane rendering, and workspace routing. **Consequence:** Resolve them as one
  focused integration slice; terminal, remote, ACP, Git, and persistence still
  need semantic tests even though they merged textually.
- **Observation:** Upstream `run_tests.yml` is gated to repositories owned by
  `zed-industries` or `zed-extensions`. **Consequence:** Dez needs fork-owned
  identity and upstream checks instead of assuming upstream CI runs here.
- **Observation:** `AppSession` persists a GUI-launch ID and window order,
  `WorkspaceStore` holds window-bound weak entities, and `MultiWorkspace` owns
  each window's retained workspace set. **Consequence:** Existing restoration
  is substantial. `AppSession` now owns the ordered restore lifecycle and the
  identity-keyed durable membership registry, while SQLite serializes state and
  `MultiWorkspace` still owns live composition. Do not collapse that remaining
  viewport gap by retaining window-bound `Workspace` entities in the app
  session.
- **Observation:** Storing one optional viewport on global Workspace membership
  cannot represent the same Workspace in two windows and lets removal from one
  window erase the shared identity. **Consequence:** App Session now persists
  ordered viewport records separately, migrates the compatibility fields, and
  treats viewport removal independently from global membership until the final
  presentation disappears.
- **Observation:** The available Superzed screenshots are historical rather
  than evidence for current source, but consistently expose generic welcome
  copy, a project-first start path, repeated untitled chrome, unexplained blank
  windows, and stale Zed identity. **Consequence:** Use them as a defect
  baseline only; repair source now and defer visual completion claims until the
  one intended Dez artifact is built and inspected.
- **Observation:** Startup treated a non-focus initial open request as an
  alternative to session restoration, and the continuing listener was not
  gated on restore completion. **Consequence:** Establish one startup barrier:
  restore first, apply the initial request second, then process queued requests.
- **Observation:** `TerminalThreadMetadataStore` restores title, path, and
  identity by spawning a new shell, while `terminal::Terminal` still owns the
  PTY or subprocess. **Consequence:** Saved terminal history must not be called
  a detached or reattached session; process continuity begins with a separate
  Host/Session identity and protocol.
- **Observation:** Full Cargo dependency resolution for the terminal contract
  completed and updated `Cargo.lock`, but fetching the clean cache and resolving
  the real host adapter reduced free space on the workspace volume from 17 GiB
  to 11 GiB. **Consequence:** Keep
  the agreed consolidated build gate deferred; a GPUI terminal build can
  exhaust the remaining volume and would not yet prove the unconnected host
  contract end to end.
- **Observation:** Retaining a terminal entity after pane closure would make
  the process invisible if the Session Rail only scanned live views and saved
  agent metadata. **Consequence:** The adapter slice includes a detached-session
  projection, stable session-derived row identity, reattachment into a normal
  pane, and explicit termination; invisible retained processes are not an
  acceptable intermediate UX.
- **Observation:** Existing terminal-item and agent-terminal tables can be
  extended with nullable Host/Session columns without assigning new meaning to
  legacy item UUIDs. **Consequence:** Null pairs are the explicit legacy path;
  valid pairs are reconciled against the host; partial or malformed pairs must
  fail closed instead of starting computation.
- **Observation:** The upstream Alacritty event loop combines PTY I/O with its
  in-process terminal grid, so moving that loop wholesale would also move the
  renderer state out of the GUI. **Consequence:** The helper owns the raw PTY
  through the same platform constructor but uses a small poll-driven I/O loop;
  raw bytes feed bounded host replay and will feed a client-side emulator.
- **Observation:** A helper snapshot store created after the Session Rail has
  no entity subscription path in an already-constructed sidebar.
  **Consequence:** Host snapshot changes publish a lightweight app-global
  revision; projections observe that revision and rebuild only when snapshots
  or connection truth actually changes.
- **Observation:** Workspace item restoration races the asynchronous helper
  handshake during application startup.
  **Consequence:** Valid Host/Session references wait for the opt-in host for a
  bounded interval before rendering the explicit unavailable surface; they
  never create replacement computation.

## Decisions {#decisions}

- **2026-07-22:** Keep permanent architecture, product hypotheses, upstream
  procedure, release state, and execution progress in separate linked files.
- **2026-07-22:** Treat agent supervision as the initial market wedge while
  retaining durable workspace and host-owned session architecture as the
  foundation.
- **2026-07-22:** Build the first complete vertical loop before sequentially
  implementing every long-range architecture milestone.
- **2026-07-22:** Do not replace the `AGENTS.md` symlink or add architecture to
  `.rules`; the new documentation index supplies the contributor reading order.
- **2026-07-22:** Incorporate the new consolidated plan selectively instead of
  replacing the canonical document hierarchy. Its Run, protocol, and security
  refinements are adopted; its late attention milestone and single-file
  authority model are not.
- **2026-07-22:** Apply the same hierarchy to the revised 1,756-line plan. The
  revision correctly brings Attention into the PMF Run slice and strengthens
  complete-product positioning, Evidence provenance, capabilities, metrics,
  and long-range integrations. Its blank progress checklist is not repository
  evidence, Session transport facts remain separate from Run state, and build
  execution stays at the agreed consolidated gate.
- **2026-07-22:** Introduce the terminal Host/Session contract inside the
  existing terminal substrate before moving PTY ownership. Keep the current
  renderer and terminal metadata paths as compatibility adapters until attach,
  detach, replay, and explicit termination are tested.
- **2026-07-22:** Keep the first real host adapter local and in-process. Do not
  register remote terminal entities with it or imply GUI-exit survival. Use the
  adapter to prove view-independent identity and lifecycle before adding local
  IPC or modifying upstream remote transport.
- **2026-07-22:** Persist Host/Session identity as an optional pair alongside
  compatibility metadata. Reattach only after the current host confirms
  ownership. When confirmation fails, preserve the reference and render a
  terminal-shaped recovery surface that states no replacement was started.
- **2026-07-22:** Give the helper a token-file handshake instead of placing a
  secret on the process command line. Require private token-file, socket-file,
  and socket-directory permissions; redact the token's debug output; cap a
  frame before allocating its payload; and keep the first transport sequential
  until live event multiplexing is required by the GUI adapter.

## Verification {#verification}

Completed for the preceding source slice:

- `cargo fmt --all -- --check`
- `git diff --check`
- `bash -n script/bundle-mac`
- `cargo metadata --no-deps --format-version 1`
- focused Prettier checks for the earlier documentation changes

Completed for the 2026-07-22 documentation reconciliation:

- Prettier write and check for every touched documentation file;
- `git diff --check`;
- relative documentation target inspection.

Completed for the first attention and start-state source slice:

- `cargo fmt --all -- --check`;
- `git diff --check`;
- focused source tests added for attention-state classification;
- no application binary launched.

Completed for the first upstream-resilience slice:

- isolated no-commit merge rehearsal and cleanup;
- `./script/dez-identity-check`;
- Bash syntax check for the identity script;
- Prettier check for the Dez guard workflow;
- `git diff --check`.

Completed for the architecture and terminal Host/Session contract slice:

- full Cargo metadata resolution and lockfile update;
- `cargo fmt --all -- --check`;
- Prettier check for the canonical Dez documentation changes;
- `git diff --check`;
- focused unit tests authored for stable identities, truthful lifecycle
  predicates, create/detach/reattach/terminate transitions, bounded replay,
  Missing, and Incompatible outcomes;
- no Rust tests or application binary executed at this source-only gate.

Completed for the in-process real-terminal adapter slice:

- Cargo metadata resolution and lockfile update for the new direct dependency
  edges;
- `cargo fmt --all -- --check`;
- `git diff --check`;
- Prettier check for the updated architecture, roadmap, and launch record;
- `./script/dez-identity-check`;
- no Rust compile, test, alternate binary launch, or live interaction claim.

Completed for the Host/Session persistence migration slice:

- reversible nullable-column migrations for terminal items and saved agent
  terminals;
- `cargo fmt --all -- --check`;
- `git diff --check`;
- `cargo metadata --no-deps --format-version 1`;
- `./script/dez-identity-check`;
- no Rust compile, test, alternate binary launch, or live interaction claim.

Completed for the ordered viewport and first-party source-polish slices:

- `cargo fmt --all -- --check` before the viewport edits and `cargo fmt --all`
  after them;
- `git diff --check`;
- `cargo metadata --no-deps --format-version 1`;
- `./script/dez-identity-check` before the viewport edits;
- focused source tests authored for legacy migration, multi-viewport identity,
  viewport reconciliation, viewport-aware removal, and compact-state roundtrip;
- no Rust compile, test, alternate binary launch, or live interaction claim.

Completed for the first helper-process source slice:

- new crate instructions and a rollbackable `dez_terminal_host` workspace
  member;
- authenticated handshake and bounded framing tests authored;
- helper lifecycle and detach-versus-terminate tests authored;
- poll-driven raw PTY ownership, bounded replay, and sequential transport
  client source;
- opt-in connect-before-launch runtime supervisor and macOS helper packaging;
- `cargo fmt --all -- --check`;
- `git diff --check`;
- Bash syntax checks for bundle and identity scripts;
- Cargo metadata and lockfile update;
- no Rust compile, test, helper launch, application launch, or cross-process
  recovery claim.

Completed at the 2026-07-22 consolidated gate:

- warning-free locked app/helper build and locked CLI build using the recorded
  single-codegen-unit, non-incremental dev profile;
- 15 focused terminal tests, 8 focused helper tests, and 3 Session Rail
  lifecycle tests;
- exact raw-binary launch with the keymap and local-first startup regressions
  corrected;
- helper PID/Host/socket survival and single-instance reuse across GUI exit and
  exact-client relaunch;
- debug bundle creation, bundle identity and executable inventory, permission
  copy, SHA-256 capture, and deep strict ad-hoc signature verification;
- `cargo fmt --all -- --check`, offline Cargo metadata, identity, Bash syntax,
  and diff checks.

Still required after the consolidated gate:

- app-facing modified-crate Clippy when enough local build storage is
  available; the complete terminal Host target graph already passes with
  warnings denied;
- a live hosted PTY with Session ID, child PID, replay cursor, and
  same-computation restart/reattach proof;
- restored/empty/offline/failed/incompatible UI scenarios and the structured
  Codex attention/review/restart hero flow;
- visual density/theme/width matrix, keyboard/pointer parity, and macOS
  accessibility audit after the desktop is unlocked;
- Developer ID signing, notarization, installation/uninstallation, official
  Zed coexistence, and design-partner proof.

The build gate has run, but the public release remains unverified until these
remaining runtime, visual, accessibility, distribution, and partner gates pass.
