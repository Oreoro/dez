# FORK.md — dez divergence ledger

Every intentional divergence of dez from Flint (which itself diverges from
Zed — see `docs/terminal-first-fork.md` for that layer). Each entry
lists what, where, and why. If a divergence isn't listed here, it's a bug.

## Identity layer (commit: "Rebrand Flint as Dez across the entire tree")

| Divergence | Location | Why |
|---|---|---|
| Binary/crate rename `flint`→`dez` | `crates/dez/*`, workspace `Cargo.toml` | Product identity |
| Bundle IDs `dev.dez.Dez{,-Dev,-Nightly,-Preview}` | `crates/dez/Cargo.toml`, `crates/release_channel/src/lib.rs` | macOS app identity |
| URL scheme `dez://` | `crates/dez/Cargo.toml`, `crates/install_cli` | OS deep links |
| Env vars `DEZ_*` (was `FLINT_*`) | `crates/remote`, `crates/localization`, `crates/agent_threads` | Fork-scoped runtime knobs |
| Remote handshake `__DEZ_REMOTE_TARGET__` | `crates/remote/src/transport.rs` | Remote server protocol marker |
| Terminal env `TERM_PROGRAM=dez` | `crates/terminal/src/terminal.rs` (`insert_dez_terminal_env`) | Terminal identification |
| Release/docs URLs → `dez.dev` | `crates/release_channel/src/lib.rs`, docs | Product surface |
| dezctl skill (was flintctl) | `crates/agent_control_skill/skills/dezctl` | Agent control skill name |
| Identity guard script | `script/dez-identity-check` | CI gate against branding regressions |
| CI workflow `dez-build.yml` | `.github/workflows/` | Build + identity checks |

## Protected upstream surfaces (NOT renamed — extension compatibility)

- `zed_extension_api` crate name, `zed:api-version`, `zed:extension/*` WIT
  namespaces (`crates/extension_host`) — existing Zed extensions declare
  these identifiers.
- `zed-industries` git dependencies in `Cargo.toml` (tree-sitter grammars,
  reqwest fork `zed-reqwest`, etc.).
- `ZED_*` environment variables (`ZED_STATELESS`, `ZED_TERM`, task vars
  `ZED_FILE`/`ZED_COLUMN`, …) — load-bearing external interfaces.
- `api.zed.dev` / `cloud.zed.dev` endpoints via `build_dez_api_url` etc.
  (`crates/http_client`) — Flint runs no registry/LLM proxy of its own;
  dez inherits that proxying.

## Terminal host layer (commit: same)

| Divergence | Location | Why |
|---|---|---|
| `session_host` protocol module | `crates/terminal/src/session_host{,.rs}` | Stable host-owned terminal session protocol (ported from dez v0) |
| `TerminalHostId`, `TerminalSessionId` identity | `crates/terminal/src/session_host.rs` | Durable session identity across GUI restarts |
| `HostedTerminalController` trait | `crates/terminal/src/terminal.rs` | Command boundary for host-owned PTYs |
| `Event::ProcessInfoChanged`, `Event::ProcessExited` | `crates/terminal/src/terminal.rs` | Host metadata/exit tracking |
| `Terminal::session_id()`, `exit_code()`, `set_hosted_foreground_command()` | `crates/terminal/src/terminal.rs` | Accessors consumed by session_host |
| `dez_terminal_host` daemon | `crates/dez_terminal_host/` | Out-of-process PTY owner |
| `TerminalHostRuntime` wiring | `crates/dez/src/terminal_host_runtime.rs`, `main.rs` | Connects GUI to host daemon; host id derived from stable `installation_id` KVP |
| `polling`, `net`, `uuid`, `serde_json` deps added to `terminal` | `crates/terminal/Cargo.toml` | session_host requirements |

### Status: transport gated, not yet compiling

The protocol types (`TerminalSessionId`, snapshots, commands, events) are
always available, but the transport and host adapters (`in_process`, `local`,
`pty_process`, `transport`) are a partial port that still references a
`Terminal` process lifecycle this base does not have (`terminate_process`,
`write_hosted_replay`, `finish_hosted_replay`, `hosted_process_exited`,
`observed_exit_code`, the `TerminalType::Hosted` variant). They are therefore
behind the `hosted-terminal` Cargo feature, off by default, in `terminal`,
`dez`, and `dez_terminal_host`. The `dez-terminal-host` binary has
`required-features = ["hosted-terminal"]`. Finish the port against this base's
`Terminal` lifecycle, then enable the feature and wire `TerminalType::Hosted`.

