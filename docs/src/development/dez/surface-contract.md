# Dez surface contract

This document is the source-controlled wireframe and interaction contract for
Dez's native shell. It complements the visual review sheet with exact labels,
ownership, state, and responsive behavior. When a rendered sketch and this
document disagree, this document wins.

The [tmux and Claude Code Navigation Wireframe](./tmux-claude-navigation-wireframe.md)
tracks the remaining exploration around the Workspace → Pane → Tab hierarchy.
Its activation-only **Layout** projection and bounded **Activity** feed are
authoritative here; navigation mode and return recaps remain proposals until
this contract is deliberately updated again.

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
│   ├── Layout (activation-only projection of the active Workspace)
│   ├── Activity (bounded running or actionable signals)
│   └── Workspace notices (bounded recovery states)
├── Main Work Area (authoritative native pane and tab model)
│   ├── Home, files, diffs, search, Settings, browser, diagnostics
│   ├── TerminalView surfaces for shells, agents, tmux, and Herdr
│   ├── Files, Outline, Git, Debug, and Built-in Agent tools
│   └── user-created panes and splits
└── Status bar (durable context and navigation)
```

One Workspace owns every tab, tool, Session, and user-created pane associated
with its codebase. A Workspace is not a list of terminals. **Layout** reads the
native pane model and activates its existing items; it never stores duplicate
order, focus, close, pin, dirty, or split state.

## Final wireframe

### 1. Home and launch

```text
┌ Workspaces ──────┬ Home ─ + ─────────────────────────────────────────┐
│ dez  main        │ Continue your work                                │
│ 3 agents · 2 ports│ Run a tool, supervise its work, then review it.   │
│                  │                                                    │
│ Layout           │ Start with a tool          Recent Workspaces      │
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
│ Activity         ├─────────────────────────────────────────────────────────┤
│  Codex · Working │                                                         │
│  Claude · Attention  Native TerminalView renders the provider TUI here.   │
│  Terminal · Ready│  Dez does not place a custom chat renderer around it.  │
│ Layout           │                                                         │
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
never the only state signal. The ordinary Workspace projection shows at most
five top-level Activity rows. A **Running Sessions** disclosure occupies one of
those rows; the current destination and attention-required rows take priority
over routine activity. The native Activity disclosure reveals the
complete current set, while Search and Attention scope remain exhaustive.

### 3. Supervise and recover

```text
┌ Workspaces ─────────────┬ Terminal · Herdr attach · Failed ─ + ──────┐
│ dez  main               │ Herdr attach failed · Connection refused    │
│ Workspace access required│ [Retry Attach] [Open new shell here]       │
│ [Grant Access…]         ├──────────────────────────────────────────────┤
│ Activity                │ Existing terminal output remains visible.    │
│  Codex · Working        │                                              │
│  Claude · Attention     │ Terminal Details                             │
│  Terminal · Ready       │ Provider · Herdr                             │
│  Running Sessions · 2   │ Working directory · ~/code/dez               │
│  Legacy · Access blocked│ Host generation · legacy                     │
│ Layout                  │ Endpoint · offline                           │
│  Terminal · Failed      │ Ownership · external process unchanged       │
│  Files · Diff · Git     │                                              │
├─────────────────────────┴──────────────────────────────────────────────┤
│ Workspace: dez | main | 3 agents | 2 ports | Access required          │
└────────────────────────────────────────────────────────────────────────┘
```

**Browse Running Sessions…** focuses Workspaces, clears transient filters,
expands each matching Workspace and its nested **Running Sessions** disclosure,
refreshes discovery, and preserves the active Main Work Area tab.
It never creates a duplicate Activity page. Recovery is inline and states both cause
and next action. Terminal Details is a disclosure within the terminal surface, not a
floating inspector. An attach tab names tmux or Herdr as the external owner and
describes itself only as the attach client. Destructive legacy termination remains
behind a native confirmation.

### 4. Review and intentional split

