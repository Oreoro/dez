# FORK.md — dez divergence ledger

Every intentional divergence of dez from Flint (which itself diverges from
Zed — see Flint's `docs/terminal-first-fork.md` for that layer). Each entry
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
| `polling`, `net`, `uuid` deps added to `terminal` | `crates/terminal/Cargo.toml` | session_host requirements |

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

## Deferred (Phase 1, per MERGE.md)

- `TerminalType::Hosted { controller }` variant in `Terminal` — full
  in-GUI hosted terminal rendering. Protocol layer is in place; the
  variant threads through ~14 match sites in old dez's terminal.rs and
  needs compiler feedback to port safely.
- `dez_sidebar` / `dez_workspace_shell` — the Chrome-like bar + workspace
  navigator (see `docs/dez-workspace-shell.md`).
- Session discovery-and-attach for tmux/Herdr/cmux.
- Gram-style extension LSP/DAP permission flags.

## Sync protocol

Upstream for dez is **Flint** (`flint-upstream` remote), synced every
2 weeks into `sync/upstream-YYYY-MM-DD` branches with CI green before
merge. Every cherry-picked upstream-Zed fix keeps its `zed#NNNNN`
reference. Ledger each adaptation in `SYNC-LEDGER.md`.
