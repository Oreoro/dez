---
title: What is Dez?
description: Learn how Dez combines a native IDE, integrated terminals, coding agents, and review evidence in one Workspace.
---

# What is Dez?

Dez is a native development environment for people who edit code themselves
and supervise coding agents running in integrated terminals.

Its job is simple to state:

> Keep a project's code, active agent work, attention, changes, and review
> evidence visible in one native Workspace.

Dez inherits Zed's fast editor, language support, Git, debugger, tasks, remote
infrastructure, collaboration substrate, and agent ecosystem. It changes the
product model around those capabilities so a developer can move between direct
editing and delegated work without reconstructing context.

## The screen model

The default interface has two persistent ownership regions:

| Region              | What it is for                                                                                                 |
| ------------------- | -------------------------------------------------------------------------------------------------------------- |
| **Workspaces** · left | Optionally switch codebases and supervise their terminal and Built-in Agent Sessions                         |
| **Main Work Area**  | Edit files and use terminal, Files, Outline, Git, Debug, Built-in Agent, settings, diagnostics, and review tabs |

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

The tab-strip **+** is the single **Add to Main Work Area** control. Its
**Open Terminal** submenu launches the configured agent command, a native
shell, a Workspace-named tmux session, Codex, Claude Code, or OpenCode. It also
opens the optional Built-in Agent, a file, Files, Git, Debug, Workspace search,
or symbol search. **Browse tmux, Herdr & cmux…** focuses Workspaces, where each
discovered external session remains attached to its owning Workspace.
**Open Workspace in cmux** performs the explicit external handoff. Every
destination is the existing Zed Surface in the same pane system, so it can be
focused and arranged without creating a nested panel or a second navigation
model. The tab strip and Add control remain visible on an empty Main Work
Area, including first-run Home.

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
Debug**. They live in **View** and Command Search rather than occupying the
default titlebar. Files, Git, Debug, and Built-in Agent each select that exact
native surface. Focus closes contextual tools. Split Work Area only arranges
existing Main Work Area surfaces; it never starts a process or opens an
unrelated tool. No recipe may manufacture an empty column.
When that work area is empty, one restrained launch panel states the product
purpose and offers only Open Agent Terminal, Find File, and New File. It does
not repeat Home's product summary. It is an operational start state for the
current Workspace, not a second Home screen.
**Workspaces** is a projection over the real owners. Each open codebase
remains visible even before an agent starts; its Agent Sessions appear beneath
it as they are detected or managed. Selecting a Session focuses or reattaches
its existing Surface instead of opening a duplicate. Selecting a Workspace
activates that codebase; its dedicated disclosure control folds or expands its
Sessions. An ordinary shell is not an Agent Session and does not create a
generic row. Provider icons identify Codex, Claude Code, OpenCode, Herdr, and
other detected agents. Native status treatments distinguish Running, Needs
Input, Waiting for Permission, Reconnecting, Completed, and Error without
rendering a second terminal transcript.
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
| Session list | Supervise Sessions and attention                   |
| Clock        | Open Agent Session history                         |
| Settings     | Configure Agent tools and application behavior     |
| Robot        | Create or identify a Dez Agent Session             |
| Sparkle      | Invoke Inline Assist on the current editor context |

Icons support labels; they never replace them. A creation icon does not stand
in for an object-specific Terminal or File icon, and review/details controls
do not reuse each other's symbols. Dez does not reuse Zed-branded assistant
marks for these controls. Command Palette labels follow the same grammar: Dez
exposes `projects: ...`, `files: ...`, `git: ...`, and
`navigator: ...` display names instead of inherited implementation
namespaces. Terminal creation appears under `terminal: open agent terminal …`,
and the six layout transitions and their management commands appear under
`layout: ...`; neither route exposes inherited Thread or Canvas terminology.

Hierarchy follows the next useful action. Home places **Open Workspace**
first when no codebase is open, or **Open Agent Terminal** first inside an
active Workspace. These are native command rows on the editor surface, not
filled dashboard cards. Dense Files & Git and Built-in Agent toolbars use
compact icons, but every control has a specific accessible name, tooltip, and
place in the keyboard tab order. A critical action is never available only on
pointer hover.

First run follows the same rule. Dez opens its native Home/launchpad rather
than a separate setup page, modal tour, floating card, or restored split.
Theme, keymap, optional Agent providers, imports, and trust preferences stay in
native Settings. The launchpad does not touch a previous Workspace folder
until the user opens it, avoiding a startup privacy prompt for a stale recent
path. A stable signed release can retain the resulting macOS folder grant;
ad-hoc development snapshots may be asked again after their code identity
changes.