```text
┌ Workspaces ──────┬ Diff · app.rs | app.rs | + ┬ Terminal | Files | Git | + ┐
│ dez  main        │                              │                             │
│ Layout           │ diff                         │ native terminal              │
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
│ Layout                  │                                            │
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
│ Layout          │ Agents                 │ Workspaces Position  Left │
│                 │ Appearance             │ Show on Startup       Off │
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
control's meaning or accessible identity. At less than 760px of window width,
the density-scaled Workspaces mark remains the same keyboard, tooltip,
attention, expanded-state, and accessibility target while its visible
Workspace label yields space to editor and terminal status.

## Operational-state wireframe

### 9. Empty Main Work Area

```text
┌ Workspaces ───────┬ + ──────────────────────────────────────────────┐
│ dez · main        │ Main Work Area                                  │
│                   │ Start a terminal or resume running work.         │
│ No activity yet   │ Files and Git review open as tabs here.          │
│                   │                                                  │
│ superzed          │ [Open Terminal · Codex]                          │
│ website           │ [Browse Running Sessions] [Find File]            │
│                   │ [Review Changes]                                 │
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
┌─────────────────────────────────────────────────────────────────────────────┐
│ Codex · Working | app.rs* | Diff · config.rs | Terminal | + | Switch Tab    │
│ Add to Main Work Area             Tabs in This Pane                         │
│   Open Terminal ›                 Codex · Working · Active                  │
│     Default · Shell · Agents      app.rs · Modified                         │
│   Resume Existing Agent ›         Diff · config.rs                          │
│   Built-in Agent                  Terminal · Pinned                         │
│   Sessions and Multiplexers       Settings                                  │
│     Workspace tmux                                                          │
│     Browse Running Sessions…                                                │
│     Open Workspace in cmux                                                  │
│   Files · Review · Task · Debug                                             │
│   Home · Recent Workspaces                                                  │
└─────────────────────────────────────────────────────────────────────────────┘
```

The two menus are alternate anchored native menus and never appear together in
the product. `+` follows the final visible tab and remains pinned to the tab
viewport edge during overflow. **Switch Tab** lists only the owning pane and
preserves active, modified, and pinned meaning in text as well as icon state.

Dez disables preview tabs by default so opening another file does not replace
the previous file tab. Users may explicitly enable preview tabs for Zed-style
single-click browsing. The Files titlebar control activates the existing Files
tab; activating it again returns to the pane's most recently active different
tab. It never opens a second Files drawer or changes the Workspace layout.

The Add menu keeps **Sessions and Multiplexers** visible as a first-class
group: **Workspace tmux**, **Browse Running Sessions…**, and the applicable
**Open Workspace in cmux** handoff. **Open Terminal → Claude Code** always
starts `claude`; **Resume Existing Agent → Claude Code · Last Session** is the
separate opt-in `claude --continue` path.

### 11. Discover and supervise

```text
┌ Workspaces ─────────────────┬ app.rs ─ + ────────────────────────────┐
│ Search · All · Attention    │ Existing editor remains active.        │
│ dez · main                  │                                        │
│ Activity                    │ Browse Running Sessions never opens    │
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

**Other Running Sessions** remains absent during ordinary work when it has no
rows. After the user explicitly chooses **Browse Running Sessions…**, it may
stay open long enough to report **Checking…**, **All running sessions belong to
open Workspaces**, **No running sessions**, or **Discovery needs attention**.
It omits a decorative zero count, exposes **Retry** only for failed discovery,
and uses a keyboard-operable disclosure with an announced expanded state.

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
│ Layout          │ [Reject All Changes] [Keep All Changes]           │
│                 ├──────────────────────────────────────────┤
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
whole-review decisions are visibly secondary. The related but distinct Dez
Subagent mark preserves hierarchy without reusing a provider logo. Subagent
state remains text, not color alone. **Open Subagent Session** is a visible
outlined handoff that focuses the existing child Session rather than creating
a new one.

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

