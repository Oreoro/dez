# Codex Terminal Adapter

The first Dez terminal-agent adapter uses Codex lifecycle hooks. It does not
scrape terminal output or reinterpret a transcript. Codex remains an ordinary
process in an ordinary terminal; structured state is attached to the terminal
Host/Session record and projected into the Session Rail and review brief.

This source slice is opt-in with the durable terminal host:

```sh
DEZ_EXPERIMENTAL_TERMINAL_HOST=1 dez
```

## Enable the hooks {#enable-hooks}

Codex hooks require explicit review and trust. Dez does not modify global or
project Codex configuration automatically. Add the following groups to an
existing `hooks.json` at the user or trusted-project layer, then review them
with `/hooks` in Codex:

For a detected Codex terminal owned by the durable helper, the same JSON is
available from **Copy Codex Hook Setup** in both the Session Rail row's hover
controls and context menu. Eligible rows say **Hook setup** until structured
state arrives; both affordances then disappear.

```json
{
  "description": "Project structured Codex lifecycle into Dez terminals.",
  "hooks": {
    "SessionStart": [{ "hooks": [{ "type": "command", "command": "if [ -n \"${DEZ_TERMINAL_HOST_BIN:-}\" ]; then \"$DEZ_TERMINAL_HOST_BIN\" agent-event; fi", "timeout": 5 }] }],
    "UserPromptSubmit": [{ "hooks": [{ "type": "command", "command": "if [ -n \"${DEZ_TERMINAL_HOST_BIN:-}\" ]; then \"$DEZ_TERMINAL_HOST_BIN\" agent-event; fi", "timeout": 5 }] }],
    "PermissionRequest": [{ "hooks": [{ "type": "command", "command": "if [ -n \"${DEZ_TERMINAL_HOST_BIN:-}\" ]; then \"$DEZ_TERMINAL_HOST_BIN\" agent-event; fi", "timeout": 5 }] }],
    "PreToolUse": [{ "hooks": [{ "type": "command", "command": "if [ -n \"${DEZ_TERMINAL_HOST_BIN:-}\" ]; then \"$DEZ_TERMINAL_HOST_BIN\" agent-event; fi", "timeout": 5 }] }],
    "PostToolUse": [{ "hooks": [{ "type": "command", "command": "if [ -n \"${DEZ_TERMINAL_HOST_BIN:-}\" ]; then \"$DEZ_TERMINAL_HOST_BIN\" agent-event; fi", "timeout": 5 }] }],
    "Stop": [{ "hooks": [{ "type": "command", "command": "if [ -n \"${DEZ_TERMINAL_HOST_BIN:-}\" ]; then \"$DEZ_TERMINAL_HOST_BIN\" agent-event; fi", "timeout": 5 }] }]
  }
}
```

The environment guard makes the hook a no-op outside a hosted Dez terminal.
Start a new terminal after enabling the durable host so Codex inherits the
authenticated socket path and stable Host/Session identity. The token itself
is never placed in the terminal environment.

## Truth and retention {#truth-retention}

The adapter maps only supported structured events:

| Codex event         | Dez projection                                      |
| ------------------- | --------------------------------------------------- |
| `SessionStart`      | Starting, resumable Codex actor                      |
| `UserPromptSubmit`  | Running; prior attention acknowledged by new intent  |
| `PermissionRequest` | Waiting for permission; attention required           |
| `PreToolUse`        | Running; bounded tool-start evidence                  |
| `PostToolUse`       | Running; bounded command and exit evidence if present |
| `Stop`              | Turn completed; ready-for-review attention            |

Process-name detection remains a lower-confidence fallback and appears as
**Detected**. It never invents permission, completion, failure, or reasoning
state. Structured rows omit that qualifier and show the adapter state.

The helper retains at most 32 structured events per terminal session. Each
text field is capped, transcript contents are never read, and the event feed is
authenticated by the same private socket/token boundary as terminal control.
Opening the owning terminal acknowledges attention without deleting the event
evidence. Host exit or explicit termination marks the adapter Exited.

## Current limits {#current-limits}

- Hook installation is manual and Unix-shell-oriented in v0.0.1.
- The adapter records observed shell command/exit evidence when Codex supplies
  it. Known validation commands with observed exits become check evidence;
  unknown commands and missing outcomes stay unclassified. Modified-file
  evidence is not yet available from this hook slice.
- Agent state survives a GUI restart only when the opt-in helper and terminal
  session survive. It is intentionally not copied into a second Run database.
- The consolidated build and live restart demonstration remain pending.
