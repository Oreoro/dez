# dez v1.0 — The Merge Plan

*Best of Gram + Flint, base = Flint, identity = dez's workspace structure.*

---

## 1. The Core Decision

**Base: Flint** (not Zed, not Gram, not dez).

Why:
- dez's only uniqueness is the **workspace structure** — a Chrome-like sidebar/tab bar organizing the editor around agent sessions and surfaces.
- dez's current sidebar reads `acp_thread::ThreadStatus`, `agent::ThreadStore`, `agent_ui::*` (44 references in `sidebar.rs`). If we based dez on Zed or Gram and removed native AI/ACP, the sidebar's data source would vanish.
- **Flint already removed the native ACP/AI stack AND already rebuilt the replacement primitives**: `agent_threads` with `attention_detection.rs` (Working/Idle/Blocked states, 782 lines, manifest-driven per-agent), terminal control, thread history for Claude/Codex/Pi/OpenCode, cross-project attention rollup, desktop notifications.
- So dez on Flint = dez's Activity/Agent features get re-pointed from ACP to `agent_threads` — a *port*, not a *rebuild*. dez on Zed-minus-AI = a rebuild from zero.

**What dez contributes (the identity):**
- The workspace structure: sidebar as the organizing surface, Home/Files/Git/Settings as workspace-owned tabs, "Surfaces" (movable/splittable main work area), multi-root workspace headers with live git branch + changed-file counts.
- The Chrome-like bar concept: `platform_title_bar`-integrated tab strip where any Zed feature (files, diffs, terminals, threads, settings) is a tab — Flint keeps terminals in center panes but has no unified bar concept; dez does.
- Session discovery-and-attach for tmux/Herdr/cmux (Flint *manages* its own threads; dez *attaches to anything* — complementary, keep both).
- `dez_terminal_host` daemon (clean, self-contained).

**What Flint contributes (the base):**
- The AI/ACP-amputated codebase (~53 crates already removed, breakage already fixed).
- `agent_threads` + attention detection + `flintctl` control protocol — the non-ACP session backbone.
- Terminal-as-center-pane workspace item registration.
- Release pipeline, self-versioning, PR-per-change discipline.