## Native-tool wireframe

### 17. Find, inspect, and diagnose

```text
┌ Workspaces ─────────┬ Files | Search | + ──────┬ Workspace Diagnostics | + ┐
│ paykit              │ Files tree | editor       │ No problems in Workspace   │
│ main · 2 panes      │                           │ [Refresh]                   │
│ Activity            │ Search this Workspace     │                             │
│  Claude · Working   │ No matches                │                             │
│  tmux · Attached    │ Broaden the query or      │                             │
│ Layout              │ remove path filters.      │                             │
│  Pane 1             │                           │                             │
│   Files · Search    │                           │                             │
│  Pane 2             │                           │                             │
│   Diagnostics       │                           │                             │
├─────────────────────┴───────────────────────────┴─────────────────────────────┤
│ main | 0 errors | 0 warnings | agents healthy | Ln 14, Col 22               │
└───────────────────────────────────────────────────────────────────────────────┘
```

Files, Search, and Workspace Diagnostics are ordinary pane tabs. Workspaces
projects them for navigation only; it does not invent a second tool mode or tab
order. Loading, empty, and failed states use the same compact inline grammar.
Every icon-only diagnostics action is keyboard reachable and names its action.
The empty diagnostics state keeps an explicit **Refresh** action without
turning a healthy state into a warning.

### 18. Run work where it belongs

```text
┌ Workspaces ─────────┬ main.rs | Task · test | + ─────────────────────────────┐
│ paykit              │ Task · test                 Running       [Stop Task] │
│ Activity            │ cargo test --workspace                                  │
│  Claude · Working   │ running 42 tests…                                       │
│  tmux · Attached    │                                                         │
│  Task · test        │ ┌ Run Task ──────────────────────────────────────────┐  │
│    Running          │ │ Find a task, or run a command                      │  │
│ Layout              │ │ test · test:unit · test:watch · fmt · clippy       │  │
│  Pane 1             │ │ [Rerun Last Task]                            [Run] │  │
│   main.rs           │ └────────────────────────────────────────────────────┘  │
│   Task · test       │                                                         │
├─────────────────────┴─────────────────────────────────────────────────────────┤
│ main | Task: Running | agents healthy                                        │
└───────────────────────────────────────────────────────────────────────────────┘
```

A Task owns its native terminal output tab and appears in Workspaces only as
live activity. The task picker says **Run**, not spawn, in Dez. While inventory
loads it reports that state; an empty inventory explains that typed input can
run once; and an unmatched non-empty query offers the same honest one-shot
command path instead of a dead **No matches** message.

### 19. Debug without a dead end

```text
┌ Workspaces ─────────┬ main.rs | Debug | + ───────────────────────────────────┐
│ paykit              │ Start debugging                                       │
│ Layout              │ [Start Debug Session] [Configure debug.json]           │
│  Pane 1             │ Documentation · Adapter Logs                           │
│   main.rs           ├───────────────────────────┬────────────────────────────┤
│   Debug             │ Breakpoints               │ Console                    │
│  Pane 2             │ No breakpoints yet.       │ No debug session running.  │
│   Diagnostics       │ Set one in an editor      │                            │
│                     │ gutter.                   │ Launch failed              │
│                     │                           │ exact adapter reason       │
│                     │                           │ [Retry] [Adapter Logs]      │
├─────────────────────┴───────────────────────────┴────────────────────────────┤
│ main | Debug: Idle | 1 diagnostic                                            │
└───────────────────────────────────────────────────────────────────────────────┘
```

Debug owns setup, running controls, output, and recovery in one native surface.
Idle copy names the result, configuration remains a secondary action, and
breakpoint guidance points back to the editor. Failures preserve the adapter
reason and expose Retry and logs inline; they never open an overlay or create a
second Debug navigation system.

### 20. Resume and hand off honestly

