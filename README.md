# Dez

[English](./README.md) | [简体中文](./README.zh-CN.md)

Dez is a terminal-first fork of [Flint](https://github.com/shenghsi/flint) — itself a fork of [Zed](https://github.com/zed-industries/zed) — built for developers who use tools such as Codex, Claude Code, Pi, and OpenCode from the command line.

It keeps Zed's fast, GPU-accelerated editor, language support, Git tooling, and extension ecosystem while replacing the built-in AI product with a focused workspace for terminal-based coding agents. See [`docs/dez-mission.md`](./docs/dez-mission.md) for what Dez is — and what it deliberately is not.

![Dez workspace with an agent thread panel and a Claude Code terminal session](assets/screenshots/dez-workspace.png)

## Why Dez?

Dez brings an IDE's editing and project tools together with the directness of a terminal-native workflow. Files, terminals, diffs, Git state, and long-running coding-agent sessions share one workspace, so you can delegate from the command line and review the result without moving between separate applications.

| | Dez | Conventional IDEs | Terminal emulators |
| --- | --- | --- | --- |
| Code intelligence and extensions | Built in | Built in | Added through command-line tools |
| Terminals as primary workspaces | Yes | Usually secondary panels | Yes |
| Persistent CLI agent sessions | Organized as threads alongside files and diffs | Often replaced by proprietary chat surfaces | Available, but managed as terminal tabs or windows |
| Git changes and visual diff review | Integrated | Integrated | Primarily command-line driven |
| Agent credentials and configuration | Uses your existing CLI setup | Often configured separately by the IDE | Uses your existing CLI setup |

Dez is for developers who want the project awareness and visual review tools of an IDE without giving up command-line agents, terminal workflows, or control of their local configuration.

### Added

- **First-class terminals:** New terminals open as tabs in the center workspace by default, alongside files and diffs.
- **Codex, Claude Code, Pi, and OpenCode threads:** Launch any supported CLI directly in a terminal-backed thread using its existing authentication and configuration.
- **Agent Threads panel:** Organize sessions, discover recent threads on the local machine or connected remote host, and resume work using titles from each agent's history.
- **Agent status and continuity:** See Codex and Claude Code plan usage and reset countdowns, receive desktop notifications when a thread needs attention, and optionally reopen resumable sessions after restarting Dez.
- **Agent-controlled terminals:** Install Dez's optional `Dezctl` control skill so agents can inspect terminal state, send input, wait for output, and create terminals or sibling Agent Threads from local and supported remote sessions.
- **Cross-agent handoff:** Preview a bounded handoff document from a live local thread, then continue the work in a fresh thread with another supported agent.
- **Remote agent threads:** Run supported agents on SSH remotes using either the remote's configured CLI (`Direct`) or pinned Dez-managed binaries whose traffic is routed through local Dez (`Tunneled`).
- **Configurable agent workflows:** Set commands, arguments, environment variables, working directories, visibility, panel location, and default resume options.
- **Rich Markdown previews:** Render inline and display LaTeX equations through bundled MathJax when Node.js is available, plus expanded Mermaid diagram types and diagrams with YAML frontmatter.
- **CSV table previews:** Open saved `.csv` files as tables with independently resizable columns and a pinned row-number column.
- **Localized interface:** Use Dez in English or Simplified Chinese, with language selection available during onboarding and in the settings.
- **Faster visual navigation and review:** Distinguish file types with theme-aware colors and agents with recognizable brand icons, open project changes directly from the editor toolbar, and compare the working tree directly with a branch or a commit using Dez's Git and diff views.

![Agent Threads menu with Hand off to Codex and Hand off to Pi actions](assets/screenshots/handoff.png)

### Removed

Dez does not ship Zed's native agent and chat interface, hosted AI models, model-provider configuration, Copilot or edit predictions, account and billing UI, or real-time collaboration and calls. The result is a smaller, local-first product surface that leaves agent behavior and credentials with the CLI tools you already use.

## Try Dez

Download the latest stable build for macOS, Linux, or Windows from
[GitHub Releases](https://github.com/Oreoro/dez/releases/latest). Nightly
builds are available from the moving
[`nightly` release](https://github.com/Oreoro/dez/releases/tag/nightly).

### macOS

After moving Dez into `/Applications`, remove the quarantine attribute so macOS will allow the unsigned app to open:

```sh
xattr -cr /Applications/Dez.app
```

### Linux

Install Dez into `~/.local` (no root required, and in-app auto-update works):

```sh
curl -f https://raw.githubusercontent.com/Oreoro/dez/main/script/install.sh | sh
```

To install the Nightly channel instead of Stable, set `ZED_CHANNEL=nightly`:

```sh
curl -f https://raw.githubusercontent.com/Oreoro/dez/main/script/install.sh | ZED_CHANNEL=nightly sh
```

This installs the app bundle alongside a Stable install as
`~/.local/Dez-nightly.app`. Nightly checks the moving `nightly` release for
updates every six hours. The `Dez` command in `~/.local/bin` points at
whichever channel you installed most recently.

If `~/.local/bin` isn't already on your `PATH`, add it so you can launch Dez with `Dez`.

The `.deb` and `.rpm` packages install Dez system-wide under `/usr/lib/Dez`. Those builds are managed by your package manager, so in-app auto-update is disabled — update them with `apt`, `dnf`, etc.

### Remote Development

Dez supports SSH and WSL remote development while keeping the editor UI local. Files, terminals, tasks, language servers, and agent threads run on the remote host.

Remote agent threads can use either the remote host's own network (`Direct`) or a Dez-managed route (`Tunneled`). With `Tunneled`, Dez can provision pinned Codex, Claude Code, Pi, and OpenCode binaries on the remote host and route supported provider traffic back through the local Dez connection, which helps when a remote machine has restricted internet (VPN) access. Dez marks tunneled SSH projects in the title bar and project picker.

See [Remote Development](./docs/src/remote-development.md) for setup, SSH connection settings, port forwarding, and the `agent_route` option.

### Developing Dez

- [Building Dez for macOS](./docs/src/development/macos.md)
- [Building Dez for Linux](./docs/src/development/linux.md)
- [Building Dez for Windows](./docs/src/development/windows.md)

### Extensions

Dez is compatible with the [Zed extension registry](https://zed.dev/extensions). Extensions install and work without modification.

### Licensing

Dez source code is licensed primarily under GPL-3.0-or-later, with Apache-2.0 components where marked.

License information for third party dependencies must be correctly provided for CI to pass.

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/Dez-licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/Dez-licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).

### Acknowledgements

Dez stands on the work of three open-source projects:

- [Zed](https://github.com/zed-industries/zed) by Zed Industries — the editor, GPUI, and language tooling at the core.
- [Flint](https://github.com/shenghsi/flint) by shenghsi — the terminal-first fork and Agent Threads workspace Dez builds directly on.
- **Gram** — the security-hardened extension trust boundary ported into `extension_host` (see [`FORK.md`](./FORK.md) for the exact commits).

[`FORK.md`](./FORK.md) records every intentional divergence from Flint;
[`SYNC-LEDGER.md`](./SYNC-LEDGER.md) records every upstream adaptation.
