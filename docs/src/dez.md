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

The default interface has four named regions:

| Region              | What it is for                                                                   |
| ------------------- | -------------------------------------------------------------------------------- |
| **Projects**        | Navigate codebases and supervise their detected or managed Agent Sessions        |
| **Workspace Tools** | Inspect Files, Outline, Git, and Debug for the active Workspace                  |
| **Main Work Area**  | Edit files and open terminal, search, settings, diagnostics, and review Surfaces |
| **Built-in Agent**  | Use the optional provider-backed conversation surface                            |

These are not four separate products.

The Main Work Area is one pane grid. A file, terminal, diff, search result,
settings page, or review can be tabbed, split, moved, and focused through the
same rules. Sessions, Workspace Tools, and Built-in Agent share one optional
auxiliary slot around that grid; they are never simultaneous permanent
columns. Opening one closes the visible competitor at laptop, desktop, and
ultrawide sizes. A
contextual drawer is capped at 22% of visible horizontal space when room
permits, and Dez keeps at least 60% for the Main Work Area. The same policy
applies after resizing, reopening, and restoring a saved layout.
Returning to a one-work-area recipe removes surplus empty split panes while
preserving every pane that contains a file, terminal, or other user Surface.
Workspace restoration applies that same cleanup to the default layout and all
six public recipes, so stale empty splits cannot return as unexplained blank
columns. Public multi-surface recipes arrange up to two populated work
areas and never create an empty pane merely to satisfy a diagram.
The public **Workspace Layout** picker names destinations instead of abstract
arrangements: **Work Area + Files**, **Work Area + Built-in Agent**, **Focus
Work Area**, **Split Work Area**, **Work Area + Git**, and **Work Area +
Debug**. Files, Git, Debug, and Built-in Agent each select that exact native
surface. Focus closes every auxiliary surface. Split Work Area only arranges
existing Main Work Area surfaces; it never starts a process or opens an
unrelated tool. The three multi-surface recipes use a second work area only
when it already contains user work, so a workflow can never open as a grid of
unexplained blank columns. If a layout's named native panel is not registered,
Dez collapses that empty auxiliary surface and keeps focus in the Main Work
Area; it never substitutes a stale tab or presents a dead drawer.
**Next Workspace Layout** advances through those same six states in that
listed order. It does not enter inherited matrix, tiled, or implementation
layouts, and invoking it after a custom or legacy state rejoins the public
sequence at **Work Area + Files**. Workspace menus and command search use the
same name.
When that work area is empty, one restrained launch panel states the product
purpose and offers only Open Agent Terminal, Find File, and New File. It does
not repeat Home's workflow diagram. It is an operational start state for the
current Workspace, not a second Home screen.
The **Projects** rail is a projection over the real owners. Each open codebase
remains visible even before an agent starts; its Agent Sessions appear beneath
it as they are detected or managed. Selecting a Session focuses or reattaches
its existing Surface instead of opening a duplicate. Selecting a Project
activates that codebase; its dedicated disclosure control folds or expands its
Sessions. An ordinary shell is not an Agent Session and does not create a
generic row.
Each Main Work Area pane keeps native Back and Forward controls in its tab
bar. They traverse files, terminals, diffs, settings, and other native
Surfaces without inventing a separate browser or duplicating Project
navigation.
Home and Terminal Details use that same name so **Supervise** never implies
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
exposes `sessions: ...`, `files: ...`, `git: ...`, and
`workspace tools: ...` display names instead of inherited implementation
namespaces. Terminal creation appears under `terminal: open agent terminal …`,
and the six layout transitions and their management commands appear under
`layout: ...`; neither route exposes inherited Thread or Canvas terminology.

Hierarchy follows the next useful action. Home places **Open Workspace**
first when no codebase is open, or **Open Agent Terminal** first inside an
active Workspace. These are native command rows on the editor surface, not
filled dashboard cards. Dense Workspace Tools and Built-in Agent toolbars use
compact icons, but every control has a specific accessible name, tooltip, and
place in the keyboard tab order. A critical action is never available only on
pointer hover.

First run follows the same rule. Setup is a top-anchored editor page for theme,
keymap, optional Agent providers, imports, and trust preferences. A short
in-flow list explains the Run, Supervise, Review loop. It does not open a
pathless terminal, expose hook installation, or place a promotional card over
the Workspace. **Finish Setup** returns to the normal Workspace activation
flow.

Home keeps that first choice concrete. Without a Workspace, its start
actions are **Open Workspace** and **Clone Repository**. Dez does not offer an
Agent Terminal until a codebase can supply file and Git review context. Inside
a Workspace, the actions become **Open Agent Terminal**, **Open Files**, and
**New File**. The terminal action opens a normal integrated terminal; you then
start a supported agent CLI. The terminal enters Sessions only after agent
evidence exists.

Home is a normal Main Work Area surface, not a modal dashboard. It always
teaches the compact **Run → Supervise → Review** route, stacks that route below
760 px, and names its tab **Home**. Recent Workspaces reserve a stable native
section while local history loads, state clearly when no history exists, and
become ordinary keyboard-reachable rows when ready. The section never appears
as a floating card or repaints the Lumin window material.

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
Split, Zoom, and the Workspace Tools/Built-in Agent hide controls are
keyboard-focusable and specifically named. Files, Git, Outline, and Debug are
persistent Workspace Tool destinations: each has one icon, is keyboard
reachable, and does not repeat a close button or editor lifecycle menu beside
**Hide Workspace Tools**. Tool tabs stay in their dedicated strip rather than
dragging into the Main Work Area. The active unpinned Main Work Area Surface
keeps its close control visible in Dez; inactive tabs remain visually quiet.

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