```text
┌ Workspaces ─────────┬ Home | + ──────────────────────────────────────────────┐
│ paykit              │ Recent Workspaces                                      │
│ Activity            │ paykit · ~/dev/paykit                                  │
│  Claude · Working   │ client-app · Access required     [Grant Folder Access] │
│  tmux · Attached    │ Recent Workspaces unavailable                 [Retry]  │
│ Layout              │                                                        │
│  Pane 1 · Home      │ External tools                                         │
│  Pane 2 ·           │ [Open Workspace in cmux]                               │
│   Diagnostics       │ cmux owns its tabs, splits, and browser.               │
│                     │ [Open URL in System Browser]                           │
│                     │                                                        │
│                     │ Legacy · Access blocked                                │
│                     │ [Open New Shell Here] [Keep Running] [Terminate…]      │
├─────────────────────┴────────────────────────────────────────────────────────┤
│ main | Permissions: Access required | Host: Ready                            │
└───────────────────────────────────────────────────────────────────────────────┘
```

Workspace permission is requested once per root and blocks dependent work
before Git, Search, language services, agents, or terminals begin. A replacement
shell is always described as new computation. Dez never claims to migrate a
live process. cmux owns its tabs, splits, browser, hooks, and action registry;
Dez provides an explicit Workspace handoff. Ordinary URLs open in the system
browser rather than a fake embedded browser surface.

The final shell contract is: **one sidebar, many Workspaces, many panes, any
native tool, honest ownership, inline recovery**.

## Surface inventory

| Surface | Owner | Primary job | Entry | Empty, failure, or recovery state |
| --- | --- | --- | --- | --- |
| Home | Main Work Area tab | Start or resume the product loop | tab `+`, Help, first run | install-first and recent-history retry are inline |
| Workspaces | optional window navigator | switch codebases and supervise activity | status bar, View menu, shortcut | one Open Workspace action; bounded notices |
| Layout | activation-only Workspaces projection | return to an existing pane or tab | active Workspace header | appears once there are at least two open tabs and then includes every open tab; single-pane headings are omitted; split-pane focus is explicit; tab ownership and overflow stay pane-scoped |
| Activity | bounded Workspaces projection | observe active, running, actionable, recoverable, or review-ready agents, terminals, tasks, tmux, Herdr, and cmux | Browse Running Sessions or select a row | absent when empty; default preview is at most five top-level rows and preserves the current and attention-required destinations; external work begins as one Running Sessions disclosure; native expansion, Search, and Attention scope reveal the complete relevant set; completed history remains in Agent History; sources retain truthful Missing, Empty, Failed, Ready, or last-known state |
| Terminal | Main Work Area tab | run shell, TUI, task, or attach command | Home, tab `+`, File, Workspace menu | preserves output; launch failure deep-links to Terminal Launch settings; attach failure offers Retry or a fresh shell |
| Terminal Details | inline Terminal disclosure | inspect authoritative status, Agent or process, path, Git context, and ownership | terminal context strip | connection uncertainty never claims process death; tmux and Herdr remain external owners while the tab owns only its attach client; terminal output remains authoritative and unobscured |
| Editor, diff, Files, Search, Diagnostics | Main Work Area tabs | inspect and modify the codebase | native Zed actions, tab `+`, Workspaces projection | inline idle/loading/no-match states; Diagnostics keeps keyboard actions and explicit Refresh |
| Tasks | Main Work Area terminal tab | run a saved task or a one-shot command | tab `+`, command palette, pane Add | loading names Workspace inventory; empty and unmatched states explain the one-shot command path |
| Agent Review | Main Work Area tab | inspect and decide agent edits | Review Changes, changed-file Review | pending edits disable decisions with an explanation; no custom overlay |
| Git Changes and History | Main Work Area tabs | review repository state and commits | Review Changes, Git tool | loading has status feedback; failure alone offers Retry |
| Outline and Debug | Main Work Area tabs | inspect symbols or own a complete Debug lifecycle | tab `+`, View, terminal handoff | Debug owns setup, output, breakpoints, Retry, and adapter logs; no permanent second drawer |
| External web and cmux | system browser or cmux | continue work in the external owner | explicit URL or Open Workspace in cmux action | no fake embedded browser and no ownership or migration claim |
| Built-in Agent | Main Work Area tab | use Zed's model-backed agent and supervise Subagents | tab `+`, Workspace menu | configure provider instead of opening a dead surface; Subagent handoff focuses the existing child Session |
| Settings | native Settings surface | configure Workspace and terminal launch first, then agents and appearance | app menu, Command Palette | Workspace-dependent sections bind to the active Workspace |
| Status bar | window | durable Workspace and editor context | visible by default | explicit preference may hide it; closed Workspaces keeps a labeled restore control |
| Installation and access | Home plus Workspaces notice | unblock safe startup | startup preflight | install/relaunch or grant one exact folder; never background prompt loops |

