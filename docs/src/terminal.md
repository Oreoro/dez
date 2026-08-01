---
title: Built-in Terminal - Dez
description: Dez's integrated terminal with native tabs, splits, agent supervision, durable Host ownership, custom launch commands, and Workspace navigation.
---

# Terminal

Dez treats terminals as first-class Surfaces beside files, search, diagnostics,
and review. A terminal opens in the Main Work Area as a normal tab or split.
Ordinary shells stay there. Workspaces only lists a terminal after Dez detects
a supported foreground agent or explicitly owns it as a managed agent
terminal.

The terminal workflow is Workspace-first: use **Open Workspace**, start a new
terminal tab or attach a discovered tmux or Herdr Session, supervise that work
in **Workspaces**, then return through **Open Files** or **Review Changes**.
The terminal remains an ordinary native tab throughout the flow.

Workspaces starts closed in a fresh Dez window and may restore open when that
window previously used it. When open, **Hide Workspaces** lives in the
Workspaces overview; when closed, the window chrome and status bar expose **Open
Workspaces**. Compact Workspaces uses icon-only utilities with named tooltips, while
the detailed width restores their visible labels. This keeps supervision
optional without hiding its recovery path.

Workspace headers stay useful at narrow widths. A multi-root Workspace shows
the first root followed by a root count instead of joining every root into one
clipped title. Search still matches every root, and the full root list remains
in the header tooltip and accessibility label. Agent provider and lifecycle
state use the row beneath a terminal or Session title, so status never competes
with the primary navigation label.

Workspace Tools and Built-in Agent also begin closed in a fresh default
Workspace, so the terminal or editor remains the obvious primary surface.
**Files**, **Outline**, **Git**, and **Debug** open the named native tab on
demand; repeating a destination focuses it instead of closing it.
Restored layouts and explicit `project_panel.starts_open` preferences remain
respected.

## Opening Terminals

On macOS, a copy running outside `/Applications` shows one inline **Install Dez
to continue** callout on Home with **Install and Relaunch**. Workspace restore
and durable-terminal startup wait behind that state. No startup dialog, prompt,
modal, or overlay is opened.

| Action                   | macOS              | Linux/Windows      |
| ------------------------ | ------------------ | ------------------ |
| Open configured terminal | `` Ctrl+` ``       | `` Ctrl+` ``       |
| Open shell terminal      | `` Ctrl+Shift+` `` | `` Ctrl+Shift+` `` |
| Command palette          | `Cmd+Shift+P`      | `Ctrl+Shift+P`     |
| Split terminal           | `Cmd+D`            | `Ctrl+Shift+5`     |

**Open Terminal** opens a normal terminal in the Main Work Area and uses the
guided **Default Terminal** choice. Native Shell keeps the shell prompt;
provider presets start their TUI after the configured shell is ready; tmux uses
the Workspace-named native attach/create path. Command Palette also exposes
**Open Native Shell**, **Launch Codex**, **Launch Claude Code**, and **Launch
OpenCode**, so switching providers does not require changing the default. These commands,
Workspaces, the tab-strip add control, and an empty Workspace all converge on the
same native terminal Surface.

Set the default under **Settings → Workspaces & Terminals → Terminal Launch →
Default Terminal**. Choose Native Shell, Codex, Claude Code, OpenCode, Gemini
CLI, Aider, Herdr, tmux Session, or Custom Command. Custom Command reveals a
raw command field for another terminal-native tool or wrapper, and existing
command-only configurations remain compatible. The dropdown shows the same
provider or tmux mark used by native launch surfaces, with a separate trailing
check for the current choice. The adjacent `+` after each native pane's tabs
offers that default, Native Shell, **Workspace tmux**, Codex,
Claude Code, and OpenCode as separate choices. Choosing a provider once does
not rewrite the default. In **File → Open Terminal**, the first row makes the
resolved choice explicit as **Default · Native Shell**, **Default · Codex**,
**Default · Claude Code**, **Default · OpenCode**, or **Default · Custom
Command**; the pane `+` retains **Default Terminal** for compactness.
The chooser, Home, the empty Main Work Area, pane `+`, and Workspaces show the
corresponding provider or tmux mark on that default action. Wrapped and
absolute commands retain known provider identity; Native Shell and unknown
custom commands keep the terminal mark.

Choose the route by ownership, not by appearance:

| Choice                                      | What Dez does                                                                                                     |
| ------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| **Default Terminal**                        | Opens the configured shell, agent TUI, tmux session, or custom command in a native terminal tab                   |
| **Native Shell**                            | Opens the Workspace shell without starting an agent                                                               |
| **Codex**, **Claude Code**, or **OpenCode** | Starts that CLI and keeps its real TUI in the native terminal                                                     |
| **Workspace tmux**                          | Attaches or creates the primary-root-scoped tmux session inside a native terminal                                 |
| **Browse Running Sessions…**                | Refreshes Workspaces so a discovered tmux or Herdr row can be attached, or a cmux Workspace can be opened in cmux |
| **Open Workspace in cmux**                  | Hands the Workspace path to the external cmux application and keeps Dez open                                      |

The tmux launcher combines a shell-safe Workspace label with a stable identity
for the primary root. Two repositories with the same folder name therefore get
different sessions. If an older basename-only session already exists, Dez
reattaches it only when tmux reports that its active pane is inside the current
Workspace root. Otherwise Dez creates or attaches the root-scoped session and
leaves the unrelated legacy process untouched.

**Built-in Agent** is a separate provider-backed conversation Surface. It is
not a terminal renderer and does not wrap a Codex, Claude Code, or OpenCode TUI.

Dez lists that terminal under the matching Workspace when it detects a
supported agent. The add control stays available when focus moves to Workspace
Tools or Built-in Agent; those auxiliary regions have their own hide controls
and never present a second terminal destination.

### One Terminal Model

Dez has no separate Terminal Panel destination. Every **Open Terminal**
action opens an ordinary Main Work Area terminal Surface in the active
Workspace. You can:

- keep it as a tab beside files;
- split it into the same pane grid;
- move it with other Surfaces;
- run a supported agent and select its Agent Session row to return to the
  existing Surface; or
- reattach a Host-owned terminal Session when the packaged Terminal Host owns
  it. Source or partial installations without the helper use the non-durable
  in-process fallback.

The packaged Host bounds connection, reconnection, and command cycles. It does
not replay a command whose outcome is uncertain, and it rejects work queued
behind a broken transport as stale. The GUI groups every frame-safe chunk for
one user-input batch into one queue item, so queue admission is all-or-nothing.
A batch above the helper's four-mebibyte PTY budget is rejected, and an aggregate
byte cap bounds input waiting in the GUI queue. A rejected admission is shown
inside the terminal Surface. After transport starts sending an accepted batch,
a later failure can still leave a prefix delivered; Dez treats that delivery as
uncertain, logs the transport failure, and does not replay it. Awaited
commands also have bounded enqueue and response deadlines instead of waiting
forever.

### Foreground agents stay in the terminal

Starting `codex`, `claude`, or another recognized terminal agent does not open
Built-in Agent or create a second terminal. The existing terminal remains in
the Main Work Area and is listed as a concise Agent Session row such as **Codex
· Running**. Selecting that row returns to the same terminal Surface. When an
ordinary detected agent exits back to its shell, the terminal stays open in
the Main Work Area and leaves Workspaces.

For Host-owned terminals, Dez observes the PTY foreground process group and
stores only its normalized command name in the bounded Session snapshot. This
is process evidence, not inference from terminal text. A configured provider
adapter can add structured progress, attention, commands, and checks; plain
process detection deliberately promises only who appears to be running.

For ordinary terminals, foreground inspection coalesces output bursts while
retaining one trailing refresh. Starting Codex while an earlier shell
inspection is still running therefore cannot leave Workspaces showing the shell
after the TUI becomes quiet.

### What Dez can and cannot observe

Dez observes terminals created inside Dez. It may inspect bounded process
metadata to detect supported agent work, but Workspaces does not list arbitrary
current-user TTYs owned by another application. Those terminals are neither
owned nor safely controllable by Dez and must not leak into a codebase's
navigation.

Dez does not capture that terminal's transcript or arguments, accept its input,
adopt its PTY, restore its process, or attribute its work. No Workspaces row is
created for an unrelated external TTY.

Dez adds a narrow, explicit control boundary for tmux, Herdr, and cmux.
Dez discovers live tmux sessions through `list-panes`, asks `herdr session list
--json` for the live default and named Herdr endpoints before querying the
documented snapshot API, and discovers cmux Workspaces through the documented
`list-workspaces --json` CLI. The namespaced `workspace list --json` form is a
bounded compatibility fallback, not the normal polling path. A session whose
working directory is inside an open root appears beneath the most specific
matching Workspace. Sessions with no working directory or no matching open root
remain visible under **Other Running Sessions**. **Browse Running Sessions…**
opens or refocuses Workspaces, clears temporary filters, expands matching
groups, and refreshes every source.
tmux and Herdr open the documented attach command in an ordinary Main Work Area
terminal. A cmux Workspace opens in cmux through `select-workspace`; Dez does
not manufacture an attachment terminal for it. The external application
remains authoritative, closing a Dez tab detaches rather than terminates, and
Dez never requests a Herdr takeover automatically.

