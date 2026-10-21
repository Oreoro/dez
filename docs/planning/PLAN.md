# dez v1.0 — Master Plan

*Generated 2026-09-20 from full analysis of zed-upstream, Gram, Flint, and the current dez fork.*

---

## 1. Situation Assessment

### What dez is today (verified)
A full-history fork of zed-industries/zed (41,130 commits) with:
- **`crates/sidebar/`** (~38k lines) — Workspace navigator: Home/Files/Git/Settings as workspace tabs, "Surfaces" (movable/splittable main work area), Activity section with agent-session states (Running / Needs Input / Waiting for Permission / Review-ready), tmux/Herdr/cmux session *discovery-and-attach*, multi-root workspace headers with live git branch + changed-file counts.
- **`crates/dez_terminal_host/`** (1,615 lines) — isolated release-channel terminal host daemon + socket runtime.
- **Branding/packaging** — `dez` binary, `dev.dez.Dez*` bundle IDs, `dez*` URL schemes, 5 dez CI workflows, `dez-identity-check`/`dez-theme-check` guard scripts.
- Workspace persistence patches (`persistence.rs` dez migrations, `prepare_windows_to_quit`).

### Problems (verified)
- **19% of all commits match fix/oops/typo/revert/attempt.** Post-upstream-merge, ~41% of commits are "Fix …".
- Fix-of-fix chains within days: 3 consecutive fixes to `paths.rs`; 3 commits fixing the same markdown_preview rename; 3+ re-fixes of sidebar after upstream API changes; CI added *after* breakage.
- Only 2 upstream merges ever; currently ~13 days / ~700 upstream PRs behind; each merge triggers a fix storm.
- No test/CI gate before merges — breakage lands on main.

### The three reference forks teach opposite lessons
| Fork | Strategy | Result |
|---|---|---|
| **Gram** (Codeberg) | Full-history fork, **rip out entire AI/cloud/telemetry crate graph** (~115 crates), curated cherry-picks of upstream fixes tagged `zed#NNNNN`, complete rename everywhere (`.gram/` config dir, own desktop file, own site), written manifesto | Very active (last commit 2026-09-19), shipped 1.0→3.3.0 in 6 months, small focused identity |
| **Flint** (GitHub) | Forked 2026-06-05, **subtraction as product** (~50 crates removed: all native AI/LLM/collab/ACP), fork logic isolated into ~12 `flint*`/`agent_*` crates, upstream reconciliation in **documented "waves"** with an integration ledger, every change via PR | 742 own commits in 3 months, self-versioned v0.11.3, own release pipeline; but 8-week sync gaps created conflict debt |
| **dez (current)** | Keep everything, merge upstream directly to main, agent-driven commits, no gates | Fix storms, buggy, low trust in own history |

**Core lesson:** both successful forks (a) *subtract massively*, (b) *isolate fork identity into few crates*, (c) *have a disciplined upstream-sync protocol*. dez does none of the three.

---

## 2. Product Definition: dez v1.0

**dez v1.0 = "the default-browser-like editor for agentic coding"** — a Zed fork where the workspace is organized around agent sessions the way a browser is organized around tabs.

### Keep (the product)
1. **Sidebar / Workspace navigator** (salvaged, rewritten against current Zed APIs) — the flagship differentiator.
2. **Agent Activity** — Running / Needs Input / Waiting for Permission / Review-ready states with attention rollup across projects (Flint proves this pattern works; Flint's "cross-project attention rollup + desktop notifications" is the benchmark).
3. **Terminal host daemon** (`dez_terminal_host`) — keep; it's clean and self-contained.
4. **Session discovery-and-attach** for tmux/Herdr/cmux — dez's unique non-ownership angle; Flint owns its threads, dez *attaches to anything*. Lean into this.
5. **Multi-root workspace headers** with live git branch + changed-file counts.
6. Zed's full native editor, LSP, git_ui, remote dev, extensions (keep extension registry compatibility sacred — Flint's lesson).

### Cut (the debt)
- The 70 post-merge fix commits, cleanup commits, dead-code churn — none of it.
- The broken partial-clone repo state — fresh full clone for v1.0.

### Explicit non-goals for v1.0
- No own AI provider stack beyond what upstream ships (don't compete with Zed's agent roadmap; integrate with it).
- No auto-update infrastructure changes, no rebranding beyond what exists.
- No new features until the sync protocol (§4) has survived 2 cycles.

---

## 3. Fork Architecture (the Gram/Flint hybrid)

