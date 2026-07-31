---
title: Terminals - Dez
description: Run shells, developer tools, and agent CLIs in first-class terminal Surfaces while Dez supervises their lifecycle and attention.
---

# Terminals

A Terminal is a shell or terminal-native tool running in the **Main Work
Area**. It remains a normal native Zed Surface: it can sit beside
files, Workspace Search, diagnostics, and Agent Review.

**Workspaces** is a compact supervisory view of that same computation. It
adds a Session row after managed ownership or supported agent evidence exists,
then shows lifecycle, detected agent state, attention, and review evidence.
Selecting the row focuses or reattaches the owning Terminal; it does not open a
second terminal inside Workspaces or the Built-in Agent.

Use Terminals for shells, build tools, test runners, servers, and native
agent CLIs or TUIs. [External Agents](./external-agents.md) are different: they
integrate through ACP and render as Agent Sessions.

## What Dez Owns {#what-zed-owns}

Dez owns:

- the native Terminal Surface in the Main Work Area
- Workspace ownership and supported-agent supervision in Workspaces
- terminal title, Workspace placement, and lifecycle presentation
- packaged Host-owned Session identity, detach, reattach, and explicit termination
- bell and supported structured attention signals
- evidence-backed review links when observations are available

## What the CLI Owns {#what-the-cli-owns}

The CLI or TUI running inside the terminal owns its own:

- authentication
- model/provider configuration
- subscriptions or API keys
- tool configuration
- skills and instruction files
- MCP configuration

Agent profiles, Agent permissions, Dez Skills, and Agent MCP settings do not
automatically apply to a native CLI running in a Terminal.

## Opening a Terminal {#opening-a-terminal-thread}

Use **Open Terminal** in the active Workspace, or choose **Open Terminal** from
**Add to Main Work Area** in the native pane tab strip. The
terminal opens as Zed's existing `TerminalView` in the Main Work Area; Dez
does not wrap it in a separate terminal renderer. Workspaces observes that
Surface and adds the corresponding Workspace-owned Session row after managed
ownership or supported agent evidence exists.

Opening a Terminal is intentionally separate from creating a **New Agent
Session**. The Built-in Agent owns conversations; it is not a terminal
container.
You can open multiple terminals and move among them like other IDE Surfaces.

## Terminal Titles {#terminal-thread-titles}

The terminal title updates automatically to reflect the running shell or
process. You can set a custom name from its tab or Workspaces row. At normal
Workspaces widths, hover or keyboard-select the row and use its pencil. At narrow
widths, use the row's context menu or the selected-row rename action. Editing
the name does not freeze a running-agent or shell status prefix.

The Main Work Area tab and Workspaces row receive the full title. Each visual
surface truncates it only when its own available width requires it, so tooltips,
switching, and restored Sessions retain useful context.

## Attention {#terminal-thread-notifications}

When an unfocused terminal emits a bell, Dez can raise attention in Workspaces
and show a notification. Selecting the Session focuses its terminal
Surface. Acknowledging the notification changes presentation; it does not
pretend the underlying work condition has been resolved.

The same `agent.notify_when_agent_waiting` and `agent.play_sound_when_agent_done` settings apply.

## Closing and Ending {#closing-terminal-threads}

Terminals are not archived into Agent History. Hover over a supervised row to
reveal its state-specific lifecycle action, open its context menu, or
select it and press {#kb agent::ArchiveSelectedThread}.

The action names its actual effect:

- **Detach Live Terminal** closes an attached Surface while preserving the
  persistent computation.
- **End Terminal…** stops the
  shell and foreground process after a critical confirmation.
- exited, missing, incompatible, and saved records use **Close** or **Remove**
  rather than pretending a process can still be terminated.

## CLI/TUI Setup Notes {#cli-setup}

Some agent CLIs and TUIs can send terminal signals, such as bell notifications
or title updates, that Dez uses to show useful context in Workspaces after the
tool is recognized as supported agent work.

### Claude Code Notifications {#claude-code-notifications}

Claude Code can notify you when it finishes a task or pauses for permission. To enable this, set `preferredNotifChannel` to `"terminal_bell"` in your Claude Code user settings:

```json
{
  "preferredNotifChannel": "terminal_bell"
}
```

You can also set this from within Claude Code by running `/config`, selecting `Local Notifications`, and choosing `Terminal Bell`.

> If you run Claude Code inside tmux, bell notifications may not reach the outer terminal unless passthrough is enabled. Add this to `~/.tmux.conf`:
>
> ```
> set -g allow-passthrough on
> ```

For more, see the [Claude Code documentation](https://code.claude.com/docs/en/terminal-config).

### Amp Notifications {#amp-notifications}

Amp updates terminal titles automatically and can also notify you when it needs
your attention. To enable notifications in Dez Terminal Sessions, add
`AMP_FORCE_BEL=1` to your terminal environment settings:

```json [settings]
{
  "terminal": {
    "env": {
      "AMP_FORCE_BEL": "1"
    }
  }
}
```

Restart Amp after adding the environment variable.

### OpenCode Notifications {#opencode-notifications}

OpenCode can update terminal titles automatically. For Dez attention, add an
OpenCode plugin that emits a terminal bell when OpenCode needs your attention.

Create `.opencode/plugins/dez-bell.js` in your Workspace, or
`~/.config/opencode/plugins/dez-bell.js` to use it globally:

```js
export const DezBell = async () => {
  return {
    event: async ({ event }) => {
      if (process.env.OPENCODE_CLIENT === "acp") return;

      if (event.type === "session.idle" || event.type === "permission.asked") {
        process.stdout.write("\x07");
      }
    },
  };
};
```

Restart OpenCode after adding the plugin.

### Pi Notifications {#pi-notifications}

Pi can use an extension to emit a notification when it finishes a turn. Create
`.pi/extensions/dez-bell.ts` in your Workspace, or
`~/.pi/agent/extensions/dez-bell.ts` to use it globally:

```ts
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";

export default function (pi: ExtensionAPI) {
  pi.on("agent_end", async () => {
    process.stdout.write("\x07");
  });
}
```

Restart Pi after adding the extension, or run `/reload` if the extension is in one of Pi's auto-discovered extension locations.

### Codex Terminal Titles {#codex-terminal-titles}

Codex can update the terminal title as it works. Dez uses that title as useful
Workspaces context, such as the Workspace, current state, branch, model, or
task progress.

To configure this from within Codex, run `/title` and use the picker to choose which fields appear and in what order. Codex saves the selection to `tui.terminal_title` in `~/.codex/config.toml`. You can also edit it directly:

```toml
[tui]
terminal_title = ["spinner", "project-name", "run-state", "thread-title"]
```

## Credentials and Remote Workspaces {#credentials-and-remote-projects}

Credentials come from the terminal session and the CLI/TUI running inside it.

In remote Workspaces, the CLI may read the remote shell environment and remote
configuration files. In local Terminals, it reads the local shell environment
and local configuration files. Dez does not copy API keys from model-provider
settings into Terminals.

## When to Use Terminals {#when-to-use-terminal-threads}

Use Terminals when:

- you want the tool's native CLI/TUI experience
- no ACP integration exists
- you want subscription behavior owned by the CLI
- you want the CLI to use its own native config files

For ACP-integrated agents, see [External Agents](./external-agents.md).
