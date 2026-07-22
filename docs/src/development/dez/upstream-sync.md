# Upstream Synchronization

Upstream synchronization is Milestone 0 and a permanent development loop. Dez
inherits Zed through Git merges, not by copying individual features.

## Sources and remotes {#sources-and-remotes}

The canonical code upstream is `https://github.com/zed-industries/zed.git`.
Zed release notes and documentation describe released behavior but are not the
source repository.

The normal remotes are:

```text
origin    Dez repository
upstream  zed-industries/zed
```

Track `upstream/main` for early conflict discovery and the latest upstream
stable tag as the release-quality compatibility floor.

## Current local baseline {#current-local-baseline}

Refreshed on 2026-07-22:

- Dez source checkpoint: `c2335969f994af4c7de6fa43e91eb1c93b3f1bb5`
- integration merge: `2be63cfea347006e407934754086bbef62d482c2`
- incorporated `upstream/main`: `9d0ef37a25711c00bf6d1ba1142e9de4f4a122a9`
- current merge base: `9d0ef37a25711c00bf6d1ba1142e9de4f4a122a9`
- divergence: 242 Dez commits and 0 upstream commits after the merge base
- latest stable tag fetched: `v1.11.3`
- `origin`: `maxktz/superzed`
- active branch at capture: `codex/canvas-plan`
- Rust toolchain at the post-merge static gate: `rustc 1.95.0` and
  `cargo 1.95.0`

The tag fetch again reported existing local-name collisions for `nightly` and
`collab-staging`; `upstream/main` still refreshed successfully. Versioned
stable tags were queried independently and continue to top out at `v1.11.3`.
Resolve or namespace the colliding moving tags before treating a future
`--tags` fetch as an all-or-nothing integration gate.

The 2026-07-22 train is integrated as a real two-parent merge. Eleven conflicts
were resolved according to Fork Notes; the exact set and treatment are in the
[Upstream Feature Ledger](./upstream-ledger.md).

## Merge train {#merge-train}

1. **Prepare:** Finish or isolate the current reviewable slice. Do not rehearse
   a merge in a dirty product worktree.
2. **Fetch:** Fetch `origin` with pruning and `upstream` with tags and pruning.
3. **Record:** Capture Dez `HEAD`, `upstream/main`, merge base, stable tag, and
   ahead/behind counts.
4. **Rehearse:** Merge `upstream/main` without committing on a temporary
   `integration/zed-YYYY-MM-DD` branch or isolated worktree.
5. **Classify:** Inventory identity, update, app session, workspace,
   persistence, `Project`, Git, pane, terminal, remote, agent, settings, and
   generated-file conflicts before editing.
6. **Resolve:** Preserve Fork Notes invariants and the newest compatible
   upstream implementation. Add a focused regression test for recurring
   semantic conflicts.
7. **Audit features:** Classify user-visible upstream behavior as inherited,
   adapted, deferred, excluded, or irrelevant.
8. **Verify:** Run focused checks, then the consolidated format, build, and
   manual gates from the roadmap.
9. **Land:** Use a dedicated review with old and new SHAs, conflict categories,
   adaptations, exclusions, verification, and follow-ups.

Use real Git merges. Do not squash upstream into a vendor snapshot or
continually rebase published Dez history. Temporary compatibility adapters must
have a documented removal condition.

## Conflict priority {#conflict-priority}

Resolve semantic conflicts in this order:

1. Dez identity, storage, and update isolation.
2. Durable app-session and workspace ownership.
3. Evidence and workspace-scoping semantics.
4. Host and session lifetime semantics.
5. New upstream architecture and compatible API names.
6. Upstream tools adapted through workspace-scoped `Project` behavior.
7. Pane-first presentation where appropriate.
8. New upstream functionality unless it breaks a locked invariant.

Record deliberate exclusions and their user impact. A small diff is valuable,
but not at the cost of incorrect ownership.

## Automation requirements {#automation-requirements}

The Dez repository should:

- report upstream commit drift on a schedule;
- rehearse a no-commit merge and upload useful conflict artifacts;
- update one upstream-sync issue instead of opening duplicates;
- never push an unresolved merge automatically;
- run fork identity and updater isolation checks on every pull request;
- run workspace-scope and session-ownership regression tests;
- embed the upstream SHA in release provenance.

Merge weekly during active development and before a large milestone when the
fork is materially behind.

## Release provenance {#release-provenance}

Every Dez release records the upstream base commit, latest incorporated stable
Zed version, Dez commit, deliberately excluded upstream behavior, and migration
requirements. Automatic upstream binary updates remain prohibited.
