# Dez surface contract

This document is the source-controlled wireframe and interaction contract for
Dez's native shell. It complements the visual review sheet with exact labels,
ownership, state, and responsive behavior. When a rendered sketch and this
document disagree, this document wins.

## Product loop

```text
Open or resume a Workspace
-> run a shell, terminal agent, task, or multiplexer in a native tab
-> supervise owned activity and attention in Workspaces
-> inspect files, diagnostics, and Git in native tabs
-> Review Changes in the same Main Work Area
```

The primary user is a developer or vibe coder coordinating several terminal
tools without giving up a complete editor. Dez is therefore a Workspace-first
development environment, not a terminal dashboard and not a chat client.

## Ownership model

```text
Window
├── Workspaces (optional global navigator)
│   ├── Workspace rows
│   ├── Sessions (live activity only)
│   ├── Open Tabs & Tools (projection of the active Workspace)
│   └── Workspace notices (bounded recovery states)
├── Main Work Area (authoritative native pane and tab model)
│   ├── Home, files, diffs, search, Settings, browser, diagnostics
│   ├── TerminalView surfaces for shells, agents, tmux, and Herdr
│   ├── Files, Outline, Git, Debug, and Built-in Agent tools
│   └── user-created panes and splits
└── Status bar (durable context and navigation)
```

One Workspace owns every tab, tool, Session, and user-created pane associated
with its codebase. A Workspace is not a list of terminals. **Open Tabs &
Tools** reads the native pane model and activates its existing items; it never
stores duplicate order, focus, close, pin, dirty, or split state.

## Final wireframe

### 1. Home and launch

```text
┌ Workspaces ──────┬ Home ─ + ─────────────────────────────────────────┐
│ dez  main        │ Continue your work                                │
│ 3 agents · 2 ports│ Run a tool, supervise its work, then review it.   │
│                  │                                                    │
│ Open Tabs & Tools│ Start with a tool          Recent Workspaces      │
│  Home            │  Open Terminal · Default   superzed               │
│  Codex · Working │  Codex                     website                │
│  app.rs          │  Claude Code               infra                  │
│  Terminal · Ready│  OpenCode                  tools                  │
│  Files           │  Workspace tmux                                   │
│  Git Changes     │  Open Workspace in cmux                           │
│                  │                                                    │
│ superzed         │ Inspect and resume                                │
│ website          │  Browse Running Sessions                          │
│                  │  Open Files · Review Changes                      │
├──────────────────┴────────────────────────────────────────────────────┤
│ Workspace: dez | main | 3 agents | 2 ports | Permissions: Healthy    │
└───────────────────────────────────────────────────────────────────────┘
```

Home is a launcher in a normal closeable tab. It has no hero illustration,
provider promotion, setup wizard, or overlay. The adjacent `+` remains visible.

### 2. Run and active work

```text
┌ Workspaces ──────┬ Codex · Working | app.rs | Terminal | Files | Git | + ┐
│ dez  main        │ Workspace: dez · main · ~/code/dez · Codex working     │
│ Sessions         ├─────────────────────────────────────────────────────────┤
│  Codex · Working │                                                         │
│  Claude · Attention  Native TerminalView renders the provider TUI here.   │
│  Terminal · Ready│  Dez does not place a custom chat renderer around it.  │
│ Open Tabs & Tools│                                                         │
│  Codex · Working │                                                         │
│  app.rs          │                                                         │
│  Terminal        │                                                         │
│  Files · Git     │                                                         │
├──────────────────┴─────────────────────────────────────────────────────────┤
│ Workspace: dez | main | 3 agents | 2 ports | Permissions: Healthy          │
└─────────────────────────────────────────────────────────────────────────────┘
```

Provider and subagent glyphs identify activity; adjacent state text says
Working, Needs attention, Completed, Failed, or Available. Color is supportive,
never the only state signal.

### 3. Supervise and recover

