# Gram & Flint — Fork Strategy Deep-Dive

*Companion to PLAN.md. Focused analysis of the two reference forks, ignoring dez.*

---

## Side-by-side

| | **Gram** (Codeberg) | **Flint** (GitHub) |
|---|---|---|
| Fork point | Full Zed history (35k commits kept) | ~2026-06-05, full history kept |
| Identity crates | `gram`, `app_actions`, `git_graph`, `panel`, `nc`, `recent_files`, `docs` (7 added) | `flint*`, `agent_*`, `localization`, `csv_preview`, `latex_render` (12 added) |
| Crates removed | **108** of 245 (all AI/LLM, collab, telemetry, cloud, auto-update, call, audio, sandbox, sidebar, web_search) | **53** of 245 (all AI/LLM/provider crates, collab, ACP/agent stack, web_search, context_server, eval) |
| Positioning | "No AI, no telemetry, no subscriptions" — privacy/hand-coding manifesto | "Terminal-first workspace for CLI coding agents" — agents run in terminals, editor organizes them |
| Own features | git history panel (`git_graph`), commit ref chips, special-comment highlighting, smart project-panel sort, extension LSP/DAP permission flags | center-pane terminal threads, Agent Threads panel w/ cross-project attention rollup + notifications, `flintctl` agent control CLI/protocol/skill, cross-agent handoff docs, remote agent threads over SSH/WSL |
| Upstream sync | Curated cherry-picks referenced as `zed#NNNNN`; no merge-from-upstream; CHANGELOG attributes lineage | Documented "Waves 1–8" integration program + ledger; one big `Reconcile Zed v1.13.1` merge (Aug 1); upstream remote pinned to `main` only |
| Cadence | ~296 commits since Aug 1; release 1.0.0→3.3.0 in 6 months | ~742 commits in 3 months (PR-per-change, `fix/*`/`feature/*` branches); self-versioned v0.11.3 |
| Last commit | 2026-09-19 | 2026-09-10 |
| Docs | mission.md manifesto, full user docs site (mdbook), HUMAN-CONTRIBUTION-POLICY, glossary | `docs/terminal-first-fork.md` design doc, `openspec/` (specs + changes: remove-ai-features, terminal-first-workspace, terminal-agent-threads, ci-release-pipeline, markdown-inline-editing, focused-product-surface…), AGENTS.md/CLAUDE.md/GEMINI.md for agent contributors |
| Governance note | AI-generated PRs explicitly banned; GPL additions on top of Apache/GPL Zed | Solo dev, spec-driven, every change through PR |

---

## What Gram does that's worth copying

1. **Complete surface rename, done once.** Binary, desktop file, config dir `.gram/`, env var `GRAM_SYSTEM_EXTENSIONS_DIR`, website, icon. One identity-check script guards it.
2. **Cherry-pick provenance.** Every ported upstream fix keeps `zed#NNNNN` in the commit message and CHANGELOG. Makes dedup + future merges trivial.
3. **Pruning with a written recovery plan.** CONTRIBUTING.md is literally a checklist: strip AI → make breakage work again → replace losses with open alternatives. They accepted short-term loss for long-term identity.
4. **Security as differentiation.** After a PSA about extensions downloading executables, they built permission flags for extension-provided LSPs/DAPs — a trust-boundary feature upstream ignores.
5. **Docs site as product surface.** mdbook with per-topic pages (globs, multibuffers, supertab, uninstall…). A fork that documents itself feels like a product.

## What Flint does that's worth copying

1. **Design doc before code.** `terminal-first-fork.md` states purpose, product shape, explicit "the fork should NOT become another native AI client", keep/cut lists, even settings schemas. Every feature traces to it.
2. **openspec/ discipline.** `specs/` = durable product specs; `changes/` = in-flight proposals (e.g. `add-flintctl-terminal-control`). Spec-driven development at solo scale.
3. **Wave-based upstream reconciliation with a ledger.** Each upstream adaptation is recorded ("Record terminal mouse-mode adaptation", "Record Wave 8 pull request"), then a single reconcile merge closes the wave. Debt is visible, not ambient.
4. **Fork logic quarantined.** All divergence lives in `flint*`/`agent_*` crates; renames done early; upstream crates stay upstream-shaped.
5. **Own release pipeline from day one.** Self-versioning, stable+nightly, own install script, auto-update pointed at their GitHub — not waiting on upstream infra.
6. **Platform-bug tax acknowledged.** Wayland backpressure/surface-hang fixes dominate recent commits — forks inherit the platform-bug burden alone; budget for it.

## Where each one hurts (anti-lessons)

- **Gram**: 108-crate deletion means months of "make what broke work again" (their own roadmap admits it). Don't prune more than your identity requires.
- **Flint**: 8-week gap between reconciliations created conflict debt (their v1.13.1 reconcile was heavy). Sync cadence matters more than sync method.
- **Both**: platform-layer bugs (Wayland/macOS/Windows) are the recurring tax; neither escapes it.

## Direct implications for dez v1.0

1. **Fork from upstream stable tag, keep full history** (both do this).
2. **Decide the subtraction line now.** dez's identity is agent-session workspace UX, not AI plumbing → keep upstream's agent stack (unlike Gram/Flint, who cut it — dez's differentiator *builds on* it), but cut anything unrelated to the workspace/product identity. Don't copy their AI-amputation; copy their *decisiveness*.
3. **Quarantine dez code into `dez_*` crates** (Flint) with a `FORK.md` divergence ledger (Flint's wave ledger) and `zed#NNNNN` cherry-pick provenance (Gram).
4. **Sync cadence: 2 weeks, ledgered, CI-gated** — tighter than Flint, more structured than Gram.
5. **Write the design doc first** (`docs/dez-v1.md` in the style of `terminal-first-fork.md`): purpose, product shape, keep/cut, non-goals — before any porting.
6. **One atomic identity commit + guard script** (Gram), **own release pipeline from Phase 0** (Flint).