Dez also correlates the documented `list-notifications --json` result with the
Workspace list. Unread notifications become **Needs Input**; the latest
notification or conversation summary and up to three listening ports remain
compact metadata in the native Workspace row. They are a read-only projection
of cmux state. Dez does not mark notifications read, send terminal input,
reproduce cmux panes, or infer ownership from notification text.

cmux protects its socket with an access mode. Its default may reject a CLI
launched outside a cmux-owned terminal. Dez treats that documented refusal as
**Access required**, shows the informational **cmux activity sharing is off**
notice, and keeps any prior rows as **last known**. **Open Workspace in cmux**
still works because path handoff does not require control-socket access. Dez
never weakens cmux's access mode automatically. The secure default is **cmux
processes only**; cross-app discovery requires the documented
`CMUX_SOCKET_MODE=allowAll` environment override when the user deliberately
accepts that local access boundary. Unexpected API failures remain **Failed**
and are logged once rather than on every five-second discovery cycle. The
current command and access contract is in the [cmux CLI/API
reference](https://cmux.com/docs/api).

Each integration reports its source truth independently: **Missing** means the
executable is unavailable, **Access required** means cmux is installed but is
not sharing live activity across its secure process boundary, **Empty** means an
available source returned no sessions, **Failed** means discovery did not
complete, and **Ready** means it returned sessions. Access-required and failure
states preserve only that source's rows as **last known**; successful sources
continue updating. A ready source may still have no session matching the
selected Workspace, in which case its unmatched items stay under
**Other Running Sessions** rather than disappearing.

If current cmux Workspace discovery succeeds but notification discovery fails,
Dez keeps the current Workspace rows, reports the cmux source as **Failed**, and
preserves missing rows as **last known**. A compatibility-only cmux release may
omit notification support without hiding the legacy Workspace metadata it did
return.

Herdr registry discovery and endpoint queries share one source-wide deadline;
endpoint snapshots run concurrently with an individual deadline for each
session. A large or unresponsive endpoint set therefore becomes **Failed**
instead of extending one refresh cycle indefinitely. A Retry requested while a
scan is active queues one immediate follow-up scan rather than disappearing.

For the current local codebase, **Workspace: Open in cmux** in Command Palette
hands the Workspace path to cmux and keeps the Dez window intact. It reports
success or the exact launch failure through a native toast and refreshes the
external Session projection afterward. Missing-cmux and failure notices remain
visible with one recovery action: **Get cmux** when it is absent, or **Open cmux
API guide** when the installed CLI cannot complete the handoff.

For cmux installed from its DMG or Homebrew cask, Dez checks the CLI bundled at
`/Applications/cmux.app/Contents/Resources/bin/cmux` as well as standard
Homebrew and shell PATH locations. A separate CLI symlink is therefore not
required for a standard `/Applications` install.

Reattaching a detached Session shows **Opening…** in its existing row
immediately. Repeated clicks do not create duplicate attachment work or another
terminal. When attachment completes, the row returns to the observed lifecycle
state and the restored terminal receives focus.

If the saved computation is no longer available, Dez keeps the current Main
Work Area intact and changes the existing row to **Missing**. It does not open
a placeholder terminal. Select the row again to retry after its Host returns,
or remove the dead reference from Workspaces.

### Moving from a Session into the IDE

Agent Sessions and terminal rows share navigation shortcuts, but their detail
destinations are deliberately different:

| Intent                                 | Workspaces shortcut      |
| -------------------------------------- | ------------------------ |
| Return to the existing Session         | `Enter` or `Shift+Enter` |
| Open its Workspace files               | `Shift+F`                |
| Review its changes                     | `Shift+G`                |
| Open an Agent Session's Review Brief   | `Shift+V`                |
| Open a terminal row's Terminal Details | `Shift+V`                |

An Agent Session **Review Brief** organizes observed intent, changes, commands,
checks, failures, and unresolved risk for review. A terminal row's **Terminal
Details** reports terminal and Host evidence such as lifecycle, ownership, and
working-directory context. Terminal Details does not imply that an ordinary
shell is an Agent Session or that Dez has a review result for it.

The terminal context bar exposes **Files**, **Review Changes**, and **Terminal
Details**. A detected or managed agent terminal projects its live state into
Workspaces; an ordinary shell remains only a terminal and never becomes a
synthetic Session. The Workspaces context menu retains the same handoff actions
when a compact row has no room for every control. Compatibility command
identifiers may still include the inherited Session Rail name, but visible Dez
chrome uses **Workspaces**.

After the welcome guide disappears, **Terminal Details** keeps a compact **How
Dez Works** explanation available: run work in the terminal, supervise detected
agent state and attention in Workspaces, then review the same Workspace through
Files and Git. This keeps orientation one click away without adding a permanent
help row.

Dez does not advertise a pathless Terminal before a Workspace is open.
If an existing scratch terminal is present, it does not appear in Workspaces
until a supported agent is detected. Its context strip shows **Open Workspace**
instead of pretending that Files or Git review are already available. Its
picker accepts folders only. Every selected folder is added to the same window,
so the running terminal remains intact and gains the normal Files and review
handoff.

These actions first activate the selected Session and its owning Workspace. If
that Workspace is closed, **Files** reopens it, restores the selected Session,
and only then reveals the existing project tree. It never fails silently or
creates a replacement Session. **Files** reveals the existing Workspace tree.
**Review Changes** opens the Agent change review for an Agent Session. For a
detected CLI-agent terminal, it reveals Git Changes and opens the first
uncommitted diff in the Main Work Area. In a multi-repository Workspace, it
keeps the active repository when that repository has changes; otherwise it
selects the first dirty repository deterministically.
A completely clean Workspace remains on Git's explicit clean state. The review
tab is named **Diff · filename**, with diff base and relative path retained in
its tooltip.
**Terminal Details** opens the evidence-backed terminal summary; an Agent
Session's **Review Brief** remains the separate run-review destination. None of
these actions starts another terminal or creates a second project context.

**Files** and **Review Changes** are idempotent destination actions. Repeating
them keeps the requested tool visible and focused instead of toggling it closed.
At narrow pane widths, the action labels yield to their icons before the
lifecycle/repository metadata or toolbar can clip. Every icon retains its full
accessible name and matching tooltip, and the full metadata remains available
in **Terminal Details**.

Inside Terminal Details, **Evidence** explains the boundary behind those facts:
lifecycle is observed from the Terminal and Host, Git counts belong to the
Workspace rather than automatically to this Session, and agent confidence or
checks require trusted evidence. Arbitrary terminal text remains display
content, not proof.

## Working with Multiple Terminals

Create additional terminals from the Main Work Area **+** menu or **Open
Terminal**. Each terminal is an independent Main Work Area tab and keeps the
active Workspace's directory context. Only detected or managed agent terminals
appear in Workspaces.

Split terminals horizontally with `Cmd+D` (macOS) or `Ctrl+Shift+5` (Linux/Windows).

### Naming Terminals

An ordinary shell terminal follows the title supplied by its shell. Dez retains
that full title for its tab and tooltip. When the terminal becomes a detected
or managed agent Session, the same identity is reused by Workspaces, Session
Switcher, and Host ownership metadata.

Double-click a terminal tab or use its **Rename Terminal…** context action.
Detected or managed agent terminals can also be renamed from Workspaces. A
normal-width Workspaces row exposes a pencil when the row is hovered or
keyboard-selected. Narrow rows retain rename in the context menu and the
selected-row rename action instead of crowding the lifecycle
controls. Leading and trailing whitespace is removed. Clearing the custom name
returns to the live terminal title. A running-agent status prefix continues to
update around the custom name. Task terminals retain their task label.

Hover a terminal tab for its current status and ownership. When available, the
tooltip distinguishes the **Working directory**, **Process ID**, and Host
**Session ID** instead of presenting those values as generic metadata.

## Close, Detach, and Terminate

These actions have deliberately different meanings:

- **Close Terminal Tab** closes an ordinary GUI-owned terminal Surface. If its
  shell is still running, Dez uses the normal dirty-item protection before
  discarding it.
- **Detach Terminal** closes a Host-owned terminal Surface without stopping
  its Host-owned process. Its Workspaces row remains available while the Host
  continues to report that process.
- **End Terminal…** is destructive. It is separated from close/detach in the
  terminal context menu and opens a critical confirmation explaining that the
  shell and any foreground command will stop. It is not offered for an exited
  or unavailable terminal.

Termination always goes through the selected terminal's own controller. The
presence of another local Host does not change which process the action owns.

## Unavailable Terminals

When Dez cannot reconnect a saved terminal, it preserves the original tab title
and shows one **Terminal unavailable** warning outside the terminal grid. The
warning names the concrete failure and confirms that Dez did not start a
replacement shell. The inactive grid contains no synthetic output or fake
cursor.

Choose **Start Fresh Terminal** only when you want separate computation in the
Main Work Area. It does not reconnect, replay, or replace the unavailable
Session.

## Configuring the Shell

By default, Dez uses your system's default shell (from `/etc/passwd` on Unix systems). To use a different shell:

```json [settings]
{
  "terminal": {
    "shell": {
      "program": "/bin/zsh"
    }
  }
}
```

To pass arguments to your shell:

```json [settings]
{
  "terminal": {
    "shell": {
      "with_arguments": {
        "program": "/bin/bash",
        "args": ["--login"]
      }
    }
  }
}
```

## Working Directory

Control where new terminals start:

| Value                                         | Behavior                                                                            |
| --------------------------------------------- | ----------------------------------------------------------------------------------- |
| `"current_file_directory"`                    | Uses the current file's directory, then its Workspace root, then the first root     |
| `"current_project_directory"`                 | Uses the current file's compatible project directory within the Workspace (default) |
| `"first_project_directory"`                   | Uses the first root in the Workspace                                                |
| `"always_home"`                               | Always starts in your home directory                                                |
| `{ "always": { "directory": "~/projects" } }` | Always starts in a specific directory                                               |

```json [settings]
{
  "terminal": {
    "working_directory": "first_project_directory"
  }
}
```

## Environment Variables

Add environment variables to all terminal sessions:

```json [settings]
{
  "terminal": {
    "env": {
      "EDITOR": "dez --wait",
      "MY_VAR": "value"
    }
  }
}
```

> **Tip:** Use `:` to separate multiple values in a single variable: `"PATH": "/custom/path:$PATH"`

### Python Virtual Environment Detection

Dez can automatically activate Python virtual environments when opening a terminal. By default, it searches for `.env`, `env`, `.venv`, and `venv` directories:

```json [settings]
{
  "terminal": {
    "detect_venv": {
      "on": {
        "directories": [".venv", "venv"],
        "activate_script": "default"
      }
    }
  }
}
```

The `activate_script` option supports `"default"`, `"csh"`, `"fish"`, and `"nushell"`.

To disable virtual environment detection:

```json [settings]
{
  "terminal": {
    "detect_venv": "off"
  }
}
```

## Fonts and Appearance

The terminal can use different fonts from the editor:

```json [settings]
{
  "terminal": {
    "font_family": "JetBrains Mono",
    "font_size": 14,
    "font_features": {
      "calt": false
    },
    "line_height": "comfortable"
  }
}
```

Line height options:

- `"comfortable"` — 1.618 ratio, good for reading (default)
- `"standard"` — 1.3 ratio, better for TUI applications with box-drawing characters
- `{ "custom": 1.5 }` — Custom ratio

### Cursor

Configure cursor appearance:

```json [settings]
{
  "terminal": {
    "cursor_shape": "bar",
    "blinking": "on"
  }
}
```

Cursor shapes: `"block"`, `"bar"`, `"underline"`, `"hollow"`

Blinking options: `"off"`, `"terminal_controlled"` (default), `"on"`

### Minimum Contrast

Dez adjusts terminal colors to maintain readability. The default value of `45` ensures text remains visible. Set to `0` to disable contrast adjustment and use exact theme colors:

```json [settings]
{
  "terminal": {
    "minimum_contrast": 0
  }
}
```

## Scrolling

Navigate terminal history with these keybindings:

| Action           | macOS                          | Linux/Windows    |
| ---------------- | ------------------------------ | ---------------- |
| Scroll page up   | `Shift+PageUp` or `Cmd+Up`     | `Shift+PageUp`   |
| Scroll page down | `Shift+PageDown` or `Cmd+Down` | `Shift+PageDown` |
| Scroll line up   | `Shift+Up`                     | `Shift+Up`       |
| Scroll line down | `Shift+Down`                   | `Shift+Down`     |
| Scroll to top    | `Shift+Home` or `Cmd+Home`     | `Shift+Home`     |
| Scroll to bottom | `Shift+End` or `Cmd+End`       | `Shift+End`      |

Adjust scroll speed with:

```json [settings]
{
  "terminal": {
    "scroll_multiplier": 3.0
  }
}
```

## Copy and Paste

| Action | macOS   | Linux/Windows  |
| ------ | ------- | -------------- |
| Copy   | `Cmd+C` | `Ctrl+Shift+C` |
| Paste  | `Cmd+V` | `Ctrl+Shift+V` |

### Copy on Select

Automatically copy selected text to the clipboard:

```json [settings]
{
  "terminal": {
    "copy_on_select": true
  }
}
```

### Keep Selection After Copy

By default, text stays selected after copying. To clear the selection:

```json [settings]
{
  "terminal": {
    "keep_selection_on_copy": false
  }
}
```

## Search

Search terminal content with `Cmd+F` (macOS) or `Ctrl+Shift+F` (Linux/Windows). This opens the same search bar used in the editor.

## Vi Mode

Toggle vi-style navigation in the terminal with `Ctrl+Shift+Space`. This allows you to navigate and select text using vi keybindings.

## Clear Terminal

Clear the terminal screen:

- macOS: `Cmd+K`
- Linux/Windows: `Ctrl+Shift+L`

## Option as Meta (macOS)

For Emacs users or applications that use Meta key combinations, enable Option as Meta:

```json [settings]
{
  "terminal": {
    "option_as_meta": true
  }
}
```

This reinterprets the Option key as Meta, allowing sequences like `Alt+X` to work correctly.

## Alternate Scroll Mode

When enabled, mouse scroll events are converted to arrow key presses in applications like `vim` or `less`:

```json [settings]
{
  "terminal": {
    "alternate_scroll": "on"
  }
}
```

## Path Hyperlinks

Zed detects file paths in terminal output and makes them clickable. `Cmd+Click` (macOS) or `Ctrl+Click` (Linux/Windows) opens the file in Zed, jumping to the line number if one is detected.

Common formats recognized:

- `src/main.rs:42` — Opens at line 42
- `src/main.rs:42:10` — Opens at line 42, column 10
- `File "script.py", line 10` — Python tracebacks

By default, `Cmd+Click`/`Ctrl+Click` opens links even when the running application has enabled mouse reporting (e.g. vim with `mouse=a`, htop). If you prefer those clicks to be forwarded to the application instead, disable `open_links_in_mouse_mode`; links can then still be opened with `Shift+Cmd+Click` (`Shift+Ctrl+Click`):

```json
{
  "terminal": {
    "open_links_in_mouse_mode": false
  }
}
```

## Compatibility Panel Settings

Dez opens terminals in the Main Work Area and exposes no separate terminal
panel destination. The inherited `terminal.dock`, `terminal.default_width`,
`terminal.default_height`, and `terminal.button` settings remain available for
official Zed compatibility, but Dez hides them from graphical Settings and
does not use them to place Main Work Area terminals. Use native tabs, splits,
and Workspace Layout commands to arrange terminals in Dez.

## Optional Terminal Breadcrumbs

Show the terminal title in a breadcrumb toolbar:

```json [settings]
{
  "terminal": {
    "toolbar": {
      "breadcrumbs": true
    }
  }
}
```

The title can be set by your shell using the escape sequence `\e]2;Title\007`.

## Integration with Tasks

The terminal integrates with Zed's [task system](./tasks.md). When you run a task, it executes in the terminal. Rerun the last task from a terminal with:

- macOS: `Cmd+Alt+R`
- Linux/Windows: `Ctrl+Shift+R` or `Alt+T`

## AI Assistance

Get help with terminal commands using the [Inline Assistant](./ai/inline-assistant.md):

- macOS: `Ctrl+Enter`
- Linux/Windows: `Ctrl+Enter` or `Ctrl+I`

This opens the Inline Assistant to help explain errors, suggest commands, or troubleshoot issues. AI agents in the [Agent Panel](./ai/agent-panel.md) can also run terminal commands as part of their workflow.

## Sending Text and Keystrokes

For advanced keybinding customization, you can send raw text or keystrokes to the terminal:

```json [keymap]
{
  "context": "Terminal",
  "bindings": {
    "alt-left": ["terminal::SendText", "\u001bb"],
    "ctrl-c": ["terminal::SendKeystroke", "ctrl-c"]
  }
}
```

## All Terminal Settings

For the complete list of terminal settings, see the [Terminal section in All Settings](./reference/all-settings.md#terminal).

## What's Next

- [Tasks](./tasks.md) — Run commands and scripts from Zed
- [REPL](./repl.md) — Interactive code execution
- [CLI Reference](./reference/cli.md) — Command-line interface for opening files in Zed