```text
┌ Workspaces ─────────────┬ Terminal · Herdr attach · Failed ─ + ──────┐
│ dez  main               │ Herdr attach failed · Connection refused    │
│ Workspace access required│ [Retry Attach] [Open new shell here]       │
│ [Grant Access…]         ├──────────────────────────────────────────────┤
│ Sessions                │ Existing terminal output remains visible.    │
│  Codex · Working        │                                              │
│  Claude · Attention     │ Terminal Details                             │
│  Terminal · Ready       │ Provider · Herdr                             │
│  tmux · Attach failed   │ Working directory · ~/code/dez               │
│  Legacy · Access blocked│ Host generation · legacy                     │
│ Open Tabs & Tools       │ Endpoint · offline                           │
│  Terminal · Failed      │ Ownership · external process unchanged       │
│  Files · Diff · Git     │                                              │
├─────────────────────────┴──────────────────────────────────────────────┤
│ Workspace: dez | main | 3 agents | 2 ports | Access required          │
└────────────────────────────────────────────────────────────────────────┘
```

**Browse Running Sessions…** focuses Workspaces, clears transient filters,
expands groups with activity, refreshes discovery, and preserves the active
Main Work Area tab. It never creates a duplicate Sessions page. Recovery is
inline and states both cause and next action. Terminal Details is a disclosure
within the terminal surface, not a floating inspector. Destructive legacy
termination remains behind a native confirmation.

### 4. Review and intentional split

```text
┌ Workspaces ──────┬ Diff · app.rs | app.rs | + ┬ Terminal | Files | Git | + ┐
│ dez  main        │                              │                             │
│ Open Tabs & Tools│ diff                         │ native terminal              │
│ Pane 1 · Focused │                              │ or review-adjacent tool      │
│  Diff · app.rs   │                              │                             │
│  app.rs          │                              │                             │
│ Pane 2           │                              │                             │
│  Terminal        │                              │                             │
│  Files           │                              │                             │
│  Git Changes     │                              │                             │
├──────────────────┴──────────────────────────────┴─────────────────────────────┤
│ Workspace: dez | main | 3 agents | 2 ports | Permissions: Healthy            │
└────────────────────────────────────────────────────────────────────────────────┘
```

A pane group exists only after the user invokes a split action. Each pane owns
its native tab strip and adjacent `+`. Closing Workspaces preserves both panes,
their tabs, and the focused Main Work Area item.

## Secondary-state wireframe

### 5. Install-first Home

```text
┌ Workspaces ─────────┬ Home ─ + ──────────────────────────────────────┐
│                    │ Continue your work                              │
│ No Workspaces yet  │                                                 │
│ Install Dez to get │ ┌ Install Dez to continue ────────────────────┐ │
│ started.           │ │ Install in Applications and relaunch before│ │
│                    │ │ restoring Workspaces or starting durable   │ │
│                    │ │ terminals.                                 │ │
│                    │ │                         [Install and Relaunch]│ │
│                    │ └─────────────────────────────────────────────┘ │
├────────────────────┴─────────────────────────────────────────────────┤
│ Installation required · Install Dez in Applications to continue     │
└──────────────────────────────────────────────────────────────────────┘
```

Home keeps its normal identity while installation is gated. No Workspace,
branch, agent, port, permission, recent-history, or durable-host evidence is
rendered because none has restored. Home owns the only primary recovery action;
Workspaces supplies a quiet explanation rather than a duplicate button.

### 6. Workspace access recovery

```text
┌ Workspaces ─────────────┬ Codex · Working ─ + ───────────────────────┐
│ dez  main               │ Existing Main Work Area content            │
│ ┌ Workspace access ───┐ │ remains selected, visible, and unchanged.  │
│ │ required            │ │                                            │
│ │ “zed 3.0” needs     │ │                                            │
│ │ access before Git,  │ │                                            │
│ │ search, agents, or  │ │                                            │
│ │ terminals start.    │ │                                            │
│ │ [Grant Access…]     │ │                                            │
│ └─────────────────────┘ │                                            │
│ Open Tabs & Tools       │                                            │
├─────────────────────────┴────────────────────────────────────────────┤
│ Workspace: dez | main | Permissions: Access required                │
└──────────────────────────────────────────────────────────────────────┘
```

