---
title: What is Dez?
description: Learn how Dez combines a native IDE, terminal Sessions, coding agents, and review evidence in one Workspace.
---

# What is Dez?

Dez is a native development environment for people who edit code themselves
and supervise coding agents or long-running terminal work.

Its job is simple to state:

> Keep the code, computation, attention, and evidence for a piece of work
> visible in one place.

Dez inherits Zed's fast editor, language support, Git, debugger, tasks, remote
infrastructure, collaboration substrate, and agent ecosystem. It changes the
product model around those capabilities so a developer can move between direct
editing and delegated work without reconstructing context.

## The screen model

The default interface has four named regions:

| Region              | What it is for                                                                   |
| ------------------- | -------------------------------------------------------------------------------- |
| **Sessions**        | Navigate Workspaces and Sessions; see lifecycle, attention, and recent activity  |
| **Workspace Tools** | Inspect Files, Outline, Git, and Debug for the active Workspace                  |
| **Main Work Area**  | Edit files and open terminal, search, settings, diagnostics, and review Surfaces |
| **Agent**           | Work with native or external coding-agent conversations                          |

These are not four separate products.

The Main Work Area is one pane grid. A file, terminal, diff, search result,
settings page, or review can be tabbed, split, moved, and focused through the
same rules. Workspace Tools and Agent are hideable drawers around that grid,
not permanent columns. On laptop and portrait windows, opening one closes the
other. Both may coexist only in an ultrawide shell, where each is capped at 22%
of visible horizontal space and Dez keeps at least 60% for the Main Work Area.
The same policy applies after resizing, reopening, and restoring a saved layout.
Returning to a one-work-area recipe removes surplus empty split panes while
preserving every pane that contains a file, terminal, or other user Surface.
When that work area is empty, one restrained launch panel states the product
loop—**Run. Supervise. Review.**—and offers only Start Terminal Session, Find
File, and New File. A compact route row names where the loop happens: Run in
the Main Work Area, Supervise in Sessions, and Review in Files + Git. It is
onboarding for the current Workspace, not a second welcome screen.
Sessions is a projection over the real owners: selecting a row focuses or
reattaches its existing Surface instead of opening a duplicate. Its true-empty
state uses the same route language—Run in the Main Work Area, Supervise in
Sessions, Review in Files + Git—so the rail explains its job before any
Terminal or Agent Session exists.
Notifications and toasts are bounded transient shelves over the Main Work
Area. They never become another full-height column, and overflowing alerts
scroll inside their shelf instead of covering the editor or terminal.

## The control grammar

Dez uses the same icon for the same object or transition everywhere:

| Icon role    | Meaning                                                  |
| ------------ | -------------------------------------------------------- |
| Terminal     | Start or return to terminal computation                  |
| Folder open  | Open a Workspace or reveal its Files                     |
| File         | Create a new file                                        |
| Diff         | Review observed Workspace changes                        |
| Information  | Inspect Terminal Session details and evidence boundaries |
| Session list | Supervise Sessions and attention                         |
| Clock        | Open Agent Session history                               |
| Settings     | Configure Agent tools and application behavior           |
| Robot        | Create or identify a Dez Agent Session                   |
| Sparkle      | Invoke Inline Assist on the current editor context       |

Icons support labels; they never replace them. A creation icon does not stand
in for an object-specific Terminal or File icon, and review/details controls
do not reuse each other's symbols. Dez does not reuse Zed-branded assistant
marks for these controls. Command Palette labels follow the same grammar: Dez
exposes `sessions: ...`, `files: ...`, `git: ...`, and
`workspace tools: ...` display names instead of inherited implementation
namespaces.

Hierarchy follows the next useful action. Welcome gives one filled,
keyboard-focusable recommendation—**Open Workspace** when no codebase is open,
or **Start Terminal Session** inside an active Workspace—while related actions
remain quieter alternatives. Dense Workspace Tools and Agent toolbars use
compact icons, but every control has a specific accessible name, tooltip, and
place in the keyboard tab order. A critical action is never available only on
pointer hover.

