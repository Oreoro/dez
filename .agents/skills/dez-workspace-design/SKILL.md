---
name: dez-workspace-design
description: Design, audit, wireframe, and implement Dez's native Workspace-first desktop experience. Use for Dez or Zed-derived work involving Home, Workspaces navigation, panes, tabs, terminal agents, tmux, Herdr, cmux handoff, status bars, onboarding, permissions, recovery, Settings, product copy, responsive behavior, or UI/UX polish. Require an annotated monochrome wireframe before structural UI changes and preserve native Zed ownership.
---

# Dez Workspace Design

Design Dez as a Workspace-first development environment for developers and
vibe coders coordinating terminal-native agents without giving up a complete
editor.

## Start from authority

Read these files before changing structure or public copy:

1. `AGENTS.md`
2. `docs/src/development/dez/surface-contract.md`
3. `docs/src/development/dez/product-strategy.md`
4. `docs/src/development/dez/tmux-claude-navigation-wireframe.md`
5. the source files that currently own the affected native surface

Treat current source and rendered evidence as authoritative. Treat sketches as
proposals until the surface contract and source deliberately adopt them.

Read [native-surface-rules.md](references/native-surface-rules.md) for the
ownership, navigation, copy, terminal, recovery, and responsive rules. Read
[workflow-state-matrix.md](references/workflow-state-matrix.md) when designing
or auditing an end-to-end flow.

## Use the contract

Keep this hierarchy true:

> Workspace is session-level identity. Pane is target. Tab is content.
> Activity is signal.

- Keep one stable Workspaces tree and one authoritative native pane model.
- Let **Layout** activate existing pane tabs; never let it own close, pin,
  reorder, drag, preview, or split state.
- Show **Activity** only for authoritative active, running, actionable,
  recoverable, or review-ready work. Route idle open tabs through Layout and
  inactive completed Agent Sessions through Agent History.
- Keep terminal TUIs visually authoritative inside native TerminalView tabs.
- Keep tmux and Herdr externally owned when attaching; hand cmux Workspaces to
  cmux rather than simulating its UI.
- Keep one native status line for durable context and temporary named modes.
- Keep onboarding, access, and recovery inline in their owning surface. Do not
  add custom overlays, floating mascots, modal tours, or duplicate sidebars.

## Follow the design workflow

### 1. Audit the current state

- Inspect the exact screenshot, source, settings, and persisted behavior in
  scope.
- Name the real owner of every visible region and action.
- Record contradictions: duplicate navigation, hidden focus, ambiguous nouns,
  inaccessible icon actions, background permission loops, or inferred process
  state.
- Separate source evidence, remote-CI evidence, and installed-runtime evidence.

### 2. Write the user story

Describe one primary loop in verbs:

`Open Workspace → start or resume work → supervise Activity → inspect and review`

Add return, failure, permission, narrow-width, keyboard-only, and screen-reader
states from the workflow matrix. Remove any surface that does not advance this
loop or provide safe recovery.

### 3. Wireframe before structural edits

- Produce a black-and-white annotated wireframe with an ASCII companion.
- Use no color, gradients, shadows, glass, illustrations, or decorative cards.
- Show native titlebar, adjacent tab `+`, Workspaces, real pane boundaries,
  native tab strips, Main Work Area, and status line.
- Number annotations and state the owner, action, invariant, and failure rule.
- Include at least focused work, navigation, return-to-work, and recovery
  frames.
- Inspect the rendered artifact. Correct text, duplicated controls, false
  overlays, unclear focus, and impossible ownership before implementation.

Use the `imagegen` skill for a raster pencil-wireframe artifact when available.
Preserve the ASCII wireframe as the exact text and interaction contract because
raster text is not authoritative.

### 4. Decide before coding

For every proposed element, answer:

- Which existing Zed entity owns its state?
- Does it activate existing content or create content?
- What is the keyboard and accessibility path?
- What disappears at narrow width?
- What happens when data is loading, missing, denied, stale, or failed?
- Is the state authoritative, or is the UI guessing from terminal text?

Reject the element if ownership is duplicated or recovery is unclear.

### 5. Implement natively

- Prefer existing actions, panes, tabs, panels, lists, disclosures, status
  items, theme tokens, icons, focus handles, and persistence.
- Modify the existing owning file unless a genuinely new logical component is
  required.
- Keep Dez-specific behavior behind explicit product policy and preserve
  official Zed behavior.
- Use text plus semantic treatment for focus and lifecycle; never color alone.
- Add source-level policy tests and extend `script/dez-identity-check` for
  invariants that could silently regress.
- Update the surface contract, roadmap, and release evidence without claiming
  runtime proof that does not exist.

### 6. Validate honestly

When the user prohibits local builds, run only permitted source checks such as:

- `cargo fmt --all -- --check`
- `bash -n script/dez-identity-check`
- `./script/dez-identity-check --source-only`
- `git diff --check`

Leave compilation, tests, packaging, signing, and installed-runtime validation
to GitHub Actions. A green identity guard is not proof that the app rendered or
that a package runs.

## Finish with evidence

Before claiming completion, verify:

- the annotated monochrome wireframe exists and was visually inspected;
- the ASCII companion and surface contract match the implemented hierarchy;
- native owners still own their state;
- happy, return, recovery, permission, narrow, and accessibility states are
  represented;
- source guards cover the new product invariant;
- the branch is clean and synchronized; and
- the exact remote commit has the required CI evidence.

Report any unimplemented frame or unverified runtime state explicitly.
