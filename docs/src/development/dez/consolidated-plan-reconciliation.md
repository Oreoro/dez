# Consolidated Plan Reconciliation

This page records how the revised **Dez Product and Execution Plan** supplied
on 2026-07-22 was reconciled with the repository. The supplied document is a
planning input, not a replacement for the authority hierarchy in
[Dez development](./index.md). Its SHA-256 is
`5f698a25c90d0a4dee9f5bedc6a9e2246453a1934601c19897a5184fb6bde552`.

## Outcome {#outcome}

The plan is directionally compatible with Dez, but it cannot be adopted
verbatim. It resets implemented work, mixes permanent decisions with temporary
execution state, requires a build after every slice, and combines Session,
agent, and Run lifecycle states. Fork Notes remain permanent, the Roadmap
remains the live execution record, and builds remain grouped at an explicit
verification gate.

The plan does improve the product description, Evidence vocabulary, adapter
capability model, PMF measures, protocol requirements, and long-range
integration map. Those parts are adopted in the documents that own them.

## Collision and treatment {#collision-and-treatment}

| Proposal                                                               | Repository collision                                                                  | Treatment                                                                                                       |
| ---------------------------------------------------------------------- | ------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| One canonical plan replaces all prior Dez documents                    | Fork Notes permanently own product and architecture decisions; Roadmap owns execution | Rejected. Keep the linked hierarchy and preserve this page as the intake record                                 |
| Complete native development environment with direct and delegated work | Existing strategy emphasizes the agent-supervision wedge                              | Adapted. Dez is a complete environment; multi-agent supervision remains the first customer and delivery wedge   |
| One App Session and future viewport windows                            | Matches Fork Notes, but live entities remain window-composed today                    | Adopted as target; do not retain GPUI entities in `AppSession` or disable useful multi-window behavior          |
| Run states include Detached, Reconnecting, Missing, and Incompatible   | These are Session transport/lifecycle facts, not purposeful-work facts                | Rejected as one enum. Preserve separate Session, agent, attention, and review state and derive Run presentation |
| Attention clears only when the underlying condition resolves           | Existing UI also needs unread/acknowledged behavior                                   | Adapted. Keep `requires_action`, acknowledgement, mute, and resolution distinct                                 |
| Change Set and Environment become permanent primitives                 | No PMF owner or acceptance slice exists for a new database                            | Adopted as relationships and vocabulary; implementation is deferred and Git remains authoritative               |
| Build and run after every reviewable slice                             | User direction and Fork Notes defer expensive builds                                  | Rejected. Continue formatting/static checks, then run one consolidated build and live gate                      |
| Route New Window to New Workspace in one singleton viewport            | Upstream supports useful multi-window workflows                                       | Adapted. Windows become viewport adapters over the same App Session; do not remove the capability as a shortcut |
| Complete Dez storage isolation immediately                             | v0.0.1 intentionally retains the Superzed storage boundary                            | Deferred until a transactional, reversible migration exists                                                     |
| Browser, DevPod, Dagger, Jujutsu, relay, mobile, and team coordination | Exceeds the PMF vertical slice                                                        | Retained as post-PMF hypotheses, not current implementation scope                                               |

## Milestone crosswalk {#milestone-crosswalk}

The supplied plan's blank checklist is not repository truth. Current status is:

| Supplied milestone               | Current evidence                                                                                                  | Remaining acceptance                                                                                       |
| -------------------------------- | ----------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| 0 Baseline and audit             | Architecture baseline, upstream ledger, merge rehearsal, identity checker, and guard workflow exist               | Fresh upstream decision, consolidated build evidence, and release provenance                               |
| 1 Upstream and identity          | Public Dez binary, bundle, schemes, channels, and updater guards exist                                            | Next upstream merge, remote identity audit, first-party finish, and build proof                            |
| 2 Durable App Session            | Restore barrier, ordered restore lifecycle, and identity-keyed membership registry exist in source                | Workspace order/active ownership, unresolved records, blank Workspace durability, and viewport composition |
| 3 Scoped Project and Evidence    | Each Workspace owns a Project; review evidence distinguishes roots and terminal cwd                               | Workspace `EvidenceSet`, lifecycle recomputation, Host paths, shared-store scope, and demand tests         |
| 4 Opening and universal Surfaces | Pane/Canvas repair, panel-to-pane work, and initial open ordering exist                                           | Evidence-aware routing, tool scoping, Surface movement, and mixed-layout proof                             |
| 5 Local Host proof               | Versioned authenticated helper, PTY ownership, bounded replay, and lifecycle commands exist behind an opt-in flag | Compile, run, slow-client/liveness proof, and GUI-disconnect scenario                                      |
| 6 Persistent terminals           | Host/Session references, recovery states, detach/reattach, and explicit terminate exist in source                 | Full restart demonstration and default-backend decision                                                    |
| 7 PMF Run slice                  | Codex hooks, attention, bounded activity, observed checks, and deterministic review briefs exist                  | Objective/provenance relations, file/Git evidence, second adapter, and hero-flow proof                     |
| 8 ACP and integrated agents      | Upstream ACP/native agent substrate is retained                                                                   | Projection into common Host/Session/Run relationships without duplicate ownership                          |
| 9 Power-user operations          | Session Rail, search, layouts, and upstream worktree actions provide fragments                                    | Conflict Radar, broker, templates, completion gates, service discovery, and authenticated CLI              |
| 10 Remote Hosts                  | Zed SSH/remote/container substrate is retained                                                                    | Host abstraction, Dez remote identity, multi-Host scope, terminal continuity, and remote Evidence          |
| 11–12 Platform/commercial        | No required PMF implementation                                                                                    | Remain deferred until retention and willingness-to-pay evidence                                            |

## Reconciled execution order {#reconciled-execution-order}

1. Finish source-level App Session ownership: order, active Workspace,
   unresolved records, empty Workspace durability, and viewport associations.
2. Add the minimal Workspace Evidence model for open files and terminal cwd,
   including provenance, Host identity, lifecycle recomputation, and no eager
   heavy work.
3. Finish the already-started local Host, persistent terminal, attention, and
   review vertical slice without expanding it into a broad runtime platform.
4. Run the consolidated compile, test, GUI restart, visual, security, identity,
   and packaging gate.
5. Only after that evidence, project ACP/native agents into the common model and
   begin conflict, worktree, remote, and template work.

Change Set storage, Environment orchestration, browser automation, platform
providers, relay, mobile, and team policy are not prerequisites for Dez
v0.0.1.