Welcome keeps that first choice concrete. Without a Workspace, its three start
actions are **Open Workspace**, **Clone Repository**, and **Open Scratch
Terminal**. Inside a Workspace, they become **Start Terminal Session**, **Open
Files**, and **New File**. The empty Main Work Area uses the same Session
vocabulary and does not describe ordinary GUI-owned terminals as durable.

The empty Sessions region follows the same activation loop. **Start with a
Workspace** explains that the codebase supplies context to Terminal or Agent
Sessions and that their changes return to the IDE for review. **Open
Workspace** remains the primary same-window action, **Open Scratch Terminal**
is the secondary pathless option, and the primary action becomes **Start
Terminal Session** once a Workspace is ready. This prevents a first-run
terminal in the home directory from looking connected to Files or Git when it
is not. Start, recovery, and All/Attention scope actions remain keyboard
reachable as the rail changes state.

Workspace controls follow focus. Selecting or keyboard-focusing a Workspace
keeps its Start Terminal Session and Options actions visible; opening the
Options menu keeps its scoped close controls visible as well. Search clearing
and banner dismissal are keyboard-focusable, so pointer hover is never the only
route to a visible shell action.

The Main Work Area follows the same rule. Back, Forward, Add, Switch Surface,
Split, Zoom, and the Workspace Tools/Agent hide controls are keyboard-focusable
and specifically named. The active unpinned Surface keeps its close control
visible in Dez; inactive tabs remain visually quiet.

Existing generated Dez profiles are upgraded consistently. A known legacy
profile that pinned `.ZedSans`, One Light, and light-only appearance migrates
to JetBrains Mono with system-selected Lumin Light/Lumin Blur. Custom font or
theme choices are not treated as generated defaults.

Creation emphasis also follows state. A ready Workspace without a Session
shows a filled **Start Terminal Session** action. Once work exists, a compact
**Start Terminal** utility remains available as an outlined action while the
Session list becomes the primary content.

The bottom Workspace status strip is intentionally terse. Search and a healthy
diagnostics state use familiar icons with complete tooltips and accessible
names; actual errors, warnings, counts, and messages remain visible. Terminal
focus does not expand routine healthy state into another row of prose.

## The core objects

You only need four concepts for everyday use:

- A **Workspace** is durable human context: its Surfaces, pane layout, focus,
  navigation history, and Project scope.
- A **Surface** is something you can work with in the pane grid, such as a
  file, terminal, search result, debugger, settings page, Agent Session, or
  review.
- A **Session** is computation or an agent conversation whose identity can
  outlive a single view.
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

### 2. Work directly or delegate

Open a file to edit directly, create a terminal in the Main Work Area, or start
an Agent Session in the Agent region.

Agent edits land in ordinary buffers and Git changes. A terminal starts in the
Workspace's working-directory context. Both sit beside files in the same pane
grid, so direct and delegated work can be compared rather than hidden behind
mode switches. Agent Options and New Agent Session are explicit popovers: their
triggers stay highlighted while open and announce that state to keyboard and
assistive-technology users.

If no provider is configured, Agent shows a named provider-setup state rather
than a Zed account or subscription pitch. **Configure Agent Providers** opens
the relevant settings; after a non-Zed-cloud provider is ready, **Start Agent
Session** becomes the primary action. Both actions are keyboard reachable, and
the surface consistently names the resulting object an Agent Session.

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

- which Agent and Terminal Sessions exist;
- whether they are running, waiting, failed, exited, saved, or unavailable;
- which Session needs attention;
- when meaningful activity last occurred; and
- what evidence is available for review.

Sessions does not own the terminal process or Agent conversation. It routes
back to the Surface or Host Session that does.

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
Its context bar and selected Sessions row provide the same direct handoff:
**Files** opens the owning Workspace tree, **Review Changes** opens Agent review
or Git Changes as appropriate, and **Session Details** opens the observed run
summary. Terminal handoff labels stay visible in ordinary split panes before
collapsing to named icons on very narrow surfaces. Returning to the row focuses
the existing Session rather than starting another shell. A Git review
destination identifies itself in the Main Work Area as **Diff · filename**; its
tooltip retains the diff base and relative path, so switching between terminal,
file, and review never leaves a generic “Uncommitted Diff” surface.

