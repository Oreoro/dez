# Dez development

This section defines Dez's product direction and the order of work. Read the
documents according to their authority, not their age.

## Document authority {#document-authority}

1. [Fork Notes](./fork-notes.md) contains permanent product and architecture
   decisions. It wins when another plan or historical document conflicts.
2. [v0.0.4 External Sessions](./v0.0.4-external-sessions.md) is the current
   release train and its ordered acceptance gates. It does not override Fork
   Notes.
3. [v0.0.3 Production Readiness](./v0.0.3-production-readiness.md) preserves
   the previous hardening train and its evidence. It is historical input, not a
   competing execution order.
4. [Roadmap](./roadmap.md) is the long-range execution record. Keep its
   discoveries, decisions, and verification current while work is active, but
   do not promote deferred work into the current release without a product
   decision.
5. [v0.0.2 Completion Plan](./v0.0.2-completion-plan.md),
   [v0.0.2 Source Ledger](./v0.0.2-active-plan.md), and
   [Installed-build UX Audit](./v0.0.2-runtime-ux-recovery-plan.md) preserve the
   previous train's source claims and installed-build findings. They are
   historical inputs, not competing execution orders.
6. [Product Strategy](./product-strategy.md) records the market hypothesis,
   initial customer, product loop, and measures of product fit. These are
   hypotheses and should change when evidence contradicts them.
7. [Upstream Synchronization](./upstream-sync.md) defines the permanent merge
   train and release provenance requirements.
8. [Upstream Feature Ledger](./upstream-ledger.md) records the current merge
   target, conflict inventory, and capability treatment.
9. [Architecture Baseline](./architecture-baseline.md) maps the current code to
   Dez ownership, records gaps, and identifies safe seams for the next change.
10. [Codex Terminal Adapter](./codex-adapter.md) documents the opt-in structured
    lifecycle feed, trust boundary, retention limits, and current constraints.
11. [Live Preview and Agent Model](./live-preview-and-agent-model.md) defines
    the terminal-first recommendation and the first honest embedded-preview
    vertical slice. It is deferred beyond v0.0.4, not evidence that an embedded
    browser exists.
12. [Consolidated Plan Reconciliation](./consolidated-plan-reconciliation.md)
    records how the revised long-range plan was adopted, adapted, deferred, or
    rejected. It is an intake record, not a competing source of truth.
13. [Dez v0.0.1](../dez-v0.0.1.md) is the historical first-release snapshot
    and launch checklist. It does not describe the active v0.0.4 train.
14. [Release Evidence](./release-evidence.md) records direct build, runtime,
    visual, coexistence, and packaging proof for the current release candidate.
15. [v0.0.1 Release Runbook](./v0.0.1-release-runbook.md) preserves historical
    release notes, recovery semantics, artifact identity, checksums, known
    limitations, rollback, and promotion gates.

The older Superzed and Canvas documents remain design research and
implementation history. They do not override this section.

## Working rule {#working-rule}

Before changing workspace, project, window, terminal, pane, search, Git, agent,
persistence, opening, bundle, update, or release behavior:

1. Read the Fork Notes.
2. Read the active Completion Plan milestone and relevant long-range roadmap
   context.
3. Audit current upstream-compatible code instead of trusting stale symbol or
   file names.
4. Record a discovery before continuing when it invalidates the active plan.
5. Keep changes independently reviewable and preserve a working rollback path.

Do not rewrite the Fork Notes to excuse an implementation shortcut. Record a
deliberate product-direction change in its decision log first.
