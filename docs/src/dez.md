---
title: What is Dez?
description: Learn how Dez combines a native IDE, integrated terminals, coding agents, and review evidence in one Workspace.
---

# What is Dez?

Dez is a native, Workspace-first development environment for people who build
software by mixing direct coding with terminal agents. It is for professional
developers, technical founders, product engineers, and AI-native independent
builders who still want to inspect and own the code they ship.

Its product promise is simple:

> Keep code, running work, attention, and review evidence together without
> replacing the editor or terminal with a dashboard.

Built on Zed, Dez keeps its fast editor, language tooling, Git, debugger, tasks,
remote development, and native pane system. Dez adds a Workspace-first
navigation model for moving between direct editing, terminal-native agents,
attention, and review without reconstructing context.

The main journey uses native surfaces throughout:

1. If Dez asks, choose **Install and Relaunch** before restoring a Workspace or
   starting durable terminals.
2. **Open Workspace** to establish the codebase, Git, and working-directory
   context.
3. Use **Open Terminal** or the tab-strip **+** to start the configured default,
   a native shell, tmux, Codex, Claude Code, or OpenCode in native tabs. **More
   Agent CLIs** adds Gemini CLI, Aider, and Herdr without crowding that primary
   list. Use **Open Workspace in cmux** for an explicit external handoff.
4. Use **Continue Agent** to resume the last Codex, Claude Code, or OpenCode
   session in this Workspace. Use **Browse Running Sessions…** for discovered
   tmux, Herdr, and cmux work.
5. Supervise attention in **Workspaces**, then use **Open Files**, **Run Task…**,
   diagnostics, Debug, and **Review Changes** against the same codebase.

## Terminal agents and multiplexers

Dez does not re-render a terminal application or replace its interface. Codex,
Claude Code, OpenCode, tmux, and Herdr run inside the inherited Zed terminal
emulator as real PTY applications. Their colors, alternate-screen behavior,
mouse input, keyboard input, and TUI layout therefore remain their own. Dez
adds native tab ownership, Workspace context, durable terminal ownership where
eligible, attention projection, and routes back to Files and Git review.

| Route           | Start new                            | Continue or attach                                      | Ownership                                 |
| --------------- | ------------------------------------ | ------------------------------------------------------- | ----------------------------------------- |
| **Codex**       | `codex`                              | `codex resume --last`                                   | Native Dez terminal                       |
| **Claude Code** | `claude`                             | `claude --continue`                                     | Native Dez terminal                       |
| **OpenCode**    | `opencode`                           | `opencode --continue`                                   | Native Dez terminal                       |
| **Gemini CLI**  | `gemini`                             | Start a new terminal-owned CLI session                  | Native Dez terminal                       |
| **Aider**       | `aider`                              | Start a new terminal-owned CLI session                  | Native Dez terminal                       |
| **tmux**        | `tmux new-session -A -s <workspace>-<root-id>` | The same root-scoped command attaches when the named session exists | tmux process inside a native Dez terminal |
| **Herdr**       | `herdr`                              | Select a discovered pane in Workspaces                  | Herdr; Dez attaches explicitly            |
| **cmux**        | Start work in cmux                   | **Open Workspace in cmux** uses `cmux open <path>`      | cmux remains the external app             |

The Start and Continue routes are available from the pane **+**, **File**, a
Workspace's options menu, and Command Palette. They create a normal Main Work
Area terminal in the active Workspace; they do not open a provider onboarding
overlay, manufacture a chat transcript, or rewrite the default launcher.

Set the frequent path in **Settings → Workspaces & Terminals → Terminal Launch
→ Default Terminal**. The native dropdown offers Native Shell, Codex, Claude
Code, OpenCode, Gemini CLI, Aider, Herdr, tmux Session, and Custom Command. A
custom command is the escape hatch for another TUI or wrapper. Provider and
tmux marks lead each choice while the current choice keeps a separate trailing
check. Use **Continue Agent** when returning to the most recent provider
session. Provider authentication and subscriptions remain owned by each
provider CLI.

The default action carries its resolved identity across native navigation.
The Default Terminal chooser, Home, the empty Main Work Area, pane `+`, and
Workspaces show the corresponding provider or tmux mark; Native Shell and
unknown custom commands keep the terminal mark. Wrappers and absolute
executable paths do not erase a known provider identity.

