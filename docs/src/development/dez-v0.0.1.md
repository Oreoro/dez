# Dez v0.0.1 launch record

> This page is a release snapshot. The permanent product and architecture
> decisions live in the [Dez Fork Notes](./dez/fork-notes.md), and active work
> is tracked in the [Dez Roadmap](./dez/roadmap.md).

Dez is an opinionated, pane-first development environment built from Zed. It
keeps Zed's speed and native feel, but centers editors, durable terminals,
project navigation, agents, review, and development tools as ordinary workspace
items. Upstream collaboration remains compatibility capability, not launch
positioning.

The longer Canvas documents remain useful research and implementation history.
The Dez documentation set wins when priorities or defaults conflict.

## Product promise

Dez helps terminal-native developers supervise coding agents and long-running
sessions without losing context. It should feel quiet before it feels clever.
Every visible control must answer one of five questions:

- Where am I?
- What is running?
- What needs my attention?
- What changed?
- What can I do next?

The canvas is the product. Editors, terminals, agents, project navigation, and
other tools use the same split, focus, move, resize, zoom, and persistence
model. A side rail may organize sessions, but it must not become a second pane
system.

## Launch defaults

- Use a dark, restrained visual baseline with a clear focus treatment.
- Keep project tools left, editable work in the center, and agent control right.
- Open project navigation as a canvas pane for worktree projects.
- Keep the session rail closed until it has useful, truthful content.
- Show a useful launch surface in every empty editor pane.
- Keep one obvious primary action in each empty or error state.
- Derive shortcut labels from the active keymap; never print assumed keys.
- Preserve pointer and keyboard parity for all pane operations.
- Prefer local state and explicit ownership over cloud-only behavior.
- Do not authenticate or connect to cloud services at startup. Upstream account
  and collaboration chrome stays out of the default Dez shell.

## Identity and upgrade contract

The public product, executable, bundle names, URL schemes, and release-channel
labels use **Dez** and version **0.0.1**. The first preview intentionally keeps
reading and writing the existing `Superzed` user-data directories. That is a
compatibility boundary, not leftover branding: changing those directories
before a transactional migration exists would make settings, workspace
history, extensions, and local agent state appear to vanish. The storage name
is isolated in `paths::APP_STORAGE_NAME` and can be retired by a later,
explicit migration.

Release signing credentials and an optional provisioning profile must be
provided by the Dez publisher. The bundle script must never fall back to an
upstream Zed signing identity.

## Live baseline observed on July 22, 2026

The running debug build was inspected in place. No alternate SuperZed binary
was launched.

The shell already has a calm visual character, but several states are too
sparse to explain themselves:

- A restored project can render as an entirely black canvas with no file tree,
  active item, or central action.
- A new window can expose multiple untitled panes with nearly identical chrome
  and weak hierarchy.
- Important global controls are represented by very small icon-only targets.
- Native accessibility exposes window chrome and menus, but the GPUI canvas is
  not represented in the macOS accessibility tree.
- Network-dependent collaboration and agent failures produce repeated log
  noise without a coherent offline product state.

The first v0.0.1 slice fixes the dead project canvas: project navigation starts
as a regular pane, all empty tabbed panes show actionable content, and the
session rail remains opt-in. Detected terminal agents distinguish saved records
from live processes and expose the live PID, so the rail does not imply that a
restored entry is still executing. Startup also stays offline unless the user
enables `auto_connect` or invokes sign-in/reconnect explicitly. GPUI
accessibility is enabled in normal builds rather than hidden behind an
experimental environment variable, and each canvas pane exposes a named pane
region.

The next source slice adds a derived attention view to the Session Rail. It
includes waiting, failed, completed-unread, and terminal-notified work, links to
the existing owning surface, and shows a deliberate caught-up state. The empty
rail now starts from New Terminal, New File, and Open instead of teaching
project ownership. Session search is visible and operational again, including
explicit clearing and composition with attention mode. These changes are
source-formatted but remain pending the consolidated build and live visual
smoke gate.

The current terminal-continuity slice introduces a versioned Host/Session
contract and an in-process local host. Local terminal computation now has a
stable identity independent of its pane. Closing a view detaches it, the
Session Rail exposes the detached session and reattaches the same terminal
entity, and termination is a separate explicit action. Process exit has its own
observed event, so an exited terminal is not labeled live. This slice does not
survive a full Dez process exit yet and remains pending the consolidated build
and interaction smoke gate.