Home keeps that first choice concrete. Without a Workspace, its start
actions are **Open Workspace** and **Clone Repository**. Dez does not offer an
Agent Terminal until a codebase can supply file and Git review context. Inside
a Workspace, the actions become **Open Agent Terminal**, **Open Files**, and
**New File**. The terminal action opens a normal integrated terminal; you then
start a supported agent CLI. The terminal enters Sessions only after agent
evidence exists.

Home is a normal, top-anchored Main Work Area surface, not a modal dashboard or
persistent walkthrough. One sentence states the terminal → Workspaces → review
loop, then native command rows own every action. It names its explicit tab
**Home**. Recent Workspaces reserve a stable native section while local history
loads, state clearly when no history exists, and become ordinary
keyboard-reachable rows when ready. Multi-root rows lead with the first root
and a root count while their full paths remain available as metadata. The
section never appears as a floating card or repaints the Lumin window material.

The empty Sessions region follows the same activation loop. **No Workspace
open** explains that a codebase supplies context to agent work and review.
**Open Workspace** is its only start action. Once a Workspace is ready, the
scoped action becomes **Open Agent Terminal**. Sessions stays empty until agent
evidence exists. Start, recovery, and All/Attention scope actions remain
keyboard reachable as the rail changes state.

Workspace controls follow focus. Selecting or keyboard-focusing a Workspace
keeps its Open Agent Terminal and Options actions visible; opening the
Options menu keeps its scoped close controls visible as well. Search clearing
and banner dismissal are keyboard-focusable, so pointer hover is never the only
route to a visible shell action. Workspace names and their action cluster share
one bounded inline row: text truncates within its allocation, actions never
overlap it, and no gradient mask is painted over either side of the header. An
expanded Workspace with no Sessions shows one labeled **Open Agent Terminal**
action below the header; its compact terminal icon is suppressed
until the Workspace is collapsed or contains Sessions. Readiness remains in the
overview summary and the Workspace header's accessible name instead of being
repeated as a decorative dot-and-caption row.

The Main Work Area follows the same rule. Back, Forward, Add, Switch Surface,
Split, Zoom, and the Workspace Tools hide controls are keyboard-focusable and
specifically named. Files, Git, Outline, and Debug use the contextual right
pane; each has one icon, is keyboard reachable, and does not repeat a close
button or editor lifecycle menu. Work tabs remain native and draggable. The
active unpinned Main Work Area Surface keeps its close control visible in Dez;
inactive tabs remain visually quiet.

Existing generated Dez profiles are upgraded consistently. A known legacy
profile that pinned `.ZedSans`, One Light, and light-only appearance migrates
to IBM Plex Sans for interface chrome, Lilex for code and terminals, and
system-selected Lumin Light/Lumin Blur. Custom font or theme choices are not
treated as generated defaults.

Creation emphasis also follows state. A ready Workspace without a Session
shows one quiet **Open Agent Terminal** command row. Once work exists, a
compact terminal action remains in each Workspace header while the Sessions
overview stays focused on status and All/Attention scope. Dez does not repeat a
global launcher for the already-active Workspace.

Workspaces uses one native titlebar label and one options menu for secondary
destinations such as Agent Tools, Agent History, and Recent Workspaces. It does
not repeat **Workspaces** inside a dashboard header or reserve a permanent
footer. Empty, caught-up, search, and recovery states stay in normal sidebar
flow and never become floating overlays. An empty Workspace keeps one compact
terminal action in its header instead of expanding into a provider onboarding
block. At compact widths, the caught-up action shortens to **Show All** without
losing source or status in each row. Explicit tmux and Herdr rows attach in the
Main Work Area; cmux Workspace rows open in cmux. Each row sits beneath the
most specific matching Workspace and uses its secondary metadata row for
source, semantic state, working directory or worktree, and attention. The
Workspace header owns Git branch and changed-file metadata. Unrelated machine
terminals do not render.
Explicit external rows never hide the primary **Open Workspace…** path when no
Workspace is open.

The bottom Workspace status strip is evidence-driven. It does not repeat a
global Search launcher or show a decorative checkmark when diagnostics are
healthy. Search remains available through normal Workspace navigation and the
Command Palette. Actual errors, warnings, counts, active diagnostic messages,
language health, and file context remain visible when relevant. Terminal focus
does not expand routine healthy state into another row of controls or prose.

