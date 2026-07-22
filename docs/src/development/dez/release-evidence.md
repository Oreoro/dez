# Dez v0.0.1 Release Evidence

This log records direct evidence for the consolidated release gate. A source
claim is not a runtime claim, and an unchecked scenario remains unverified.

## Frozen source and intended artifacts {#frozen-source-and-intended-artifacts}

- Integration merge: `2be63cfea347006e407934754086bbef62d482c2`
- Post-merge documentation commit: `5ed16fd907`
- Incorporated upstream: `9d0ef37a25711c00bf6d1ba1142e9de4f4a122a9`
- Lockfile: repository `Cargo.lock` at the tested commit
- Toolchain: `rustc 1.95.0`, `cargo 1.95.0`, host
  `aarch64-apple-darwin`
- Intended development executable:
  `/Users/test/Documents/zed 3.0/target/debug/dez`
- Intended terminal Host helper:
  `/Users/test/Documents/zed 3.0/target/debug/dez-terminal-host`
- Intended development bundle:
  `/Users/test/Documents/zed 3.0/target/aarch64-apple-darwin/debug/bundle/osx/Dez Dev.app`
- Legacy untracked artifact excluded from all evidence:
  `/Users/test/Documents/zed 3.0/dist/Superzed.app`

No Dez, Superzed, or Zed application process was running at the pre-build
capture. Only the intended paths above may be launched during this gate.

## Pre-build coexistence correction {#pre-build-coexistence-correction}

The preflight found that bundle metadata and the URL registration action still
claimed official `zed://` schemes. Dez now owns only its channel-specific
`dez://` scheme. Explicitly supplied legacy links remain parseable for
compatibility, but Dez no longer registers itself as their operating-system
handler. The dormant CLI installer also targets `/usr/local/bin/dez` and uses
Dez copy, so it cannot overwrite the official Zed CLI namespace.

## Automated gates {#automated-gates}

- [x] `cargo fmt --all -- --check`
- [x] `git diff --check`
- [x] `cargo metadata --no-deps --format-version 1`
- [x] `./script/dez-identity-check`
- [x] Focused Prettier checks for canonical Dez documentation
- [ ] Focused unit and migration tests
- [ ] `cargo clippy` at modified-crate scope
- [ ] `cargo build -p zed --bin dez`
- [ ] `cargo build -p dez_terminal_host --bin dez-terminal-host`
- [ ] Debug bundle build and static bundle audit

## Runtime and manual gates {#runtime-and-manual-gates}

- [ ] Intended-binary first and normal launch
- [ ] Restored and empty-workspace launch
- [ ] Offline, failed-Host, and incompatible-Host states
- [ ] Persistent terminal GUI-exit/restart/reattach proof
- [ ] Structured Codex attention/review/restart proof
- [ ] Visual state matrix and keyboard/pointer parity
- [ ] Accessibility tree, focus order, labels, contrast, and reduced-motion audit
- [ ] Official Zed coexistence without scheme, bundle, storage, CLI, or updater
      takeover
- [ ] Bundle identity, helper inclusion, entitlements, signing, install, and
      uninstall audit

## Known external release dependencies {#known-external-release-dependencies}

Public Developer ID signing and Apple notarization require Dez publisher
credentials. An ad-hoc local signature can validate bundle structure and local
launch behavior, but it is not evidence of public notarization. Design-partner
testing also requires actual target users and is recorded separately from local
engineering verification.