### 3.1 Fresh start
- Create `dez-v1.0` from **zed-upstream `v1.20.2` tag** (stable, 2026-09-17), NOT from the dez repo's HEAD.
- Keep full Zed history (Gram's lesson: enables `git log --follow`, clean cherry-picks, `zed#NNNNN` attribution).
- Then apply the salvaged dez work as a small set of reviewed commits.

### 3.2 Isolation rules (Flint's lesson — this is the anti-bugfix-storm measure)
- ALL dez-specific logic lives in `crates/dez_*` crates + `crates/zed` wiring only.
- Fork-only crates: `dez_sidebar` (moved out of `sidebar`), `dez_terminal_host`, `dez_activity` (new: session-state model), `dez_actions`.
- Touches to upstream crates (`workspace`, `editor`, `project`) must be **minimal, marked, and listable**: maintain `FORK.md` documenting every divergence from upstream with file:line. If a divergence exceeds ~50 lines, it must become a dez crate or an upstream-extractable hook.
- Rename surfaces completely (Gram's lesson): binary, bundle IDs, config handling, desktop file, icon themes — done once, in one commit, guarded by `script/dez-identity-check`.

### 3.3 Upstream sync protocol (the single most important process change)
Adopt Flint's wave model with Gram's cadence:
1. **Sync every 2 weeks** (not 8 weeks — Flint's conflict debt was self-inflicted).
2. Never merge to main directly. Sync lands on `sync/upstream-YYYY-MM-DD` branch.
3. Before merging: full `cargo check` + `cargo test -p workspace -p editor -p dez_*` + `script/dez-identity-check` green. (dez added CI *after* breakage; v1.0 inverts this.)
4. Every cherry-pick references upstream PR number in the commit message.
5. Maintain `SYNC-LEDGER.md`: one line per upstream adaptation (what, where, why) — this is what makes the *next* sync tractable.
6. Watch the upstream high-risk refactors identified in research: gpui binary-size generics campaign, `OsWatcher` fs rewrite, `RelPath` crate split, config-files-moved-out-of-root, ACP 2.1, rust toolchain 1.98.1 lockstep.

---

## 4. Phased Roadmap

### Phase 0 — Foundation (week 1)
- [ ] Fresh full clone; branch `v1.0` from `v1.20.2`
- [ ] Apply dez branding in one atomic commit (Cargo.toml IDs, bundle-mac, identity/theme guard scripts, CI workflows)
- [ ] Port `dez_terminal_host` + `terminal_host_runtime.rs` (self-contained, low risk)
- [ ] Port workspace persistence patches (migrations + `prepare_windows_to_quit`)
- [ ] Wire CI: dez-build, dez-guards (identity check must gate merges)
- [ ] Write `FORK.md` (divergence ledger) and `SYNC-LEDGER.md`

### Phase 1 — Sidebar as dez crate (weeks 2–5)
- [ ] Move `sidebar` → `dez_sidebar`; audit its ~38k lines against current upstream pane/workspace APIs (it needed 3 re-fix cycles because it reached into upstream internals)
- [ ] Split the monolith: navigator / surfaces / activity / session-discovery become modules with test coverage — the fix-storm pattern came from one 21,500-line `sidebar.rs`
- [ ] Re-implement Activity states against upstream's *current* agent APIs (ACP 2.1 era), not the ~#63775-era APIs dez was patched against
- [ ] Acceptance: full feature parity with dez's README description, zero "Fix" commits in the port

### Phase 2 — Product polish (weeks 6–8)
- [ ] Cherry-pick dez's small UX wins: review-ready state (`c667db313f`), local Notes (`13651cfb27`), status bar spacing (`5cb37a1f35`), thread/terminal rename (`2c3c447f22`)
- [ ] Cross-project attention rollup + desktop notifications (Flint benchmark)
- [ ] First 2-week upstream sync cycle executed cleanly → proof the protocol works

### Phase 3 — v1.0 release (weeks 9–10)
- [ ] Own versioning starts at `1.0.0`; release pipeline (dez-release.yml) with macOS/Linux artifacts
- [ ] Written positioning: one-page README ("what dez is, what it isn't") — Gram's manifesto lesson: positioning is a feature
- [ ] Tag v1.0.0 only after: 2 clean sync cycles, zero known fix-of-fix chains, CI green on all platforms

---

## 5. Risk Register

| Risk | Mitigation |
|---|---|
| Sidebar port balloons (38k lines, coupled to pane APIs) | Timebox Phase 1; if >5 weeks, ship v1.0 with Activity-only and defer Surfaces |
| Upstream refactor breaks dez hooks (gpui generics, OsWatcher) | Ledger makes impact findable; sync cadence keeps diffs small |
| Agent-driven commit workflow reintroduces fix storms | Rule: every PR must pass CI + identity check; "Fix" commits on own features require a linked failing test |
| Scope creep | Non-goals list in §2 is binding until v1.0 ships |

---

## 6. Salvage Manifest (exact artifacts from old dez)

| Artifact | Path in old dez | Action |
|---|---|---|
| Sidebar feature | `crates/sidebar/` | Port → `dez_sidebar`, split + test |
| Terminal host | `crates/dez_terminal_host/`, `crates/zed/src/terminal_host_runtime.rs` | Port as-is |
| Persistence patches | `crates/workspace/src/persistence.rs` (dez migrations), `f6aa8fe32b`, `7679f3f761` | Cherry-pick |
| Branding | `crates/zed/Cargo.toml`, `script/bundle-mac` (`de3a05ff7b`) | Re-apply atomically |
| CI | 5 `dez-*.yml` workflows, `script/dez-identity-check`, `script/dez-theme-check` | Re-add, wire as merge gates |
| UX patches | `c667db313f`, `13651cfb27`, `5cb37a1f35`, `2c3c447f22` | Cherry-pick in Phase 2 |
| Everything else | — | Discard; re-derive from upstream |
