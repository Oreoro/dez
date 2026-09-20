## Why

dez's local control helper has a long, agent-specific command name and cannot inspect or operate ordinary terminals. dez needs one stable, versioned local CLI that preserves Agent Thread automation and lets a process in one dez terminal safely control another terminal in the same workspace.

## What Changes

- Install `dezctl` as the primary local control command, with noun-first `thread` and `terminal` command groups plus a capability-reporting `status` command.
- **BREAKING**: Stop building and packaging the `dez-agent-control` executable. Package only `dezctl` as the local control client.
- **BREAKING**: Remove the old flat command forms, legacy request names, and legacy request framing. The installed dez version provides only its current commands and protocol.
- On every application launch, write the current release-channel-scoped executable marker and update only previously installed dez control skills to the version bundled with that dez release.
- Let users inspect, install, update, and uninstall a dedicated dez control skill without changing global `AGENTS.md`, `CLAUDE.md`, or other general instruction files.
- Add process-local terminal identities and lifecycle registration for live PTY-backed terminals.
- Resolve callers from operating-system process ancestry, with the existing constrained Agent Thread fallback, and restrict terminal access to the caller's workspace.
- Add commands to identify and list terminals, read bounded snapshots, send validated text or keys, run a command, and wait for matching output.
- Version requests and responses, add typed results and error codes, cap request and response sizes, and use length-prefixed framing for all commands and cancellable long-lived waits.
- Keep terminal control local to the machine that owns the dez control server. Do not expose remote PTYs, cross-workspace terminals, pane management, persistent terminal hosting, or raw byte input.
- Package `dezctl` on macOS, Linux, and Windows, remove the old executable from package and updater manifests, and update new Agent Thread instructions to discover `dezctl` through the existing release-channel-scoped marker.

## Capabilities

### New Capabilities

- `local-terminal-control`: Defines the current-only `dezctl` command and protocol surface, terminal identity and caller boundary, terminal read/input/wait behavior, packaging, and local-only scope.

### Modified Capabilities

- `terminal-agent-threads`: Changes Agent Thread control commands to `dezctl thread` and adds an opt-in dez-owned skill that later dez versions can update safely.

## Impact

- Affects the Agent Control client, server, transport framing, Unix socket and Windows named-pipe handling, and release-channel endpoint discovery.
- Adds terminal lifecycle registration and workspace-aware lookup across terminal, terminal view, workspace, and application state.
- Extends terminal content access, key mapping reuse, foreground entity updates, and asynchronous output observation.
- Changes application bundles and Linux and Windows packages to include `dezctl` and remove `dez-agent-control`.
- Replaces Agent Thread global instruction handling with an opt-in, versioned skill that dez can update after installation.
- Requires protocol, CLI, server, terminal behavior, route-boundary, instruction, and package tests on supported platforms.
