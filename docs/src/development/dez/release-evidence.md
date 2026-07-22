# Dez v0.0.1 Release Evidence

This log records direct evidence for the consolidated release gate. A source
claim is not a runtime claim, and an unchecked scenario remains unverified.

## Frozen source and intended artifacts {#frozen-source-and-intended-artifacts}

- Protocol 4 app and Host build commit: `d0b0d9a908`
- Packaging and permission-copy foundation: `ce11c4ed3d`
- Inside-out local bundle signing: `fcd1d06564`
- Post-build lint compatibility commit: `3ad224dfd6`
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

The initial gate launched only the intended raw `target/debug/dez`. The current
gate launches the matching `Dez Dev.app` through LaunchServices so the approved
macOS visual and accessibility path can target it when the desktop is unlocked.
The excluded Superzed artifact has never been opened.

## Build evidence {#build-evidence}

The original app and helper completed together, warning-free, in 25 minutes 31
seconds. After the Session Rail and replay corrections, the protocol 4 app and
helper completed again with the same locked, storage-constrained profile:

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
single-codegen-unit profile. The captured unsigned build artifacts were arm64
Mach-O files:

| Artifact            | Size | SHA-256                                                            |
| ------------------- | ---- | ------------------------------------------------------------------ |
| `target/debug/dez`  | 1.0G | `ccc84c35cc2ef037a0f4ebcfe41ea8a14918df95e369b0989fef6235eaa10db5` |
| `target/debug/cli`  | 12M  | `e9bde80f1d951a6f9b7da53b0175de23db31c642b368c67c19451a04fbc9eaed` |
| `dez-terminal-host` | 13M  | `500845d7e3c27ba205803330865c92ebbd55a533c261a915eeb7422f715b6113` |

After hashing and copying it into the signed bundle, the 1.0G raw `dez` file
was removed to provide safe link headroom for the Session Rail regression test.
The signed bundle copy remains present and running; the raw file is reproducible
with the recorded build command.

## Debug bundle and coexistence evidence {#debug-bundle-and-coexistence-evidence}

`script/bundle-mac -d` now reuses a complete host debug artifact set, restores
the temporary manifest on bundler failure, works around the pinned bundler's
invalid terminal-colour failure through its plain-output path, omits the
release-only remote server, and creates the bundle without a second app build.
Local ad-hoc signing now signs nested executables inside-out before sealing the
app, matching the reliable release ordering instead of relying on one fragile
`codesign --deep` pass.

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
| `dez`               | `8e7c203a4e4b5da5c577cc37ef0661ca113e2702fa4e4263a83a8bdba75e5b0a` |
| `cli`               | `19e7c4b56c0f85249d8347b2eb219a640ee047fd06612c748e7c6dbe2ade1821` |
| `dez-terminal-host` | `82e3b34f4ddff9f5cc5d67d0a03564c08a46b34acea97edde8220bf71e808f62` |
| `git`               | `3785db4c9db29936c32339b92d530c5c519ae1ab493ed41ab9b5f693bbb54281` |

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

This proves Host-process survival and reuse. It does **not** by itself prove the
complete terminal acceptance scenario.

The desktop later became available and exposed a blocking macOS shell defect:
the Session Rail occupied the whole client area, compressed its contents into
word-level wrapping, covered the welcome/editor surface, and clipped the lower
workspace chrome. The client-decoration render branch had absolute positioning
with both horizontal edges pinned and, unlike the server-decoration branch, no
explicit rail width. Commit `36d8024280` gives the rail its configured width,
anchors only the active edge, and replaces the cramped empty-project row with
a vertical empty state and full-width New Terminal action. Follow-up work
preserves recorded terminal dimensions for every replay fragment, keeps durable
Workspace identity after the last viewport closes, projects ordinary live
shells into the Session Rail, and constrains footer content to a single
truncating row.

The audited `Dez Dev.app` is now registered and launched as launchd child PID
`57957`, with `DEZ_EXPERIMENTAL_TERMINAL_HOST=1`, through its exact bundle path.
The desktop is currently locked, and the approved accessibility controller
cannot unlock it automatically. A fresh rendered screenshot of the corrected
artifact therefore remains required before the visual matrix can be checked
complete.

The packaged helper also accepted a direct authenticated protocol 4 exercise.
Host ID `d9670db8-e498-5537-a9d8-f99ad098f4aa` created Session
`040b4465-5f0a-416b-9cb3-549da1a2a28b` with shell PID `53394`, emitted 88
bounded replay chunks, resized from 80x24 to 132x41, retained both dimensions
and both before/after markers in replay, and ended in explicit `Detached` state
at sequence 88. This proves the packaged protocol boundary, PTY ownership,
resize retention, and detach truth. It does not prove GUI-exit reattachment;
that scenario still requires the unlocked UI and a graceful application quit.

## Automated gates {#automated-gates}

- [x] `cargo fmt --all -- --check`
- [x] `git diff --check`
- [x] `cargo metadata --offline --no-deps --format-version 1`
- [x] `./script/dez-identity-check`
- [x] `bash -n script/bundle-mac`
- [x] Focused Prettier checks for the canonical Dez documentation slices
- [x] Focused tests: 15 terminal client/model tests, 8 Host/helper tests, and
      three Session Rail terminal lifecycle tests
- [x] Plain workspace-shell regression: the Session Rail includes a non-agent
      terminal as `Live`, leaves its agent classification empty, and selects its
      active row
- [x] `cargo clippy -p dez_terminal_host --all-targets -- -D warnings` with the
      recorded storage-constrained dev profile
- [ ] App-facing modified-crate `cargo clippy` (the full app graph exceeds the
      remaining local storage budget)
- [x] Intended app and helper consolidated build
- [x] Intended CLI build
- [x] Debug bundle build and static bundle audit

## Runtime and manual gates {#runtime-and-manual-gates}

- [x] Intended raw-binary first launch and exact signed-bundle normal launch
- [ ] Restored and empty-workspace interaction audit (the full-window Session
      Rail overlay found in the first unlocked screenshot is fixed in source
      and rebuilt; a fresh corrected-artifact capture remains open)
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

The approved macOS UI-control path was retried after the exact packaged launch.
The application is targetable, but the desktop is locked and automatic unlock
fails. No alternate screenshot mechanism, AppleScript, or historical binary
path is used as a substitute.

## Known external release dependencies {#known-external-release-dependencies}

Public Developer ID signing and Apple notarization require Dez publisher
credentials. The ad-hoc local signature proves bundle structure, not public
notarization. Design-partner testing requires actual target users and remains
separate from local engineering verification. The exact packaged artifact is
already running; unlocking the desktop is the remaining environmental
prerequisite for its visual, interaction, accessibility, and GUI-driven
hosted-PTY recovery matrix.
