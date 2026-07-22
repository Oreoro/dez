# Dez v0.0.1 Release Evidence

This log records direct evidence for the consolidated release gate. A source
claim is not a runtime claim, and an unchecked scenario remains unverified.

## Frozen source and intended artifacts {#frozen-source-and-intended-artifacts}

- App and Host build commit: `da562e14bb403af815cbab9802225dda0b2418c8`
- App and Host build tree: `56cb7714537073db1aeff2e6cf24809c9a79fb95`
- Packaging and permission-copy commit: `ce11c4ed3db138fe6ca0a8890bfb6db8b7f7bd52`
- Integration merge: `2be63cfea347006e407934754086bbef62d482c2`
- Incorporated upstream: `9d0ef37a25711c00bf6d1ba1142e9de4f4a122a9`
- `Cargo.lock` SHA-256:
  `e2d477160b09d24220d13113a04ab067a4eb9c8685173b4e30b20923b5f01901`
- Toolchain: `rustc 1.95.0 (59807616e 2026-04-14)`,
  `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`, host
  `aarch64-apple-darwin`
- Intended development executable:
  `/Users/test/Documents/zed 3.0/target/debug/dez`
- Intended terminal Host helper:
  `/Users/test/Documents/zed 3.0/target/debug/dez-terminal-host`
- Intended development CLI:
  `/Users/test/Documents/zed 3.0/target/debug/cli`
- Intended development bundle:
  `/Users/test/Documents/zed 3.0/target/debug/bundle/osx/Dez Dev.app`
- Legacy untracked artifact excluded from all evidence:
  `/Users/test/Documents/zed 3.0/dist/Superzed.app`

Only the intended raw `target/debug/dez` executable was launched. The excluded
Superzed artifact and the generated Dez bundle were not opened.

## Build evidence {#build-evidence}

The app and helper completed together, warning-free, in 25 minutes 31 seconds:

```sh
cargo build --locked --profile dev \
  --config 'profile.dev.debug=0' \
  --config 'profile.dev.incremental=false' \
  --config 'profile.dev.codegen-units=1' \
  --config 'profile.dev.split-debuginfo="off"' \
  --config 'profile.dev.build-override.debug=0' \
  --config 'profile.dev.build-override.codegen-units=1' \
  --config 'profile.dev.build-override.split-debuginfo="off"' \
  -p zed --bin dez -p dez_terminal_host --bin dez-terminal-host -j1
```

The CLI completed separately with the same locked, non-incremental,
single-codegen-unit profile. Unsigned build artifacts are arm64 Mach-O files:

| Artifact            | Size | SHA-256                                                            |
| ------------------- | ---- | ------------------------------------------------------------------ |
| `target/debug/dez`  | 1.0G | `9b209289555689fdbdc67bc3d9514b772d4bdfec979bd82f3706990add2ba186` |
| `target/debug/cli`  | 12M  | `e9bde80f1d951a6f9b7da53b0175de23db31c642b368c67c19451a04fbc9eaed` |
| `dez-terminal-host` | 13M  | `2ac370c716c76e6a37979ab8e8c5454cdabc42847a96a949b14b20e4f7177ea8` |

## Debug bundle and coexistence evidence {#debug-bundle-and-coexistence-evidence}

`script/bundle-mac -d` now reuses a complete host debug artifact set, restores
the temporary manifest on bundler failure, works around the pinned bundler's
invalid terminal-colour failure through its plain-output path, omits the
release-only remote server, and creates the bundle without a second app build.

The resulting 1.0G bundle passed `codesign --verify --deep --strict` and has:

- bundle identifier `dev.dez.Dez-Dev`;
- display and bundle name `Dez Dev`;
- version `0.0.1`;
- executable `dez`;
- only the `dez-dev` URL scheme;
- arm64 `dez`, `cli`, `dez-terminal-host`, and bundled `git` executables;
- the required document icon and Dez document labels;
- an ad-hoc local signature with no team identifier;
- privacy prompt copy that identifies a developer tool launched from Dez
  instead of ambiguously claiming that “an application in Dez” requested data.

Signed bundle-executable SHA-256 values are:

