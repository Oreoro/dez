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
| **Session Rail**    | Navigate Workspaces and Sessions; see lifecycle, attention, and recent activity  |
| **Workspace Tools** | Inspect Files, Outline, Git, and Debug for the active Workspace                  |
| **Main Work Area**  | Edit files and open terminal, search, settings, diagnostics, and review Surfaces |
| **Agent**           | Work with native or external coding-agent conversations                          |

These are not four separate products.

The Main Work Area is one pane grid. A file, terminal, diff, search result,
settings page, or review can be tabbed, split, moved, and focused through the
same rules. Workspace Tools and Agent are hideable regions around that grid.
The Session Rail is a projection over the real owners: selecting a row focuses
or reattaches its existing Surface instead of opening a duplicate.

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

Opening a folder therefore enables one coherent IDE scope. Files, Outline, Git,
and Debug are views of the same Project, not separate roots.

### 2. Work directly or delegate

Open a file to edit directly, create a terminal in the Main Work Area, or start
an Agent Session in the Agent region.

Agent edits land in ordinary buffers and Git changes. A terminal starts in the
Workspace's working-directory context. Both sit beside files in the same pane
grid, so direct and delegated work can be compared rather than hidden behind
mode switches.

### 3. Supervise without polling every tab

The Session Rail groups work by Workspace and projects:

- which Agent and Terminal Sessions exist;
- whether they are running, waiting, failed, exited, saved, or unavailable;
- which Session needs attention;
- when meaningful activity last occurred; and
- what evidence is available for review.

The rail does not own the terminal process or Agent conversation. It routes
back to the Surface or Host Session that does.

### 4. Review with the IDE

Use Files and Outline to understand structure, diagnostics and Debug to inspect
behavior, Search to trace relationships, and Git to review the actual changes.
Agent Review supports interactive Keep/Reject decisions. A Review Brief is a
different Surface: it summarizes observed evidence and calls missing evidence
missing.

Dez does not treat an agent saying “tests passed” as equivalent to an observed
command with an exit status.

### 5. Resume honestly

Workspace composition and session metadata are restored where the source owns
them. If a saved Terminal Session cannot be reconnected, Dez preserves its
title and displays one **Terminal Session unavailable** warning. It does not
silently start a replacement shell or print fake recovery text into the
terminal grid.

**Start Fresh Terminal** creates separate computation in the Main Work Area; it
does not claim to reconnect, replay, or replace the unavailable Session.

## Terminal and Agent integration

Dez does not put the terminal inside chat.

- Ordinary terminals are normal Main Work Area Surfaces.
- Agent conversations are normal Agent Surfaces.
- The active Workspace supplies shared Project context.
- Agent edits appear in the same buffers and Git repository the developer uses.
- Structured terminal-agent adapters can add lifecycle, attention, command,
  exit, and file-target evidence without making process-name detection a source
  of truth.

The v0.0.1 source contains an experimental local terminal Host that can own PTYs
outside the GUI process. It is intentionally not the default until consolidated
build, restart, transport-loss, and crash evidence is complete. Default task
terminals remain GUI-owned because retaining a task after the UI reports
cancellation would be dishonest.

## Visual design

Dez follows the system appearance with **Lumin Blur** and **Lumin Light**.
JetBrains Mono is bundled for editor, terminal, prompt, and code roles, while a
bundled sans-serif face keeps interface labels readable.

Blur belongs to the stable window shell. Focus borders, selected rows, active
lines, pane boundaries, and scrollbars remain visible, while elevated menus
stay solid enough to read. High-motion terminal and Agent content does not add
independent nested blur layers.

## What Dez is not

Dez is not:

- a terminal dashboard with a token editor;
- an agent chat product with a terminal attachment;
- a replacement Git database;
- a process-name guesser presented as reliable agent state;
- a second project tree hidden in the Session Rail; or
- a claim that every session already survives every crash.

The v0.0.1 goal is a complete native IDE with one sharp wedge: trustworthy
supervision and review of terminal-native and agent-driven work.

## Source-preview limits

This repository currently represents a source candidate, not a signed public
binary. A release still requires consolidated platform builds plus rendered,
restart, crash, accessibility, upgrade, and coexistence evidence.

For precise implementation state, read:

- [Fork Notes](./development/dez/fork-notes.md)
- [Architecture Baseline](./development/dez/architecture-baseline.md)
- [Roadmap](./development/dez/roadmap.md)
- [v0.0.1 Release Runbook](./development/dez/v0.0.1-release-runbook.md)
- [Release Evidence](./development/dez/release-evidence.md)