Permission recovery belongs to the affected Workspace root. One aggregated
notice names the exact folder and gates Git, search, LSP, agents, and terminals
behind the same preflight. Selecting **Grant Access…** never opens or replaces
a Workspace, and Home does not mirror the notice over the Main Work Area.

### 7. Product-first Settings

```text
┌ Workspaces ─────┬ Settings ─ + ──────────────────────────────────────┐
│ dez             │ Workspaces & Terminals │ Workspaces                │
│ Open Tabs &     │ Agents                 │ Workspaces Position  Left │
│ Tools           │ Appearance             │ Show on Startup       Off │
│  Settings       │ Workspace & Privacy    │───────────────────────────│
│  Terminal       │ Keyboard & Vim         │ Terminal Launch           │
│  Files          │ Editor                 │ Default Terminal           │
│                 │ Languages & Tools      │  Native Shell              │
│                 │ Search & Files         │  Codex · Claude Code       │
│                 │ Navigation & Layout    │  OpenCode · tmux · Custom  │
│                 │ Workspace Tools        │ Open Workspace in cmux  ↗  │
├─────────────────┴────────────────────────┴────────────────────────────┤
│ Workspace: dez | main | Permissions: Healthy                        │
└──────────────────────────────────────────────────────────────────────┘
```

Settings begins with the choices required to run work: Workspace behavior and
Terminal Launch, then Agents and Appearance. Privacy remains prominent before
inherited editor customization. Native shell and TUI profiles launch inside
Dez; **Open Workspace in cmux** is an explicit external handoff and therefore
an action or documentation route, never an availability toggle.

### 8. Compact navigation

```text
┌ app.rs | Terminal | Files | Git Changes | + ─────────────────────────┐
│ Main Work Area preserves every tab, pane, split, and focused item.   │
│ Compact toolbar actions retain icons, tooltips, accessible names,    │
│ keyboard targets, and native pane ownership.                         │
├──────────────────────────────────────────────────────────────────────┤
│ Workspaces | dez | main | 3 agents | 2 ports | Permissions: Healthy │
└──────────────────────────────────────────────────────────────────────┘
```

Closing Workspaces gives its width back to the Main Work Area. The labeled
status-bar control remains the recovery route and uses **Open Workspaces** for
its tooltip and action. Compact layouts may hide secondary text, never the
control's meaning or accessible identity.

## Operational-state wireframe

### 9. Empty Main Work Area

```text
┌ Workspaces ───────┬ + ──────────────────────────────────────────────┐
│ dez · main        │ Main Work Area                                  │
│                   │ Start a terminal or resume running work.         │
│ No sessions yet   │ Files and Git review open as tabs here.          │
│                   │                                                  │
│ superzed          │ [Open Terminal · Codex]                          │
│ website           │ [Browse Sessions] [Find File] [Review Changes]   │
├───────────────────┴──────────────────────────────────────────────────┤
│ Workspace: dez | main | Permissions: Healthy                       │
└──────────────────────────────────────────────────────────────────────┘
```

This is an operational start state for an open Workspace, not another Home.
The tab strip remains visible but contains no invented placeholder tab. Its
adjacent `+` is available immediately. The one primary action names the
resolved configured destination—Native Shell, tmux Session, a recognized
provider, or Custom Command—while the provider glyph reinforces that identity.

### 10. Add and switch tabs

```text
┌ Codex · Working | app.rs* | Diff · config.rs | Terminal | + | Switch Tab ┐
│ Add to Main Work Area             Tabs in This Pane                      │
│  Default Terminal                 Codex · Working · Active               │
│  Native Shell                     app.rs · Modified                       │
│  Workspace tmux                   Diff · config.rs                        │
│  Codex · Claude Code · OpenCode   Terminal · Pinned                       │
│  More Agent CLIs ›                Settings                                │
│  Continue Agent ›                                                         │
│  Browse Running Sessions…                                                  │
│  Open Workspace in cmux                                                   │
│  Files · Review · Task · Debug                                             │
│  Home · Recent Workspaces                                                  │
└─────────────────────────────────────────────────────────────────────────────┘
```