## Interaction and copy rules

- Use one primary action per state. Put secondary actions in ordinary rows or
  an overflow menu.
- Use native Zed buttons, tabs, menus, prompts, focus, and theme tokens.
- Use the shared density scale for top-bar, tab-bar, sidebar title and search
  rows, and status-bar controls. Compact navigation uses 22px targets with
  12px marks; Balanced uses 22–28px targets with 14px marks; Spacious uses
  28–32px targets with 14px marks. Status type remains supporting text and the
  Dez status bar scales from 24–30px.
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
- Top and status chrome must change size together when Compact, Balanced, or
  Spacious density changes; individual surfaces do not invent local scales.
- Pane Back, Forward, adjacent Add, overflow, split, zoom, and close controls
  share one tab-bar metric. Workspace disclosure, terminal launch, options,
  search, and Layout rows share the Workspace navigation metrics.
- Secondary row metadata truncates before the title or primary recovery action.
- Compact widths hide button text but retain the icon, tooltip, accessibility
  label, and keyboard target.
- Layout is omitted while the active Workspace has only one tab because the
  native tab strip already exposes that sole destination. Beginning with the
  second tab, Layout stays flat for a single-pane Workspace and reports only the
  open-item count. With a user-created split it adds the real pane count,
  groups rows under **Pane 1**, **Pane 2**, and so on, and names the focused
  pane in text. Selection styling and accessibility copy carry focused and
  visible state without repeating an `Active` badge. Rows activate native tabs
  but do not close, pin, reorder, drag, or split them.
- Long detail text wraps inside the owning surface; it does not create a new
  persistent column.
- Every curated Settings field has a registered native renderer. A missing
  renderer or unresolved default is a release blocker; Dez falls back to the
  native settings file instead of showing `NO RENDERER` or `NO DEFAULT` as a
  control.

## Acceptance checklist

- Every open panel, tool, terminal, file, diff, and browser surface is reachable
  as a native tab. Once the active Workspace has at least two tabs, every open
  surface appears in Layout.
- Browse Running Sessions focuses Workspaces without replacing the current tab.
- A fresh Workspace starts in the Main Work Area without an automatic Files
  column; restored layouts and an explicit Files startup preference still win.
- Activity excludes inactive completed history while preserving active,
  attention, recovery, and review-ready rows. Layout remains the route to idle
  open tabs and Agent History remains the route to completed Agent Sessions.
- Activity shows at most five top-level rows before native expansion, reserves
  one row for Running Sessions when present, and prioritizes the current and
  attention-required managed destinations in that preview.
- Empty Workspaces and search states expose one visually primary recovery;
  caught-up scope changes remain subordinate.
- Native pane tabs remain the source of order, focus, dirty, close, and split
  truth.
- Agent review disclosures and decisions remain keyboard reachable, and Git
  History failure retries only the completed failed request.
- Subagent supervision keeps state in text and opens the existing child Session.
- Terminal Details expands inline, summarizes Status, Agent or Process, Path,
  Git, and Ownership, and never obscures or re-renders terminal output.
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