cmux is an external Workspace handoff, not a Dez terminal profile. Install it
separately, then use **Open Workspace in cmux**. Its
[custom commands](https://cmux.com/docs/custom-commands) own Codex, Claude Code,
and OpenCode actions as well as any multi-pane agent layouts. Its current action
registry can also place user-defined agent actions in cmux's own tab bar and
Command Palette; Dez does not import or rewrite that configuration. Dez
deliberately does not reproduce that layout inside the editor. **Settings →
Workspaces & Terminals → Terminal Launch → cmux Integration** links to the
documented local API and access modes. Its external-arrow affordance is a
documentation handoff, not a toggle that mutates cmux. Internal Settings
actions use their own reset, keyboard, reference, or run marks instead. cmux
may reject API calls made outside a cmux-owned terminal because **cmux processes
only** is its secure default. Cross-app
discovery requires the documented `CMUX_SOCKET_MODE=allowAll` environment
override when the user deliberately accepts that local access boundary; Dez
never changes it. A process-only refusal is the expected **Access required**
state, rendered as the informational **cmux activity sharing is off** notice
with an **Open API Guide** action. It is not reported as a broken integration,
and **Open Workspace in cmux** remains available because path handoff does not
require the control socket. Unexpected API failures still produce one native
source issue per state change instead of flooding logs on every refresh. For
cmux-owned notification and restore metadata, review and run its hook setup
explicitly:

```bash
cmux hooks setup
cmux hooks setup codex
cmux hooks setup --agent opencode
```

Dez never installs or edits cmux, Codex, Claude Code, or OpenCode hooks without
the user's deliberate action. cmux restores its own app layout and supported
agent bindings; tmux owns its server sessions; Dez owns only terminals created
under its current durable terminal Host. Arbitrary running processes cannot be
silently transferred between those owners.

The integration contracts follow the current primary documentation for the
[Zed terminal](https://zed.dev/docs/terminal),
[Codex CLI](https://github.com/openai/codex),
[Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code/cli-usage),
[OpenCode CLI](https://dev.opencode.ai/docs/cli/), and
[cmux](https://github.com/manaflow-ai/cmux).

## Zed features in the Dez workflow

Dez keeps Zed's capabilities, but promotes only the ones that close the active
Workspace loop. The pane **+** and Command Palette route to the same native
actions; Dez does not fork a second task runner, debugger, diagnostics view, or
file navigator.

| Need                      | Native route in Dez                                                              |
| ------------------------- | -------------------------------------------------------------------------------- |
| Find code                 | Find File, Workspace Search, Symbol Search, Files, and Outline                   |
| Run or watch              | **Run Task…** for repeatable project commands; Terminal for interactive tools    |
| Understand code           | Language servers, active toolchains, diagnostics, references, and symbol outline |
| Debug                     | **Open Debug**, native breakpoints, variables, stack frames, and debug tasks     |
| Inspect generated changes | Files, Workspace Diagnostics, Git, native diffs, and Review Changes              |
| Work remotely             | Open Remote Workspace while preserving the Workspace ownership model             |

Tasks and terminals share the active Workspace directory and environment;
toolchains feed terminals and language tooling; diagnostics and Debug inspect
the same buffers the agent edits. See the primary
[tasks](https://zed.dev/docs/tasks),
[toolchains](https://zed.dev/docs/toolchains),
[diagnostics](https://zed.dev/docs/diagnostics), and
[debugger](https://zed.dev/docs/debugger) documentation for their full native
behavior.

## The screen model

The interface has two ownership regions. The Main Work Area is always present;
Workspaces is optional and collapsible:

| Region                    | What it is for                                                                                                  |
| ------------------------- | --------------------------------------------------------------------------------------------------------------- |
| **Workspaces** · optional | Switch codebases and supervise their terminal and Built-in Agent Sessions on the configured window edge         |
| **Main Work Area**        | Edit files and use terminal, Files, Outline, Git, Debug, Built-in Agent, settings, diagnostics, and review tabs |

Workspaces is global and collapsible. It does not turn into Files, Git, or chat,
and opening a Workspace Tool does not dismiss it. Each codebase remains visible
before an agent starts; its Agent Sessions appear beneath it as they are
detected or managed.

Workspace Tools is a category, not another fixed column. Files, Outline, Git,
Debug, and the optional provider-backed Built-in Agent open in the active
Workspace's Main Work Area tab strip. A restored pre-v0.1 tool drawer is migrated
into that strip, so reopening a Workspace cannot recreate an unexplained
side-by-side pane.

The Main Work Area is one native pane grid. A file, agent terminal, ordinary
terminal, diff, search result, settings page, or review can be tabbed, split,
moved, and focused through the same rules. Its tab strip provides native
reorder, cross-pane drag, preview replacement, pin, and close behavior; Dez
does not imitate browser tabs with a separate navigation system. Terminals can
be split below or beside code while keeping their Workspace ownership.

When tabs overflow, **Switch Tab** opens a native **Tabs in This Pane** menu.
Its rows retain active, modified, and pinned state, so compact navigation does
not flatten pane ownership or conceal unsaved work.

The tab-strip **+** is the single **Add to Main Work Area** control. Its
**Open Terminal** submenu launches the **Default Terminal**, a **Native Shell**,
a **Workspace tmux** session, Codex, Claude Code, or OpenCode. **More Agent
CLIs** keeps Gemini CLI, Aider, and Herdr one level deeper. The Add menu also
opens the optional Built-in Agent, a file, Files, Review Changes, Run Task, Debug,
Workspace search, or symbol search. **Continue Agent** resumes the last native
provider session without mixing it into running-session discovery. The control
follows the final tab while space remains and stays pinned to the tab viewport
edge when tabs overflow. **Browse Running
Sessions…** clears temporary filters,
refreshes discovery, and focuses Workspaces. Path-matched Sessions stay beneath
their Workspace; unmatched or pathless Sessions stay in **Other Running
Sessions**. Opening an unmatched tmux or Herdr Session with a known working
directory first establishes that directory as a native Workspace, then attaches
the external client. Pathless Sessions use the active Workspace; cmux remains
an explicit external handoff through **Open Workspace in cmux**.
Every destination uses the existing native pane system, so it can be focused
and arranged without creating a nested panel or a second navigation model. The
tab strip and Add control remain visible on an empty Main Work Area, including
Home.

The Built-in Agent route is readiness-aware. When an authenticated default
model is available, the Add menu and Workspace Options show a Built-in Agent
action with the Dez Agent mark. Until then, both native menus show **Configure
Built-in Agent…** with the Settings mark and open provider settings. Neither
route leads a first-time user into a model-less Agent surface. Terminal agents
remain independent of this provider setup.

The Dez **File → Open Terminal** submenu mirrors the native **+** launch routes
in the same order, followed by **Continue Agent**. Its first row previews the
configured result as **Default · Native Shell**, **Default · Codex**, **Default
· Claude Code**, **Default · OpenCode**, **Default · tmux Session**, a detected
agent, or **Default · Custom Command**; the pane **+** keeps the shorter
**Default Terminal** label. Native Shell, Workspace tmux, Codex, Claude Code, and
OpenCode remain explicit alternatives. Gemini CLI, Aider, and Herdr are grouped
under **More Agent CLIs** so the frequent path stays short.
Continue routes use the providers' documented last-session commands. **Browse
Running Sessions…** follows those menus. Starting new work, continuing provider
state, and reopening externally owned work therefore stay adjacent without
creating a second navigation model.

Workspaces starts closed in fresh windows and remains available as an on-demand
supervisor. Workspace Tools start closed and open as ordinary Main Work Area
tabs. Explicitly opened and restored Workspaces layouts remain open. The same
policy applies after resizing, reopening, and restoring a saved layout.
Returning to a one-work-area destination consolidates every populated split
into the Main Work Area tab strip; files, terminals, diffs, and tools remain
open as native tabs. Workspace restoration applies the same repair to those
destination layouts, so a stale review or tool split cannot return as an
unexplained second column. **Split Work Area** alone may retain up to two
already-populated work areas and never creates an empty pane merely to satisfy
a diagram.
Optional **Workspace Layout** commands name destinations instead of abstract
arrangements: **Work Area + Files**, **Work Area + Built-in Agent**, **Focus
Work Area**, **Split Work Area**, **Work Area + Git**, and **Work Area +
Debug**. They live in **View** and Command Palette rather than occupying the
default titlebar. Files, Git, Debug, and Built-in Agent each select that exact
native surface. Focus closes contextual tools. Split Work Area only arranges
existing Main Work Area surfaces; it never starts a process or opens an
unrelated tool. No recipe may manufacture an empty column.
When that work area is empty, one restrained launch panel states the product
purpose and offers **Open Terminal**, **Browse Sessions**, **Find File**, and
**Review Changes**. It does not repeat Home's product summary. It is an
operational start state for the current Workspace, not a second Home screen.
**Workspaces** is a projection over the real owners. Each open codebase
remains visible even before an agent starts; its Agent Sessions appear beneath
it as they are detected or managed. Selecting a Session focuses or reattaches
its existing Surface instead of opening a duplicate. Selecting a Workspace
activates that codebase; its dedicated disclosure control folds or expands its
Agent Sessions. An ordinary shell is not an Agent Session and does not create a
generic row. Provider icons identify Codex, Claude Code, OpenCode, Herdr, and
other detected agents. Native status treatments distinguish Running, Needs
Input, Waiting for Permission, Reconnecting, Completed, and Error without
rendering a second terminal transcript.
When Workspaces is open, its collapsible **Workspace Tabs** section projects the
active Workspace's native Main Work Area tabs above the Workspace and Session list
when at least two tabs are open. A one-pane layout remains a flat list. Real splits add quiet **Pane 1**,
**Pane 2**, and later group labels, so the hierarchy describes the layout that
already exists instead of manufacturing one. Each row uses the Surface's
native icon, disambiguated tab title, active state, dirty state, and pinned
state. Terminal rows preserve their task, Agent provider, or tmux mark.
Built-in and external Agent tabs preserve the same Dez or provider mark used
by their native tab, and the projection announces each row's position in the
native tab set. In split layouts, each pane's visible tab has a quiet selected
row and the focused pane keeps the accent icon, so pane visibility does not
disappear when focus moves. The collapsed summary reports tabs and panes; when
expanded, the focused pane's group label names that state directly. Activating
a row calls the owning Workspace's native item activation path. An unpinned row
exposes the Surface's native close icon on hover, keeps it
visible while that Surface is active in its pane, and accepts middle-click.
Both routes call the owning pane's normal close path, including unsaved-work
confirmation. Pinned rows remain protected. Ordering, dragging, preview
replacement, pinning, and split ownership remain in the native tab strip. The
disclosure state persists, while Workspace search temporarily hides the section
to keep filtering focused.
Each Main Work Area pane keeps native Back and Forward controls in its tab
bar. They traverse files, terminals, diffs, settings, and other native
Surfaces without inventing a separate browser or duplicating Workspace
navigation.
Home and Terminal Details name Workspaces directly so the product never implies
that Dez captures or relocates every shell on the machine.
Notifications and toasts are bounded transient shelves over the Main Work
Area. They never become another full-height column, and overflowing alerts
scroll inside their shelf instead of covering the editor or terminal.

## The control grammar

Dez uses the same icon for the same object or transition everywhere:

| Icon role    | Meaning                                            |
| ------------ | -------------------------------------------------- |
| Terminal     | Start or return to terminal computation            |
| Folder open  | Open a Workspace or reveal its Files               |
| File         | Create a new file                                  |
| Diff         | Review observed Workspace changes                  |
| Information  | Inspect terminal details and evidence boundaries   |
| Session list | Supervise Agent Sessions and attention             |
| Clock        | Open Agent Session history                         |
| Settings     | Configure Agent tools and application behavior     |
| Robot        | Create or identify a Dez Agent Session             |
| Sparkle      | Invoke Inline Assist on the current editor context |

Icons support labels; they never replace them. A creation icon does not stand
in for an object-specific Terminal or File icon, and review/details controls
do not reuse each other's symbols. Dez does not reuse Zed-branded assistant
marks for these controls. Command Palette labels follow the same grammar: Dez
uses `workspace: ...` for actions scoped to the active codebase,
`workspaces: ...` for global navigation, and the specific `files: ...` and
`git: ...` destinations instead of inherited implementation namespaces.
Terminal creation appears under `terminal: open terminal …`, and the six
layout transitions and their management commands appear under `layout: ...`;
neither route exposes inherited Project, Thread, or Canvas terminology.

Hierarchy follows the next useful action. Home places **Open Workspace** first
when no codebase is open. Inside an active Workspace it presents **Open
Terminal**, **Browse Running Sessions…**, **Open Files**, and **Review Changes**
in that order: Start → Resume → Inspect → Review. These are native command rows
on the editor surface, not filled dashboard cards. Home's first row identifies
the configured result as **Default · Native Shell**, **Default · Codex**,
**Default · Claude Code**, **Default · OpenCode**, **Default · tmux Session**,
another detected terminal agent such as **Default · Aider** or **Default ·
Herdr**, or **Default · Custom Command**, using the same shared identity as File
and Workspace menus. Simple `env`, `exec`, environment-variable, and
absolute-path wrappers retain the underlying provider name. Dense
Files & Git and Built-in Agent toolbars use compact icons, but every control has
a specific accessible name, tooltip, and place in the keyboard tab order. A
critical action is never available only on pointer hover.

First run follows the same rule. Dez opens **Home** as a normal Main Work Area
tab, not a setup modal, tour, floating card, or restored split. When the app is
running from a temporary location, Home shows one inline **Install and
Relaunch** action and does not restore Workspaces or start durable terminals.
Theme, keymap, optional Agent providers, imports, and trust preferences stay in
native Settings.

Without a Workspace, Home offers **Open Workspace** and **Clone Repository**.
It does not offer a pathless terminal before a codebase can supply file, Git,
and working-directory context. Inside a Workspace, Home offers **Open
Terminal**, **Browse Running Sessions…**, **Open Files**, and **Review Changes**.
**Open Terminal** uses the configured **Default Terminal**. Its inline detail
names that resolved default before launch, so a returning user does not need to
reopen Settings to remember whether **Open Terminal** starts a shell, agent, or
**Workspace tmux** session. The adjacent **+**
and Command Palette keep Native Shell, tmux, Codex, Claude Code, and OpenCode
available as one-off launches, with Gemini CLI, Aider, and Herdr grouped under
**More Agent CLIs**. **New File** remains available from File, the
native **+**, and keyboard shortcuts, but is no longer a primary Home or
empty-state action.
Recent Workspaces remain ordinary keyboard-reachable rows rather than a
separate dashboard.

The empty Workspaces navigator follows the same activation loop. **No Workspace
open** explains that a codebase supplies context to agent work and review.
**Open Workspace** is its only start action. Once a Workspace is ready, the
scoped action becomes **Open Terminal**. The Workspace remains visible,
but its Agent Session list stays empty until agent evidence exists. Start,
recovery, and All/Attention scope actions remain keyboard reachable as the
navigator changes state.

Workspace controls follow focus. Selecting or keyboard-focusing a Workspace
keeps its Options action visible; opening that menu keeps its scoped close
controls visible as well. Terminal creation remains visible in the active Main
Work Area, an inactive empty-Workspace row, or a collapsed Workspace header, so
pointer hover is never its only route. Search clearing and banner dismissal are
keyboard-focusable. Workspace names and their action cluster share one bounded
inline row: text truncates within its allocation, actions never overlap it, and
no gradient mask is painted over either side of the header. An
inactive expanded Workspace with no Agent Sessions shows one labeled **Open
Terminal** action below the header. The active Workspace does not repeat that
launcher because its Main Work Area already presents **Open Terminal** as the
primary action. Collapsing a Workspace restores the compact header action, and
a Workspace with Session activity keeps that compact route available.
Readiness remains in the overview summary and the Workspace header's accessible
name instead of being repeated as a decorative dot-and-caption row.

The Main Work Area follows the same rule. Back, Forward, Add, Switch Surface,
Split, Zoom, and tab close controls are keyboard-focusable and specifically
named. Files, Git, Outline, Debug, and Built-in Agent use the native Main Work
Area tab strip; each has one icon, is keyboard reachable, and follows the same
tab lifecycle as files, terminals, and diffs. Work tabs remain native and
draggable. The active unpinned Main Work Area Surface keeps its close control
visible in Dez; inactive tabs remain visually quiet.

Existing generated Dez profiles are upgraded consistently. A known legacy
profile that pinned `.ZedSans`, One Light, and light-only appearance migrates
to IBM Plex Sans for interface chrome, Lilex for code and terminals, and
system-selected Lumin Light/Lumin Blur. Custom font or theme choices are not
treated as generated defaults.

Creation emphasis also follows state. An inactive, ready Workspace without a
Session shows one quiet **Open Terminal** command row; the active Workspace uses
the same action in its Main Work Area. Once work exists, a compact terminal
action remains in each Workspace header while the Workspaces overview stays
focused on status and All/Attention scope. Dez does not repeat a launcher for
the already-active Workspace.

Workspaces uses one native titlebar label and one options menu for secondary
destinations such as Command Palette, Recent Workspaces, Workspace activity,
attention filtering, and Workspaces Settings. It does
not repeat **Workspaces** inside a dashboard header or reserve a permanent
footer. Empty, caught-up, search, and recovery states stay in normal sidebar
flow and never become floating overlays. An empty inactive Workspace keeps one
scoped terminal row, while a collapsed Workspace keeps the compact header
action; the active Workspace relies on its visible Main Work Area launcher.
None expands into a provider onboarding block. At compact widths, the caught-up
action shortens to **Show All** without
losing source or status in each row. Explicit tmux and Herdr rows attach in the
Main Work Area; cmux Workspace rows open in cmux. A row with a current path sits
beneath the most specific matching Workspace and uses its secondary metadata
row for source, semantic state, working directory or worktree, and attention.
cmux rows keep the first API-reported listening port and a hidden-port count in
the native row's compact trailing metadata. Their tooltip and accessibility
label retain the richer bounded port list without turning Workspaces into a
server scanner or treating passive port metadata as an accent action.
Pathless and unmatched discovered rows remain in **Other Running Sessions**.
The Workspace header owns Git branch and changed-file metadata. Repository
association follows path-component ancestry in both directions, so opening a
repository subdirectory or a parent folder that contains repositories does not
erase Git identity. Similar string prefixes such as `dez` and `dez-tools` never
match. At narrow widths, the title and Git summary truncate inside their own
flexible region before they can displace Workspace controls. Hover and
accessibility descriptions retain the full root and Git details. Unrelated
machine terminals do not render.
Explicit external rows never hide the primary **Open Workspace…** path when no
Workspace is open.

The bottom Workspace status strip is evidence-driven. Its compact **Search
Workspace Files** control follows the native **Workspace Search Button** setting
and opens the same Workspace Search Surface as Command Palette. It does not add a
second search model. Healthy diagnostics do not occupy space with a decorative
checkmark; actual errors, warnings, counts, active diagnostic messages,
language health, and file context remain visible when relevant. Terminal focus
does not expand routine healthy state into another row of controls or prose.

## The core objects

You only need four concepts for everyday use:

- A **Workspace** is codebase-scoped human context: its Surfaces, pane layout,
  focus, navigation history, and repository scope.
- A **Surface** is something you can work with in the pane grid, such as a
  file, terminal, search result, debugger, settings page, Agent Session, or
  review.
- A **Session** is supervised agent work with a stable in-app identity: either
  a Built-in Agent conversation or a terminal promoted by observed agent
  evidence. An ordinary shell remains a terminal Surface, not a Session.
- **Evidence** is what Dez actually knows about work: roots, files, terminal
  working directories, commands, check outcomes, Git state, lifecycle, and
  provenance.

The longer-term model also distinguishes Hosts, Actors, Runs, Environments, and
Change Sets. Those concepts are introduced only where the source can preserve
their ownership and truth.

## How coding work flows

### Before you begin: install and relaunch when asked

Dez must run from `/Applications` before it restores a Workspace or starts its
durable Terminal Host. A temporary or DMG launch shows this requirement inline
on Home. Choose **Install and Relaunch**; there is no setup overlay or
“continue anyway” path.

### 1. Open a Workspace

A Workspace supplies one Zed-compatible Project. That Project owns language
servers, buffers, diagnostics, search, Git, tasks, debugger state, terminal
context, and Agent context.

**Open Workspace** accepts folders and keeps the current Dez window so existing
Agent Sessions stay visible. Opening a folder therefore enables one coherent IDE
scope. Files, Outline, Git, and Debug are views of the same Workspace, not
separate roots. **Open Files** always reveals and focuses Files; repeating it
does not close the destination. The Agent region uses this exact recovery
route when it needs Workspace context, so its **Open Workspace** control cannot
quietly accept a loose file or move the work into another window. Its Open and
Clone recovery actions are keyboard-reachable.

If a terminal cannot start, Dez opens no substitute process. The terminal
surface keeps its native material and presents an edge-anchored recovery state
with the launch error, one **Edit Settings** action, and a secondary settings
menu. Center-terminal and compatibility-panel launch paths use the same
hierarchy, so a failure never turns into a centered promotional card or a
different workflow.

### 2. Run or attach work in native tabs

Open a file to edit directly, choose **Open Terminal** for the configured
default, or use the adjacent **+** for a Native Shell, tmux, Codex, Claude Code,
OpenCode, or a **More Agent CLIs** entry. Each opens in a native Main Work
Area terminal tab. Use
**Built-in Agent** only when you want a provider-backed Agent Session beside the
editor.

Terminal-tab identity follows observed foreground-process evidence rather than
the launch label. Supported agents use their provider mark as soon as the
process is visible; Aider uses the native edit-agent mark, Herdr uses the
orchestration mark and reports **Herdr running**, tmux uses a split-session
mark, and ordinary shells keep the standard terminal icon. Tooltips state the
observed foreground owner, while task status icons continue to take precedence
for task terminals. cmux remains external and therefore never masquerades as a
Dez terminal tab.

Agent edits land in ordinary buffers and Git changes. A terminal starts in the
Workspace's working-directory context. Both sit beside files in the same pane
grid, so direct and delegated work can be compared rather than hidden behind
mode switches. Agent Options and New Agent Session are explicit popovers: their
triggers stay highlighted while open and announce that state to keyboard and
assistive-technology users.

Dez creates a new Built-in Agent Session only when the provider registry has a
usable default model. Until then, Workspace Options says **Configure Built-in
Agent…** and opens the relevant provider settings without manufacturing a dead
draft. A restored Built-in Agent surface that still needs setup keeps its
informational recovery visible until a provider and model are selected; it
cannot be dismissed into a blank composer. Once a usable default exists, the
action becomes **New Built-in Agent Session…**. Terminal agents do not use this
model picker.

#### Which agent surface should I use?

Start with **Open Terminal**. It opens the configured default and falls back to
the native Workspace shell. Codex, Claude Code, OpenCode, and other
terminal-native tools keep their real PTY, subscription, authentication, TUI,
commands, and plugins. Use **Built-in Agent** for a structured, provider-backed
conversation beside the editor. It is optional and requires a usable provider
and model.

The tab-strip **+** exposes native terminal launch choices: **Default
Terminal**, **Native Shell**, **Workspace tmux**, **Codex**, **Claude Code**, and
**OpenCode**, followed by **More Agent CLIs** for **Gemini CLI**, **Aider**, and
**Herdr**. Workspace Options identifies the configured default launcher and keeps
a separate native-shell choice beside the explicit providers.
The guided default is editable under **Settings → Workspaces & Terminals →
Terminal Launch → Default Terminal**. Native Shell and the supported launchers
need no command entry; **Custom Command** reveals the existing raw command field
for another TUI or wrapper. Legacy command-only configurations are inferred and
preserved. Known commands reuse the same terminal-agent vocabulary as
Workspaces, so Gemini CLI, Aider, Amp, Herdr, Pi, and other detected providers
do not fall back to anonymous **Custom Command** copy.
Provider shortcuts are per-launch choices and never rewrite that setting.
The same choices are native Command Palette actions: **Terminal: Open
Terminal**, **Open Native Shell**, **Open Workspace tmux**, **Launch Codex**,
**Launch Claude Code**, **Launch OpenCode**, **Launch Gemini CLI**, **Launch
Aider**, and **Launch Herdr**. **Open Workspace tmux** derives a
shell-safe name from the primary Workspace root and a stable root identity, then
runs `tmux new-session -A -s <workspace>-<root-id>` after the native login shell
is ready. Repositories with the same folder name therefore cannot attach to one
another. During compatibility recovery, Dez attaches an older basename-only
session only when its active pane is inside the current Workspace root; it never
renames or terminates that session. **Workspace: Open in cmux** hands the active
local Workspace to cmux without replacing or closing Dez. **Browse Running
Sessions…** opens or refocuses Workspaces, removes
temporary search and attention filters, refreshes all supported sources, and
reveals their current destinations instead of guessing which target the user
intended.
Dez waits for the configured shell startup before submitting the selected
command, so login-shell initialization, remote/WSL behavior, and native PTY
keyboard handling remain intact.

Both paths edit the same Workspace and return evidence to the same Files, Git,
diagnostics, diff, and review surfaces. Starting a terminal agent does not
require creating a Built-in Agent Session, and opening the Built-in Agent does
not wrap or replace the terminal.

An embedded **Live Preview** is planned as a normal Main Work Area Surface but
is not present in the v0.2.2 source candidate. URL actions currently open the
system browser, while Markdown, SVG, and CSV use native file previews. A real
Live Preview must own pane-scoped browser navigation without owning or
restarting the terminal's dev-server process. The platform and security gate is
documented in
[Live Preview and Agent Model](./development/dez/live-preview-and-agent-model.md).

The Agent composer is the control point for the current conversation. Its
context, follow, speed, thinking, effort, send/queue, stop, size, and sandbox
controls are keyboard-reachable and announce their action or current state.
Follow changes presentation, Add Context changes prompt input, and Sandbox
opens the applicable settings; none of them moves work into a hidden terminal
or creates another Workspace.

Response actions stay visible and keyboard-reachable: copy an Agent response,
return to its user prompt, return to the top, or submit feedback. Queued prompts
are a named ordered list rather than hidden editor state. Each row exposes
Remove, Edit, Steer, and Send Now; Steer means “interrupt at the next agent
step,” while Send Now targets that exact queued prompt.

When an Agent requests permission, Allow, Deny, Retry, and provider-supplied
choices remain keyboard-reachable. Permission Scope announces both the current
selection and whether its menu is open. The decision applies to the pending
tool call shown in that card, not globally unless the selected scope explicitly
says so.

### 3. Supervise without polling every tab

Workspaces groups supervised work by codebase:

- which Agent Sessions and detected CLI-agent terminals exist;
- whether they are running, waiting, failed, exited, saved, or unavailable;
- which Session needs attention;
- when meaningful activity last occurred; and
- what evidence is available for review.

Workspaces does not own the terminal process or Agent conversation. It routes
back to the Surface or Host Session that does.

Each Agent Session row presents one state-appropriate next action instead of a
toolbar. A running Session shows **Stop**; a nonempty draft shows **Discard**;
a completed Session with reviewable changes shows **Review**; otherwise it
shows **Brief** when observed evidence is available. Rename, archive, and the
complete action set remain available through the selected-Session commands and
context menu.

### 4. Open Files or Review Changes

Use Files and Outline to understand structure, diagnostics and Debug to inspect
behavior, Search to trace relationships, and Git to review the actual changes.
Agent Review supports interactive Keep/Reject decisions. A Review Brief is a
different Surface: it summarizes observed evidence and calls missing evidence
missing.

Changed-file rows keep Review, Reject, and Keep visible and keyboard-reachable.
Each decision targets that exact file; pending edits explain why Keep and
Reject are temporarily unavailable. Review Changes opens the interactive Agent
diff, while Keep All and Reject All remain explicit whole-review decisions.

Subagent controls distinguish stopping work from returning to the parent Agent
Session. Restoring a checkpoint is different again: because it replaces
Workspace files with their earlier content, Dez names that scope and requires
confirmation before **Restore Files** runs.

You do not need to hunt for the matching Workspace after supervising a terminal.
Its context bar provides direct **Files**, **Review Changes**, and **Session
Details** destinations. The selected Agent Session row keeps one contextual
handoff readable without becoming a toolbar: **Review** when the owning
Workspace has changes, otherwise **Details**. Rename, hook setup, Files, and the
complete action set remain in the Session context menu. Terminal handoff labels
stay visible in ordinary split panes and navigator widths before collapsing to
named icons on very narrow surfaces. Returning to the row focuses the existing
Session rather than starting another shell. A Git review destination identifies
itself in the Main Work Area as **Diff · filename**; its tooltip retains the
diff base and relative path, so switching between terminal, file, and review
never leaves a generic “Uncommitted Diff” surface.

These are destination actions, not visibility toggles. Repeating **Files**
keeps Files open and focused; repeating **Review Changes** keeps Git Changes
open and returns to the current review. Neither action closes the destination
because it was already visible.

Git Changes keeps changed-file navigation ahead of commit composition. The
inline commit editor shows four lines by default; use its full-height or modal
expansion for a longer message. View Diff, stage/unstage, commit, remote, and
split-menu controls remain keyboard reachable and announce their action and
open state.

Git History uses the same native tab hierarchy. Its tab is keyboard reachable and
announces whether it is selected. Missing-repository, loading, no-commit, and
load-failure states begin at the tab content edge with a specific title and next-step
explanation instead of switching to a generic centered label.

If a saved Session owns a closed Workspace, **Files** restores that exact
Workspace and Session before revealing Files. It does not silently
do nothing or manufacture a replacement Session.

Terminal Details also states the trust boundary. Lifecycle comes from the
terminal and Host; Git counts belong to the Workspace and are not automatically
attributed to one Session; agent confidence and checks require trusted adapter
evidence. Dez never treats arbitrary terminal text as proof that a command or
check succeeded.

Dez does not treat an agent saying “tests passed” as equivalent to an observed
command with an exit status.

Agent recovery controls name what they actually do. Retry, Authenticate,
Configure Provider, Select Model, Open Skill, environment recovery, updates,
Copy Error, and dismiss actions are keyboard-reachable. Dismissing a warning
only hides that notice; it does not pretend to repair the condition. Automatic
provider retry messages identify Dez rather than inherited Zed.

Provider data-retention consent requires a separate warning confirmation. The
dialog states that consent is saved in Dez settings, Anthropic may retain
inference logs, and the current request will be retried. **Cancel** changes
nothing; **Accept and Retry** performs the disclosed setting update and retry.

Tool cards are compact controls over real work, not opaque chat decoration.
Their expand/collapse and copy actions remain visible and keyboard reachable.
A running command has one exact **Stop This Command** action. Truncation and
failure marks report status without pretending to be buttons. A subagent card
separates previewing its work, stopping it, and opening that existing Subagent
Session in the Agent work area. It names running, waiting, completed, canceled,
or failed state to assistive navigation and offers preview only when content is
available.

Agent identity remains stable while state changes. The built-in Dez Agent uses
the four-node Dez Agent mark; its Subagents use the related parent-and-child
mark. Codex, Claude Code, OpenCode, Gemini, and other recognized terminal agents
retain provider-aware marks, including Aider's edit mark and Herdr's
orchestration mark, while hosted in a native terminal or projected from an
externally owned tmux, Herdr, or cmux Session. A draft keeps its selected Agent
mark rather than collapsing to an anonymous dot. Running, waiting, attention,
error, and completion are separate indicator treatments, so a spinner or
warning never makes unrelated agents look identical. Subagents remain scoped
to their parent Agent Session instead of becoming duplicate top-level
Workspaces rows.

### 5. Resume honestly

Workspace composition and agent-session metadata are restored where the source
owns them. If a saved current-Host terminal cannot be reconnected, Dez keeps one
honest unavailable state with its reason instead of silently starting a
replacement shell or printing fake recovery text into the PTY. An eligible
saved reference from an older Host remains a legacy Workspaces record, but its
label does not claim that the process is reachable or alive. Opening a fresh
terminal is always separate computation.

If the terminal service itself is connecting, reconnecting, or failed,
Workspaces states whether any shell started and whether running processes were
touched. A failed startup offers **Retry** without replacing running processes;
**Open Local Log** and **Copy Details/Error** expose diagnostics without putting
transport jargon in the main notice. If a Workspace cannot reopen, **Open
Recent Workspaces** retries through the normal picker; **Remove Recovery
Entry** removes only that recovery entry and keeps recent Workspace data.
These recovery actions are keyboard reachable.

## Terminal and Agent integration

Dez does not put the terminal inside chat.

- Ordinary terminals are normal Main Work Area Surfaces.
- Ordinary shells do not appear in Workspaces.
- A terminal is promoted into Workspaces when Dez detects a supported foreground
  agent or explicitly owns it as a managed agent terminal.
- Agent conversations are normal Agent Surfaces.
- The active Workspace supplies shared repository context.
- Agent edits appear in the same buffers and Git repository the developer uses.
- Structured terminal-agent adapters can add lifecycle, attention, command,
  exit, and file-target evidence without making process-name detection a source
  of truth.

Codex, Claude Code, OpenCode, and Herdr keep their real terminal interfaces.
Workspaces is only the low-noise switcher: it summarizes provider and semantic
state, then returns to the existing full-size terminal when selected. On
narrow windows Workspaces yields to the Main Work Area instead of competing
with the agent's own tmux- or Herdr-style pane layout.

Dez follows the upstream
[native terminal model](https://zed.dev/docs/terminal): terminal tabs use the
built-in emulator, the current Workspace directory, standard TUI line height,
alternate-screen scrolling, path links, search, and task integration. Optional
terminal title breadcrumbs remain off in the default Dez profile and can be
enabled through native terminal settings. cmux is not configured as the shell
because its documented [CLI](https://github.com/manaflow-ai/cmux) defines a
separate macOS workspace and terminal application. Instead, native terminal,
Continue Agent, and Built-in Agent actions lead Workspace Options. **Open
Workspace in cmux** begins the external-work group beside discovered
multiplexer Sessions. It invokes the documented `cmux open <path>` handoff,
keeps Dez open, and
then refreshes path-matched cmux activity. Existing cmux Workspaces remain
selectable beneath their associated Dez Workspace. This preserves native Dez
editing and review while making cmux a first-class opt-in owner for users who
want its `claude-teams`, `codex-teams`, or `omo` workflows.

Packaged Dez builds contain a local terminal service beside the application.
When that helper is installed, new local interactive terminals are host-owned
by default: the GUI attaches to their PTYs instead of owning their processes.
An accidental GUI close therefore leaves the shell and its agent running, and
the next Dez launch can reattach the saved Session. Dez never starts a
disposable replacement shell when recovery fails; it shows an unavailable or
reconnecting state with diagnostics instead. Source or partial installations
without the helper retain the ordinary in-process path. Task terminals remain
GUI-owned because retaining a task after the UI reports cancellation would be
dishonest, and remote terminals retain their remote owner.

A temporary helper disconnect does not change that ownership. Terminal Details
labels the Surface **Host-owned terminal · connection unavailable** until the
authenticated connection is healthy again. After an event-stream failure Dez
discards the old cursor and requires a new authoritative Session list; it
announces reconnection only after a valid attachment. **End Terminal…** keeps
the Surface attached until the Host acknowledges PTY teardown and returns an
exited snapshot. A rejected, dropped, or timed-out request leaves the terminal
visible with bounded retry guidance because Dez cannot assume which side of an
interrupted transport won the race.

Host connection, reconnection, and command cycles have deadlines and bounded
queues. A command with an uncertain outcome is never replayed, and work queued
behind a failed transport is rejected as stale instead of running later
against changed state. The GUI groups the frame-safe chunks for one user-input
batch into one bounded queue item, so queue admission accepts or rejects that
batch as a unit. It rejects a batch above the helper's four-mebibyte PTY budget
and caps aggregate queued GUI input at sixteen mebibytes. Admission is not an
end-to-end delivery guarantee: after transport or helper processing begins, a
later failure can leave a prefix delivered. Dez reports that outcome as
uncertain and never replays it. Awaited control commands have bounded enqueue
and end-to-end response deadlines. Partial PTY writes resume only after the
descriptor is writable again.

An app bundle must be installed in `/Applications` before this service starts
or Workspaces restore. A DMG, App Translocation, temporary, or user-local app
launch opens native Home with one inline **Install Dez to continue** callout
and **Install and Relaunch** action. It does not open a startup dialog, prompt,
modal, or overlay. Dez does not provide a “continue anyway” path because a
temporary process owner cannot provide durable terminal or protected-folder
identity.

The current helper uses one generated `TerminalHostEndpoint`: generation,
socket path, token-file path, and stable Host identity advance together.
Terminal-agent hooks receive the exact endpoint from their authenticated
connection, so they cannot accidentally address an inherited Zed path.
Eligible saved references from older Hosts appear as **Legacy · Access
blocked**; the label makes no claim that the process is reachable or alive.
The user may leave the record untouched, open a separate shell in its recorded
directory, or confirm **Terminate Legacy Session…** so Dez can contact only a
matching legacy owner. A failed termination attempt leaves the record and any
process untouched. Dez never claims to transfer process ownership between
Hosts.

Before a local Workspace restore, Dez enumerates each root once. A macOS
protected-folder denial becomes one **Workspace access required** notice with
the native **Grant Access…** single-folder picker. Select each exact blocked
root once. Dez validates the selected directory without opening or replacing a
Workspace; a readable root leaves the access warning while other denied roots
remain visible. Relaunch retries startup restoration, and **Open Recent
Workspaces…** remains the explicit retry if a Workspace is still missing. Git,
Workspace Search, LSP, agent, and terminal startup wait behind that preflight.
Workspace Search deduplicates permission diagnostics per root, and active
searches are cancelled during quit so denied trees cannot flood the log or hold
`app_will_quit` open.

Workspaces does not list arbitrary current-user TTYs from Terminal.app, iTerm,
Warp, another IDE, or an unrelated application. Those processes are neither
owned nor safely controllable by Dez, so they never become Workspace or Agent
Session rows.

Dez v0.2 retains explicit integration for tmux, Herdr, and cmux. Each source
updates independently and reports one truthful state:

- **Missing** — the source executable is unavailable;
- **Access required** — cmux is installed but its secure process-only API
  boundary is intentionally not sharing live activity with Dez;
- **Empty** — the source is available and authoritatively returned no sessions;
- **Failed** — discovery did not complete, with any previous rows retained as
  **last known**; or
- **Ready** — discovery returned one or more current sessions.

A failed source does not freeze or erase successful peer integrations. Each
source has a bounded command deadline shorter than the refresh interval; a
hung CLI is cancelled and becomes **Failed** rather than blocking every later
refresh. tmux is empty only after its canonical missing-server response.
Unexpected permission, protocol, Herdr registry, and Herdr snapshot errors are
failures rather than authoritative empty scans. cmux's documented process-only
refusal is **Access required** instead: it preserves last-known rows, avoids a
warning-state failure, and continues automatic discovery so an explicit access
change can recover without restarting Dez. **Retry** scans tmux, Herdr, and cmux
again without starting, attaching, selecting, or terminating any external session.
Herdr first reports its live default and named sessions through `herdr session
list --json`. Those endpoints are then queried concurrently with individual
two-second deadlines, and the complete Herdr source scan has a four-second
deadline. Results from endpoints that finish before the source deadline are
preserved; unfinished endpoints become failures so their exact last-known rows
can remain visible.
One hung server therefore cannot hide healthy Sessions reported by another or
extend the refresh indefinitely.
The explicit **Open Workspace in cmux** handoff is separately bounded to eight
seconds. A timeout leaves the Workspace unchanged and replaces the progress
notice with a persistent native toast linked to the cmux API guide. If cmux is
not installed, the same native surface offers **Get cmux** instead. Successful
handoffs use a short confirmation and do not leave another navigation surface
behind.
Last-known external rows refresh their source instead of attaching blindly;
select the refreshed row again to open it. **Retry Attach** resolves the current
session by stable ID, reports sessions that ended during refresh, and surfaces
a missing terminal provider as an attach failure rather than a successful
no-op.
Dez discovers tmux sessions through the documented CLI format, asks Herdr's JSON
session registry which endpoints are live before using its snapshot API, and
discovers cmux Workspaces through its JSON CLI. tmux and Herdr open their
documented attach command in a normal terminal Surface.
cmux Workspaces stay in cmux and open through its `select-workspace` command.
Discovery updates automatically; **Refresh Running Sessions** in a
Workspace's options menu requests an immediate scan, shows when discovery is
running, and explains when no path-matched activity exists.
Attach terminals keep the native rerun control. Failure shows an explicit
**Retry Attach** action and refreshes discovery after completion without
starting a duplicate attach automatically. Raw Herdr shells without structured
agent state are labeled **Available**, not unknown.
The external application remains authoritative; closing a Dez tab detaches,
Herdr never receives automatic takeover, and Workspaces never becomes a second
process, transcript, or layout owner.

Workspaces projects each discovered session by ownership evidence. A current
working directory inside an open root places the item beneath the most
specific matching Workspace. A session with no working directory, a directory
outside every open root, or no current Workspace match appears under **Other
Running Sessions**. Arbitrary current-user PTYs remain excluded. **Browse
Running Sessions…** switches to the Workspaces list, clears temporary search,
scope, and attention filters, expands Workspace groups that own discovered
sessions, requests a fresh scan, and focuses the navigator.

Selecting an unmatched tmux or Herdr Session never attaches it beneath an
unrelated active or remote Workspace. If the Session reports a working
directory, Dez uses the native Workspace-opening path for that directory and
attaches only after it succeeds. A pathless Session has no stronger ownership
evidence and therefore attaches in the active Workspace. cmux selection stays
external and does not create a Dez Workspace.

### External session prerequisites and troubleshooting

The empty-state copy distinguishes source availability from Workspace matching.
**Running session status is not ready** appears before any source has reported.
**Install tmux, Herdr, or cmux to browse running sessions** means every source
executable is missing. **Running session discovery needs attention** means at least one
source failed. **cmux activity sharing is off; no other running sessions** means
cmux is installed with its secure process-only boundary and no peer source has
current activity; the informational notice links to the official access guide.
**No running session matches this Workspace** means a ready
source has activity, but none belongs beneath the selected Workspace; check
**Other Running Sessions** for unmatched or pathless items. **No running tmux,
Herdr, or cmux sessions** means the available sources returned no activity. No
state means Dez adopted or ended an external process.

- tmux is discovered at `/opt/homebrew/bin/tmux`, `/usr/local/bin/tmux`, or on
  `PATH`. Start or attach to a tmux server and ensure the pane's working
  directory is inside the intended Workspace. **Open Workspace tmux** remains
  available when you want Dez to create or attach to the root-scoped session
  explicitly. A basename-only session from an older Dez build remains visible
  and is reused only when its active pane still belongs to that Workspace root.
- cmux is discovered at
  `/Applications/cmux.app/Contents/Resources/bin/cmux`,
  `/opt/homebrew/bin/cmux`, `/usr/local/bin/cmux`, or on `PATH`. Open the Workspace
  in cmux first, or use **Open Workspace in cmux**; Dez leaves the Workspace
  open if the handoff fails or times out. Live rows are optional: a secure
  process-only refusal appears as **cmux activity sharing is off** and never
  changes cmux's access mode. Use **Open API Guide** only if you deliberately
  want cross-app Workspace and notification metadata.
- Herdr requires the `herdr` CLI and at least one live session reported by
  `herdr session list --json`. Start the default or named Herdr server before
  refreshing activity. Dez follows Herdr's configured registry and does not
  assume that its sockets live under `~/.config`.

Use **Retry** in the provider warning or **Refresh Running Sessions** after
correcting a prerequisite. A **last known** row means that source's latest scan
failed and the external application may still own the session. Refresh that
source before selecting the row again; if attach fails, use **Retry Attach**
after reading the terminal diagnostic. Neither action starts a duplicate
attachment automatically.

Dez only renders metadata with an authoritative owner. Git supplies branch and
changed-file counts; terminal and multiplexer snapshots supply working
directory, provider, lifecycle, client count, and attention. cmux additionally
supplies authoritative `listening_ports`; matching cmux rows keep the first
`:port` plus a remaining count in compact, muted trailing metadata. Their
tooltip and accessibility label retain the richer bounded port list. Other
local Workspace processes do not receive inferred port labels because no
Workspace-scoped server or port-forwarding model owns that evidence yet. Dez
does not scrape terminal output or infer ports from process names.

## Visual design

Dez follows the system appearance with **Lumin Blur** and **Lumin Light**.
IBM Plex Sans gives native interface chrome a calm proportional voice. Lilex
keeps editors, terminals, prompts, and review code compact and legible. Users
can still override any role through normal settings.

Empty Main Work Area panes avoid a centered onboarding card: they use compact
top-left native chrome with **Open Terminal**, **Browse Sessions**, **Find
File**, and **Review Changes**. The Dez visual profile keeps the status bar
visible and includes active-file and line-ending context alongside the
inherited language, diagnostics, and cursor controls. Native Back and Forward
controls are visible in every Main Work Area tab strip, alongside Add, tab
overflow, and split controls. They traverse the pane's actual history of files,
terminals, settings, diffs, and other Surfaces; Dez does not add browser chrome
over the editor.

### Settings and navigation visibility

Dez keeps native editor customization, but Settings starts with the product
flow: **Workspace & Privacy**, **Workspaces & Terminals**, and **Agents**.
Within Agents, the primary terminal workflow comes first: **Terminal Agents &
Privacy**, then **Attention & Notifications**. Optional conversation features
follow under **Built-in Agent & Providers** and **Built-in Agent Behavior**.
Provider, ACP, and MCP links name the Built-in Agent surface they configure, so
they cannot be mistaken for the Codex, Claude Code, or OpenCode terminal
launchers. **Navigation & Layout** owns tabs, status-bar controls, and window
behavior. **Workspace Tools** configures Files, Outline, Git, and the optional
Built-in Agent without assigning them a separate region.
Inherited collaboration, staff-only
instrumentation, legacy dock geometry, and controls for removed sidebar chrome
stay out of the public Settings navigation.

For most users, setup is three decisions rather than a tour: choose the
**Default Terminal**, decide whether and where Workspaces should open,
and select an Agent provider only if the optional Built-in Agent is needed.
Explicit agent CLI, tmux, and cmux launch actions do not
rewrite the default terminal. Appearance recovery remains one scoped action,
not a reset of unrelated preferences.

Workspaces is optional chrome, not a permanent editor column. Fresh windows
keep it closed; the status bar exposes **Open Workspaces**, and the same toggle
remains available through **View** and Command Palette. **Settings → Workspaces
& Terminals → Show Workspaces on Startup** makes it persistent for people who
prefer that layout. **Workspaces Position** chooses its window edge. Closing
Workspaces never closes a Workspace, terminal, or Agent Session.

Workspaces intentionally omits the inherited Workspace Bar, centered Command
Search row, and duplicate project/branch identity strip. Each Workspace row
already owns its roots, Git branch, and change count; the header ellipsis opens
Command Palette and Workspace-level actions without adding another toolbar.
The compact native header still preserves the cross-platform application menu,
Restricted Mode security state, and an active multiplexer prefix indicator.

If imported settings hide that identity, **Settings → Appearance → Restore
Native Dez Appearance** restores Lumin, balanced density, IBM Plex Sans, Lilex,
the built-in Dez icons, native tab navigation, and the editor status bar while
preserving font sizes and unrelated preferences. The same recovery remains
available from **Dez → Settings → Restore Dez Visual Profile** and Command
Palette.

### Keyboard and Vim

Dez keeps Zed's native command and keymap system rather than adding a shortcut
layer. **Settings → Keyboard & Vim** can search actions, record bindings, show
conflicts, choose a base keymap, and enable native Vim or Helix editing. Vim is
optional rather than forced on new users; enabling it preserves motions, text
objects, registers, macros, marks, command mode, and Workspace-aware navigation.

The default tab model follows familiar browser behavior:

| Intent                     | macOS              | Linux and Windows               | Vim                      |
| -------------------------- | ------------------ | ------------------------------- | ------------------------ |
| Open tab 1–8 / last tab    | `⌘1`–`⌘8` / `⌘9`   | `Alt+1`–`Alt+8` / `Alt+9`       | `[b`, `]b`, `gt`, `gT`   |
| Recent-tab switcher        | `Ctrl+Tab`         | `Ctrl+Tab`                      | `[b` / `]b`              |
| Previous / next tab        | `⌘{` / `⌘}`        | `Ctrl+PageUp` / `Ctrl+PageDown` | `gT` / `gt`              |
| Move between split panes   | `⌘K`, then arrow   | `Ctrl+K`, then arrow            | `Ctrl+W`, then `h/j/k/l` |
| Workspaces                 | `⌘B`               | `Ctrl+B`                        | Command Palette          |
| Files                      | `⌘⇧E`              | `Ctrl+Shift+E`                  | `Space f`                |
| Open configured terminal   | `` Ctrl+` ``       | `` Ctrl+` ``                    | `Space t`                |
| Open native shell terminal | `` Ctrl+Shift+` `` | `` Ctrl+Shift+` ``              | `Space T`                |
| Edit shortcuts             | `⌘K ⌘S`            | `Ctrl+K Ctrl+S`                 | Command Palette          |

Number shortcuts activate tabs in the focused native pane; deliberate split
navigation remains on the pane-navigation chords. This keeps one-pane work as
simple as a browser while retaining Zed's full multi-pane model for code,
terminals, diffs, Debug, and review.

Regular Vim and Helix mode share the same native leader destinations:
`Space b` opens the recent-tab switcher, `Space f` finds a file, `Space t`
opens the configured terminal, `Space T` opens the native shell, and `Space /`
searches the Workspace. The bindings dispatch normal Zed actions, so they stay
customizable and appear in Keyboard & Vim conflict inspection.

Blur belongs to the stable window shell. On macOS the window uses the native
under-window backdrop and follows active/inactive system state; Lumin layers sit
on top of that material rather than simulating blur with opaque panels. Focus
borders, selected rows, active lines, pane boundaries, and scrollbars remain
visible. Workspaces, the Main Work Area, native tabs, and elevated menus use
distinct semantic layers instead of blending into one sheet.
Controls in Lumin Light are translucent glass layers, not beige blocks.
Error, warning, information, hidden, ignored, and predictive states use
low-alpha semantic tints in both blurred variants, so a status callout or row
never flattens the backdrop into an opaque patch. The non-blurred **Lumin**
fallback deliberately keeps opaque semantic surfaces for reduced-transparency
environments.
Structural dividers remain restrained; hover, active, selected, scrollbar, and
focus states strengthen in a consistent order, and focus/selection carry the
stronger accent. The active Main Work Area is identified by its title and
selected tab instead of a saturated rectangle around the entire pane; users who
prefer a pane border can restore it through the normal
`pane_grid.focus_indicator` setting. High-motion terminal and Agent content
does not add independent nested blur layers.

Transient feedback follows the same ownership model. Workspace notifications are
a small top shelf inside the Main Work Area, and toasts sit above the status bar
without reserving an invisible full-screen interaction layer. On glass windows,
nonblocking feedback uses lighter elevation and no modal shadow.

Agent attention stays in Workspaces instead of opening a shaped window over the
editor or terminal. Unread state, action-needed counts, sound policy, configured
notifications, and accessible window-attention requests remain independent.
Dez release builds do not add a floating Agent navigation surface.

Stable secondary windows—including About, account verification, Settings,
Audio Test, and profiling—inherit the same active Lumin window material and UI
font instead of becoming opaque or reverting to a platform-default typeface.
Retained incoming-call and project-sharing popups follow the same material
rule.

## What Dez is not

Dez is not:

- a terminal dashboard with a token editor;
- an agent chat product with a terminal attachment;
- a replacement Git database;
- a process-name guesser presented as reliable agent state;
- a second Files tree hidden in Workspaces; or
- a claim that arbitrary terminals owned by other applications survive through
  Dez.

The v0.2 product promise is a complete native IDE with one sharp wedge:
trustworthy supervision, reattachment, and review of terminal-native and
agent-driven work.

## Source-preview limits

This repository currently represents a source candidate, not a signed public
binary. A release still requires consolidated platform builds plus rendered,
restart, crash, accessibility, upgrade, and coexistence evidence.

### Recorded Preview and current source

The recorded package is a historical `0.2.0` Apple Silicon macOS Preview, not a
supported or notarized v0.2 release. It predates the current source's
install-first gate, Workspace-access recovery, generated terminal-Host endpoint,
and latest external-session reconciliation. It therefore cannot validate those
flows.

To inspect that historical navigation baseline safely:

1. Follow its exact artifact link and evidence boundary in [Release
   Evidence](./development/dez/release-evidence.md).
2. Keep the downloaded DMG beside its `SHA256SUMS.txt` and run
   `shasum -a 256 -c SHA256SUMS.txt` from that directory.
3. Open the DMG, drag **Dez Preview.app** into `/Applications`, eject the DMG,
   and launch the installed copy.
4. Attribute observations only to that package and commit. Do not treat them as
   proof of the newer source tree.

For the current source, the intended first-run path is install and relaunch,
grant a selected Workspace root when macOS requires it, then use **Open
Terminal** or the Main Work Area **+** to start a shell or agent CLI. Those
steps remain source claims until an exact newer package records runtime proof.

For precise implementation state, read:

- [Fork Notes](./development/dez/fork-notes.md)
- [v0.2 Workspace Polish](./development/dez/v0.2-workspace-polish.md)
- [v0.1 Product Hardening](./development/dez/v0.1-product-hardening.md)
- [v0.0.4 External Sessions](./development/dez/v0.0.4-external-sessions.md)
- [Architecture Baseline](./development/dez/architecture-baseline.md)
- [Roadmap](./development/dez/roadmap.md)
- [Release Evidence](./development/dez/release-evidence.md)
