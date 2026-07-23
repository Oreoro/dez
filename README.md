# Dez

**A native IDE for terminal-native developers supervising code, commands, and
coding agents in one durable workspace.**

Dez is a source-available preview built on
[Zed](https://github.com/zed-industries/zed). It keeps Zed's fast native
editor, language tooling, Git, debugger, tasks, remote-development substrate,
and agent ecosystem, then reorganizes them around a clearer product promise:

> See what is running, what needs attention, what changed, and what is ready
> for review without reconstructing terminal and editor state.

This repository is the **Dez v0.0.1 source candidate**. It is not yet a signed
or supported binary release.

## What Dez does

Dez treats editing, terminal work, and agent work as parts of the same
Workspace instead of separate applications or hidden panel modes.

- **Main Work Area** — files, terminals, search results, settings, diagnostics,
  previews, and reviews are ordinary movable and splittable Surfaces.
- **Session Rail** — a compact supervision view of Workspaces, Agent Sessions,
  Terminal Sessions, attention, lifecycle, and recent activity. It navigates
  real work; it does not own duplicate tabs or processes.
- **Workspace Tools** — Files, Outline, Git, and Debug are different views of
  the active Workspace's single Project.
- **Agent** — native and external-agent conversations stay beside the editor.
  Agent edits land in ordinary buffers and Git changes, so the same diagnostics,
  diff, and review tools apply.
- **Terminal Sessions** — terminals open in the Main Work Area. Session identity,
  deliberate detach/terminate behavior, honest unavailable-session recovery,
  and an experimental process-owning Host form the path toward durable
  computation.
- **Evidence and review** — Dez distinguishes observed facts from reported or
  unknown state, then uses Workspace, terminal, command, check, file, and Git
  evidence to make review safer.

The result is an IDE that can follow the full loop:

```text
open a Workspace
→ edit or delegate work
→ observe Sessions and attention
→ inspect files, diagnostics, commands, and Git changes
→ review the result
→ resume without rebuilding context
```

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
- the Session Rail observes and routes to those owners without copying them.

A terminal is therefore not embedded in chat, and the editor is not a separate
mode. They are peer Surfaces in one native pane grid.

## Visual baseline

Dez ships with an attributed adaptation of
[Lumin](https://github.com/frypan05/Lumin):

- **Lumin Blur** in dark mode;
- **Lumin Light** in light mode;
- **JetBrains Mono** for editor, terminal, prompts, and code;
- a bundled sans-serif face for readable interface chrome;
- restrained focus, selection, pane, active-line, and scrollbar contrast.

The application follows the operating system appearance by default. All theme
and typography roles remain configurable through normal settings.

## Current status

The v0.0.1 source candidate already contains the opinionated Dez shell,
identity isolation, Workspace composition, Session Rail, session vocabulary,
terminal lifecycle safeguards, first-run experience, Lumin/JetBrains visual
defaults, and a large set of static product-contract checks.

Before a public binary release, the project still requires a consolidated
release build and rendered, restart, crash, accessibility, and coexistence
evidence on supported platforms. The exact state and open gates are documented
in the [v0.0.1 release runbook](./docs/src/development/dez/v0.0.1-release-runbook.md)
and [release evidence](./docs/src/development/dez/release-evidence.md).

## Documentation

- [What is Dez?](./docs/src/dez.md) — public product guide
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

The public contributor workflow is being prepared for v0.0.1. Until its
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

- Lumin by Daksh Sharma under the MIT License; and
- JetBrains Mono under the SIL Open Font License 1.1.

Dez is an independent fork and is not an official Zed Industries product.
