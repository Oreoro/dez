---
title: Terminal Sessions - Dez
description: Run shells, developer tools, and agent CLIs in first-class terminal Surfaces while Dez supervises their lifecycle and attention.
---

# Terminal Sessions

A Terminal Session is a shell or terminal-native tool running in the **Main Work
Area**. Its terminal Surface remains a normal part of the IDE: it can sit beside
files, Workspace Search, diagnostics, and Agent Review.

The **Session Rail** is a compact supervisory view of that same computation. It
shows the terminal's title, lifecycle, detected agent state, attention, and
review evidence. Selecting the row focuses or reattaches the owning terminal;
it does not open a second terminal inside the rail or Agent region.

Use Terminal Sessions for shells, build tools, test runners, servers, and native
agent CLIs or TUIs. [External Agents](./external-agents.md) are different: they
integrate through ACP and render as Agent Sessions.

## What Dez Owns {#what-zed-owns}

Dez owns:

- the terminal Surface in the Main Work Area
- grouping and supervision in the Session Rail
- terminal title, Workspace placement, and lifecycle presentation
- optional persistent Host identity, detach, reattach, and explicit termination
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
automatically apply to a native CLI running in a Terminal Session.

## Opening a Terminal Session {#opening-a-terminal-thread}

Use **New Terminal** in the active Workspace header, the Session Rail's start
state, or **Add to Main Work Area**. The terminal opens in the Main Work Area
and the Session Rail receives one corresponding row.

Opening a terminal is intentionally separate from creating a **New Agent
Session**. The Agent region owns conversations; it is not a terminal container.
You can open multiple terminals and move among them like other IDE Surfaces.

## Terminal Session Titles {#terminal-thread-titles}

The terminal title in the toolbar updates automatically to reflect the running shell or process. You can also set a custom name by clicking the title or the pencil icon that appears on hover.

The Main Work Area tab and Session Rail row receive the full title. Each visual
surface truncates it only when its own available width requires it, so tooltips,
switching, and restored Sessions retain useful context.

## Attention {#terminal-thread-notifications}

When an unfocused terminal emits a bell, Dez can raise attention in the Session
Rail and show a notification. Selecting the Session focuses its terminal
Surface. Acknowledging the notification changes presentation; it does not
pretend the underlying work condition has been resolved.

The same `agent.notify_when_agent_waiting` and `agent.play_sound_when_agent_done` settings apply.

## Closing and Terminating {#closing-terminal-threads}

Terminal Sessions are not archived into Agent History. Hover over a terminal
row to reveal its state-specific lifecycle action, open its context menu, or
select it and press {#kb agent::ArchiveSelectedThread}.

The action names its actual effect:

- **Detach Live Terminal** closes an attached Surface while preserving the
  persistent computation.
- **Terminate Running Terminal…** or **Terminate Detached Terminal…** stops the
  shell and foreground process after a critical confirmation.
- exited, missing, incompatible, and saved records use **Close** or **Remove**
  rather than pretending a process can still be terminated.

## CLI/TUI Setup Notes {#cli-setup}

Some agent CLIs and TUIs can send terminal signals, such as bell notifications
or title updates, that Dez uses to show useful context in the Session Rail.

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
Session Rail context, such as the Workspace, current state, branch, model, or
task progress.

To configure this from within Codex, run `/title` and use the picker to choose which fields appear and in what order. Codex saves the selection to `tui.terminal_title` in `~/.codex/config.toml`. You can also edit it directly:

```toml
[tui]
terminal_title = ["spinner", "project-name", "run-state", "thread-title"]
```

## Credentials and Remote Workspaces {#credentials-and-remote-projects}

Credentials come from the terminal session and the CLI/TUI running inside it.

In remote Workspaces, the CLI may read the remote shell environment and remote
configuration files. In local Terminal Sessions, it reads the local shell
environment and local configuration files. Dez does not copy API keys from
model-provider settings into Terminal Sessions.

## When to Use Terminal Sessions {#when-to-use-terminal-threads}

Use Terminal Sessions when:

- you want the tool's native CLI/TUI experience
- no ACP integration exists
- you want subscription behavior owned by the CLI
- you want the CLI to use its own native config files

For ACP-integrated agents, see [External Agents](./external-agents.md).