Terminal items and saved agent terminals now store optional Host/Session pairs
through additive database migrations. Within the current GUI process, restore
reattaches the retained terminal entity. After that process is gone, the
in-process host correctly cannot confirm the session, so Dez shows a
display-only unavailable terminal and explicitly does not start a replacement
shell or rerun the agent initialization command. Legacy rows with no pair keep
their compatibility restoration path. Cross-process continuity still depends
on the planned local helper.

The first helper source slice now exists as `dez-terminal-host`. It uses a
private local socket and token-file handshake, owns raw PTYs outside GPUI,
retains bounded sequence-addressed output, and implements the Host lifecycle
commands including input, resize, detach, and terminate. This is infrastructure
evidence, not a successful-run claim: the default app does not start or connect
to the helper, and the helper binary has not been compiled or run under the
deferred build agreement.

The macOS bundle now includes and signs that helper. An experimental runtime
supervisor can connect to an existing helper or launch the bundled sibling when
`DEZ_EXPERIMENTAL_TERMINAL_HOST=1`; the default remains off. In that mode,
ordinary local shells now use helper-owned PTYs and client-side emulators.
Their title, working directory, PID, replay position, and lifecycle project
into the Session Rail; terminal-item and agent restores wait briefly for host
startup and reattach without spawning replacement work. Task terminals remain
on the existing backend. When the host is explicitly enabled, startup and
reconnection health appear in the rail and new shells fail visibly rather than
falling back to a disposable GUI process. This source path still does not
satisfy the restart-recovery acceptance test until the deferred build and live
smoke gate produce evidence.

Unavailable saved sessions remain visible as evidence. Their terminal-shaped
surface states that no replacement was started and provides one explicit New
Terminal action, so recovery is actionable without conflating old and new
computation.

The Session Rail now offers an initial Open Review Brief action for native
agent and terminal-agent rows. It opens a regular Markdown surface, reuses
existing thread, terminal, workspace, and action-log facts, and explicitly
marks unobserved commands and checks. This is the deterministic shell of the
review loop. The Codex adapter now contributes structured lifecycle,
permission, command, exit, and completion events, and review briefs classify
observed validation commands conservatively. Terminal file/Git provenance, a
second adapter, and live restart verification remain before terminal briefs
can become release-grade.

The latest source hardening makes that loop more explicit. Terminal attention
stores condition separately from unread presentation, with acknowledge,
snooze, resume, resolve, priority, and stale expiry. Review opens beside its
existing owner, links workspace/open-file/changed-file evidence, and offers a
direct native diff action. Settings now group Workspace & Privacy, Sessions &
Terminal, Agents, Attention, Evidence, Appearance, Network & Compatibility,
and Advanced without inventing unsupported controls. Eligible Codex rows expose
manual hook setup without modifying user configuration.

The helper handshake now negotiates Host and adapter capabilities, supports a
nonce-correlated heartbeat, and retains bounded cursor-addressed snapshot
events for reconnect recovery. The GUI begins from an authoritative list, then
uses a separate authenticated subscription socket for pushed event batches.
Reconnect resumes after the last cursor and retention loss triggers a full
resync. Older helpers that do not negotiate event streaming keep the bounded
polling fallback. Task terminals intentionally remain GUI-owned in v0.0.1
because their cancellation and completion contracts do not yet define safe
cross-GUI survival.

The Dez shell no longer renders upstream sign-in, plan/user, collaboration
connection, or Collab-menu chrome. Misleading upstream support, social, and
hiring links are removed; retained upstream documentation, repository,
provider, protocol, and compatibility labels are explicit. No current Dez,
Zed, or Superzed process was available for a new visual inspection after these
source changes, and no alternate binary was launched.

The outward supervision pass now gives Session Rail live All/Attention totals,
stable identity-based keyboard selection, per-Workspace New Terminal recovery,
matching-session search status, and source-backed check or command summaries.
Missing and incompatible Host states remain distinct. Shift-A toggles Attention
scope and Shift-V opens the selected Review Brief. Review now names the Run
objective and projects the owning Git worktree, main worktree, branch, status,
conflicts, untracked paths, bounded changed-path links, and truncation directly
from the Git store. Repository-wide changes remain explicitly unattributed to a
single agent. Structured activity, command, and check evidence retains its
observed working directory as a local link. Recognized Codex write, edit, and
patch hooks also retain bounded direct file targets, labeled as intended scope
rather than proof of a successful mutation. Detached or reconnecting
termination is a red Stop action behind a critical confirmation;
Host rejection raises a visible toast. File/View menus, collapsed chrome, and
settings now use Session Rail terminology, and inert upstream account controls
are absent from visible Dez settings while their schema remains compatible.
Editable Review Briefs provide Continue, Request changes, and Accept as reviewed
notes without claiming those checkboxes mutate the owning Run.

