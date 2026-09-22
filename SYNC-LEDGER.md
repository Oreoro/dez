# SYNC-LEDGER.md

One line per upstream adaptation. Format:
`<date> | <source (flint vN / zed#NNNNN)> | <what> | <where>`

## Base

- 2026-09-20 | flint v0.11.3 (17b9230ce3) | Base import, rebranded to dez | whole tree
- 2026-09-20 | dez v0 | session_host protocol module + dez_terminal_host daemon + runtime wiring | crates/terminal/src/session_host*, crates/dez_terminal_host, crates/dez/src/terminal_host_runtime.rs
- 2026-09-20 | gram ce82a212e | Gate extension fetch/download/exec WIT entry points on manifest capabilities; add github releases_url/tags_url helpers | crates/extension_host/src/wasm_host/wit/{since_v0_1_0,since_v0_8_0}.rs, crates/http_client/src/github.rs
- 2026-09-20 | gram 7a6951c46 + 8fc62076c | CapabilityGranter BinaryOptions; thread LanguageServerBinaryOptions into extension LSP command; per-call permissive reset | crates/extension_host/src/{capability_granter.rs,wasm_host.rs,wasm_host/wit.rs}, crates/extension/src/extension.rs, crates/language_extension/src/extension_lsp_adapter.rs
- 2026-09-20 | dez v0 sidebar (concept) | New dez_sidebar crate: browser-like view model, multi-root headers, panel launching, attention notifications, settings, default-open | crates/dez_sidebar, crates/agent_threads/src/store.rs, crates/settings_content/src/dez_sidebar.rs, crates/dez/src/dez.rs
- 2026-09-21 | dez v0 sidebar (chrome) | Sidebar chrome pass: labeled tab strip, per-view action rows, workspace headers with changed-file count + status indicator, Activity entry point, Git Graph surfaced | crates/dez_sidebar/src/sidebar.rs
- 2026-09-21 | dez v0 terminal host (partial) | Gate the unfinished session_host transport/host adapters behind the `hosted-terminal` feature; keep protocol types always available; fix SettingsContent for the dez_sidebar field | crates/terminal/{Cargo.toml,src/session_host.rs,src/terminal.rs}, crates/dez/{Cargo.toml,src/main.rs}, crates/dez_terminal_host/{Cargo.toml,src/lib.rs}, crates/settings/src/vscode_import.rs
- 2026-09-21 | gram (special comments) | New `dez_highlights` crate highlights TODO/FIXME/HACK-style markers in comments; adds additive `HighlightKey::SpecialComment` | crates/dez_highlights, crates/editor/src/display_map.rs, crates/dez/src/dez.rs, Cargo.toml, crates/dez/Cargo.toml
- 2026-09-21 | gram (commit ref chips) | Extend `CommitDetails` with `%D` ref names and render branch/tag chips in the commit view header; shared `parse_decorated_ref_names` with the git graph | crates/git/src/repository.rs, crates/proto/proto/git.proto, crates/project/src/git_store.rs, crates/git_ui/src/commit_view.rs