| Bundle executable   | SHA-256                                                            |
| ------------------- | ------------------------------------------------------------------ |
| `dez`               | `331d757b8367c67ba2b5189c17abe7bfb0b3a45bcd35c14110e497f0bd3aeef4` |
| `cli`               | `bd55c3c41241664551d6e971fb20869e2c323ec9947b2e96e403ff358b18b2f0` |
| `dez-terminal-host` | `d0b36ef644c1321983b8aaee2bd05d5e374749de109308e65554445c573dac0b` |
| `git`               | `831f1e097bde9599afe7c298637fdc7f26f8788846eec78365d50d470f29bc47` |

The signed copies differ byte-for-byte from the raw Cargo outputs because the
ad-hoc signing step rewrites Mach-O signatures. Static identity checks pass,
but an installed coexistence exercise with official Zed remains open.

## Runtime evidence {#runtime-evidence}

The exact raw executable was launched with
`DEZ_EXPERIMENTAL_TERMINAL_HOST=1`. The first runtime pass found and then drove
two real fixes: stale onboarding actions no longer make the built-in keymap
panic, and `auto_connect = false` now prevents eager cloud-provider
authentication. The final launch emitted neither failure.

The app launched the exact sibling helper at
`target/debug/dez-terminal-host`. The accepted v0.0.1 compatibility boundary
uses `/Users/test/.local/state/Superzed/terminal-host`; the directory was mode
`0700`, and `auth.token` and `local.sock` were mode `0600`.

Host survival and reuse were observed directly:

1. GUI PID `48519` connected to helper PID `48768`.
2. After `SIGTERM` to the GUI, helper PID `48768` stayed alive and reparented
   to PID 1 with the same socket and Host identity.
3. Relaunching the exact GUI as PID `50092` reused helper PID `48768`; exactly
   one helper existed and Host ID
   `d9670db8-e498-5537-a9d8-f99ad098f4aa` remained unchanged.

This proves Host-process survival and reuse. It does **not** prove the complete
terminal acceptance scenario because the locked desktop prevented creating an
ordinary hosted PTY and capturing Session ID, child PID, output cursor, replay,
and same-process reattachment.

## Automated gates {#automated-gates}

- [x] `cargo fmt --all -- --check`
- [x] `git diff --check`
- [x] `cargo metadata --offline --no-deps --format-version 1`
- [x] `./script/dez-identity-check`
- [x] `bash -n script/bundle-mac`
- [x] Focused Prettier checks for the canonical Dez documentation slices
- [x] Focused tests: 15 terminal client/model tests, 8 Host/helper tests, and
      three Session Rail terminal lifecycle tests
- [ ] Modified-crate `cargo clippy` (not completed; the consolidated graph
      exceeded the remaining local storage budget)
- [x] Intended app and helper consolidated build
- [x] Intended CLI build
- [x] Debug bundle build and static bundle audit

## Runtime and manual gates {#runtime-and-manual-gates}

- [x] Intended raw-binary first and corrected normal launch
- [ ] Restored and empty-workspace interaction audit
- [ ] Offline, failed-Host, and incompatible-Host rendered states
- [ ] Persistent terminal GUI-exit/restart/reattach proof
- [ ] Structured Codex attention/review/restart proof
- [ ] Visual state matrix and keyboard/pointer parity
- [ ] Accessibility tree, focus order, labels, contrast, and reduced-motion audit
- [ ] Official Zed installed coexistence without scheme, bundle, storage, CLI,
      or updater takeover
- [x] Local bundle identity, helper inclusion, entitlements, and ad-hoc
      signature audit
- [ ] Developer ID signing, notarization, install, launch, and uninstall audit

The approved macOS UI-control path was retried after the final launch, but the
desktop remained locked and automatic unlock failed. No alternate screenshot,
accessibility, AppleScript, or historical binary path was used as a substitute.

## Known external release dependencies {#known-external-release-dependencies}

Public Developer ID signing and Apple notarization require Dez publisher
credentials. The ad-hoc local signature proves bundle structure, not public
notarization. Design-partner testing requires actual target users and remains
separate from local engineering verification. Unlocking the current macOS
desktop is required to finish the live visual, interaction, accessibility, and
hosted-PTY recovery matrix on this artifact.
