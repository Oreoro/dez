# tmux and Claude Code Navigation Wireframe

> Status: partially implemented. The active Workspace owns an activation-only
> **Layout** projection of its native panes and tabs plus bounded **Activity**
> derived from current authoritative lifecycle state. Temporary navigation
> mode and return recap remain design work and are not release claims.

This direction keeps Dez native to Zed while borrowing interaction principles
from two terminal-first products:

- tmux uses a strict session → window → pane hierarchy, makes one pane active,
  keeps the current target visible in a compact status line, and activates a
  temporary prefix-key table without covering pane content.
- Claude Code keeps background work and its task list subordinate to the main
  terminal interaction, names permission mode in status, and provides a
  one-line recap when a user returns after time away.

The proposal adapts those ideas rather than copying their visual chrome:

> **Workspace is session-level identity. Pane is target. Tab is content.
> Activity is signal.**

Primary references:

- [tmux: Getting Started](https://github.com/tmux/tmux/wiki/Getting-Started)
- [Claude Code: Interactive mode](https://code.claude.com/docs/en/interactive-mode)
- [Claude Code: Status line](https://code.claude.com/docs/en/statusline)
- [Claude Code: Permission modes](https://code.claude.com/docs/en/permission-modes)

## Information architecture

```text
Dez window
├── Workspaces                         activation and supervision
│   ├── Workspace                      stable codebase identity
│   │   ├── Layout                     projection of real pane ownership
│   │   │   ├── Pane 1 · focused
│   │   │   │   └── pane-owned tabs
│   │   │   └── Pane 2
│   │   │       └── pane-owned tabs
│   │   └── Activity                   only running or actionable signals
│   └── Other Running                  external owners without a Workspace
├── Main Work Area                     real pane grid and content ownership
│   ├── Pane 1
│   └── Pane 2
└── Status line                        target, mode, attention, repository state
```

The previous separate **Sessions** and **Open Tabs & Tools** sections were too
easy to read as two competing navigation models. The selected Workspace now
uses **Layout** for its native pane/tab projection and separates live signals
into:

- **Layout**, a hierarchy of real panes and their real tabs. Its rows activate
  existing targets. It owns no close, pin, drag, reorder, or split controls.
- **Activity**, a bounded, collapsible list of running or actionable work. It
  disappears when empty and never repeats terminal transcript text.
- **Other Running**, the existing honest boundary for tmux, Herdr, or cmux work
  that does not map to an open Workspace.

The pane remains the only owner of tab order, selected state, preview, pin,
close, drag, and split membership.

## 21. Focused work

```text
┌ Workspaces ─────────────────┬ main.rs | Files | + ─┬ Claude Code | Terminal | + ┐
│ paykit · main · 2 panes     │ native editor         │ provider TUI               │
│ infra · main · 1 pane       │                       │                            │
│                             │                       │                            │
│ Layout                      │                       │                            │
│  1 Pane · main.rs · Focused │                       │                            │
│    main.rs                  │                       │                            │
│    Files                    │                       │                            │
│    Git Changes              │                       │                            │
│  2 Pane · Claude Code       │                       │                            │
│    Claude Code              │                       │                            │
│    Terminal                 │                       │                            │
│                             │                       │                            │
│ Activity · 2                │                       │                            │
│  Claude Code · Waiting      │                       │                            │
│  test · Running · 42s       │                       │                            │
├─────────────────────────────┴───────────────────────┴────────────────────────────┤
│ paykit / main        Pane 1 of 2 · 1 waiting        PR #4 · Draft  Ln 14, Col 22 │
└──────────────────────────────────────────────────────────────────────────────────┘
```

The Workspace row is session-level identity. Layout provides exact targeting
without becoming a Files tree. Activity is absent when nothing is running or
actionable. The active pane uses restrained native focus treatment rather than
a saturated rectangle around the entire pane.

## 22. Navigate without covering work

```text
┌ Workspaces ─────────────────┬ [1] main.rs | Files | + ┬ [2] Claude Code | + ┐
│ selected Workspace          │ existing content         │ existing TUI          │
│ Layout                      │ remains visible          │ remains visible       │
│  1 Pane · Focused           │                          │                       │
│  2 Pane                     │                          │                       │
├─────────────────────────────┴──────────────────────────┴───────────────────────┤
│ NAVIGATE  [1] Pane 1  [2] Pane 2  [w] Workspace  [n] New Tab  [s] Split      │
│           [?] Keys   [Esc] Cancel                                             │
└────────────────────────────────────────────────────────────────────────────────┘
```

This is a tmux-prefix principle expressed through native Zed chords, not a new
terminal prefix. Entering the mode changes the status line and adds small target
chips inside pane chrome. It never opens a palette or draws labels over content.
One action or Escape exits. The final shortcut must reuse or compose with Zed's
existing keymap after a collision audit; this proposal does not assign it.

## 23. Return to background work

```text
┌ Workspaces ─────────────────┬ main.rs | Files | + ───────────────────────────┐
│ paykit                      │ Since you left · 3 files changed · test passed │
│ Layout                      │ review ready                     [Review] [×]   │
│  1 Pane · main.rs · Focused ├─────────────────────────────────────────────────┤
│  2 Pane · Claude Code       │ native editor                                   │
│                             │                                                 │
│ Activity · 3                │                                                 │
│  Claude Code · Waiting      │                                                 │
│                         Open│                                                 │
│  Codex · Working · 6m 32s   │                                                 │
│  test · Passed        Review│                                                 │
├─────────────────────────────┴─────────────────────────────────────────────────┤
│ paykit / main       Pane 1 of 2 · 2 active · 1 waiting       PR #4 · Draft   │
└───────────────────────────────────────────────────────────────────────────────┘
```

The Activity shelf follows Claude Code's content-dependent task visibility: it
has no empty placeholder and shows at most five top-level rows before native
expansion. Running Sessions consumes one of those rows; the current destination
and attention-required rows displace routine activity from the preview. Search
and Attention scope remain exhaustive. State stays in text, and selecting a row
focuses its existing owner.

The return recap appears once at the top edge of the owning pane, then yields
space after review or dismissal. It may summarize only authoritative events:
repository changes, observed task exit status, authenticated agent lifecycle,
or a pending review. It never summarizes arbitrary PTY output.

## 24. Choose and recover honestly

```text
┌ Workspaces ─────────────────┬ Claude Code | Terminal | + ────────────────────┐
│ Find Workspace, pane, tab… │ $ tmux attach -t sandbox                       │
│                             ├─────────────────────────────────────────────────┤
│ paykit                      │ Attach failed · tmux session ended             │
│  Pane 1 · main.rs · Focused │ The external owner no longer reports it.       │
│    main.rs · Files          │              [Refresh Sessions] [New Shell]    │
│  Pane 2 · Claude Code       ├─────────────────────────────────────────────────┤
│    Claude Code · Waiting    │ preserved terminal output                      │
│    Terminal                 │ ~/paykit %                                      │
│ infra                       │                                                 │
│  Pane 1 · Terminal          │                                                 │
│ Other Running              │                                                 │
│  tmux · sandbox · Available │                                                 │
├─────────────────────────────┴─────────────────────────────────────────────────┤
│ paykit / main       Pane 2 of 2 · Claude Code · Terminal      PR #4 · Draft │
└───────────────────────────────────────────────────────────────────────────────┘
```

Search replaces only the Workspaces list body, like a native choose-tree. The
Main Work Area never moves or dims. Recovery remains a flat shelf in the owning
terminal, with preserved output beneath it. **Open New Shell Here** is separate
work and never claims attachment, transfer, or migration.

## Interaction rules

1. One stable tree: Workspace → Pane → Tab. External owners remain separate.
2. One focused pane. Focus is always visible through text and treatment, not
   color alone.
3. One status line. Normal status, navigation mode, and bounded progress reuse
   the same reserved region instead of stacking toolbars.
4. Zero duplicate tabs. Workspaces can activate a tab but never manages it.
5. Activity is conditional. Empty activity consumes no space.
6. Signals are bounded and authoritative. No transcript scraping or inferred
   success.
7. Modes name themselves. Navigation, permission, restricted, and multiplexer
   prefix states cannot be invisible.
8. Recovery stays with its owner and always names the next safe action.
9. Motion is limited to focus, disclosure, and progress feedback, respects
   reduced motion, and never shifts pane geometry.
10. Every icon-only action has a label, every state has text, and keyboard order
    follows the visible hierarchy.

## Decisions required before remaining implementation

- Audit Zed's existing chords and action namespaces before assigning a temporary
  navigation mode shortcut.
- Define the exact trusted event fields that may feed Activity and return recaps.
- Validate the implemented narrow-window collapse with long Workspace, pane,
  and Activity names: nested Layout destinations collapse first, while pane and
  Activity attention identity remain visible without horizontal scrolling.
- Decide whether completed Activity rows disappear immediately, after review, or
  after a short bounded acknowledgement period.
- Validate the hierarchy with one pane, four panes, multiple Workspaces, long
  names, external sessions, keyboard-only use, and assistive navigation.

The activation-only **Layout** and bounded **Activity** slices are implemented.
This document must not be used as evidence that temporary navigation mode or
return recaps ship in the current application.

## 25. Final cmux-first Workspace shell

```text
┌ Workspaces ───────────────┬ main.rs | Claude Code | Files | + ───────────────┐
│ paykit                    │                                                   │
│ main · 2 panes            │ native editor, terminal, diff, or tool surface   │
│                           │                                                   │
│ Layout · 4 open           │                                                   │
│  Pane 1 · Focused         │                                                   │
│   main.rs                 │                                                   │
│   Claude Code             │                                                   │
│  Pane 2                   │                                                   │
│   Files                   │                                                   │
│   Git Changes             │                                                   │
│                           │                                                   │
│ Activity · 2              │                                                   │
│  Codex · Working · 3m     │                                                   │
│  test · Needs attention   │                                                   │
│                           │                                                   │
│ [Open Workspace in cmux]  │                                                   │
├───────────────────────────┴───────────────────────────────────────────────────┤
│ Workspaces · paykit   main   Pane 1 of 2   2 active   Ln 14, Col 22          │
└───────────────────────────────────────────────────────────────────────────────┘
```

Annotations:

1. **cmux is preferred, not embedded.** **Open Workspace in cmux** opens the current
   local Workspace in cmux. cmux then owns its windows, tabs, splits, browser,
   hooks, and action registry. tmux remains an in-Dez terminal fallback and a
   discoverable external process owner.
2. **Native pane tabs remain authoritative.** Their order, active state, close,
   drag, pin, overflow, and adjacent `+` stay owned by Zed's pane tab strip.
   Chrome-like polish means clear selected shape, balanced spacing, consistent
   glyphs, and native material—not a replacement browser-tab implementation.
3. **The sidebar is a navigator, not an inventory dump.** Workspace identity is
   visually strongest; **Layout** appears only with at least two open items;
   **Activity** appears only for bounded running or actionable work. Files stay
   in the Files tab rather than being duplicated here.
4. **Typography follows role.** Workspace names and active destinations use the
   interface face at medium weight. Section labels and durable metadata use a
   smaller muted role. Terminal and editor content keep the code face.
5. **Lumin owns one native material.** The macOS window supplies the blur.
   Sidebar, tab bar, editor, terminal, and status line use translucent semantic
   theme layers without nesting another blur or floating glass card. Generic
   selection tokens stay neutral across blurred, opaque, and light variants;
   the warm accent remains available for focus, attention, and recovery.
6. **One status line closes the hierarchy.** It reports Workspace, repository,
   focused pane, bounded attention, and editor position. Transient handoff or
   discovery messages remain inline with their owner.
7. **Selected tabs use typographic emphasis.** The active native tab keeps the
   same IBM Plex Sans interface face at medium weight, a rounded selected shape,
   and stronger semantic contrast. Inactive tabs stay normal weight and quiet;
   dirty, close, pin, drag, overflow, and keyboard behavior remain native.
8. **Navigation icons share one density scale.** Spacious Workspaces rows use
   the same small icon role as title-bar and status controls. Focus uses the
   selected row, medium-weight label, and explicit “Focused” text; accent color
   remains reserved for attention, modification, and recovery signals.
   Activity lifecycle metadata stays plain supporting text rather than a
   badge-like pill.
9. **Pane chrome exposes its state.** The adjacent `+`, tab-overflow, and split
   controls remain native menu triggers. While open they use the same neutral
   selected material as navigation and expose their expanded state to assistive
   technology; no menu state is communicated through accent color alone.

## 26. Narrow Workspace rail

```text
┌ Workspaces ─────────┬ main.rs | + ──────────────────────────────────┐
│ paykit              │                                               │
│ main · 2 panes      │ Main Work Area remains primary                │
│                     │                                               │
│ Layout · 4 open     │                                               │
│  Pane 1 · Focused   │                                               │
│  Pane 2             │                                               │
│                     │                                               │
│ Activity · 1        │                                               │
│  test · Attention   │                                               │
├─────────────────────┴───────────────────────────────────────────────┤
│ Workspaces · paykit · Pane 1 of 2 · 1 attention                     │
└─────────────────────────────────────────────────────────────────────┘
```

Below 280px, the Layout header keeps the total and real pane identity but omits
nested tab rows; the native pane tab strip remains the authoritative destination
list. Each pane heading announces its position, focused state, and open-item
count, so the compact summary remains useful to screen readers without restoring
the crowded duplicate rows. Activity keeps current and attention-required rows.
The titlebar keeps **Open Workspace** and the overview menu, while the redundant
Search icon collapses into that menu. No content scrolls horizontally, and
closing Workspaces still returns its width to the Main Work Area.