The two menus are alternate anchored native menus and never appear together in
the product. `+` follows the final visible tab and remains pinned to the tab
viewport edge during overflow. **Switch Tab** lists only the owning pane and
preserves active, modified, and pinned meaning in text as well as icon state.

### 11. Discover and supervise

```text
┌ Workspaces ─────────────────┬ app.rs ─ + ────────────────────────────┐
│ Search · All · Attention    │ Existing editor remains active.        │
│ dez · main                  │                                        │
│ Sessions                    │ Browse Running Sessions never opens    │
│  Codex · Working            │ or replaces this Main Work Area tab.   │
│  Claude Code · Attention    │                                        │
│  tmux · Available           │                                        │
│ Herdr unavailable           │                                        │
│ Discovery timed out.        │                                        │
│ Last-known rows stay visible│                                        │
│ [Retry]                     │                                        │
│ Other Running Sessions      │                                        │
│  cmux · Available           │                                        │
├─────────────────────────────┴────────────────────────────────────────┤
│ Workspace: dez | main | 3 agents | 2 ports | Permissions: Healthy   │
└──────────────────────────────────────────────────────────────────────┘
```

Each external source owns one Missing, Empty, Failed, or Ready state. One
bounded failed-source notice exposes one non-destructive **Retry**; concurrent
refresh requests coalesce, and last-known rows remain visibly qualified. A
truly empty list uses one primary **Open Terminal** action. Search recovery uses
one primary **Clear Search** action, while a caught-up Attention scope uses a
subordinate **Show All** action.

### 12. Terminal launch failure

```text
┌ Terminal did not start | + | app.rs ─────────────────────────────────┐
│ [!] Terminal did not start                                           │
│   codex: command not found                                           │
│                                                                      │
│   No terminal process was started. Review Terminal Launch settings, │
│   then open a new terminal.                                          │
│                                                                      │
│   [Edit Terminal Settings ▾]                                         │
│      Open Settings                                                   │
│      Edit settings.json                                              │
├──────────────────────────────────────────────────────────────────────┤
│ Workspace: dez | main | Permissions: Healthy                        │
└──────────────────────────────────────────────────────────────────────┘
```

The failure remains in its owning native tab, uses terminal material, names the
cause, and starts no substitute process. The primary split action deep-links to
**Workspaces & Terminals → Terminal Launch → Default Terminal**; its menu keeps
the general Settings and raw JSON alternatives available.

## Review-and-recovery wireframe

### 13. Agent change review

```text
┌ Workspaces ─────┬ Review · app.rs | app.rs | + ──────────────┐
│ dez · main       │ 3 files  +52  -17                              │
│                 │ [Previous Change] [Next Change]                   │
│ Open Tabs &     │ [Reject All Changes] [Keep All Changes]           │
│ Tools           ├──────────────────────────────────────────┤
│  Built-in Agent│ src/app.rs       [Review] [Reject] [Keep]          │
│  Git Changes   │ src/config.rs    [Review] [Reject] [Keep]          │
│  Terminal      │ tests/app_tests.rs [Review] [Reject] [Keep]        │
│                 │                                                       │
│                 │ native code diff                                     │
├─────────────────┴──────────────────────────────────────────┤
│ Workspace: dez | main | Permissions: Healthy                  │
└───────────────────────────────────────────────────────────────┘
```

Review is an ordinary Main Work Area surface. Previous and next controls,
file-level Review/Reject/Keep, and whole-review **Reject All Changes** and
**Keep All Changes** remain keyboard reachable. Every decision targets the
named file or the whole current review; Dez never places a custom overlay over
the diff.

### 14. Git History states

