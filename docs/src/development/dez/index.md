# Dez development

This section defines Dez's product direction and the order of work. Read the
documents according to their authority, not their age.

## Document authority {#document-authority}

1. [Fork Notes](./fork-notes.md) contains permanent product and architecture
   decisions. It wins when another plan or historical document conflicts.
2. [Roadmap](./roadmap.md) is the living execution plan. Keep its progress,
   discoveries, decisions, and verification current while work is active.
3. [Product Strategy](./product-strategy.md) records the market hypothesis,
   initial customer, product loop, and measures of product fit. These are
   hypotheses and should change when evidence contradicts them.
4. [Upstream Synchronization](./upstream-sync.md) defines the permanent merge
   train and release provenance requirements.
5. [Upstream Feature Ledger](./upstream-ledger.md) records the current merge
   target, conflict inventory, and capability treatment.
6. [Architecture Baseline](./architecture-baseline.md) maps the current code to
   Dez ownership, records gaps, and identifies safe seams for the next change.
7. [Codex Terminal Adapter](./codex-adapter.md) documents the opt-in structured
   lifecycle feed, trust boundary, retention limits, and current constraints.
8. [Consolidated Plan Reconciliation](./consolidated-plan-reconciliation.md)
   records how the revised long-range plan was adopted, adapted, deferred, or
   rejected. It is an intake record, not a competing source of truth.
9. [Dez v0.0.1](../dez-v0.0.1.md) is the current release snapshot and launch
   checklist.
10. [Release Evidence](./release-evidence.md) records direct build, runtime,
    visual, coexistence, and packaging proof for the current release candidate.
11. [v0.0.1 Release Runbook](./v0.0.1-release-runbook.md) contains release
    notes, recovery semantics, exact artifact identity and checksums, known
    limitations, safe verification, rollback, and public-preview promotion
    gates.

The older Superzed and Canvas documents remain design research and
implementation history. They do not override this section.

## Working rule {#working-rule}

Before changing workspace, project, window, terminal, pane, search, Git, agent,
persistence, opening, bundle, update, or release behavior:

1. Read the Fork Notes.
2. Read the active roadmap milestone.
3. Audit current upstream-compatible code instead of trusting stale symbol or
   file names.
4. Record a discovery before continuing when it invalidates the active plan.
5. Keep changes independently reviewable and preserve a working rollback path.

Do not rewrite the Fork Notes to excuse an implementation shortcut. Record a
deliberate product-direction change in its decision log first.
