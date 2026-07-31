# Dez

**A native IDE for terminal-native developers supervising code, commands, and
coding agents in one focused workspace.**

Dez is a source-available preview built on
[Zed](https://github.com/zed-industries/zed). It keeps Zed's fast native
editor, language tooling, Git, debugger, tasks, remote-development substrate,
and agent ecosystem, then reorganizes them around a clearer product promise:

> See what is running, what needs attention, what changed, and what is ready
> for review without reconstructing terminal and editor state.

This repository currently carries the **Dez v0.1.0 source candidate**. It is
not yet a signed or supported binary release; promotion depends on the exact
remote artifact and evidence gates documented below.

## What Dez does

Dez treats editing, terminal work, and agent work as parts of the same
Workspace instead of separate applications or hidden panel modes.

- **Main Work Area** — files, terminals, search results, settings, diagnostics,
  previews, and reviews are ordinary movable and splittable Surfaces.
- **Workspaces** — a stable, collapsible left navigator for codebases and their
  open Surfaces and Agent Sessions. Its collapsible **Active Workspace**
  section mirrors the native center-pane tabs, grouping them by pane only when
  the Workspace is split. Selecting a row activates that exact native tab;
  tab ownership, dirty state, close behavior, dragging, and ordering remain in
  Zed's pane model rather than being copied into the navigator. Agent Session
  rows show provider identity and Running, Needs Input, Waiting for Permission,
  Reconnecting, Completed, or Error state on a secondary row, leaving the
  terminal or Session title as the primary navigation label. A
  multi-root Workspace leads with its first root and a bounded root count; all
  root names remain searchable and available in the header tooltip and
  accessibility label. A Workspace can explicitly attach a path-matched tmux
  or Herdr session, or open a path-matched cmux Workspace in cmux, without
  taking ownership. Each external item appears beneath the most specific
  matching Workspace with source, semantic state, and working-directory
  metadata, beside its associated Dez terminals and agents. Workspace headers
  show live Git branch and changed-file counts when available. Unrelated
  machine terminals do not leak into the list.
- **Workspace tools** — Files, Outline, Git, Debug, and the optional
  provider-backed Built-in Agent are ordinary draggable and closeable native
  tabs. They are not nested panels or a mandatory second sidebar.
- **Built-in Agent** — is distinct from terminal agents such as Codex, Claude
  Code, OpenCode, and Herdr, which start through **Open Agent Terminal**. All
  edits still land in ordinary buffers and Git changes, so the same
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

The result is an IDE that can follow the full loop:

```text
open a Workspace
→ edit or delegate work
→ observe the Workspace and its agent Sessions
→ inspect files, diagnostics, commands, and Git changes
→ review the result
→ resume without rebuilding context
```

The primary user story is deliberately short. **Home** starts or resumes a
Workspace. **Open Agent Terminal** launches the configured terminal workflow.
**Workspace Navigator** switches codebases, returns to open tabs, and surfaces
attention. **Browse Files** and **Review Changes** bring the result back into
the same Main Work Area. The native tab-strip `+` is the shared Add menu for
every step; no separate dashboard or onboarding mode is required.

The window is deliberately not a dashboard of equal columns. Each Workspace
owns one Project, one Main Work Area, its terminals, and its Git state.
Workspaces stays global on the left. Files, Git, Outline, Debug, Built-in Agent,
commits, diffs, reviews, agent terminals, and ordinary terminals use the
native workspace tab and pane model instead of another navigation system.
When Workspaces is visible, **Active Workspace** provides a compact vertical
route to those same open tabs. It stays flat for a single pane and introduces
small **Pane 1**, **Pane 2**, and later group labels only when a real split
exists. The section is collapsible, remembers its disclosure state, and
disappears while Workspace search is active so search results keep the full
navigator.
Those tabs support reorder, cross-pane drag, preview replacement, pinning,
closing, and horizontal or vertical splits. A terminal can therefore sit
below code without Dez manufacturing a separate multiplexer UI.
The adjacent native `+` reopens Home, opens Recent Workspaces, or routes to a
terminal, file, search, Files, Changes, Debug, or Built-in Agent surface
through the existing Zed actions. Its terminal submenu names the default Agent
Terminal first, followed by Shell, tmux Workspace, and explicit provider
launchers.

Six optional layout commands remain available through **View** and Command
Search: **Work Area + Files**, **Work Area + Built-in Agent**, **Focus Work
Area**, **Code + Terminal**, **Review Changes**, and **Debug**. They are hidden
from the default titlebar so the primary navigation remains obvious.

Keyboard navigation remains first-class: macOS `⌘1`–`⌘8` selects native tabs
and `⌘9` selects the last tab; Linux and Windows use `Alt+1`–`Alt+9`.
`Ctrl+Tab` opens the recent-tab switcher, split-pane focus keeps the native
Zed chords, `Ctrl+\`` opens the configured agent terminal, and
`Ctrl+Shift+\`` always opens a shell. Command Search can launch Codex, Claude
Code, OpenCode, a shell, a Workspace-named tmux session, or cmux directly.
The native tab-strip `+` exposes the same terminal choices, **Browse tmux,
Herdr & cmux…**, and **Open Workspace in cmux** without creating a second
navigation system. **Settings → Keyboard & Vim**
exposes shortcut search, conflict inspection, base keymaps, and optional full
Vim or Helix editing. Vim and Helix share native leader destinations for
recent tabs (`Space b`), files (`Space f`), the configured agent terminal
(`Space t`), a shell (`Space T`), and Workspace search (`Space /`).

From a selected Agent Session row in Workspaces, `Enter` returns to the
existing Session, `Shift+F` opens its Workspace files, `Shift+G` opens its
change review, and `Shift+V` opens evidence-backed Session details. The same
Files, Review Changes, and Session Details handoff appears on standalone
terminals; it never creates a duplicate shell or project. If the owning
Workspace is closed, Files restores that Workspace and the exact selected
Session before revealing the tree.

Read [What is Dez?](./docs/src/dez.md) for the product model and a concrete
workflow.

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
Herdr. Dez v0.1 discovers explicitly shared tmux sessions, live Herdr panes,
and cmux Workspaces. tmux and Herdr attach through ordinary terminal tabs; cmux
Workspaces open in cmux. Discovery updates automatically and can be refreshed
explicitly from a Workspace's options menu. The menu shows when it is checking
and explains when no external session matches instead of silently removing the
integration. Process and layout ownership always stays with the external
application. **Terminal: Open tmux Workspace** attaches or creates a stable
session named from the active Workspace with `tmux new-session -A`; discovered
sessions remain available individually in Workspaces. Arbitrary PTYs remain
read-only.

## Installation, Workspace access, and terminal ownership

On macOS, an application bundle launched from a DMG, App Translocation, a
temporary directory, or any location outside `/Applications` enters an
install-required Home state. Dez does not restore Workspaces or start its
durable terminal Host until the native **Install and Relaunch** action has
copied the app to `/Applications`. There is no background override and no
custom installation overlay.

Before restoring a local Workspace, Dez verifies that each root can be read.
Protected-folder failures are aggregated into one **Workspace access required**
notice with a native **Grant Access…** folder action. Git, Workspace Search,
agents, and terminal creation wait rather than repeatedly failing against the
same root.

Each installed terminal Host has a generated endpoint identity: its generation,
socket, token file, and stable Host ID are one connection-owned value. Terminal
hooks receive those exact paths. Sessions from older Hosts remain alive and
appear as **Legacy · Access blocked**. Selecting one opens a new shell in its
recorded working directory; **Terminate Legacy Session…** is separate,
confirmed, and contacts only the legacy owner. Dez never claims to migrate or
silently terminates a running process.

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
path. GitHub Actions marks ad-hoc artifacts **Dez Preview**; they are not v0.1
releases and macOS may ask again when their signing identity changes. A stable
workflow fails unless Developer ID signing and notarization credentials are
present, and the artifact passes signature, TeamIdentifier, Gatekeeper, and
stapled-ticket validation.

## Current status

The v0.1.0 source candidate contains the opinionated Dez shell,
identity isolation, Workspace composition, persistent Workspaces navigation,
ordinary closeable workspace-tool tabs, explicit Agent Session state,
host-owned local terminal lifecycle, setting-controlled Back/Forward history,
native draggable tab navigation, first-run experience, Lumin/Plex/Lilex visual
defaults, project-scoped tmux and Herdr attachment, project-scoped cmux
Workspace opening, and a large set of static product-contract checks.
Arbitrary machine terminals are deliberately absent because Dez cannot safely
control or attribute them.

**Live Preview is not implemented in the current candidate.** URL actions
still open the system browser, while Markdown, SVG, and CSV use native file
previews. The next browser slice requires a real pane-scoped native surface and
Workspace item; Dez deliberately does not expose the inherited
geometry-only `BrowserDevelopment` recipe as a fake preview.

A public v0.1 release still requires exact build, rendered, restart, crash,
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

The public contributor workflow is being prepared for v0.1. Until its
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
