# Upstream Synchronization

Upstream synchronization is Milestone 0 and a permanent development loop. Dez
inherits Zed through Git merges, not by copying individual features.

## Sources and remotes {#sources-and-remotes}

The canonical code upstream is `https://github.com/zed-industries/zed.git`.
Zed release notes and documentation describe released behavior but are not the
source repository.

The normal remotes are:

```text
origin    https://github.com/Oreoro/dez.git
upstream  zed-industries/zed
```

Track `upstream/main` for early conflict discovery and the latest upstream
stable tag as the release-quality compatibility floor.

## Current local baseline {#current-local-baseline}

Refreshed on 2026-07-24:

- Dez source checkpoint: `4cbe99da2263e781f7aa8725e4dc67ea3d05afc3`
- integration merge: `6eabc1961e20c684edce44439c95f6d3c22f82a7`
- incorporated `upstream/main`: `b0f145f4aec671970340a528cb8197181e969e8c`
- merge base before integration:
  `9d0ef37a25711c00bf6d1ba1142e9de4f4a122a9`
- current merge base: `b0f145f4aec671970340a528cb8197181e969e8c`
- current divergence: 616 Dez commits and 0 upstream commits after the merge
  base
- latest stable release reference: `v1.12.0`
- canonical public repository: `Oreoro/dez`
- active integration branch: `agent/v0.0.2-upstream-parity`
- build gate: locked `dez` and `dez-terminal-host` build passed on macOS arm64

The 2026-07-24 train is a real two-parent merge. It resolved four textual
conflicts according to Fork Notes. Additional compile-time adaptations were
required where upstream tightened GPUI accessibility, tooltip lifetime,
keybinding, and crate-dependency APIs. The exact treatment is in the
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