These are destination actions, not visibility toggles. Repeating **Files**
keeps Files open and focused; repeating **Review Changes** keeps Git Changes
open and returns to the current review. Neither action closes the destination
because it was already visible.

Git Changes keeps changed-file navigation ahead of commit composition. The
inline commit editor shows four lines by default; use its full-height or modal
expansion for a longer message. View Diff, stage/unstage, commit, remote, and
split-menu controls remain keyboard reachable and announce their action and
open state.

If a saved Session owns a closed Workspace, **Files** restores that exact
Workspace and Session before revealing the project tree. It does not silently
do nothing or manufacture a replacement Session.

Session Details also states the trust boundary. Lifecycle comes from the
Terminal and Host; Git counts belong to the Workspace and are not automatically
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

Workspace composition and session metadata are restored where the source owns
them. If a saved Terminal Session cannot be reconnected, Dez preserves its
title and displays one **Terminal Session unavailable** warning. It does not
silently start a replacement shell or print fake recovery text into the
terminal grid.

**Start Fresh Terminal** creates separate computation in the Main Work Area; it
does not claim to reconnect, replay, or replace the unavailable Session.

If the terminal session service itself is connecting, reconnecting, or failed,
Sessions states whether any shell started and whether running processes
were touched. **Open Local Log** and **Copy Details/Error** expose diagnostics
without putting transport jargon in the main notice. If a Workspace cannot
reopen, **Open Recent Workspaces** retries through the normal picker; **Remove
Recovery Entry** removes only that rail record and keeps recent Workspace data.
These recovery actions are keyboard reachable.

## Terminal and Agent integration

Dez does not put the terminal inside chat.

- Ordinary terminals are normal Main Work Area Surfaces.
- Agent conversations are normal Agent Surfaces.
- The active Workspace supplies shared Project context.
- Agent edits appear in the same buffers and Git repository the developer uses.
- Structured terminal-agent adapters can add lifecycle, attention, command,
  exit, and file-target evidence without making process-name detection a source
  of truth.

The v0.0.2 source contains an experimental local terminal Host that can own PTYs
outside the GUI process. It is intentionally not the default until consolidated
build, restart, transport-loss, and crash evidence is complete. Default task
terminals remain GUI-owned because retaining a task after the UI reports
cancellation would be dishonest.

## Visual design

Dez follows the system appearance with **Lumin Blur** and **Lumin Light**.
JetBrains Mono is bundled and used across interface, editor, terminal, prompt,
and review roles so the installed product has one unmistakable typographic
identity. Users can still override any role through normal settings.

If imported settings hide that identity, **Dez → Settings → Restore Dez Visual
Profile** restores only Lumin, JetBrains Mono, and the built-in Dez icons while
preserving font sizes and non-visual preferences.

Blur belongs to the stable window shell. On macOS the window uses the native
under-window backdrop and follows active/inactive system state; Lumin layers sit
on top of that material rather than simulating blur with opaque panels. Focus
borders, selected rows, active lines, pane boundaries, and scrollbars remain
visible. Sessions, Workspace Tools, Agent, the Main Work Area, tab strips, and
elevated menus use distinct semantic layers instead of blending into one sheet.
Controls in Lumin Light are translucent glass layers, not beige blocks.
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

Stable secondary windows—including About, account verification, Settings,
Audio Test, and profiling—inherit the same active Lumin window material and UI
font instead of becoming opaque or reverting to a platform-default typeface.
Small shaped notification popups stay transparent around their own surface,
while their text still uses the configured Dez UI font.

## What Dez is not

Dez is not:

- a terminal dashboard with a token editor;
- an agent chat product with a terminal attachment;
- a replacement Git database;
- a process-name guesser presented as reliable agent state;
- a second project tree hidden in Sessions; or
- a claim that every session already survives every crash.

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
