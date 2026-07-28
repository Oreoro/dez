# Dez

**A native IDE for terminal-native developers supervising code, commands, and
coding agents in one focused workspace.**

Dez is a source-available preview built on
[Zed](https://github.com/zed-industries/zed). It keeps Zed's fast native
editor, language tooling, Git, debugger, tasks, remote-development substrate,
and agent ecosystem, then reorganizes them around a clearer product promise:

> See what is running, what needs attention, what changed, and what is ready
> for review without reconstructing terminal and editor state.

This repository is the **Dez v0.0.2 source candidate**. It is not yet a signed
or supported binary release.

## What Dez does

Dez treats editing, terminal work, and agent work as parts of the same
Workspace instead of separate applications or hidden panel modes.

- **Main Work Area** — files, terminals, search results, settings, diagnostics,
  previews, and reviews are ordinary movable and splittable Surfaces.
- **Projects** — the stable navigation rail for open codebases and their Agent
  Sessions. It shows attention and lifecycle without owning duplicate tabs or
  processes. On macOS, a separate **Machine Terminals** section can list
  current-user terminals from other applications as ephemeral, read-only
  observations.
- **Workspace Tools** — Files, Outline, Git, and Debug are different views of
  the active Workspace's single Project.
- **Built-in Agent** — an optional provider-backed conversation surface stays
  beside the editor. It is distinct from terminal agents such as Codex and
  Claude Code, which start through **Open Agent Terminal**. All edits still land
  in ordinary buffers and Git changes, so the same diagnostics, diff, and
  review tools apply.
- **Terminal Sessions** — terminals open in the Main Work Area. Session
  identity, deliberate close/end behavior, and honest unavailable-session
  recovery define the default. Packaged Dez builds place local interactive
  shells under a small process-owning terminal service, so closing or
  accidentally losing the GUI does not end the computation. Reopening Dez
  reattaches the same Session; task and remote terminals keep their existing
  lifecycle semantics.
- **Evidence and review** — Dez distinguishes observed facts from reported or
  unknown state, then uses Workspace, terminal, command, check, file, and Git
  evidence to make review safer.

The result is an IDE that can follow the full loop:

```text
open a Workspace
→ edit or delegate work
→ observe the Project and its agent Sessions
→ inspect files, diagnostics, commands, and Git changes
→ review the result
→ resume without rebuilding context
```

The window is deliberately not a dashboard of equal columns. Each Workspace
owns one Project, one Main Work Area, its terminals, and its Git state.
Projects, Workspace Tools, and Built-in Agent compete for one optional
auxiliary slot. The active Workspace exposes one **Workspace Layout** picker.
Its six public layouts are named after the surfaces they show: **Work Area +
Files**, **Work Area + Built-in Agent**, **Focus Work Area**, **Code +
Terminal**, **Review Changes**, and **Debug**. The first three use one work
area. The last three use at most two populated work areas and never manufacture
an empty column.

From a selected Project Session row, `Enter` returns to the existing Session,
`Shift+F` opens its Workspace files, `Shift+G` opens its change review, and
`Shift+V` opens evidence-backed Session details. The same Files, Review Changes,
and Session Details handoff appears on standalone terminals; it never creates a
duplicate shell or project. If the owning Workspace is closed, Files restores
that Workspace and the exact selected Session before revealing the tree.

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
- Projects observes and routes to those owners without copying them.

A terminal is therefore not embedded in chat, and the editor is not a separate
mode. They are peer Surfaces in one native pane grid.

## Visual baseline

Dez ships with an attributed adaptation of
[Lumin](https://github.com/frypan05/Lumin):

- **Lumin Blur** in dark mode;
- **Lumin Light** in light mode;
- **IBM Plex Sans** for native interface chrome and readable onboarding;
- **Lilex** for editor, terminal, prompt, and review code;
- **Dez (Default)** as the product-facing built-in file and folder icon set;
- distinct rail/drawer, Main Work Area, tab, and elevated-overlay surfaces;
- restrained structural boundaries, with ordered hover, active, selection,
  scrollbar, and focus signals.

The application follows the operating system appearance by default. All theme
and typography roles remain configurable through normal settings.

## Current status

The v0.0.2 source candidate already contains the opinionated Dez shell,
identity isolation, Workspace composition, Project navigation, session vocabulary,
host-owned local terminal lifecycle, browser-like Back/Forward navigation,
first-run experience, Lumin/Plex/Lilex visual defaults, read-only
machine-terminal discovery on macOS, and a large set of static
product-contract checks.

Before a public binary release, the project still requires a build of the
current source checkpoint plus rendered, restart, crash, accessibility, and
coexistence evidence on supported platforms. The exact state and open gates are
documented in the
[v0.0.2 Completion Plan](./docs/src/development/dez/v0.0.2-completion-plan.md)
and
[release evidence](./docs/src/development/dez/release-evidence.md). The v0.0.1
runbook remains historical evidence, not the current release plan.

## Documentation

- [What is Dez?](./docs/src/dez.md) — public product guide
- [v0.0.2 Completion Plan](./docs/src/development/dez/v0.0.2-completion-plan.md)
  — current release order and acceptance gates
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

The public contributor workflow is being prepared for v0.0.2. Until its
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