## Extension security hardening (ported from Gram)

Adapted from Gram commits `ce82a212e` ("Close capability holes in extensions")
and `7a6951c46` ("Improved permissions for LSP and DAP from extensions") by
Kristoffer Grönlund. dez's base predates these; the port is adapted to the
older API surface.

| Divergence | Location | Why |
|---|---|---|
| Gate `fetch`, `fetch_stream`, `download_file`, `latest_github_release`, `github_release_by_tag_name` on manifest download capability | `crates/extension_host/src/wasm_host/wit/since_v0_1_0.rs`, `since_v0_8_0.rs` | Closes the hole where extensions could download arbitrary URLs without a declared `download_file` capability |
| Gate `make_file_executable` on manifest process-exec capability | same | Extensions must declare `process:exec` before making a downloaded file executable |
| `releases_url` / `tags_url` helpers | `crates/http_client/src/github.rs` | Lets the WIT layer check the exact URL a GitHub release request targets |
| `BinaryOptions` on `CapabilityGranter`, gating `grant_exec` on path lookup and `grant_download_file`/`grant_npm_install_package` on binary download | `crates/extension_host/src/capability_granter.rs` | Routes the user's LSP binary settings into every extension binary operation |
| Thread `LanguageServerBinaryOptions` into `Extension::language_server_command` | `crates/extension/src/extension.rs`, `crates/extension_host/src/wasm_host.rs`, `wit.rs`, `crates/language_extension/src/extension_lsp_adapter.rs` | LSP-provided language servers now respect `allow_path_lookup` / `allow_binary_download` |
| Reset granter binary options to permissive at the start of every extension call; set them from LSP settings for LSP calls | `crates/extension_host/src/wasm_host.rs`, `wit.rs` | Prevents restrictive LSP settings from leaking into unrelated extension calls |

### Adaptation notes (differences from Gram)

- Gram's `BinaryOptions` has an `enable_auto_updates` field sourced from
  `LanguageServerBinaryOptions::enable_auto_updates`; dez's base only has
  `pre_release`, so dez's `BinaryOptions` has two fields
  (`allow_path_lookup`, `allow_binary_download`). The auto-update dimension is
  not gated yet.
- Gram sources DAP options from a `DapSettings` type that dez's base does not
  have. dez therefore resets DAP calls to permissive defaults instead of
  gating them; LSP calls are gated. Adding `DapSettings` is a candidate for a
  future upstream sync.
- dez's default is permissive-with-per-call-reset rather than Gram's
  default-deny, because dez's base does not set options on every extension
  entry point. This preserves extension behavior while still gating LSP
  binary operations on user settings.

## Workspace shell (dez_sidebar)

| Divergence | Location | Why |
|---|---|---|
| New `dez_sidebar` crate implementing `workspace::Sidebar` | `crates/dez_sidebar/` | dez's flagship workspace navigator; Flint removed the sidebar implementation but kept the workspace-side trait |
| Browser-like view model (Home/Files/Git/Settings) | `crates/dez_sidebar/src/sidebar.rs` | dez's product identity: every Zed capability reachable as a sidebar surface |
| Multi-root group headers with joined names, active branch, and a changed-file count + status indicator | `crates/dez_sidebar/src/sidebar.rs` | Workspace structure is dez's differentiator; live git state belongs in the chrome |
| Labeled tab strip + per-view action rows that dispatch panel/settings actions | `crates/dez_sidebar/src/sidebar.rs` | Makes the sidebar a launcher for Zed features, not a static view |
| Git view surfaces the Git Graph (`git_ui::git_graph::Open`) beside Changes | `crates/dez_sidebar/src/sidebar.rs` | Gram's `git_graph` feature is the same lineage as Flint's `git_ui` graph; this makes it reachable from the chrome |
| Activity entry point dispatches `dez_actions::agent_threads::ToggleFocus` and mirrors the attention rollup | `crates/dez_sidebar/src/sidebar.rs` | Surfaces agent sessions and attention in the chrome without forking the state machine |
| `AgentThreadStore::attention_count` public API | `crates/agent_threads/src/store.rs` | Lets chrome outside the agent panel (the sidebar) surface attention; additive, ≤20 lines |
| `dez_sidebar` settings section (`starts_open`) | `crates/settings_content/src/dez_sidebar.rs`, `assets/settings/default.json` | Configurable default-open behavior |
| Sidebar registration + default-open on window creation | `crates/dez/src/dez.rs` | Wires the shell into the app; deferred open avoids acting mid-construction |