## The core objects

You only need four concepts for everyday use:

- A **Workspace** is project-scoped human context: its Surfaces, pane layout,
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

### 1. Open a Workspace

A Workspace supplies one Zed-compatible Project. That Project owns language
servers, buffers, diagnostics, search, Git, tasks, debugger state, terminal
context, and Agent context.

**Open Workspace** accepts folders and keeps the current Dez window so existing
Sessions stay visible. Opening a folder therefore enables one coherent IDE
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

### 2. Work directly or delegate

Open a file to edit directly, choose **Open Agent Terminal** to run Codex,
Claude Code, or another terminal agent in the Main Work Area, or start a
provider-backed Agent Session in **Built-in Agent**.

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

Start with **Open Agent Terminal**. It is the durable default for Codex, Claude
Code, OpenCode, and other terminal-native tools because their real PTY,
subscription, authentication, TUI, commands, and plugins remain intact. Use
**Built-in Agent** when you specifically want a structured, provider-backed
conversation for planning, Workspace questions, edits, or tool calls beside the
editor. It is optional and requires a usable provider and model.

Workspace Options and the tab-strip **+** expose native terminal launch
choices: **Configured Agent**, **Native Shell**, **tmux Workspace**,
**Codex**, **Claude Code**, and **OpenCode**.
The default command is editable under **Settings → Agents →
Default Agent Terminal Command**; leaving it blank keeps a normal shell.
Provider shortcuts are per-launch choices and never rewrite that setting.
The same choices are native Command Search actions: **Terminal: Open Agent
Terminal**, **Open Shell**, **Open tmux Workspace**, **Launch Codex**,
**Launch Claude Code**, and **Launch OpenCode**. **Open tmux Workspace** runs
`tmux new-session -A -s <workspace>` after the native login shell is ready, so
each codebase gets a stable attach-or-create destination. **Workspace: Open in
cmux** hands the active local Workspace to cmux without replacing or closing
Dez. **Browse tmux, Herdr & cmux…** opens Workspaces for an exact discovered
session instead of guessing which target the user intended.
Dez waits for the configured shell startup before submitting the selected
command, so login-shell initialization, remote/WSL behavior, and native PTY
keyboard handling remain intact.

Both paths edit the same Workspace and return evidence to the same Files, Git,
diagnostics, diff, and review surfaces. Starting a terminal agent does not
require creating a Built-in Agent Session, and opening the Built-in Agent does
not wrap or replace the terminal.

An embedded **Live Preview** is planned as a normal Main Work Area Surface but
is not present in the current candidate. URL actions currently open the system
browser, while Markdown, SVG, and CSV use native file previews. A real Live
Preview must own pane-scoped browser navigation without owning or restarting
the terminal's dev-server process. The platform and security gate is documented
in [Live Preview and Agent Model](./development/dez/live-preview-and-agent-model.md).

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

Sessions groups work by Workspace and projects:

- which Agent Sessions and detected CLI-agent terminals exist;
- whether they are running, waiting, failed, exited, saved, or unavailable;
- which Session needs attention;
- when meaningful activity last occurred; and
- what evidence is available for review.

Sessions does not own the terminal process or Agent conversation. It routes
back to the Surface or Host Session that does.

Each Agent Session row presents one state-appropriate next action instead of a
toolbar. A running Session shows **Stop**; a nonempty draft shows **Discard**;
a completed Session with reviewable changes shows **Review**; otherwise it
shows **Brief** when observed evidence is available. Rename, archive, and the
complete action set remain available through the selected-Session commands and
context menu.

### 4. Review with the IDE

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

You do not need to hunt for the matching project after supervising a terminal.
Its context bar provides direct **Files**, **Review Changes**, and **Session
Details** destinations. The selected Sessions row keeps one contextual handoff
readable without becoming a toolbar: **Review** when the owning Workspace has
changes, otherwise **Details**. Rename, hook setup, Files, and the complete
action set remain in the Session context menu. Terminal handoff labels stay
visible in ordinary split panes and rail widths before collapsing to named
icons on very narrow surfaces. Returning to the row focuses the existing
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
Workspace and Session before revealing the project tree. It does not silently
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
Session in the Agent work area.

### 5. Resume honestly

