# Dez

**A native IDE for developers and AI-native builders who edit directly, run
terminal agents, and review what ships in one focused Workspace.**

Dez is a source-available preview built on
[Zed](https://github.com/zed-industries/zed). It keeps Zed's fast native
editor, language tooling, Git, debugger, tasks, remote-development substrate,
and agent ecosystem, then reorganizes them around a clearer product promise:

> See what is running, what needs attention, what changed, and what is ready
> for review without reconstructing terminal and editor state.

This repository currently carries the **Dez v0.2.2 source candidate**. It is
not yet a signed or supported binary release; promotion depends on the exact
remote artifact and evidence gates documented below.

## What Dez does

Dez treats editing, terminal work, and agent work as parts of the same
Workspace instead of separate applications or hidden panel modes.

- **Main Work Area** — files, terminals, search results, settings, diagnostics,
  previews, and reviews are ordinary movable and splittable Surfaces.
- **Workspaces** — an optional, collapsible navigator for codebases, open
  Surfaces, and Agent Sessions. Fresh windows keep it closed unless the user
  enables it, restored layouts may reopen it, and its window edge is
  configurable. Its collapsible **Workspace Tabs** section appears when at
  least two native Main Work Area tabs are open, grouping them by pane only
  when the Workspace is split. Selecting a row activates that exact native tab;
  tab ownership, dirty state, close behavior, dragging, and ordering remain in
  Zed's pane model rather than being copied into the navigator. Agent Session
  rows show provider identity and Running, Needs Input, Waiting for Permission,
  Reconnecting, Completed, or Error state on a secondary row, leaving the
  terminal or Session title as the primary navigation label. A
  multi-root Workspace leads with its first root and a bounded root count; all
  root names remain searchable and available in the header tooltip and
  accessibility label. A Workspace can explicitly attach a discovered tmux or
  Herdr session, or open a discovered cmux Workspace in cmux, without taking
  ownership. Each item with a matching working directory appears beneath the
  most specific Workspace with source, state, and working-directory metadata.
  Discovered tmux, Herdr, and cmux activity without a matching open root stays
  reachable in **Other Running Sessions** instead of being hidden or assigned
  to the wrong codebase. Opening an unmatched tmux or Herdr item with a known
  working directory first establishes that directory as a native Workspace,
  then attaches the external client there. Pathless items use the active
  Workspace; cmux remains an external handoff. Workspace headers show live Git
  branch and changed-file counts when available. Unrelated machine terminals
  do not leak into the list.
- **Workspace tools** — Files, Outline, Git, Debug, and the optional
  provider-backed Built-in Agent are ordinary draggable and closeable native
  tabs. They are not nested panels or a mandatory second sidebar.
- **Built-in Agent** — is distinct from terminal agents such as Codex, Claude
  Code, OpenCode, and Herdr. **Open Terminal** launches the configured default,
  while the Add menu keeps explicit provider and native-shell choices nearby.
  All edits still land in ordinary buffers and Git changes, so the same
  diagnostics, diff, and review tools apply.
- **Terminal Sessions** — terminals open in the Main Work Area. Session
  identity, deliberate close/end behavior, and honest unavailable-session
  recovery define the default. Packaged Dez builds place local interactive
  shells under a small process-owning terminal service, so closing or
  accidentally losing the GUI does not end the computation. Reopening Dez
  reattaches the same Session; task and remote terminals keep their existing
  lifecycle semantics. Workspace Options can launch the configured default,
  a plain shell, a Workspace-named tmux session, Codex, Claude Code, or
  OpenCode in the native terminal, and can hand the Workspace path to cmux
  without pretending cmux is a shell.
- **Evidence and review** — Dez distinguishes observed facts from reported or
  unknown state, then uses Workspace, terminal, command, check, file, and Git
  evidence to make review safer.

The primary loop stays inside one native window:

```text
Open Workspace
→ start or continue an agent in a native terminal tab
→ supervise Sessions and attention in Workspaces
→ inspect with Files, tasks, diagnostics, and Debug
→ Review Changes in the Main Work Area
```

The primary user story is deliberately short. **Home** starts or resumes a
Workspace. **Open Terminal** launches the configured terminal workflow.
**Workspaces** switches codebases, returns to current Workspace tabs, and
surfaces attention. **Open Files** and **Review Changes** bring the result
back into the same Main Work Area. The native tab-strip `+` is the shared Add
menu for every step; no separate dashboard or onboarding mode is required.

Inside an active Workspace, Home makes the **Start → Watch → Inspect → Verify**
route concrete with four actions in order: **Open Terminal**, **Browse Running
Sessions…**, **Open Files**, and **Review Changes**. Its native Dez Agent mark
and outcome copy distinguish this loop from a generic terminal launcher. An
active empty Main Work Area uses the same route with space-aware labels:
**Open Terminal**, **Browse Sessions**, **Find File**, and **Review Changes**. **New File** remains
available from File, the native `+`, and keyboard shortcuts, but is not a
primary Home or empty-state action.

The window is deliberately not a dashboard of equal columns. Each Workspace
owns one codebase context, one Main Work Area, its terminals, and its Git state.
Workspaces stays global and optional on its configured window edge. Files, Git,
Outline, Debug, Built-in Agent, commits, diffs, reviews, agent terminals, and
ordinary terminals use the native Workspace tab and pane model instead of
another navigation system.
When Workspaces is visible, **Workspace Tabs** provides a compact vertical route
to those same open tabs once there is something useful to switch between. It
stays flat for a single pane and introduces small **Pane 1**, **Pane 2**, and
later group labels only when a real split exists. The section is collapsible,
remembers its disclosure state, and disappears while Workspace search is active
so search results keep the full navigator.
Those tabs support reorder, cross-pane drag, preview replacement, pinning,
closing, and horizontal or vertical splits. A terminal can therefore sit
below code without Dez manufacturing a separate multiplexer UI.
The adjacent native `+` reopens Home, opens Recent Workspaces, or routes to a
terminal, file, search, Files, Review Changes, Run Task, Debug, or Built-in Agent surface
through the existing Zed actions. Its terminal submenu names the configured
Default Terminal first, followed by Native Shell, tmux Session, and explicit
provider launchers. **Continue Agent** resumes the last Codex, Claude Code, or
OpenCode session. **Browse Running Sessions…** refreshes external discovery and
refocuses Workspaces without adding another navigation surface.

Dez's **File → Open Terminal** submenu mirrors those native `+` terminal
launch routes in the same order, followed by **Continue Agent**. Its first row previews the configured result as
**Default · Native Shell**, **Default · Codex**, **Default · Claude Code**,
**Default · OpenCode**, or **Default · Custom Command**; the pane `+` keeps the
shorter **Default Terminal** label. Native Shell, tmux Session, Codex, Claude
Code, and OpenCode remain explicit alternatives. Continue uses
`codex resume --last`, `claude --continue`, or `opencode --continue` in the
active Workspace. **Browse Running Sessions…** follows those menus, so starting,
continuing, and reopening externally owned work stay adjacent.

Six optional layout commands remain available through **View** and Command
Palette: **Work Area + Files**, **Work Area + Built-in Agent**, **Focus Work
Area**, **Split Work Area**, **Work Area + Git**, and **Work Area + Debug**. They
are hidden from the default titlebar so the primary navigation remains obvious.

Keyboard navigation remains first-class: macOS `⌘1`–`⌘8` selects native tabs
and `⌘9` selects the last tab; Linux and Windows use `Alt+1`–`Alt+9`.
`Ctrl+Tab` opens the recent-tab switcher, split-pane focus keeps the native
Zed chords, `Ctrl+Backtick` opens the configured terminal, and
`Ctrl+Shift+Backtick` always opens the native shell. Command Palette can start
or continue Codex, Claude Code, or OpenCode, launch a shell or Workspace-named
tmux session, or hand the Workspace to cmux. The native tab-strip `+` exposes the same terminal choices, **Browse
Running Sessions…**, and **Open Workspace in cmux** without creating a second
navigation system. The default launch command lives under **Settings →
Workspaces & Terminals → Terminal Launch → Default Terminal Command**;
**cmux Workflows** beside it opens cmux's own custom-command guide for provider
actions and multi-pane layouts. Dez keeps those layouts externally owned.
Provider launchers and Continue actions remain one-off choices. **Settings → Keyboard & Vim**
exposes shortcut search, conflict inspection, base keymaps, and optional full
Vim or Helix editing. Vim and Helix share native leader destinations for
recent tabs (`Space b`), files (`Space f`), the configured agent terminal
(`Space t`), a shell (`Space T`), and Workspace search (`Space /`).

Most users only need three product decisions: choose the **Default Terminal
Command**, decide whether and where Workspaces should open, and configure an
Agent provider only if the optional Built-in Agent is needed. **Restore Native
Dez Appearance** remains one scoped repair action that preserves font sizes and
unrelated preferences. Codex, Claude Code, OpenCode, tmux, and cmux remain
explicit per-launch choices; choosing one never rewrites the default.

From a selected Agent Session row in Workspaces, `Enter` returns to the
existing Session, `Shift+F` opens its Workspace files, `Shift+G` opens its
change review, and `Shift+V` opens its evidence-backed **Review Brief**. On a
standalone terminal row, `Shift+V` instead opens **Terminal Details** for that
terminal and Host. Files and Review Changes remain available from both routes;
none of these actions creates a duplicate shell or project. If the owning
Workspace is closed, Files restores that Workspace and the exact selected
Session before revealing the tree.

Read [What is Dez?](./docs/src/dez.md) for the product model and a concrete
workflow.

### Terminal-agent ownership

Codex, Claude Code, and OpenCode render as their normal TUIs inside Dez's native
terminal; Dez does not replace their keyboard handling, colors, or full-screen
layout with chat chrome. tmux owns its server sessions. Herdr owns its panes.
cmux stays an external native application and receives the active path through
its documented `cmux open <path>` command. Dez contributes Workspace context,
native tabs and splits, durable ownership for eligible Dez-created terminals,
attention, and direct Files/Git review routes.

cmux notification and supported-session restore hooks remain an explicit cmux
setup choice:

```bash
cmux hooks setup
cmux hooks setup codex
cmux hooks setup --agent opencode
```

Dez never edits provider or cmux hook configuration automatically. See the
[Zed terminal documentation](https://zed.dev/docs/terminal),
[Codex CLI](https://github.com/openai/codex),
[Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code/cli-usage),
[OpenCode CLI](https://dev.opencode.ai/docs/cli/), and
[cmux](https://github.com/manaflow-ai/cmux) for the upstream contracts Dez
preserves.

## How the IDE is integrated

Dez does not bolt an editor onto a terminal dashboard. Every Workspace retains
one Zed-compatible Project:

- editors and language servers resolve through that Project;
- terminals inherit the Workspace's working-directory context;
- Files, Outline, Search, Git, tasks, and Debug inspect the same Project;
- Agent context comes from the active Workspace;
- Agent edits become normal buffers and Git changes;
- The Workspaces navigator observes and routes to those owners without copying
  them.

A terminal is therefore not embedded in chat, and the editor is not a separate
mode. They are peer Surfaces in one native pane grid. Dez detects supported
agents running in its terminals, including Codex, Claude Code, OpenCode, and
Herdr. Dez v0.2 discovers explicitly shared tmux sessions, live Herdr panes
through Herdr's JSON session registry and snapshot API, and cmux Workspaces.
tmux and Herdr attach through ordinary terminal tabs; cmux Workspaces open in
cmux. Native terminal tabs use observed foreground-process evidence for their
identity: supported agents receive provider marks, tmux and Herdr receive the
split-session mark, and ordinary shells remain generic terminals. Each
integration reports one truthful state:
**Missing** when its executable is unavailable, **Empty** when an available
source has no sessions, **Failed** when discovery did not complete, or
**Ready** when it returned sessions. A failed source preserves only its own
rows as **last known** while successful sources continue updating. Every
discovery command is bounded and cancelled before the next refresh cycle, so
an unresponsive external tool cannot freeze Workspace activity. Herdr applies
both per-endpoint deadlines and one deadline to the complete source scan, so a
large endpoint set cannot extend one refresh indefinitely. **Browse Running
Sessions…** clears transient navigator filters, refreshes all sources, expands
matching Workspace groups, and focuses Workspaces. Matching items appear under
the most specific Workspace; unmatched or pathless items remain visible under
**Other Running Sessions**. Selecting an unmatched tmux or Herdr item with a
known working directory opens that directory as a Workspace before attaching;
it never borrows a lexically similar remote Workspace. Process and layout
ownership always stays with the external application. **Terminal: Open tmux
Session** attaches or creates a stable session named from the active Workspace
with `tmux new-session -A`;
discovered sessions remain available individually in Workspaces. Arbitrary
machine PTYs remain excluded.

Opening the active Workspace in cmux also has a bounded handoff. If cmux does
not respond within eight seconds, Dez keeps the Workspace open, ends the
progress state, and reports a retryable failure instead of leaving an endless
“Opening…” notice.

Last-known external rows are never attached blindly. Their action refreshes
that source first; select a fresh row again before opening it. Retry resolves
the session by stable ID after discovery, reports when it has ended, and treats
a missing terminal provider as a visible failure instead of a successful no-op.

## Installation, Workspace access, and terminal ownership

On macOS, an application bundle launched from a DMG, App Translocation, a
temporary directory, or any location outside `/Applications` enters an
install-required Home state. Home renders **Install Dez to continue** and
**Install and Relaunch** as one native inline callout. It is not a startup
dialog, prompt, modal, or overlay. Dez does not restore Workspaces or start its
durable terminal Host until the action has copied the app to `/Applications`
and relaunched it; there is no background override.

Before restoring a local Workspace, Dez verifies that each root can be read.
Protected-folder failures are aggregated into one **Workspace access required**
notice with a native **Grant Access…** folder action. Select each exact blocked
root once in the single-folder picker. Dez validates the selection without
opening or replacing a Workspace; a readable root leaves the blocked set while
other denied roots remain visible. Relaunch retries startup restoration, and
**Open Recent Workspaces…** remains the explicit retry when a Workspace is
still missing. Git, Workspace Search, agents, and terminal creation wait rather
than repeatedly failing against the same root.

Each installed terminal Host has a generated endpoint identity: its generation,
socket, token file, and stable Host ID are one connection-owned value. Terminal
hooks receive those exact paths. Saved references from older Hosts appear as
**Legacy · Access blocked**; that label does not claim that the old process is
reachable or alive. Selecting one opens a separate new shell in its recorded
working directory. **Terminate Legacy Session…** is confirmed and contacts only
a matching legacy owner; failure leaves the record and any process untouched.
Dez never claims to migrate or silently terminate a running process.

Host connection, reconnection, and command cycles are bounded. An uncertain
command is never replayed, and queued work behind a broken connection fails as
stale instead of running later against a different state. The GUI groups every
frame-safe chunk for one user-input batch into one queue item, so queue admission
accepts or rejects the batch as a unit. It rejects a batch above the helper's
four-mebibyte PTY budget, caps aggregate bytes waiting in the GUI queue, and
shows an admission failure in the terminal Surface. Once transport starts
sending an accepted batch, a later failure can still leave a prefix delivered;
Dez treats that delivery as uncertain, logs the transport failure, and does not
replay it.
Awaited terminal-service commands also have bounded enqueue and response
deadlines.

tmux and Herdr attach commands run in the native terminal with a visible rerun
control. A failed attach keeps the external session unchanged, shows
**Retry Attach**, and refreshes discovery after the command finishes. A raw
Herdr terminal with no agent lifecycle is labeled **Available**. cmux remains
an external Workspace owner and is opened through its CLI instead of being
configured as Dez's shell.

## Visual baseline

Dez ships with an attributed adaptation of
[Lumin](https://github.com/frypan05/Lumin):

- **Lumin Blur** in dark mode;
- **Lumin Light** in light mode;
- **IBM Plex Sans** for native interface chrome and readable Settings;
- **Lilex** for editor, terminal, prompt, and review code;
- **Dez (Default)** as the product-facing built-in file and folder icon set;
- distinct Workspaces navigation, Main Work Area, native tab, and elevated-menu
  surfaces;
- restrained structural boundaries, with ordered hover, active, selection,
  scrollbar, and focus signals.

The application follows the operating system appearance by default. All theme
and typography roles remain configurable through normal settings.
**Settings → Appearance → Restore Native Dez Appearance** provides one scoped
recovery route for the full visual profile, while **Navigation & Layout →
Show Status Bar** exposes the native status strip directly.

The first-run visual profile keeps the native status bar visible with the
active file, language, line endings, diagnostics, and cursor context. Empty
Main Work Area panes use compact top-left editor chrome and direct actions
instead of a centered onboarding or workflow overlay.

Fresh Dez windows open the top-anchored native Home launcher inside the normal
Main Work Area tab frame. The tab strip and its adjacent Add control remain
visible before the first file or terminal opens. Home does not auto-read a
previous Workspace folder, avoiding a macOS privacy prompt for a stale recent
path. GitHub Actions marks ad-hoc artifacts **Dez Preview**; they are not v0.2
releases and macOS may ask again when their signing identity changes. A stable
workflow fails unless Developer ID signing and notarization credentials are
present, and the artifact passes signature, TeamIdentifier, Gatekeeper, and
stapled-ticket validation.

## Current status

The v0.2.2 source candidate contains the opinionated Dez shell,
identity isolation, Workspace composition, persistent Workspace state with
optional Workspaces navigation, ordinary closeable workspace-tool tabs,
explicit Agent Session state, host-owned local terminal lifecycle,
setting-controlled Back/Forward history, native draggable tab navigation,
first-run experience, Lumin/Plex/Lilex visual defaults, truthful tmux, Herdr,
and cmux discovery, explicit external attach/open actions, and a large set of
static product-contract checks.
Arbitrary machine terminals are deliberately absent because Dez cannot safely
control or attribute them.

**Live Preview is not implemented in the v0.2.2 source candidate.** URL actions
still open the system browser, while Markdown, SVG, and CSV use native file
previews. The next browser slice requires a real pane-scoped native surface and
Workspace item; Dez deliberately does not expose the inherited
geometry-only `BrowserDevelopment` recipe as a fake preview.

A public v0.2 release still requires exact build, rendered, restart, crash,
accessibility, integration, coexistence, and packaging evidence. The ordered
release ladder and open gates remain documented in
[v0.1 Product Hardening](./docs/src/development/dez/v0.1-product-hardening.md).
The active source-polish lane is
[v0.2 Workspace Polish](./docs/src/development/dez/v0.2-workspace-polish.md);
the completed ownership baseline remains in the historical
[v0.0.4 External Sessions Plan](./docs/src/development/dez/v0.0.4-external-sessions.md),
with exact results recorded in
[release evidence](./docs/src/development/dez/release-evidence.md). The v0.0.1
runbook remains historical evidence, not the current release plan.

## Documentation

- [What is Dez?](./docs/src/dez.md) — public product guide
- [v0.2 Workspace Polish](./docs/src/development/dez/v0.2-workspace-polish.md)
  — active native Workspace shell and source-polish contract
- [v0.1 Product Hardening](./docs/src/development/dez/v0.1-product-hardening.md)
  — preserved release ladder and acceptance gates
- [v0.0.4 External Sessions](./docs/src/development/dez/v0.0.4-external-sessions.md)
  — historical ownership baseline and artifact gates
- [v0.0.3 Production Readiness](./docs/src/development/dez/v0.0.3-production-readiness.md)
  — previous hardening train and evidence
- [v0.0.2 Completion Plan](./docs/src/development/dez/v0.0.2-completion-plan.md)
  — previous source and runtime recovery train
- [Fork Notes](./docs/src/development/dez/fork-notes.md) — permanent product
  and architecture source of truth
- [Roadmap](./docs/src/development/dez/roadmap.md) — dependency-ordered work
- [Product Strategy](./docs/src/development/dez/product-strategy.md) — target
  user, job, and product loop
- [Architecture Baseline](./docs/src/development/dez/architecture-baseline.md) —
  what the source owns today
- [Upstream Synchronization](./docs/src/development/dez/upstream-sync.md) —
  Zed integration policy and merge train
- [Codex Terminal Adapter](./docs/src/development/dez/codex-adapter.md) —
  optional structured terminal-agent evidence
- [Live Preview and Agent Model](./docs/src/development/dez/live-preview-and-agent-model.md)
  — terminal-first recommendation and the embedded-preview implementation gate

The inherited Zed documentation remains in `docs/src` while it is rewritten
into Dez vocabulary. When public prose and implementation notes disagree,
[Fork Notes](./docs/src/development/dez/fork-notes.md) is authoritative.

## Development

Dez is a large Rust workspace with platform-specific native dependencies.
Start with the inherited platform setup guides:

- [macOS](./docs/src/development/macos.md)
- [Linux](./docs/src/development/linux.md)
- [Windows](./docs/src/development/windows.md)

Useful source-only checks:

```sh
cargo fmt --all -- --check
cargo metadata --locked --offline --format-version 1 --no-deps
bash -n script/dez-identity-check
./script/dez-identity-check
git diff --check
```

These checks do not replace a release build or runtime verification.

## Upstream relationship

Dez is a deliberate fork, not a rewrite. The repository keeps Zed as an
upstream source and continuously classifies upstream changes as:

- inherited unchanged;
- inherited with Dez presentation;
- inherited as runtime substrate;
- inherited with Workspace scope; or
- deliberately deferred.

Product-language and ownership conflicts resolve in favor of the
[Fork Notes](./docs/src/development/dez/fork-notes.md), while editor correctness,
language support, platform fixes, performance, and security should continue to
flow from Zed.

## Contributing

The public contributor workflow is being prepared for v0.2. Until its
fork-specific policy is complete, use [CONTRIBUTING.md](./CONTRIBUTING.md) for
the inherited engineering workflow and include:

- the user problem;
- the ownership boundary affected;
- source and documentation changes;
- non-building checks run; and
- runtime evidence when behavior or visuals change.

Do not report visual, restart, crash, or persistence behavior as verified
without observing it.

## License and attribution

Dez retains Zed's licensing structure: source is primarily
[GPL-3.0-or-later](./LICENSE-GPL), with
[Apache-2.0](./LICENSE-APACHE) components where marked. Third-party assets keep
their own licenses, including:

- Lumin by Daksh Sharma under the MIT License;
- IBM Plex Sans under the SIL Open Font License 1.1;
- Lilex under the SIL Open Font License 1.1; and
- JetBrains Mono under the SIL Open Font License 1.1.

Dez is an independent fork and is not an official Zed Industries product.