Reachable local-product copy now identifies Dez across OAuth browser handoff,
extension cards, Copilot and provider setup, remote/debugger failures, system
diagnostics, and outbound protocol identities. Actual Zed accounts, hosted
models, Edit Predictions, documentation, and repository links are labeled as
upstream. A prediction-client mismatch can be dismissed but cannot route users
to install Zed over Dez. Compatibility storage and format names remain unchanged
until a migration exists.

## Interaction model

### Pane grammar

Every pane-hosted item supports the operations that make sense for it:

- focus
- close
- move
- split
- resize
- zoom
- restore

Unsupported operations must be visibly unavailable rather than silently
ignored. A pane may host documents, terminals, agents, project navigation, or
tools, but its outer interaction grammar stays stable.

### Sessions

A session is durable work, not a transient process row. It has an identity,
workspace scope, activity state, unread state, and an honest last-known status.
The session rail is a compact index into those objects. Opening a session places
its document in the canvas; it does not create a special-purpose overlay.

### Empty and degraded states

Empty space is allowed; dead space is not. Empty panes explain what they are
for and offer a short ordered set of actions. Offline services degrade locally
and quietly. Errors should be actionable, deduplicated, and scoped to the
feature that needs attention.

## v0.0.1 launch gates

The version is ready for a public preview only when all of these are true:

1. Opening a folder always produces a comprehensible workspace.
2. Pane layout, visibility, active item, and size restore deterministically.
3. Project and agent pane toggles work from mouse, keyboard, and command palette.
4. Empty, loading, error, offline, and permission states have deliberate UI.
5. Focus location is always visible and keyboard traversal covers major regions.
6. The accessibility tree exposes meaningful canvas regions and controls.
7. Agent status reflects real process state and survives window restoration.
8. No launch-critical flow requires a Zed account or an available cloud service.
9. Brand strings, bundle metadata, and release artifacts agree on Dez 0.0.1;
   any legacy storage path is isolated and documented as an upgrade boundary.
10. A clean build passes formatting, lint, targeted tests, and a macOS smoke run.

The v0.0.1 executable and bundle are named Dez, but existing local data remains
under the Superzed storage identity until a transactional migration can move
settings, databases, extensions, and credentials safely. This compatibility
alias is deliberate and must not be removed as a cosmetic rename. Release
automation must use publisher-supplied signing and observability credentials;
it must never fall back to upstream Zed accounts.

`dez://` is the canonical public link scheme. Channel-qualified Dez schemes and
legacy `zed://` links normalize to the same internal routes so upgrades do not
break saved links.

## Delivery sequence

### 0. Upstream and identity gate

Refresh the upstream baseline before major milestones, adapt compatible Zed
features, and preserve Dez identity, storage, updater, and signing isolation.

### 1. Shell truthfulness

Remove dead canvases, repair default pane visibility, make focus unmistakable,
and ensure restore failures fall back to a useful workspace.

### 2. Pane reliability

Finish panel-as-pane lifecycle rules, singleton behavior, layout recipes,
persistence, and direct manipulation. Add regression tests before expanding the
set of pane kinds.

### 3. Durable workspace and terminal

Establish app-session workspace ownership, then move local terminal process
lifetime to a host-owned session with explicit detach, reconnect, and terminate
semantics.

### 4. Agent supervision loop

Detect Codex in a normal terminal, route attention to its existing surface, and
produce a deterministic review brief linked to observed evidence. Recover the
same truthful state after restarting the GUI.

### 5. Workspace intelligence

Derive workspace roots from file and terminal evidence, scope tools through the
workspace `Project`, and keep expensive work demand-driven.

### 6. Public-preview hardening

Complete the Dez rename, accessibility pass, settings migration, crash and
offline behavior, release packaging, and first-run experience. Performance
budgets and restore tests are release blockers, not cleanup work.

## Explicit non-goals for v0.0.1

- Reproducing every tmux, VS Code, Cursor, cmus, or terminal workflow.
- Building autonomous teams of agents before the supervision loop is proven.
- Adding decorative dashboard widgets without a daily-use purpose.
- Expanding cloud collaboration before local ownership is reliable.
- Shipping music or media controls before core pane restore is trustworthy.
- Treating visual polish as a substitute for truthful state and interaction.