Workspace composition and agent-session metadata are restored where the source
owns them. If a saved terminal cannot be reconnected, Dez drops that stale
restored tab instead of silently starting a replacement shell, printing fake
recovery text, or filling the Main Work Area with unavailable placeholders. A
fresh terminal is always separate computation.

If the terminal service itself is connecting, reconnecting, or failed, Sessions
states whether any shell started and whether running processes were touched.
**Open Local Log** and **Copy Details/Error** expose diagnostics without putting
transport jargon in the main notice. If a Workspace cannot reopen, **Open Recent
Workspaces** retries through the normal picker; **Remove Recovery Entry**
removes only that rail record and keeps recent Workspace data.
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
alternate-screen scrolling, title breadcrumbs, path links, search, and task
integration. cmux is not configured as the shell because its documented
[CLI contract](https://github.com/manaflow-ai/cmux/blob/main/docs/cli-contract.md)
defines a separate macOS workspace and terminal application. Instead,
**Open Workspace in cmux** is the first external-workspace action in Workspace
Options. It invokes the documented `cmux <path>` handoff, keeps Dez open, and
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

An app bundle must be installed in `/Applications` before this service starts
or Workspaces restore. A DMG, App Translocation, temporary, or user-local app
launch opens native Home and the operating-system prompt for **Install and
Relaunch**. Dez does not provide a “continue anyway” path because a temporary
process owner cannot provide durable terminal or protected-folder identity.

The current helper uses one generated `TerminalHostEndpoint`: generation,
socket path, token-file path, and stable Host identity advance together.
Terminal-agent hooks receive the exact endpoint from their authenticated
connection, so they cannot accidentally address an inherited Zed path.
A helper left alive by an older build remains the owner of its shells.
Workspaces labels those records **Legacy · Access blocked** and offers three
honest outcomes: open a new shell in the recorded directory, keep the legacy
process running, or explicitly confirm **Terminate Legacy Session…**. Dez
never claims to transfer process ownership between Hosts.

Before a local Workspace restore, Dez enumerates each root once. A macOS
protected-folder denial becomes one **Workspace access required** notice with
the native **Grant Access…** folder picker. Git, Workspace Search, LSP, agent,
and terminal startup wait behind that preflight. Workspace Search deduplicates
permission diagnostics per root, and active searches are cancelled during
quit so denied trees cannot flood the log or hold `app_will_quit` open.

Workspaces does not list arbitrary current-user TTYs from Terminal.app, iTerm,
Warp, another IDE, or an unrelated application. Those processes are neither
owned nor safely controllable by Dez, so they never become Workspace or Agent
Session rows.

Dez v0.1 retains explicit project-scoped integration for tmux, Herdr, and cmux.
Dez discovers tmux sessions through the documented CLI format, Herdr panes
through the local snapshot API, and cmux Workspaces through its JSON CLI. tmux
and Herdr open their documented attach command in a normal terminal Surface.
cmux Workspaces stay in cmux and open through its `select-workspace` command.
Discovery updates automatically; **Refresh External Activity** in a
Workspace's options menu requests an immediate scan, shows when discovery is
running, and explains when no path-matched activity exists.
Attach terminals keep the native rerun control. Failure shows an explicit
**Retry Attach** action and refreshes discovery after completion without
starting a duplicate attach automatically. Raw Herdr shells without structured
agent state are labeled **Available**, not unknown.
The external application remains authoritative; closing a Dez tab detaches,
Herdr never receives automatic takeover, and Workspaces never becomes a second
process, transcript, or layout owner.

Dez only renders metadata with an authoritative owner. Git supplies branch and
changed-file counts; terminal and multiplexer snapshots supply working
directory, provider, lifecycle, client count, and attention. The current
candidate does not advertise port badges because no Workspace-scoped local
server or port-forwarding model is wired into Workspaces yet. It does not
scrape terminal output or infer ports from process names.

## Visual design

Dez follows the system appearance with **Lumin Blur** and **Lumin Light**.
IBM Plex Sans gives native interface chrome a calm proportional voice. Lilex
keeps editors, terminals, prompts, and review code compact and legible. Users
can still override any role through normal settings.

Empty Main Work Area panes avoid a centered onboarding card: they use compact
top-left native chrome with direct terminal, file-finder, and new-file actions.
The Dez visual profile keeps the status bar visible and includes active-file
and line-ending context alongside the inherited language, diagnostics, and
cursor controls. Native Back and Forward controls are visible in every Main
Work Area tab strip, alongside Add, tab overflow, and split controls. They
traverse the pane's actual history of files, terminals, settings, diffs, and
other Surfaces; Dez does not add browser chrome over the editor.

### Settings and navigation visibility

Dez keeps native editor customization, but Settings starts with the product
flow: **Workspace & Privacy**, **Workspaces & Terminals**, **Agents**,
**Attention**, and **Evidence**. **Navigation & Layout** owns tabs, status-bar
controls, and window behavior. **Workspace Tools** configures Files, Outline,
Git, and the optional Built-in Agent without assigning them a separate region.
Inherited collaboration, staff-only
instrumentation, legacy dock geometry, and controls for removed sidebar chrome
stay out of the public Settings navigation.

Workspaces is optional chrome, not a permanent editor column. Fresh windows
keep it closed; the status bar exposes **Open Workspaces**, and the same toggle
remains available through **View** and Command Search. **Settings → Workspaces
& Terminals → Open Workspaces on Startup** makes it persistent for people who
prefer that layout. Closing Workspaces never closes a Workspace, terminal, or
Agent Session.

If imported settings hide that identity, **Settings → Appearance → Restore
Native Dez Appearance** restores Lumin, balanced density, IBM Plex Sans, Lilex,
the built-in Dez icons, native tab navigation, and the editor status bar while
preserving font sizes and unrelated preferences. The same recovery remains
available from **Dez → Settings → Restore Dez Visual Profile** and Command
Search.

### Keyboard and Vim

Dez keeps Zed's native command and keymap system rather than adding a shortcut
layer. **Settings → Keyboard & Vim** can search actions, record bindings, show
conflicts, choose a base keymap, and enable native Vim or Helix editing. Vim is
optional rather than forced on new users; enabling it preserves motions, text
objects, registers, macros, marks, command mode, and Workspace-aware navigation.

The default tab model follows familiar browser behavior:

| Intent | macOS | Linux and Windows | Vim |
| --- | --- | --- | --- |
| Open tab 1–8 / last tab | `⌘1`–`⌘8` / `⌘9` | `Alt+1`–`Alt+8` / `Alt+9` | `[b`, `]b`, `gt`, `gT` |
| Recent-tab switcher | `Ctrl+Tab` | `Ctrl+Tab` | `[b` / `]b` |
| Previous / next tab | `⌘{` / `⌘}` | `Ctrl+PageUp` / `Ctrl+PageDown` | `gT` / `gt` |
| Move between split panes | `⌘K`, then arrow | `Ctrl+K`, then arrow | `Ctrl+W`, then `h/j/k/l` |
| Workspaces | `⌘B` | `Ctrl+B` | Command Search |
| Files | `⌘⇧E` | `Ctrl+Shift+E` | `Space f` |
| Open configured agent terminal | `` Ctrl+` `` | `` Ctrl+` `` | `Space t` |
| Open shell terminal | `` Ctrl+Shift+` `` | `` Ctrl+Shift+` `` | `Space T` |
| Edit shortcuts | `⌘K ⌘S` | `Ctrl+K Ctrl+S` | Command Search |

Number shortcuts activate tabs in the focused native pane; deliberate split
navigation remains on the pane-navigation chords. This keeps one-pane work as
simple as a browser while retaining Zed's full multi-pane model for code,
terminals, diffs, Debug, and review.

Regular Vim and Helix mode share the same native leader destinations:
`Space b` opens the recent-tab switcher, `Space f` finds a file, `Space t`
opens the configured agent terminal, `Space T` opens a shell, and `Space /`
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
- a second project tree hidden in Workspaces; or
- a claim that arbitrary terminals owned by other applications survive through
  Dez.

The v0.1 product promise is a complete native IDE with one sharp wedge:
trustworthy supervision, reattachment, and review of terminal-native and
agent-driven work.

## Source-preview limits

This repository currently represents a source candidate, not a signed public
binary. A release still requires consolidated platform builds plus rendered,
restart, crash, accessibility, upgrade, and coexistence evidence.

For precise implementation state, read:

- [Fork Notes](./development/dez/fork-notes.md)
- [v0.2 Workspace Polish](./development/dez/v0.2-workspace-polish.md)
- [v0.1 Product Hardening](./development/dez/v0.1-product-hardening.md)
- [v0.0.4 External Sessions](./development/dez/v0.0.4-external-sessions.md)
- [Architecture Baseline](./development/dez/architecture-baseline.md)
- [Roadmap](./development/dez/roadmap.md)
- [Release Evidence](./development/dez/release-evidence.md)