```text
┌ Git Changes | Git History | + ──────────────────────────────┐
│ Loading Git History                                          │
│ (…) Reading commits from this repository.                   │
│                                                              │
│ Git History couldn't load                                    │
│ [!] Check the repository state, then retry.                  │
│ [Retry]                                                      │
│                                                              │
│ No commits yet                                               │
│ Git History will appear after the first commit.              │
├──────────────────────────────────────────────────────────────┤
│ Workspace: dez | main | Permissions: Healthy                  │
└───────────────────────────────────────────────────────────────┘
```

These are alternate inline states, never a simultaneous stack. Loading uses a
status label and a spinner. Failure uses a warning mark and one primary
**Retry** action, which discards only a completed failed graph request before
starting a new request. Missing-repository and no-commit states remain quiet
and do not advertise an action that cannot help.

### 15. Subagent supervision

```text
┌ Built-in Agent | Git Changes | Terminal | + ────────────────┐
│ > Edits · 3 files · +52 -17                                │
│ [Review Changes] [Reject All Changes] [Keep All Changes]       │
│                                                               │
│ Backend audit                         Working · 3m 24s        │
│ Scanning code for unsafe patterns.          [Stop Subagent]    │
│                                                               │
│ [Open Subagent Session]                                       │
├──────────────────────────────────────────────────────────────┤
│ Workspace: dez | main | 2 agents | Permissions: Healthy        │
└───────────────────────────────────────────────────────────────┘
```

The Edits disclosure announces its file count and expanded state and responds
to Enter and Space. **Review Changes** is the primary inspection route;
whole-review decisions are visibly secondary. Subagent state remains text, not
color alone. **Open Subagent Session** is a visible outlined handoff that
focuses the existing child Session rather than creating a new one.

### 16. Resume honestly

```text
┌ Workspaces ─────┬ Terminal · Unavailable | Git Changes | + ───────┐
│ Codex · Working  │ preserved terminal output                              │
│ Terminal ·       │                                                       │
│ Unavailable      │ Terminal unavailable                                  │
│ Legacy · Access  │ Exact reason · last seen time                          │
│ blocked          │ Dez did not start a replacement shell.                │
│                  │ [Open new shell here] [Terminal Details]               │
│                  │                                                       │
│                  │ v Terminal Details                                    │
│                  │ Owner · Workspace · Working directory                 │
│                  │ Host generation · Endpoint · Evidence                 │
├─────────────────┴──────────────────────────────────────────┤
│ Workspace: dez | main | Host: unavailable                       │
└─────────────────────────────────────────────────────────────┘
```

Unavailable and legacy records preserve evidence without claiming process
death, liveness, ownership transfer, or migration. A fresh shell is explicitly
separate computation. Terminal Details stays inline, and **Terminate legacy
session…** remains a context-menu action behind native confirmation.

## Surface inventory

| Surface | Owner | Primary job | Entry | Empty, failure, or recovery state |
| --- | --- | --- | --- | --- |
| Home | Main Work Area tab | Start or resume the product loop | tab `+`, Help, first run | install-first and recent-history retry are inline |
| Workspaces | optional window navigator | switch codebases and supervise activity | status bar, View menu, shortcut | one Open Workspace action; bounded notices |
| Open Tabs & Tools | Workspaces projection | return to an existing surface | expands with two or more tabs | hidden for one tab; pane labels only for real splits; overflow stays pane-scoped |
| Sessions | Workspaces activity group | observe agents, tasks, tmux, Herdr, and cmux | Browse Running Sessions | source-specific Missing, Empty, Failed, Ready, or last-known state |
| Terminal | Main Work Area tab | run shell, TUI, task, or attach command | Home, tab `+`, File, Workspace menu | preserves output; launch failure deep-links to Terminal Launch settings; attach failure offers Retry or a fresh shell |
| Terminal Details | inline Terminal disclosure | inspect lifecycle, ownership, cwd, and evidence | terminal context strip | connection uncertainty never claims process death |
| Editor, diff, search, diagnostics | Main Work Area tabs | inspect and modify the codebase | native Zed actions | native Zed empty and error states |
| Agent Review | Main Work Area tab | inspect and decide agent edits | Review Changes, changed-file Review | pending edits disable decisions with an explanation; no custom overlay |
| Git Changes and History | Main Work Area tabs | review repository state and commits | Review Changes, Git tool | loading has status feedback; failure alone offers Retry |
| Files, Outline, Debug | Main Work Area tabs | inspect Workspace tools | tab `+`, View, terminal handoff | no permanent second drawer or column |
| Built-in Agent | Main Work Area tab | use Zed's model-backed agent and supervise Subagents | tab `+`, Workspace menu | configure provider instead of opening a dead surface; Subagent handoff focuses the existing child Session |
| Settings | native Settings surface | configure Workspace and terminal launch first, then agents and appearance | app menu, Command Palette | Workspace-dependent sections bind to the active Workspace |
| Status bar | window | durable Workspace and editor context | visible by default | explicit preference may hide it; closed Workspaces keeps a labeled restore control |
| Installation and access | Home plus Workspaces notice | unblock safe startup | startup preflight | install/relaunch or grant one exact folder; never background prompt loops |

