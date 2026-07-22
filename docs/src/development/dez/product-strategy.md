# Dez Product Strategy

This document records a market hypothesis, not permanent architecture. Revise
it when product evidence contradicts it.

## Market thesis {#market-thesis}

Development is shifting from producing every change directly to supervising
several human and machine actors. Editors center files and projects. Terminals
center individual processes. Agent products often center isolated
conversations. Developers must reconstruct the relationship between a request,
agent, terminal, host, repository, worktree, changed files, commands, checks,
and pending review.

Dez makes that relationship durable. It is a local-first control plane for
active software work, combining Zed-quality editing, tmux-like continuity, and
a vendor-neutral agent operations layer.

## Complete product, sharp wedge {#complete-product-sharp-wedge}

Dez is a complete native development environment, not a monitoring shell
around agents. Two work modes are equally legitimate:

- **Direct work:** edit, navigate, search, refactor, run, test, debug, preview,
  collaborate, and operate Git.
- **Delegated work:** start agents, supervise parallel execution, respond to
  attention, inspect Evidence, review changes, redirect work, and approve
  outcomes.

This is a product-capability statement, not a broad launch strategy. The first
market wedge remains developers supervising two to eight concurrent coding
agent tasks. Editor quality is inherited and protected; new Dez-specific work
must first improve continuity, attention, context association, trustworthy
review, or fork sustainability.

The working promise is:

> Write it. Delegate it. Watch it. Verify it. Ship it—in Dez.

## Product pillars {#product-pillars}

- **Create:** Dez Editor, code intelligence, search, Git, tasks, tests, and
  debugger.
- **Delegate:** native, ACP, and terminal agents with explicit objectives and
  optional isolation.
- **Operate:** durable local and remote Sessions, Workspaces, Hosts, services,
  and low-noise attention.
- **Verify:** native diffs, observed commands and checks, provenance, risks,
  and review decisions.
- **Preserve:** Workspace layouts, eligible processes, bounded history, and
  explicit close, detach, terminate, archive, and delete semantics.

## Ideal customer {#ideal-customer}

The first customer is a terminal-native senior developer, technical founder,
staff engineer, or open-source maintainer who:

- uses coding agents daily and runs at least two tasks concurrently;
- works across repositories, worktrees, or SSH hosts;
- reviews generated changes before merging;
- values native speed, local control, and process continuity;
- finds disposable project windows and disconnected agent panels restrictive.

Dez does not initially target autocomplete-only workflows, beginner-first IDE
onboarding, enterprise administration, or fully autonomous software delivery.

## Job to be done {#job-to-be-done}

When I delegate development tasks across repositories and machines, help me see
what is running, what needs attention, what changed, and what is ready for
review, so I can complete more work safely without managing terminal and editor
state manually.

Supporting jobs include restoring the complete workspace after interruption,
keeping processes alive across GUI disconnection, reviewing output against real
code and Git state, detecting likely actor conflicts, and resuming without
reconstructing history.

## Core product loop {#core-product-loop}

1. **Capture:** Start a Run from an ordinary terminal, agent surface, or explicit
   objective.
2. **Contextualize:** Associate the Run with its host, session, repository,
   worktree, branch, files, and workspace.
3. **Observe:** Show structured progress without demanding focus.
4. **Interrupt:** Notify only for permission, input, failure, conflict, or
   review.
5. **Review:** Present the Run's intent, result, diff, commands, checks, and
   risks with links to observed evidence.
6. **Decide:** Respond, request changes, commit, open a pull request, or discard
   the result.
7. **Preserve:** Restore active work and useful bounded history.

## PMF-critical experience {#pmf-critical-experience}

The first vertical slice is:

```text
current upstream baseline
-> isolated Dez identity
-> one durable workspace
-> one persistent local terminal
-> Codex detection
-> attention item
-> deterministic review brief
-> restart and recover
```

The minimum product surfaces are:

- an attention inbox that links to existing owning surfaces;
- session cards with actor, host, repository, branch, state, time, changes,
  checks, and last meaningful event;
- a review brief that never invents successful checks;
- a bounded structured activity timeline;
- safe session recovery states;
- a keyboard-first switcher across workspaces, surfaces, actors, hosts, and
  attention items.

Conflict radar, SSH continuity, task recipes, worktree brokering, workspace
briefs, mobile attention, and shared team projections follow after this loop is
reliable.

## Hero workflow {#hero-workflow}

1. Start Codex in a frontend terminal.
2. Start another supported agent in a backend terminal.
3. Run a watcher on a remote host.
4. Edit a related file manually.
5. Close and reopen Dez without killing the work.
6. See one actor waiting for permission and another ready for review.
7. Open the ready task's brief and diff beside its owning terminal.
8. Receive an advisory warning if two actors touch overlapping files.
9. Approve, redirect, pause, or discard work explicitly.
10. Return later with workspace and session context intact.

If this workflow is unreliable or confusing, broad feature count does not make
the release successful.

## Success measures {#success-measures}

The primary metric is reviewed Runs completed per weekly active developer.

Activation requires restoring or creating a workspace, running two concurrent
terminals, detecting one agent, leaving and returning, responding to an
attention event, and reviewing a resulting change.

Guardrails include false active-agent states, lost sessions, accidental process
termination, incorrect repository association, missed attention, false
conflicts, startup or memory regressions relative to upstream, crash rate, and
upstream merge lag.

Do not optimize raw prompts, terminal count, time in the app, token use, or
generated line count.

PMF target hypotheses from the revised consolidated plan are deliberately
ambitious and must be measured rather than treated as launch claims:

- 50% target-user activation;
- at least 90% eligible Session recovery and zero unintended termination;
- at least 50% of completed Runs opening a Run Brief;
- at least 40% week-four retention among activated design partners;
- at least 40% of retained users reporting they would be very disappointed
  without Dez;
- demonstrated willingness to pay for continuity or coordination rather than
  bundled model tokens.

## Validation {#validation}

Recruit 15 to 25 developers who already run terminal coding agents. Test real
repositories over four weeks, progressing through workspace recovery, agent
detection and attention, review, then local and remote continuity.

Early-fit hypotheses:

- 40% of activated partners use Dez on three days in week four;
- 30% complete five reviewed agent tasks in a week;
- move toward at least 90% of eligible Session restorations recovering
  correctly, with zero unintended process termination;
- participants voluntarily leave agent sessions running overnight;
- removing Dez would force them back to a multi-tool workflow.

Narrow the product if users value the editor but do not adopt continuity,
attention routing, or agent review.