Projects uses one native titlebar label and one options menu for secondary
destinations such as Agent Tools, Agent History, and Recent Workspaces. It does
not repeat **Projects** inside a dashboard header or reserve a permanent
footer. Empty, caught-up, search, and recovery states stay in normal sidebar
flow and never become floating overlays. At compact widths, the caught-up
action shortens to **Show All** and the **Machine Terminals** count shortens to
`n observed`; the section description, row state, tooltip, and accessibility
label retain the read-only ownership boundary. Observed machine terminals never
hide the primary **Open Workspace…** path when no Project is open.

The bottom Workspace status strip is evidence-driven. It does not repeat a
global Search launcher or show a decorative checkmark when diagnostics are
healthy. Search remains available through normal Workspace navigation and the
Command Palette. Actual errors, warnings, counts, active diagnostic messages,
language health, and file context remain visible when relevant. Terminal focus
does not expand routine healthy state into another row of controls or prose.

## The core objects

You only need four concepts for everyday use:

- A **Workspace** is project-scoped human context: its Surfaces, pane layout,
  focus, navigation history, and Project scope.
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
scope. Files, Outline, Git, and Debug are views of the same Project, not
separate roots. **Open Files** always reveals and focuses Files; repeating it
does not close the destination. The Agent region uses this exact recovery
route when it needs Project context, so its **Open Workspace** control cannot
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

Git History uses the same drawer hierarchy. Its tab is keyboard reachable and
announces whether it is selected. Missing-repository, loading, no-commit, and
load-failure states begin at the drawer edge with a specific title and next-step
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
owns them. If a saved terminal cannot be reconnected, Dez preserves its title
and displays one **Terminal unavailable** warning. It does not silently start a
replacement shell or print fake recovery text into the terminal grid.

**Start Fresh Terminal** creates separate computation in the Main Work Area; it
does not claim to reconnect, replay, or replace the unavailable Session.

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
- Ordinary shells do not appear in Sessions.
- A terminal is promoted into Sessions when Dez detects a supported foreground
  agent or explicitly owns it as a managed agent terminal.
- Agent conversations are normal Agent Surfaces.
- The active Workspace supplies shared Project context.
- Agent edits appear in the same buffers and Git repository the developer uses.
- Structured terminal-agent adapters can add lifecycle, attention, command,
  exit, and file-target evidence without making process-name detection a source
  of truth.

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

On macOS, Projects can list current-user TTYs from another supported terminal
application or IDE under **Machine Terminals**. These rows are ephemeral, read-only
observations. Dez can show limited process context and reveal the owning
application, but it does not read the transcript or arguments, intercept input,
persist the row, adopt the PTY, restore the process, or claim ownership of its
work. External control requires an explicit adapter—such as a future `tmux`
integration—and is outside the v0.0.2 contract.

## Visual design

Dez follows the system appearance with **Lumin Blur** and **Lumin Light**.
IBM Plex Sans gives native interface chrome and onboarding a calm proportional
voice. Lilex keeps editors, terminals, prompts, and review code compact and
legible. Users can still override any role through normal settings.

If imported settings hide that identity, **Dez → Settings → Restore Dez Visual
Profile** restores only Lumin, IBM Plex Sans, Lilex, and the built-in Dez icons
while preserving font sizes and non-visual preferences.

Blur belongs to the stable window shell. On macOS the window uses the native
under-window backdrop and follows active/inactive system state; Lumin layers sit
on top of that material rather than simulating blur with opaque panels. Focus
borders, selected rows, active lines, pane boundaries, and scrollbars remain
visible. Projects, Workspace Tools, Agent, the Main Work Area, tab strips, and
elevated menus use distinct semantic layers instead of blending into one sheet.
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

Agent attention stays in Sessions by default instead of opening a shaped window
over the editor or terminal. Unread state, action-needed counts, sound policy,
and accessible window-attention requests remain independent; users who want
cross-window alerts can enable **Floating Attention Popups** under
**Settings → Attention**.

Stable secondary windows—including About, account verification, Settings,
Audio Test, and profiling—inherit the same active Lumin window material and UI
font instead of becoming opaque or reverting to a platform-default typeface.
When explicitly enabled, shaped Agent notification popups stay transparent
around their own surface and use the configured Dez UI font. Retained incoming
call and project-sharing popups follow the same material rule.

## What Dez is not

Dez is not:

- a terminal dashboard with a token editor;
- an agent chat product with a terminal attachment;
- a replacement Git database;
- a process-name guesser presented as reliable agent state;
- a second project tree hidden in Projects; or
- a claim that arbitrary terminals owned by other applications survive through
  Dez.

The v0.0.2 goal is a complete native IDE with one sharp wedge: trustworthy
supervision and review of terminal-native and agent-driven work.

## Source-preview limits

This repository currently represents a source candidate, not a signed public
binary. A release still requires consolidated platform builds plus rendered,
restart, crash, accessibility, upgrade, and coexistence evidence.

For precise implementation state, read:

- [Fork Notes](./development/dez/fork-notes.md)
- [v0.0.2 Active Plan](./development/dez/v0.0.2-active-plan.md)
- [Architecture Baseline](./development/dez/architecture-baseline.md)
- [Roadmap](./development/dez/roadmap.md)
- [Release Evidence](./development/dez/release-evidence.md)
