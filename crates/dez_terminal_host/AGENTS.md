# Dez terminal host

Purpose: own local terminal computation independently from Dez GUI view and
process lifetime. The first user is a terminal-native developer supervising
long-running coding agents.

First milestone: a minimal local helper accepts one authenticated client over
a user-private socket, negotiates the versioned terminal Host/Session protocol,
and serves lifecycle commands through bounded frames. PTY transfer and GUI
launch wiring follow only after this transport is source-tested.

Use the workspace Rust toolchain and dependency catalog. Format with
`cargo fmt --all`; lint with `./script/clippy -p dez_terminal_host`; test with
`cargo test -p dez_terminal_host`. The active work agreement defers compilation
and binary execution to the consolidated gate.

Never print or persist authentication tokens in normal metadata, logs, or
workspace snapshots. Bind sockets in a user-private runtime directory, reject
oversized frames before allocation, fail closed on host or protocol mismatch,
and keep detach distinct from terminate. Do not support remote terminals in
this local helper.

Done for the first milestone means framing and authentication have focused
tests, malformed input is bounded, lifecycle commands preserve truthful state,
the source formats cleanly, and no existing application binary is launched.
Keep changes additive and rollbackable by removing this crate and its workspace
member entry before GUI integration exists.
