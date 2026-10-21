# Codex Terminal Adapter

The first Dez terminal-agent adapter uses Codex lifecycle hooks. It does not
scrape terminal output or reinterpret a transcript. Codex remains an ordinary
process in an ordinary terminal. Structured state is attached to the terminal
Host/Session record and projected into the owning Workspace terminal row and
Terminal Details.

Packaged Dez uses the sibling `dez-terminal-host` by default. On macOS, install
Dez in `/Applications` before Workspace restoration or durable terminal startup
so the app and Host keep one stable code identity. A source or partial install
without the sibling helper uses the GUI-owned local terminal path, which ends
with the GUI and is not durable. `DEZ_EXPERIMENTAL_TERMINAL_HOST` remains a
diagnostic override; it is not the normal packaged enablement path.

## Enable the hooks {#enable-hooks}

Codex hooks require explicit review and trust. Dez never modifies user-level or
Workspace-scoped Codex configuration automatically. Add the following groups
to an existing `hooks.json` that applies to the intended Workspace, then review
them with `/hooks` in Codex:

For a detected Codex terminal verified against the current Host, the same JSON
is available from **Copy Codex Hook Setup** in the terminal row's context menu.
Eligible rows say **Hook setup** until structured state arrives; the setup
action then disappears.

```json
{
  "description": "Report structured Codex lifecycle events to the owning Dez terminal.",
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "if [ -n \"${DEZ_TERMINAL_HOST_BIN:-}\" ]; then \"$DEZ_TERMINAL_HOST_BIN\" agent-event; fi",
            "timeout": 5
          }
        ]
      }
    ],
    "UserPromptSubmit": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "if [ -n \"${DEZ_TERMINAL_HOST_BIN:-}\" ]; then \"$DEZ_TERMINAL_HOST_BIN\" agent-event; fi",
            "timeout": 5
          }
        ]
      }
    ],
    "PermissionRequest": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "if [ -n \"${DEZ_TERMINAL_HOST_BIN:-}\" ]; then \"$DEZ_TERMINAL_HOST_BIN\" agent-event; fi",
            "timeout": 5
          }
        ]
      }
    ],
    "PreToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "if [ -n \"${DEZ_TERMINAL_HOST_BIN:-}\" ]; then \"$DEZ_TERMINAL_HOST_BIN\" agent-event; fi",
            "timeout": 5
          }
        ]
      }
    ],
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "if [ -n \"${DEZ_TERMINAL_HOST_BIN:-}\" ]; then \"$DEZ_TERMINAL_HOST_BIN\" agent-event; fi",
            "timeout": 5
          }
        ]
      }
    ],
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "if [ -n \"${DEZ_TERMINAL_HOST_BIN:-}\" ]; then \"$DEZ_TERMINAL_HOST_BIN\" agent-event; fi",
            "timeout": 5
          }
        ]
      }
    ]
  }
}
```

The environment guard makes the hook a no-op outside a Host-owned Dez terminal.
Start a new terminal after adding the hook configuration so Codex inherits the
authenticated endpoint and stable Host/Session identity. The token itself is
never placed in the terminal environment.

## Truth and retention {#truth-retention}

The adapter maps only supported structured events:

| Codex event         | Dez projection                                        |
| ------------------- | ----------------------------------------------------- |
| `SessionStart`      | Starting, resumable Codex actor                       |
| `UserPromptSubmit`  | Running; prior attention acknowledged by new intent   |
| `PermissionRequest` | Waiting for permission; attention required            |
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

- Hook installation is manual and Unix-shell-oriented in the current source
  candidate.
- The adapter records observed shell command/exit evidence when Codex supplies
  it. Known validation commands with observed exits become check evidence;
  unknown commands and missing outcomes stay unclassified. Modified-file
  evidence is not yet available from this hook slice.
- Structured state survives a GUI restart only while the same packaged Host
  generation and terminal Session remain alive. The source/partial-install local
  owner ends with the GUI. Adapter state is intentionally not copied into a
  second lifecycle database.
- Exact-package hook, restart, and recovery demonstrations remain pending.