## Editor opinionation (special comments, commit refs)

Two small, high-visibility affordances dez adds on top of Flint's editor and
git surfaces.

| Divergence | Location | Why |
|---|---|---|
| `HighlightKey::SpecialComment` variant | `crates/editor/src/display_map.rs` | Single additive enum variant so dez can paint extra text highlights without touching the syntax/theme pipeline |
| New `dez_highlights` crate: scan comment chunks for `TODO`/`FIXME`/`HACK`/`XXX`/`NOTE`/`WIP`/`BUG`/`OPTIMIZE`/`REVIEW` and render them bold in the theme's `hint` color | `crates/dez_highlights/` | Opinionated attention markers; isolated in a `dez_*` crate and computed off-thread from the multibuffer's existing syntax chunks, so it works in editors, diff views, and multibuffers alike |
| `dez_highlights::init` wiring | `crates/dez/src/dez.rs` | Registers the per-editor addon at startup |
| `CommitDetails::ref_names` (`%D` decoration parsed from `git show`) | `crates/git/src/repository.rs`, `crates/proto/proto/git.proto`, `crates/project/src/git_store.rs` | The commit *view* previously showed no refs; the graph already had chips, so this extends the same `%D` data to the detail header over the existing remote proto |
| Ref chips in the commit header | `crates/git_ui/src/commit_view.rs` | Small UX win: see which branches/tags/remotes point at the commit you are reviewing, matching the git graph's chip language |

`parse_decorated_ref_names` is shared by the git graph's `%D` parser and the
commit view so both surfaces agree on how decorated refs are split.

## Considered and not adopted

| Candidate | Source | Why not |
|---|---|---|
| Separate `git_graph` crate and refs chips | Gram (`crates/git_graph`, commits `f7566fde3`, `96f3b0945`) | Flint already ships `crates/git_ui/src/git_graph.rs` (6,604 lines) with `ref_names`/chip support and file-history/search — a superset of Gram's 2,191-line crate (the older upstream graph). Porting Gram's would regress and duplicate. The feature is instead surfaced as a first-class sidebar action (see "Workspace shell") |
| AI/telemetry crate-graph removal | Gram (~108 crates) | Flint already removed Zed's hosted models, collab, accounts, and telemetry; dez inherits that and keeps only what runs the workspace |
| GPL-only additions / CLA rejection | Gram | dez inherits Flint's Apache/GPL dual license and contribution process; no license change |
| Full manifest-capability gating of every extension call | Gram `7a6951c46` | Requires a `DapSettings` type absent from dez's base; dez gates LSP calls and resets DAP to permissive. Revisit after the next upstream sync brings `DapSettings` |

## Deferred (Phase 1, per MERGE.md)

- `TerminalType::Hosted { controller }` variant in `Terminal` — full
  in-GUI hosted terminal rendering. Protocol layer is in place; the
  variant threads through ~14 match sites in old dez's terminal.rs and
  needs compiler feedback to port safely.
- `dez_workspace_shell` — the Chrome-like bar with draggable/splittable
  surfaces (see `docs/dez-workspace-shell.md`; the `dez_sidebar` navigator
  half has landed).
- Session discovery-and-attach for tmux/Herdr/cmux.
- Full DAP gating from Gram's `DapSettings` — LSP calls are already gated
  (see "Extension security hardening"); DAP calls reset to permissive until
  the next upstream sync brings `DapSettings`.

## Sync protocol

Upstream for dez is **Flint** (`flint-upstream` remote), synced every
2 weeks into `sync/upstream-YYYY-MM-DD` branches with CI green before
merge. Every cherry-picked upstream-Zed fix keeps its `zed#NNNNN`
reference. Ledger each adaptation in `SYNC-LEDGER.md`.
