# The dez mission

> Agents own their config. dez owns the workspace.

dez is a fork of [Flint](https://github.com/shenghsi/flint), which is itself a
terminal-first fork of [Zed](https://github.com/zed-industries/zed). This
document explains why dez exists on top of that lineage.

## Why another fork?

Flint answered "what if the IDE organized terminal-based coding agents?"
Gram answered "what if an editor shipped none of the AI, telemetry, or cloud
apparatus?" Both are good answers. dez's answer is different:

**What if the workspace itself was the product?**

Conventional IDEs treat the editor as the center and bolt panels around it.
Agent-native tools replace the editor with a chat surface. dez does neither. In
dez, the workspace is a browser-like shell: a bar of addressable tabs and a
sidebar of surfaces — files, diffs, terminals, agent threads, settings — where
any Zed capability can be a tab. Agents run in real terminals with their own
authentication and configuration, exactly as their authors intended.

## Principles

1. **Agents own their config.** Model selection, credentials, permissions, and
   tool policy belong to the CLI or TUI running in the terminal. dez does not
   become another native AI client.
2. **The workspace is the product.** The bar and sidebar are the primary
   surface, not a project panel with tabs bolted on. Every Zed feature should
   be reachable as a first-class tab.
3. **Subtract to focus.** dez inherits Flint's removal of Zed's hosted models,
   collab, accounts, and telemetry. We do not re-add them. The AI plumbing we
   keep exists only to run the workspace, never to sell a subscription.
4. **Attach to anything.** dez discovers and attaches to tmux, Herdr, and cmux
   sessions; it does not require ownership of your terminals.
5. **Trust boundaries are features.** Extensions are powerful and
   under-audited. dez gates extension binary downloads, path lookups, and
   executable permissions behind the same settings that govern language servers
   and debug adapters (ported from Gram), and keeps Zed's extension ABI intact
   so the ecosystem still works.
6. **Upstream is a river, not a wall.** dez tracks Flint as its upstream,
   reconciles on a fixed cadence with a written ledger, and keeps `zed#NNNNN`
   provenance on every ported fix.

## What dez is not

- Not an AI product. No hosted models, no chat, no subscription.
- Not a telemetry product. No analytics, no accounts, no auto-updates calling
  home beyond the user's own update channel.
- Not a rewrite. dez builds on Zed's GPUI, editor, LSP, git, and extension
  machinery; we change the workspace around them, not the foundations.

## Lineage and credit

dez is possible only because of Zed Industries' work and Flint's maintenance.
Where we take from Gram, the commit and its author are credited in the history
and recorded in `FORK.md`. Upstream fixes keep their original references.