**What Gram contributes (the method):**
- Complete-surface rename discipline (binary, desktop file, config dir, env vars, icon) done in one atomic commit + guard script.
- `zed#NNNNN` provenance on cherry-picked upstream fixes + attributed CHANGELOG.
- Docs-as-product (mdbook site), mission page.
- Security posture: extension LSP/DAP permission flags (port this — Flint doesn't have it).

---

## 2. Architecture

### 2.1 Crate layout (target)

```
crates/
  dez/                      # was flint — main binary
  dez_actions/              # was flint_actions
  dez_terminal_host/        # from old dez (unchanged)
  dez_workspace_shell/      # NEW: the Chrome-like bar + surfaces + workspace tabs
  dez_sidebar/              # ported from old dez `sidebar`, split:
    ├── navigator.rs        #   Home/Files/Git/Settings tabs
    ├── surfaces.rs         #   movable/splittable main work area
    ├── activity.rs         #   session states — backed by agent_threads, NOT acp
    ├── discovery.rs        #   tmux/Herdr/cmux attach
    └── headers.rs          #   multi-root workspace headers
  agent_threads/            # Flint's, unchanged (the session backbone)
  agent_control_*/          # Flint's, unchanged
  ... (rest of Flint's tree, untouched)
```

### 2.2 The critical port: `activity.rs`

Old dez sidebar ↔ Flint mapping:

| Old dez (ACP-based) | New dez (Flint-based) |
|---|---|
| `acp_thread::ThreadStatus` | `agent_threads::AttentionState` (Working/Idle/Blocked) |
| `agent::ThreadStore`, `agent::Thread` | `agent_threads::store` thread records |
| `agent_ui::terminal_thread_metadata_store` | `agent_threads::history` (claude/codex/pi/opencode) |
| `agent_ui::threads_archive_view` | `agent_threads::panel` |
| `language_model::LanguageModelRegistry` | delete — no model selection in dez (Flint rule: agents own their config) |
| "Waiting for Permission" state | map to `Blocked` + terminal-bell heuristic; refine later |
| "Review-ready" state | NEW: wire to `git_ui` changed-file counts per thread worktree (Flint has `thread_worktree` concept in handoff.rs) |

Keep dez's richer state vocabulary as a *presentation layer* over Flint's 3-state machine where derivable; don't extend the state machine itself.

### 2.3 Divergence budget
- All dez logic in `dez*` crates. Touches to Flint/upstream crates limited to: wiring in `dez/src/main.rs`, workspace item registration, and ≤50-line marked hooks. Every divergence listed in `FORK.md` with file:line.
- Keep Flint's `flintctl` protocol but expose it as `dezctl` (thin rename; the skill content updates accordingly).

---

## 3. Execution Phases

### Phase 0 — Base swap (week 1)
- [ ] `dez 1.0/dez-v1` = fresh clone of Flint at `v0.11.3` (stable, tagged 2026-09-01) — full history kept.
- [ ] Atomic identity commit (Gram method): rename `flint`→`dez` across binary, crates `flint*`→`dez*`, bundle IDs `dev.dez.*`, desktop file, config dir decision (`.dez/` vs keep Flint's), env vars, icons. Add `script/dez-identity-check` (from old dez) as CI gate.
- [ ] Port `dez_terminal_host` + `terminal_host_runtime.rs` from old dez; wire into `dez/src/main.rs`.
- [ ] CI: dez-build + dez-guards green before anything else lands.

### Phase 1 — Workspace shell (weeks 2–4)
- [ ] Port old dez `sidebar` → `dez_sidebar`, **splitting the 21,471-line `sidebar.rs`** into the 5 modules above (the monolith is the root cause of dez's fix-storm history).
- [ ] Delete every ACP/agent_ui import; re-point Activity onto `agent_threads` (§2.2 mapping). `sidebar_tests.rs` (16,280 lines) ported module-by-module, rewritten where they assert ACP behavior.
- [ ] Build `dez_workspace_shell`: the Chrome-like bar. Flint already registers `TerminalView` as a serializable center-pane workspace item — extend that pattern so Files/Git/Settings/Threads are all bar-manageable tabs. This is dez's flagship; give it its own spec doc first (`docs/dez-workspace-shell.md`, Flint's openspec style).
- [ ] Acceptance: parity with old dez README §26–60 feature list, zero "Fix" commits.

### Phase 2 — Convergence features (weeks 5–6)
- [ ] Session discovery-and-attach (tmux/Herdr/cmux) alongside Flint's managed threads — "attach to anything" as the dez differentiator.
- [ ] Review-ready detection via git changed-file counts per thread worktree.
- [ ] Port Gram's extension LSP/DAP permission flags (security differentiation Flint lacks).
- [ ] Cross-project attention rollup already exists in Flint — surface it in the dez bar (badges on workspace headers).

### Phase 3 — Sync & release (weeks 7–8+)
- [ ] Adopt the sync protocol from PLAN.md §3.3 (2-week cadence, ledger, CI-gated) — but now the upstream is **Flint**, not Zed. Flint is the maintenance fork doing Zed-reconciliation; dez rides one layer up. Track both: Flint releases (fast) + upstream Zed security fixes Flint hasn't taken yet (rare, via cherry-pick with `zed#NNNNN` provenance).
- [ ] Release `1.0.0`: dez-release pipeline (adapt Flint's ci-release-pipeline spec), macOS/Linux artifacts.
- [ ] README + mission page (Gram style): "dez is the browser-like workspace for agent-driven coding. Agents own their config; dez owns the workspace."

---

## 4. Risk Register

| Risk | Mitigation |
|---|---|
| Sidebar port balloons (38k lines incl. tests) | Timebox Phase 1 to 4 weeks; fallback = ship v1.0 with activity.rs + navigator only, defer surfaces.rs |
| Flint moves fast (742 commits/3mo) — base drifts | Pin to `v0.11.3`; sync every 2 weeks per protocol; Flint's ledger discipline makes their diffs readable |
| AttentionState (3 states) can't express dez's 4-state UX | Presentation-layer mapping (§2.2); never fork the state machine |
| Rename breaks Flint's self-updates/release plumbing | Identity commit touches update URLs + channel names in the same atomic commit; verify with a test release build |
| Old dez tests assert ACP semantics | Port tests per-module; rewrite assertions against `agent_threads` behavior; delete tests that only test ACP plumbing |

---

## 5. What happens to the old dez repo
Freeze it. It remains the reference for the sidebar port and the UX patches (`c667db313f`, `13651cfb27`, `5cb37a1f35`, `2c3c447f22`). No new work, no merges — everything it did right is being re-derived cleanly in v1.0.
