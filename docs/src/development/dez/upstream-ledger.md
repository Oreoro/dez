# Upstream Feature Ledger

This ledger records the current upstream integration target and the intended
Dez treatment of user-visible changes. Update it during every merge train.

## Integration target {#integration-target}

- Source checkpoint: `c2335969f994af4c7de6fa43e91eb1c93b3f1bb5`
- Integration merge: `2be63cfea347006e407934754086bbef62d482c2`
- Incorporated `upstream/main`: `9d0ef37a25711c00bf6d1ba1142e9de4f4a122a9`
- Current merge base: `9d0ef37a25711c00bf6d1ba1142e9de4f4a122a9`
- Current divergence: 242 Dez commits and 0 upstream commits after the merge base
- Latest fetched stable tag: `v1.11.3`
- Integration date: 2026-07-22

The merge was rehearsed in a detached temporary worktree, then integrated on
`codex/canvas-plan` after the Dez source slice was checkpointed. The merge has
two explicit parents and was not squashed or rebased.

## Integration result {#integration-result}

The real merge produced eleven conflicted paths:

| Path                                                   | Class                       | Integrated resolution                                                                                                                                              |
| ------------------------------------------------------ | --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `assets/settings/default.json`                         | Settings and defaults       | Preserved Dez offline, Canvas, privacy, updater, terminal-first, and sidebar-native shell defaults.                                                                |
| `crates/agent_ui/src/conversation_view/thread_view.rs` | Agent presentation          | Combined upstream Markdown-fragment behavior with Dez pane-splitting behavior.                                                                                     |
| `crates/markdown_preview/src/markdown_preview_view.rs` | Surface presentation        | Retained upstream link hover, source-position, and file-link behavior with Dez preview-first opening and readable Canvas styling.                                  |
| `crates/settings_content/src/sidebar.rs`               | Settings schema             | Preserved the consolidated Dez `SidebarChrome` owner rather than restoring an overlapping upstream title-bar schema.                                               |
| `crates/settings_ui/src/page_data.rs`                  | Settings presentation       | Kept the Dez sidebar-chrome section and terminology.                                                                                                               |
| `crates/tasks_ui/Cargo.toml`                           | Dependency wiring           | Combined Dez settings dependencies with the current upstream tree-sitter dependencies.                                                                             |
| `crates/title_bar/src/title_bar.rs`                    | Shell and collaboration     | Preserved the Dez sidebar-native hierarchy and account/collaboration demotion; compatible upstream changes outside the conflicting render branch remain inherited. |
| `crates/title_bar/src/title_bar_settings.rs`           | Modify/delete architecture  | Kept the file deleted because `sidebar_chrome_settings` is the single Dez settings owner.                                                                          |
| `crates/workspace/src/pane.rs`                         | Pane lifecycle and tests    | Preserved Dez empty-launch behavior and upstream leased-workspace pinned-tab coverage.                                                                             |
| `crates/workspace/src/pane_group.rs`                   | Pane focus and rendering    | Kept the visible-index active-pane correction with Dez focus and layout behavior.                                                                                  |
| `crates/workspace/src/workspace.rs`                    | Workspace routing and tests | Preserved Dez saved-Canvas-layout coverage and upstream remote-base-path coverage while inheriting compatible workspace fixes.                                     |

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
