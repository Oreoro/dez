# Upstream Feature Ledger

This ledger records the current upstream integration target and the intended
Dez treatment of user-visible changes. Update it during every merge train.

## Integration target {#integration-target}

- Source checkpoint: `4cbe99da2263e781f7aa8725e4dc67ea3d05afc3`
- Integration merge: pending the current two-parent merge commit
- Incorporated `upstream/main`:
  `b0f145f4aec671970340a528cb8197181e969e8c`
- Merge base before integration:
  `9d0ef37a25711c00bf6d1ba1142e9de4f4a122a9`
- Divergence before integration: 615 Dez commits and 46 upstream commits
- Latest stable release reference: `v1.12.0`
- Integration date: 2026-07-24

The merge is being integrated on `agent/v0.0.2-upstream-parity` after the Dez
source slice was checkpointed. It will retain two explicit parents and will not
be squashed or rebased.

## Integration result {#integration-result}

The real merge produced four conflicted paths:

| Path                              | Class               | Integrated resolution                                                                                                                                                            |
| --------------------------------- | ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Cargo.lock`                      | Dependency graph    | Regenerated the merged locked graph, including the internal `paths` edges required by Dez crates.                                                                                |
| `crates/editor/src/editor.rs`     | Breadcrumb behavior | Kept Dez's breadcrumb-symbol preference and inherited upstream's singleton-buffer filename fallback.                                                                             |
| `crates/terminal/src/terminal.rs` | Process lifetime    | Inherited upstream process-group shutdown for ordinary terminals while preserving detach-on-GUI-exit for hosted terminals. Explicit hosted-session termination remains separate. |
| `crates/zed/Cargo.toml`           | Product identity    | Preserved the `dez` binary and default run target, and advanced the package version to `0.0.2`.                                                                                  |

Compile-time adaptation also added stable IDs to Dez accessibility landmarks,
preserved reason-specific unavailable-session recovery, updated tooltip
lifetimes and keybinding usage, and declared the sidebar's `paths` dependency.
These are compatibility repairs rather than new product claims.

## New capability treatment in this train {#new-capability-treatment}

### Inherit

- Preserve preview tabs for reference multibuffers.
- Fix exact mid-line `edit_file` matching and Bedrock message/tool-call ID
  collisions.
- Load buffer chunks lazily and track dynamic LSP registrations by ID.
- Fall back to filenames in breadcrumbs when symbols are unavailable.
- Fix CRLF formatter cursor placement, search autoscroll, nested repository
  excludes, remote Git commit templates, context-server working-directory
  restarts, and Git refresh after reset or fetch.
- Add the deliberate Git commit `--no-verify` option.
- Close ordinary terminal process groups instead of leaving descendants.

### Adapt

- Upstream terminal process-group cleanup does not change hosted-session
  ownership: a normal GUI exit detaches from a hosted session, while explicit
  termination closes its process group.
- Upstream breadcrumb fallback continues to respect the Dez preference that
  hides breadcrumb symbols.
- New GPUI stateful accessibility requirements keep Dez labels and landmarks
  by adding stable IDs rather than removing roles.
- Unavailable saved terminal and agent sessions preserve distinct failure
  reasons without starting a replacement process.

### Deferred

- Upstream temporarily placed agent terminal sandboxing behind a disabled
  feature flag. Dez inherits that source state for parity but does not claim
  sandbox protection for v0.0.2 until the upstream withdrawal is understood,
  the threat model is rechecked, and runtime evidence exists.

## Capability treatment {#capability-treatment}

### Inherit unchanged {#inherit-unchanged}

- ACP protocol 1.3 and registry checksum validation.
- Mistral reasoning-effort support and provider model updates.
- File-link line-number opening.
- Dev-container specification fixes and Compose-service restoration.
- Terminal modal-focus protection and shift-drag selection under mouse
  tracking.
- File Finder debounce behavior.
- Command Palette reopening and submenu fixes.
- Search regex escaping, soft-wrap restoration, and terminal/editor selection
  seeding.
- Git diff search, full-file solo diffs, branch-picker fixes, context-menu
  highlighting, and GPG-agent handling.
- Editor cursor, selection, blame, multibuffer, diagnostics, runnable, snippet,
  and prepaint improvements.
- Workspace crash, preview-button targeting, dock clipping, and recent-project
  freshness fixes.
- Gutter run-status clearing, Git context-menu focus, safe unsaved-buffer gutter
  actions, recent-folder keybindings, and `ReopenLastPicker` behavior.
- Picker parent/child navigation and terminal alternate-screen bottom
  alignment.
- macOS pasteboard retention, extension toolchain checks, icon validation, and
  security-sensitive crash-handler configuration.

### Inherit with Dez presentation {#inherit-with-dez-presentation}

- Image Viewer custom zoom uses the existing Canvas density, radius, and
  contrast treatment.
- Markdown Preview link destinations retain Dez readable-sheet styling.
- Remote Markdown subfolder links retain upstream relative-link resolution and
  resolve within the owning Workspace/Host scope.
- Collaboration Panel indentation and layout remain secondary to local Session
  Rail navigation.
- Collaboration channel pressed-state fixes remain compatibility polish and do
  not restore collaboration chrome to the default Dez shell.
- Title-bar worktree naming remains contextual metadata rather than permanent
  workspace ownership.
- Agent thinking and message-control visuals use Dez Canvas tokens.
- Pane border/dimming fixes preserve one unmistakable focus indicator.

### Inherit as runtime substrate {#inherit-runtime-substrate}

- Cloud API WebAssembly support remains available to compatible upstream
  features but does not change local-first startup.
- Dev-container and remote-process improvements remain Host candidates; they do
  not establish a second Dez transport.
- Terminal, task, ACP, and agent changes remain candidates for the future
  unified Session and Run adapters.

### Inherit with workspace scoping {#inherit-workspace-scoping}

- Search, recent-project routing, worktree task setup, Git selection, runnable
  results, and title metadata must use the active workspace's visible evidence
  once scoped `Project` behavior lands.

### Deferred {#deferred}

- No upstream capability in this batch is deliberately excluded.
- Final Host/Session mapping of terminal, task, ACP, and remote changes waits
  for the native runtime boundary.
- Project-oriented labels may remain temporarily where changing them before
  ownership work would create compatibility aliases rather than correct
  semantics.

## Verification required after resolution {#verification-required}

- The merge is resolved on the reversible `codex/canvas-plan` integration
  branch with source checkpoint and two-parent merge provenance.
- Run focused tests for every conflicted crate.
- Exercise terminal focus, pane focus, Markdown links, image zoom, settings
  search, task setup, collaboration-offline behavior, and workspace restore.
- `cargo fmt --all -- --check`, `git diff --check`,
  `cargo metadata --no-deps`, and `./script/dez-identity-check` pass after
  conflict resolution.
- Run formatting, the `dez` build, bundle audit, and live smoke gate before
  landing.
