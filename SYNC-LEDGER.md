# SYNC-LEDGER.md

One line per upstream adaptation. Format:
`<date> | <source (flint vN / zed#NNNNN)> | <what> | <where>`

## Base

- 2026-09-20 | flint v0.11.3 (17b9230ce3) | Base import, rebranded to dez | whole tree
- 2026-09-20 | dez v0 | session_host protocol module + dez_terminal_host daemon + runtime wiring | crates/terminal/src/session_host*, crates/dez_terminal_host, crates/dez/src/terminal_host_runtime.rs
