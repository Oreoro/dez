# Upstream Feature Ledger

This ledger records the current upstream integration target and the intended
Dez treatment of user-visible changes. Update it during every merge train.

## Integration target {#integration-target}

- Current Dez commit: `820eb1cf5f0f3daf569a2621445cf0eb60daba64`
- Current merge base: `f14fea9bf3c93797d5161f7440ed418655bc6c57`
- Target `upstream/main`: `9d0ef37a25711c00bf6d1ba1142e9de4f4a122a9`
- Upstream commits after the merge base: 81
- Latest fetched stable tag: `v1.11.3`
- Rehearsal date: 2026-07-22

The rehearsal ran in a detached temporary worktree. It did not modify the
active product worktree and was aborted after evidence collection.

## Rehearsal result {#rehearsal-result}

The merge produced ten conflicted paths:

| Path                                                   | Class                          | Resolution intent                                                                                                                                        |
| ------------------------------------------------------ | ------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `assets/settings/default.json`                         | Settings and defaults          | Add upstream settings while preserving Dez offline, Canvas, privacy, updater, and terminal-first defaults.                                               |
| `crates/image_viewer/src/image_viewer.rs`              | Universal surface presentation | Retain upstream custom zoom and fold it into the existing Canvas-token treatment.                                                                        |
| `crates/markdown_preview/src/markdown_preview_view.rs` | Universal surface presentation | Retain upstream link-destination hover behavior and Dez preview/readability styling.                                                                     |
| `crates/settings_content/src/sidebar.rs`               | Settings schema                | Add upstream title-bar/worktree controls without losing Session Rail and Canvas settings.                                                                |
| `crates/settings_ui/src/page_data.rs`                  | Settings presentation          | Preserve upstream setting discoverability and Dez terminology/defaults.                                                                                  |
| `crates/tasks_ui/Cargo.toml`                           | Dependency wiring              | Use the newest compatible upstream dependency set plus existing Canvas dependencies.                                                                     |
| `crates/title_bar/src/title_bar.rs`                    | Shell and collaboration        | Retain upstream layout fixes and worktree controls while preserving Dez branding, offline startup, and pane-first shell behavior.                        |
| `crates/title_bar/src/title_bar_settings.rs`           | Modify/delete architecture     | Reconcile the upstream setting into the current consolidated title-bar settings owner; do not restore a duplicate owner.                                 |
| `crates/workspace/src/pane_group.rs`                   | Pane focus and rendering       | Retain upstream active-border/dimming fix and Dez focus, accessibility, and layout behavior.                                                             |
| `crates/workspace/src/workspace.rs`                    | Workspace routing and shell    | Retain upstream picker, run-status, recent-project, and workspace fixes while preserving App Session, EvidenceSet, terminal Host, and Dez shell changes. |

No conflict occurred in terminal process handling, terminal rendering, ACP,
remote transport, Git stores, workspace persistence, or agent lifecycle code.
Those areas still require focused regression tests after the merge because
clean textual application does not prove correct Dez semantics.

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

- Resolve each conflict in an isolated integration branch.
- Run focused tests for every conflicted crate.
- Exercise terminal focus, pane focus, Markdown links, image zoom, settings
  search, task setup, collaboration-offline behavior, and workspace restore.
- Run `./script/dez-identity-check` after conflict resolution.
- Run formatting, the `dez` build, bundle audit, and live smoke gate before
  landing.
