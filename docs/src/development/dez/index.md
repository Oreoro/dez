# Dez development

This section defines Dez's product direction and the order of work. Read the
documents according to their authority, not their age.

## Document authority {#document-authority}

1. [Fork Notes](./fork-notes.md) contains permanent product and architecture
   decisions. It wins when another plan or historical document conflicts.
2. [v0.5 Surface Readiness](./v0.5-surface-readiness.md) is the active
   source-readiness and product-polish plan. It does not claim that a v0.5
   artifact exists or supersede exact v0.4 build evidence.
3. [v0.4 Readiness](./v0.4-readiness.md) preserves the latest exact-artifact
   release lane and its still-open promotion gates.
4. [Native Surface Contract](./surface-contract.md) is the source-controlled
   wireframe and interaction contract for every primary Dez surface.
5. [v0.2 Workspace Polish](./v0.2-workspace-polish.md) preserves the previous
   native Workspace source-polish lane and its still-relevant ownership rules.
6. [v0.1 Product Hardening](./v0.1-product-hardening.md) preserves the
   release ladder and acceptance gates that established the current baseline.
7. [v0.0.4 External Sessions](./v0.0.4-external-sessions.md) preserves the
   historical ownership baseline and the exact build/runtime gates that fed the
   v0.1 candidate.
8. [v0.0.3 Production Readiness](./v0.0.3-production-readiness.md) preserves
   the previous hardening train and its evidence. It is historical input, not a
   competing execution order.
9. [Roadmap](./roadmap.md) is the long-range execution record. Keep its
   discoveries, decisions, and verification current while work is active, but
   do not promote deferred work into the current release without a product
   decision.
10. [v0.0.2 Completion Plan](./v0.0.2-completion-plan.md),
   [v0.0.2 Source Ledger](./v0.0.2-active-plan.md), and
   [Installed-build UX Audit](./v0.0.2-runtime-ux-recovery-plan.md) preserve the
   previous train's source claims and installed-build findings. They are
   historical inputs, not competing execution orders.
11. [Product Strategy](./product-strategy.md) records the market hypothesis,
   initial customer, product loop, and measures of product fit. These are
   hypotheses and should change when evidence contradicts them.
12. [Upstream Synchronization](./upstream-sync.md) defines the permanent merge
   train and release provenance requirements.
13. [Upstream Feature Ledger](./upstream-ledger.md) records the current merge
    target, conflict inventory, and capability treatment.
14. [Architecture Baseline](./architecture-baseline.md) maps the current code to
    Dez ownership, records gaps, and identifies safe seams for the next change.
15. [Codex Terminal Adapter](./codex-adapter.md) documents the explicit
    structured lifecycle feed, packaged Host boundary, trust and retention
    limits, and current constraints.
16. [Live Preview and Agent Model](./live-preview-and-agent-model.md) defines
    the terminal-first recommendation and the first honest embedded-preview
    vertical slice. It is deferred beyond v0.0.4, not evidence that an embedded
    browser exists.
17. [Consolidated Plan Reconciliation](./consolidated-plan-reconciliation.md)
    records how the revised long-range plan was adopted, adapted, deferred, or
    rejected. It is an intake record, not a competing source of truth.
18. [Dez v0.0.1](../dez-v0.0.1.md) is the historical first-release snapshot
    and launch checklist. It does not describe the active v0.5
    surface-readiness lane.
19. [Release Evidence](./release-evidence.md) records direct build, runtime,
    visual, coexistence, and packaging proof for the current release candidate.
20. [v0.0.1 Release Runbook](./v0.0.1-release-runbook.md) preserves historical
    release notes, recovery semantics, artifact identity, checksums, known
    limitations, rollback, and promotion gates.

The older Superzed and Canvas documents remain design research and
implementation history. They do not override this section.

## Working rule {#working-rule}

Before changing workspace, project, window, terminal, pane, search, Git, agent,
persistence, opening, bundle, update, or release behavior:

1. Read the Fork Notes.
2. Read the active Surface Readiness milestone and relevant long-range roadmap
   context.
3. Audit current upstream-compatible code instead of trusting stale symbol or
   file names.
4. Record a discovery before continuing when it invalidates the active plan.
5. Keep changes independently reviewable and preserve a working rollback path.

Do not rewrite the Fork Notes to excuse an implementation shortcut. Record a
deliberate product-direction change in its decision log first.
