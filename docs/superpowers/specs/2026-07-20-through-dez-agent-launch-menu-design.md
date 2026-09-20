# Through-dez Agent Launch and Menu Design

**Date:** 2026-07-20
**Status:** Implemented and automatically verified

## Scope

This change applies only when a remote SSH project is configured as
**Through dez**. **Direct** retains its current menu, configured
command launches, explicit dez-managed launch row, and resume behavior.

The behavior applies equally to Codex and Claude. Live Claude validation is
deferred to the user's separate Claude test remote.

## Problem

The new-thread dropdown currently presents ordinary launch options that use the
configured ambient command and a separate **New — dez-managed Agent** row.
That distinction is unnecessary in Through-dez mode: the selected route
already means dez must use the pinned managed CLI and dez-provided egress.
The managed row can also display provisioning status even though the existing
popup notification already reports checking, downloading, installing, and
reusing the CLI.

Resume routing is inconsistent between agents. Through-dez Codex resume uses
the managed CLI, while Claude resume still uses the configured ambient command.

## Through-dez Behavior

For both Codex and Claude:

- Every new-thread entry point uses the pinned managed CLI through dez.
  This includes the main New button, keyboard actions, default launch options,
  and every new-thread dropdown option.
- Every resume entry point uses the pinned managed CLI through dez. This
  includes plain resume, resume-option variants, and automatic session
  restoration.
- A route change during managed preparation aborts the launch. dez never
  falls back to the ambient remote CLI or direct remote internet.
- Existing managed-agent progress and confirmation notifications remain the
  only provisioning status surface. Repeated launch attempts while provisioning
  reuse the active notification and do not start another download.
- Remote sign-out remains available and continues to use the managed CLI
  through dez.

The Through-dez Codex new-thread dropdown contains only:

1. **New thread**
2. **New — Bypass approvals & sandbox**
3. **Sign out Codex on remote…**

The Through-dez Claude dropdown follows the same structure with its existing
**New — Skip permission prompts** option and Claude sign-out label.

The separate **New — dez-managed Agent** row, including any provisioning
status appended to that row, is not rendered in Through-dez mode.

## Not-through-dez Behavior

No behavior or copy changes:

- ordinary new-thread and resume actions use the configured command;
- the explicit **New — dez-managed Agent** row remains available on supported
  remote targets; and
- existing dropdown status behavior for that explicit row remains available.

## Architecture

### Route-aware new-thread dispatch

Make the store's ordinary new-thread entry point authoritative for route
selection. It reads the project route once:

- Through dez delegates to managed preparation and launches the prepared
  executable with Through dez as the required route.
- Direct and route-less workspaces retain the current configured
  command path.

All UI and action entry points already converge on the store launch function,
so central dispatch prevents the main button or a keyboard action from
bypassing the selected route.

Extract common new-thread command construction so configured and managed
launches preserve identical arguments, environment, working directory, and
session metadata. In particular, managed Claude launches must continue adding
the generated `--session-id`; otherwise a fresh Claude thread could not be
restored later.

The existing explicit managed-launch function remains for the Not-through-dez
dropdown row. It uses the same command construction without changing the
project's selected route.

### Route-aware resume dispatch

Generalize the existing managed-resume predicate from Codex-only to both
registered agents whenever the selected route is Through dez. The existing
managed preparation, managed resume command builder, route-change guard, and
sequential automatic-restoration flow remain authoritative.

Resume options are passed through unchanged. Self-update suppression remains
per process and idempotent.

### Menu policy

The panel determines whether the selected project route is Through dez. It
always renders the ordinary new-thread choices. It renders the explicit
dez-managed row only when the route is Direct and the remote target
has a pinned release for the selected agent.

The panel does not read or append managed provisioning status when the row is
hidden. The popup notification remains responsible for progress.

## Error Handling

- Unsupported remote platform, unavailable SSH connection, artifact failure,
  installation failure, and launch failure continue surfacing through the
  existing managed-agent notification and workspace error paths.
- If provisioning is already active, the existing notification is shown and a
  second operation is not started.
- If the route changes while preparation is active, the required-route check
  rejects the launch and asks the user to start it again under the new route.
- No managed failure falls back to an ambient executable.

## Testing

Automated tests will prove that:

- the explicit managed row is hidden only for Through dez;
- Not-through-dez keeps the existing explicit managed row;
- ordinary new-thread dispatch selects managed preparation only for Through
  dez;
- managed Codex and Claude new-thread commands retain configured command data,
  launch-option arguments, self-update suppression, and Claude's generated
  session ID;
- managed resume selection applies to Codex and Claude only in Through-dez
  mode; and
- resume option arguments and automatic restoration keep using the existing
  managed-resume path.

Tests construct command values and exercise internal routing seams without
launching Claude or contacting Anthropic. Run the focused tests, the complete
`agent_threads` suite, `cargo fmt --all -- --check`, and
`./script/clippy -p agent_threads` before delivery.

## Acceptance Criteria

- A Through-dez Codex dropdown shows New thread, New — Bypass approvals &
  sandbox, and remote sign-out, with no explicit dez-managed/status row.
- A Through-dez Claude dropdown has the equivalent three-part structure.
- Every new and resume action for both agents uses the pinned managed executable
  and Through-dez transport when the selected route is Through dez.
- Automatic restoration for both agents follows the same Through-dez rule.
- Repeated launch attempts do not start duplicate provisioning.
- Not-through-dez behavior remains unchanged.
- Live Claude validation is not claimed in this iteration.

## Implementation Verification

Implemented on 2026-07-20. Through-dez new-thread actions now enter managed
preparation automatically, and Through-dez resume and restoration do the same
for both Codex and Claude. The explicit managed/status row is hidden only in
Through-dez menus. Not-through-dez keeps configured new/resume dispatch and
the explicit managed row.

Verification completed with:

- focused RED/GREEN tests for resume parity, new-thread command construction,
  route dispatch, and menu visibility;
- `cargo test -p agent_threads` — 155 passed;
- `cargo fmt --all -- --check` — passed;
- `./script/clippy -p agent_threads` — passed with warnings denied; and
- a fresh `/tmp/dez-Local.app` whose main executable SHA-256 matched the
  signed source bundle.

Automated tests constructed Claude commands without launching Claude or
contacting Anthropic. Live Claude validation remains deferred to the user's
separate remote.
