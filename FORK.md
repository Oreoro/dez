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
| New `dez_sidebar` crate implementing `workspace::Sidebar` | `crates/dez_sidebar/` | dez's flagship workspace rail; Flint removed the sidebar implementation but kept the workspace-side trait |
| The rail is dez's **only** sidebar: workspace groups plus their live agent sessions as one list, with no Files / Git / Settings tabs | `crates/dez_sidebar/src/sidebar.rs` | Session supervision belongs in the chrome rather than a second panel, and the tabs that only toggled docked panels beside the rail are gone -- those were what produced two competing left columns. Files / Git / Settings stay in Zed's own panels |
| Rail header carries a Needs-input rollup and the single action that starts a session | `crates/dez_sidebar/src/sidebar.rs` | The rail must be able to start a session, not only list one; dez commits to an opinionated default (Codex, the hero flow) rather than a menu of four, and the app menu still offers every kind |
| Multi-root group headers with joined names, active branch, and a changed-file count + status indicator | `crates/dez_sidebar/src/sidebar.rs` | Workspace structure is dez's differentiator; live git state belongs in the chrome |
| `AgentThreadStore::attention_count` public API | `crates/agent_threads/src/store.rs` | Lets chrome outside the agent panel (the sidebar) surface attention; additive, ≤20 lines |
| Live sessions rendered **inline in the rail**, nested under their owning workspace group, attention-first, with a rollup in the rail header | `crates/dez_sidebar/src/sidebar.rs` | One sidebar instead of three competing ones: the rail is now the supervision surface, so the user no longer has to open the Agent Threads panel to see what is running |
| `AgentThreadStore::sidebar_threads_for_workspace` + `SidebarThread` / `SidebarThreadStatus` | `crates/agent_threads/src/store.rs`, re-exported from `crates/agent_threads/src/agent_threads.rs` | The store already owns which workspace each thread belongs to, so the rail renders a *projection* rather than a second copy of the thread list. Scoping the call by workspace is what keeps sessions isolated per workspace; additive, no state-machine change |
| Rail's `Activity` panel-dispatch row replaced by a quiet `Agent thread history` row | `crates/dez_sidebar/src/sidebar.rs` | Live supervision moved into the rail; the panel keeps only its distinct capability (resuming/archiving historical sessions) instead of being the default way to see running ones |
| Plain shells listed in the rail beside agent threads, with the same status/ordering vocabulary | `crates/dez_sidebar/src/sidebar.rs` (shared row renderer), `crates/agent_threads/src/agent_threads.rs` | A supervision surface that lists only agents undercounts what is running and can't explain its own rollup; the Agent Threads panel's rollup already counts shells, so the rail must too |
| `SidebarTerminal`, `sidebar_terminals_for_workspace`, `observe_terminal_activity` | `crates/agent_threads/src/agent_threads.rs` | Workspace-scoped projection of the panel's existing terminal summaries, plus a repaint hook: terminal state is classified live from the screen tail, so chrome showing it has to observe terminal activity. `observe_terminal_activity` keeps the `cfg`-gated registry out of `dez_sidebar` |
| `RegularTerminalSummary::title`, `has_registry`, `observe_activity` | `crates/agent_threads/src/terminal_control.rs` | The terminal's display title (OSC title, or the foreground process the terminal itself derives when none was set) is what makes a shell row legible; the model hands over a never-empty title and the rail owns nothing but rendering. Additive, and the summary is no longer `Copy` (it now holds a `SharedString`) |
| `dez_sidebar` settings section (`starts_open`) | `crates/settings_content/src/dez_sidebar.rs`, `assets/settings/default.json` | Configurable default-open behavior |
| Sidebar registration + default-open on window creation | `crates/dez/src/dez.rs` | Wires the shell into the app; deferred open avoids acting mid-construction |
| Rail status is a marker (dot/glyph) plus a label only for `Needs input` / `Finished`; titles use the default text color | `crates/dez_sidebar/src/sidebar.rs` | A busy workspace should read as a list, not a traffic light; `Running`/`Idle` are the calm default |
| Rail search field filtering workspace names, session titles, agent kinds, and working directories, with in-place match highlighting | `crates/dez_sidebar/src/{filter.rs,sidebar.rs}` | A supervision surface with more than a handful of sessions is unsearchable without it. The matcher is a synchronous subsequence scan rather than the async `fuzzy` crate because the rail rebuilds every row on every render, where an async pass would race its own keystrokes; it widens v0.6's contiguous-substring match to a subsequence, preferring a continued run then a word start so the highlight reads as the word the user typed |
| `Sidebar::focus_search` + `MultiWorkspace::focus_sidebar_search`, bound to `cmd-f` / `ctrl-f` in the `Workspace` key context | `crates/workspace/src/multi_workspace.rs`, `assets/keymaps/default-*.json` | Reaching the rail's search should not require first clicking the rail, but it must not steal `cmd-f` from an open editor — hence a context-scoped binding and a route that opens a closed sidebar rather than a global one |
| Group header aggregates every root (branch from the first, summed changed count, worst indicator) and is clickable; single-root groups skip the per-root row | `crates/dez_sidebar/src/sidebar.rs` | Multi-root headers must describe the group, and a single-root header already names its root |
| `terminal_control::classify_record` maps an unmatched plain terminal to `Idle` instead of dropping it | `crates/agent_threads/src/terminal_control.rs` | A shell at an ordinary prompt matches no agent manifest, but the rail promises to list plain shells |
| Rail group headers carry a disclosure chevron and collapse their rows | `crates/dez_sidebar/src/sidebar.rs` | Collapsible groups are the standard sidebar affordance once a workspace holds several roots or sessions |
| `MultiWorkspace::toggle_project_group_expanded` public method | `crates/workspace/src/multi_workspace.rs` | The rail owns group disclosure, but the group state already lives (and is persisted) in `MultiWorkspace`; the rail calls through instead of keeping a second copy of "which groups are collapsed" |
| `AgentThreadStore::focus_thread` and `terminal_control::focus_terminal` activate the session's workspace before focusing its pane | `crates/agent_threads/src/{store.rs,terminal_control.rs}` | The rail lists sessions from every open workspace, so focusing one in a background workspace has to bring that workspace forward; this matches the panel, which already activates a cross-project target before focusing it |

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
