# Native Surface Rules

## Contents

- Product identity
- Ownership
- Navigation
- Wireframe annotations
- Terminal integrations
- Recovery and permissions
- Responsive and accessible behavior
- Anti-patterns

## Product identity

Dez is a native Workspace-first development environment for supervising
terminal-native coding agents while editing and reviewing in the same app. It
is not a chat wrapper, terminal dashboard, custom browser shell, or tmux clone.

Use these public nouns consistently:

- **Workspace**: codebase identity and durable working context
- **Workspaces**: optional global navigator
- **Main Work Area**: authoritative native pane grid
- **Pane**: one split target
- **Tab**: native content item owned by a pane
- **Layout**: activation-only projection of pane/tab ownership
- **Activity**: bounded running or actionable signals
- **Terminal**: interactive shell or TUI surface
- **Terminal Details**: inline lifecycle and ownership disclosure
- **Workspace Tools**: contextual Files, Outline, Git, Debug, or Built-in Agent

## Ownership

| State | Authoritative owner | Projection may do |
| --- | --- | --- |
| Workspace identity and root | MultiWorkspace/Workspace | activate, disclose status |
| pane geometry and focus | native pane group | name and activate pane |
| tab order, close, pin, drag, preview | native pane | activate existing tab |
| shell/TUI bytes and process lifecycle | TerminalView/terminal host | summarize trusted lifecycle |
| agent run and permission state | authenticated agent/session store | show bounded Activity |
| Git, Files, Search, Diagnostics | owning native surface | deep-link to existing item |
| external tmux/Herdr/cmux process | external process | attach or hand off honestly |

Never create a second state store to make navigation easier. Derive projections
from the owning entity and subscribe to owner events.

## Activity projection

Activity is a live supervision projection, not history and not a second tab
list.

Include:

- the active Agent Session or Terminal;
- running, waiting, attention, failed, reconnecting, missing, incompatible, or
  resumable work;
- a completed Agent Session with authenticated review-ready changes; and
- current externally discovered tmux, Herdr, or cmux work, including a named
  uncertain state.

Exclude inactive completed Agent Sessions with no unread or review signal,
inactive idle or exited terminals, saved records without live state, and
completed external sessions. Keep completed Agent Sessions in Agent History.
Keep idle open tabs reachable through Layout and the native pane tab strip.

Search Workspaces and Activity against only this projection. Never load
historical transcript content merely to populate navigation.

## Navigation

- Keep native tab strips visible with an adjacent `+` in every real pane.
- Keep one visually and textually identified focused pane.
- Nest Layout under only the active expanded Workspace.
- Nest Activity under every expanded Workspace that has current projected
  rows; omit the heading when it would be empty.
- Hide Layout for one tab and while Workspace search replaces the list body.
- Let search preserve the Main Work Area without dimming or moving it.
- Reuse the status line for durable Workspace/repository context and named
  temporary modes. Do not stack another toolbar.
- Preserve a labeled recovery route when Workspaces is closed.

## Wireframe annotations

Create four minimum frames:

1. **Focused work** — active Workspace, panes, native tabs, Activity, status.
2. **Navigate** — temporary target hints without covering content.
3. **Return** — one bounded authoritative recap and review action.
4. **Recover** — inline failure with preserved content and safe next actions.

For each numbered callout record:

- **Owner**: the entity/surface that owns state.
- **Action**: what selection or button does.
- **Invariant**: what must not be duplicated or hidden.
- **Failure**: loading/denied/stale/failed behavior.

Use exact short labels in the ASCII companion. The raster artifact may use
annotation numbers and short captions, but it cannot be the sole copy source.

## Terminal integrations

- Render Codex, Claude Code, OpenCode, Gemini CLI, Aider, and other TUIs as
  their own terminal output. Add only native tab identity, trusted state, and
  recovery around them.
- Start root-scoped tmux in a native Terminal tab. Attach without claiming
  process ownership.
- Treat Herdr discovery and attach as external. Preserve output and expose Retry
  when the attach command fails.
- Open a Workspace in cmux through an explicit handoff. cmux owns its windows,
  tabs, splits, browser, hooks, and action registry.
- Never infer success, agent state, or file changes by scraping arbitrary PTY
  transcript text.

## Recovery and permissions

- Require stable installation before durable restoration or terminal-host
  startup.
- Preflight one exact Workspace root before Git, Search, LSP, agent, or terminal
  startup.
- Aggregate access failure once per root and provide one native folder-grant
  action.
- Keep recovery in its owner: installation on Home, root access in Workspaces,
  attach failure in Terminal, repository failure in Git.
- Preserve terminal output under failure UI.
- Never claim to migrate, adopt, or kill an externally owned process.

## Responsive and accessible behavior

- Preserve at least 60% of window width for the Main Work Area.
- Collapse nested tab rows before removing pane identity or actionable Activity.
- Truncate secondary metadata before titles or recovery actions.
- Keep every icon-only control labeled and keyboard reachable.
- Match focus order to visual hierarchy.
- Pair semantic color with text/icon state; do not rely on color alone.
- Use native density, type, icon, border, and material tokens.
- Avoid horizontal scrolling in navigation.

## Anti-patterns

Reject:

- Studio/Projects mode switches
- duplicate tab close buttons in Workspaces
- a terminal-only sidebar that forgets files and tools
- custom Chrome-like tab implementations
- floating onboarding, mascots, coach marks, or tours
- centered recovery cards over terminal output
- permanent inspectors unrelated to the active Workspace
- automatic unexplained splits
- raw transcript previews in Activity
- hidden status or permission modes
