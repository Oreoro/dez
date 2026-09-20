# SYNC-LEDGER.md

One line per upstream adaptation. Format:
`<date> | <source (flint vN / zed#NNNNN)> | <what> | <where>`

## Base

- 2026-09-20 | flint v0.11.3 (17b9230ce3) | Base import, rebranded to dez | whole tree
- 2026-09-20 | dez v0 | session_host protocol module + dez_terminal_host daemon + runtime wiring | crates/terminal/src/session_host*, crates/dez_terminal_host, crates/dez/src/terminal_host_runtime.rs
- 2026-09-20 | gram ce82a212e | Gate extension fetch/download/exec WIT entry points on manifest capabilities; add github releases_url/tags_url helpers | crates/extension_host/src/wasm_host/wit/{since_v0_1_0,since_v0_8_0}.rs, crates/http_client/src/github.rs
- 2026-09-20 | gram 7a6951c46 + 8fc62076c | CapabilityGranter BinaryOptions; thread LanguageServerBinaryOptions into extension LSP command; per-call permissive reset | crates/extension_host/src/{capability_granter.rs,wasm_host.rs,wasm_host/wit.rs}, crates/extension/src/extension.rs, crates/language_extension/src/extension_lsp_adapter.rs
- 2026-09-20 | dez v0 sidebar (concept) | New dez_sidebar crate: browser-like view model, multi-root headers, panel launching, attention notifications, settings, default-open | crates/dez_sidebar, crates/agent_threads/src/store.rs, crates/settings_content/src/dez_sidebar.rs, crates/dez/src/dez.rs