## Interaction and copy rules

- Use one primary action per state. Put secondary actions in ordinary rows or
  an overflow menu.
- Use native Zed buttons, tabs, menus, prompts, focus, and theme tokens.
- Keep labels beside navigation icons. Provider glyphs identify a tool; state
  text identifies lifecycle.
- Preserve visible keyboard focus and a visual reading order that matches
  accessibility navigation.
- Use cause plus recovery for errors: **Attach failed · Connection refused**,
  then **Retry Attach** or **Open new shell here**.
- Name the configured result at the point of launch. Generic **Open Terminal**
  may become **Open Terminal · Codex** or the resolved equivalent when space
  permits.
- Keep permission scope explicit: **Workspace access required** and **Grant
  Access…** for one exact root.
- Keep recovery in its owning surface: installation on Home, root access in
  Workspaces, terminal lifecycle in the terminal tab.
- Reserve the status bar for durable context. Transient progress and lengthy
  diagnostics belong to the owning surface.
- Never introduce a Studio/Projects mode switch, custom browser tabs, a chat
  wrapper around terminal tools, floating onboarding, or an automatic split.

## Responsive contract

- The Main Work Area keeps at least 60% of the window width.
- Workspaces may collapse or close; its state remains reachable from the
  labeled status-bar control.
- Secondary row metadata truncates before the title or primary recovery action.
- Compact widths hide button text but retain the icon, tooltip, accessibility
  label, and keyboard target.
- Open Tabs & Tools remains flat for one pane. With a user-created split it
  groups by pane and names the focused pane.
- Long detail text wraps inside the owning surface; it does not create a new
  persistent column.

## Acceptance checklist

- Every open panel, tool, terminal, file, diff, and browser surface is reachable
  as a native tab and appears in Open Tabs & Tools when that projection is
  useful.
- Browse Running Sessions focuses Workspaces without replacing the current tab.
- Empty Workspaces and search states expose one visually primary recovery;
  caught-up scope changes remain subordinate.
- Native pane tabs remain the source of order, focus, dirty, close, and split
  truth.
- Agent review disclosures and decisions remain keyboard reachable, and Git
  History failure retries only the completed failed request.
- Subagent supervision keeps state in text and opens the existing child Session.
- Terminal Details expands inline and never obscures terminal output.
- Provider and subagent glyphs use the shared icon family; lifecycle remains
  readable without color.
- Empty, loading, permission, attach-failure, disconnected-host, and legacy
  states each have one truthful recovery route.
- Installation renders no restored Workspace evidence and exposes only one
  **Install and Relaunch** action.
- Settings starts with **Workspaces & Terminals**, **Agents**, **Appearance**,
  and **Workspace & Privacy** before inherited editor customization.
- The status bar remains visible by default and contains no transient prose.
- No screen creates an unexplained pane, permanent inspector, custom tab bar,
  floating onboarding, or duplicated Workspace navigation.
