---
title: Built-in Terminal - Dez
description: Dez's integrated terminal with main-area tabs, splits, durable sessions, custom shells, and editor integration.
---

# Terminal

Dez treats terminals as first-class Surfaces beside files, search, diagnostics,
and review. A terminal opens in the main work area as a normal tab or split;
the Session Rail only projects its live state and attention.

## Opening Terminals

| Action            | macOS         | Linux/Windows  |
| ----------------- | ------------- | -------------- |
| Open new terminal | `` Ctrl+` ``  | `` Ctrl+` ``   |
| Command palette   | `Cmd+Shift+P` | `Ctrl+Shift+P` |
| Split terminal    | `Cmd+D`       | `Ctrl+Shift+5` |

You can also choose **New Terminal** from the Session Rail, an empty Workspace,
the persistent **Add to Main Work Area** menu, or the command palette. The add
control stays available when focus moves to Workspace Tools or Agent; those
auxiliary regions have their own hide controls and never present a second
terminal destination.

### One Terminal Model

Dez has no separate Terminal Panel destination. Every ordinary **New Terminal**
action opens a main-area terminal Surface in the active Workspace. You can:

- keep it as a tab beside files;
- split it into the same pane grid;
- move it with other Surfaces;
- select its Session Rail row to return to the existing Surface; or
- reattach a Host-owned terminal Session when durable terminals are enabled.

## Working with Multiple Terminals

Create additional terminals from **New Terminal**. Each terminal is an
independent main-area tab and keeps the active Workspace's directory context.

Split terminals horizontally with `Cmd+D` (macOS) or `Ctrl+Shift+5` (Linux/Windows).

### Naming Terminals

An ordinary shell terminal follows the title supplied by its shell. Dez retains
that full title for the Session Rail, Session Switcher, durable Host metadata,
and tooltips; tabs and rows shorten it only when space requires.

Double-click a terminal tab or choose **Rename Terminal…** from its context menu
to set a stable custom name. Leading and trailing whitespace is removed.
Clearing the custom name returns to the live shell title. Task terminals retain
their task label.

## Close, Detach, and Terminate

These actions have deliberately different meanings:

- **Close Terminal Tab** closes an ordinary GUI-owned terminal Surface. If its
  shell is still running, Dez uses the normal dirty-item protection before
  discarding it.
- **Detach Terminal** closes a Host-owned terminal Surface without stopping
  its persistent computation. Its Session remains available from the Session
  Rail.
- **Terminate Terminal Session…** is destructive. It is separated from
  close/detach in the terminal context menu and opens a critical confirmation
  explaining that the shell and any foreground process will stop. It is not
  offered for an exited or unavailable terminal.

Termination always goes through the selected terminal's own controller. The
presence of another local Host does not change which process the action owns.

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

## Panel Configuration

### Dock Position

```json [settings]
{
  "terminal": {
    "dock": "bottom"
  }
}
```

Options: `"bottom"` (default), `"left"`, `"right"`

### Default Size

```json [settings]
{
  "terminal": {
    "default_width": 640,
    "default_height": 320
  }
}
```

### Terminal Button

Hide the terminal button in the status bar:

```json [settings]
{
  "terminal": {
    "button": false
  }
}
```

### Toolbar

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
