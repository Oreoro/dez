# Dez v0.0.1 Release Evidence

This log records direct evidence for the consolidated release gate. A source
claim is not a runtime claim, and an unchecked scenario remains unverified.

## Frozen source and intended artifacts {#frozen-source-and-intended-artifacts}

- Protocol 4 app and Host build commit: `d0b0d9a908`
- Corrected shell bundle source: `679cdc28445c824482923bdfdfd8463927f9a337`
- Durable viewport test source: `a91b04809ce0fa9d26407bc1685726ef36a0f03f`
- Terminal-first empty-workspace source: `e4fbc22a3a`
- Honest Session Rail zero-state source: `d9688490ad`
- Calm zero-session rail geometry source: `1ebb7c79d4`
- Reachable isolated CLI installer source: `704314cc92`
- Initial terminal-scope evidence source: `7a20dc1d19`
- Live background viewport attachment source: `962b611605`
- Durable empty-fallback Workspace source: `e9a595fcff`
- Shared-App-Session New Window proof source: `2334fbdcfc`
- Queued-open startup barrier source: `47e769da5d`
- Failed-restore durable truth source: `d10d90648d`
- Actionable failed-restore notice source: `31cc1b1205`
- Persistent Session Rail recovery source: `fbf8443359`
- Zed coexistence endpoint isolation source: `c101fe6a43`
- Dez-only onboarding chrome source: `699cbd1bc8`
- Neutral getting-started agent icon source: `869cddcce0`
- Single-action empty Session Rail source: `4e6292ff0a`
- Deduplicated Session Rail footer source: `a9b1a961c0`
- Unified Session Rail Workspace vocabulary source: `ff91b34a81`
- Workspace-scoped zero-session creation source: `4fc53b860f`
- Clear Session Rail utility actions source: `8bcd11f4b6`
- Dead Session Rail settings removal source: `ad59a60926`
- Inherited cloud settings removal source: `a20074de26`
- Upstream prediction-data control removal source: `2435348289`
- Dez command namespace and settings-link source: `f89f55868c`
- Inherited collaboration command filter source: `f40877d4ab`
- Neutral Codex session glyph source: `526218a972`
- Deliberate Codex onboarding setup source: `bb0cf408b4`
- Explicit Workspace review evidence source: `a8ce563373`
- Persistent selected Workspace evidence source: `e101b63e43`
- State-aware review evidence tab menu source: `f535c5e6ae`
- Terminal restore evidence reconciliation source: `0e6507756e`
- Live hosted-terminal evidence lifecycle source: `ea2bb18453`
- Workspace evidence mutation boundary source: `0f8740b1a1`
- Pending Workspace evidence isolation source: `af232402f5`
- Durable terminal Workspace ownership source: `a4047d95c0`
- Packaging and permission-copy foundation: `ce11c4ed3d`
- Inside-out local bundle signing: `fcd1d06564`
- Post-build lint compatibility commit: `3ad224dfd6`
- Integration merge: `2be63cfea347006e407934754086bbef62d482c2`
- Incorporated upstream: `9d0ef37a25711c00bf6d1ba1142e9de4f4a122a9`
- `Cargo.lock` SHA-256:
  `64104e448242ff05034e6990c7ae7e7120edad21066ead5fb766e8dbb44b264e`
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
single-codegen-unit profile and was rebuilt after the public-help cleanup. The
captured unsigned build artifacts were arm64 Mach-O files:

| Artifact            | Size | SHA-256                                                            |
| ------------------- | ---- | ------------------------------------------------------------------ |
| `target/debug/dez`  | 1.0G | `c244c5501097257cb4bbe4203ffb3ced1ffa416dced857ffb6be515a445c8489` |
| `target/debug/cli`  | 12M  | `31ea17a6ddf2adf159cb55adca81c5f10d07c77c66608f6ec36242bc0c411e80` |
| `dez-terminal-host` | 13M  | `935e1e3395a37860e0a2533958e28a1a0c13aeda37c7cc20be489897951deee2` |

The final link completed in 38.17 seconds from the already compiled graph. A
redundant Darwin post-link debug-strip pass was scoped out because it attempted
to materialize a second 1.0G copy on the storage-constrained volume; the linked
Mach-O itself completed successfully. The original Rust toolchain executable
was restored immediately afterward. The raw app and helper remain present and
match the hashes above; only regenerable WebRTC build intermediates were
removed to provide bundle-signing headroom. The raw CLI is newer than the
signed bundle copy. A consolidated app and bundle rebuild is required before
the newest source can be called a release artifact.

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
- ad-hoc CDHash `0dc2e1e872b88cbd6288f1bea5455fbc48271cc5`;
- privacy prompt copy that identifies a developer tool launched from Dez
  instead of ambiguously claiming that “an application in Dez” requested data.

Signed bundle-executable SHA-256 values are:

| Bundle executable   | SHA-256                                                            |
| ------------------- | ------------------------------------------------------------------ |
| `dez`               | `85c0b9e58fe4134b58081463b3de397e058ca77a69e01e5fc881ba2c3e2c82ff` |
| `cli`               | `3055a8e7b97588b0cae57dedfc10084dbffe93926c8923b768234f4f2e1b2b0a` |
| `dez-terminal-host` | `2e36a469b57445246a9d47c6c17b6f2e69061644f7567d0613e105c36b66f775` |
| `git`               | `3785db4c9db29936c32339b92d530c5c519ae1ab493ed41ab9b5f693bbb54281` |

The signed copies differ byte-for-byte from the raw Cargo outputs because the
ad-hoc signing step rewrites Mach-O signatures. Static identity checks pass.
No official Zed installation or CLI was present in the inspected system app and
command locations, so installed coexistence cannot be demonstrated and remains
open; no alternate application was launched during that audit.

Commit `c101fe6a43` closes the source-level endpoint collisions found during the
coexistence audit. macOS Dez channels now use bases
45737/45837/45937/46037 before per-user offsets rather than official Zed's
43737 range. Linux app and CLI use the product-isolated
`dez-{channel}.sock`. Linux and Windows CLI discovery now fails clearly when a
matching Dez executable is absent instead of silently launching a neighboring
official Zed binary. Identity checks enforce all three boundaries, and
`cargo check --locked -p cli --bin cli -j1` passes in 1m22s. Installed
side-by-side proof is still not claimed because no official Zed installation
exists in the inspected locations.

Commit `699cbd1bc8` removes the remaining upstream onboarding entry points from
Dez chrome. Title-bar promotion banners and Return to Onboarding render only
for official Zed. Dez Help exposes **Getting Started**, and the optional
welcome surface uses Open Workspace, Recent Workspaces, and supervision copy
that names worktree isolation and review evidence. Follow-up `869cddcce0`
replaces its Zed Assistant glyph with the neutral Robot icon and adds an
identity regression guard. Formatting, diff, and identity checks pass;
compiled and rendered proof remains open.

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
the Session Rail occupied the whole client area, covered the welcome/editor
surface, and clipped the lower workspace chrome. The client-decoration render
branch had absolute positioning with both horizontal edges pinned and no
explicit rail width. Commit `36d8024280` gives that branch an explicit width,
anchors only the active edge, and replaces the cramped empty-project row with a
vertical empty state and full-width New Terminal action. Follow-up work
preserves recorded terminal dimensions for every replay fragment, keeps durable
Workspace identity after the last viewport closes, projects ordinary live
shells into the Session Rail, and constrains footer content to one line.

A later screenshot of the running signed bundle exposed a separate compact-mode
contract mismatch. `WorkspaceSidebar::width` reserved the capped 240 px width,
but the root render element still painted at the stored 300 px width in both
decoration branches. The clipped 60 px explains the crushed header, rows, and
footer in that screenshot. Commit `79f69b273c` resolves the width once and uses
it for painting as well as reservation, with compact, detailed, and icon-mode
regression assertions. Formatting and diff checks pass. Its focused test build
was attempted but not completed: reconstructing the deleted Cargo source cache
twice exhausted the remaining volume before the large test link, so no test
pass is claimed. The corrected source now compiles in the complete app and is
present in the signed bundle; rendered proof remains open.

The screenshot also showed a loaded project with a completely blank center.
The render decision checked `should_display_welcome_page` before checking for a
visible worktree, so a legacy/restored tabbed pane with the flag unset returned
an empty placeholder. Commit `4829f6b052` gives a loaded project priority and
renders the existing Project ready surface with Find File, New File, and New
Terminal actions; no-worktree welcome suppression retains its old meaning. A
focused model assertion covers both cases, but it shares the unfinished
storage-bound test target and is not recorded as passing.

The stale bundle log supplied a separate runtime finding: inherited user
settings could still start the upstream Zed websocket, LiveKit reconnection,
and Zed-hosted edit prediction even though Dez defaults `auto_connect` to false
and hides collaboration/account chrome. Commit `1d5c03d88b` gates automatic
cloud authentication and eager Collab-panel construction to the official Zed
product, and treats inherited Zed/Mercury prediction providers as unavailable
in Dez while retaining explicit Copilot, Codestral, Ollama, and compatible API
providers. Commit `9318b270d9` adds static identity-policy checks for all of
these boundaries. The checks pass. The corrected bundle remained alive for the
recorded runtime soak with no established or listening TCP socket, closing the
quiet local-launch runtime gate.

The screenshot's footer also exposed an independent flex-layout failure:
project identity and worktree/branch controls were nominally truncating, but
their parent row did not give them bounded shrink regions. Commit `0d8496969f`
wraps both groups in minimum-width-zero, flexible, overflow-hidden containers
and keeps the Git group bounded to the row width. Formatting and diff checks
pass. The complete app build now contains the change; narrow-width render audit
remains open.

The same footer rendered Command Search in a dedicated row immediately above a
second icon utility bar. Commit `abc4f8bedb` removes that stacked Dez-only
chrome, keeps Command Search as an accessible icon in the existing utility bar,
suppresses the unowned upstream update surface, and renders the Canvas prefix
indicator row only while prefix mode is active. Official Zed retains its prior
workspace-bar behavior. Formatting and diff checks pass, and the complete app
build contains the change; render proof remains open.

The supplied zero-session screenshot also makes the remaining density problem
plain: even after correcting reservation versus painting, compact mode itself
allowed only 240 px, while the rail still carried named creation, scope, search,
project, and utility controls. Commit `1ebb7c79d4` raises the compact cap to
280 px and the user-resize floor to 240 px. At zero sessions it omits the inert
All 0 / Attention 0 scope row and search field; a pre-existing query remains
visible so it can be cleared. The focused visibility assertion is authored and
a clean `cargo check --locked -p sidebar --lib -j1` passes in 14m34s. No
rendered claim is inferred from that source check.

Follow-up `4e6292ff0a` removes the empty rail's remaining duplicate creation
hierarchy. The full start state now owns the only New Terminal action; the
ordinary Sessions overview returns as soon as the rail has content. The state
is titled **Start a session**, explains where live state will appear, and gives
the New File and Open alternatives distinct icons. An authored model assertion
and identity guard freeze the overview handoff. Formatting, diff, and identity
checks pass; compilation and rendered proof remain in the consolidated gate.

Commit `a9b1a961c0` removes the second project/branch identity row from Dez's
Session Rail footer. Project identity remains in the rail's project headers;
Restricted Mode and embedded cross-platform application menus can still open
the footer row when they carry essential content, and official Zed retains its
upstream behavior. Two focused model tests and an identity guard cover those
product and safety boundaries. Formatting, diff, and identity checks pass;
compile and rendered proof remain deferred.

Commit `ff91b34a81` normalizes the remaining user-facing rail vocabulary around
Workspace: Remote Workspace, Workspace Options, focus/open-new-window actions,
Open Workspace, and Open Workspace Rules. Internal upstream `Project` types
and action identifiers are unchanged. The bottom entry now uses a Folder Open
glyph rather than an add-project glyph. An identity rejection prevents the old
mixed labels from returning. Formatting, diff, and identity checks pass;
rendered proof remains deferred.

Commit `4fc53b860f` removes the last zero-session duplicate from the screenshot's
open-Workspace state. At zero sessions, each Workspace group owns its scoped
New Terminal action and the global overview omits its competing copy. After a
session exists, the overview creation shortcut returns. The full no-Workspace
activation surface remains unchanged. An authored model assertion and identity
guard cover the count transition. Formatting, diff, and identity checks pass;
compile and rendered proof remain deferred.

Commit `8bcd11f4b6` aligns the remaining rail utilities with their actual scope.
The clock is Agent History rather than generic Thread History, the command icon
uses Command Palette consistently in its accessible name and tooltip, and New
File/Open expose explicit accessible labels plus action-aware shortcut
tooltips. An identity rejection freezes the visible utility names. Formatting,
diff, and identity checks pass; compiled interaction proof remains deferred.

Commit `ad59a60926` removes five now-inert Session Rail Chrome controls from
Dez's Settings UI: branch status, branch name, worktree name, duplicate project
identity, and the upstream onboarding banner. Their schema remains readable for
upstream/legacy compatibility. Files Pane, application menus, and Linux window
button layout remain exposed because they still affect rendered behavior. A
focused model test covers the full allow/remove set and an identity guard
freezes product gating. Formatting, diff, and identity checks pass; compiled
Settings UI proof remains deferred.

Commit `a20074de26` removes the inherited Calls/Collaboration page from Dez's
graphical Settings and reduces Network to the live proxy control. There is no
longer a GUI path to re-enable inherited Zed auto-connect or edit its
collaboration server; the keys remain readable for compatibility and official
Zed keeps its full presentation. Attention copy also stops referring to the
removed workspace bar. A focused product-boundary model test and identity
guards cover these gates. Formatting, diff, and identity checks pass; compiled
Settings UI proof remains deferred.

Commit `2435348289` removes upstream Zed Edit Predictions data collection from
Dez's graphical agent settings because the fork already disables its
Zed/Mercury providers. Provider setup now describes explicit providers and
states that Dez does not enable upstream Zed-hosted providers. The compatibility
key remains parseable and official Zed retains the control. The same focused
product-boundary test and an identity guard cover the filter. Formatting, diff,
and identity checks pass; compiled Settings UI proof remains deferred.

Commit `f89f55868c` productizes command and settings-link presentation without
breaking upstream action/keymap compatibility. Internal `zed::…` actions now
display as `dez: …` in the Command Palette, keymap editor, and which-key UI.
Settings copy links use the active release channel's canonical `dez-dev`,
`dez-nightly`, `dez-preview`, or `dez` scheme, and URL registration consumes
that same release-channel method. Legacy `zed://` remains input-only
compatibility. Focused model assertions cover productization and every channel;
formatting, diff, lockfile, and identity checks pass. Compiled interaction proof
remains deferred.

Commit `f40877d4ab` removes the remaining dead collaboration actions from Dez's
Command Palette at product initialization. The `collab` namespace stays
registered for action/keymap compatibility and remains visible in official
Zed; Dez filters only its presentation. A focused product-boundary assertion
and identity guard cover the gate. Formatting, diff, and identity checks pass;
compiled palette proof remains deferred.

Commit `526218a972` removes the Zed Assistant glyph from Codex sessions in both
the Session Rail and keyboard switcher. Codex remains identifiable by its
label, actor metadata, and neutral Robot glyph. A directory-wide identity
rejection prevents the upstream logo from returning to either rail surface.
Formatting, diff, and identity checks pass; rendered proof remains deferred.

Commit `bb0cf408b4` adds a deliberate Codex structured-evidence setup action to
the first-run workflow. **Copy Codex Hook** copies the exact bundled JSON,
states that Dez does not install or modify hooks, precedes New Terminal in
keyboard order, and retains the close/detach/terminate plus Host-persistence
explanation. An identity guard freezes the reachable setup action. Formatting,
diff, and identity checks pass; rendered onboarding proof remains deferred.

Commit `a8ce563373` adds explicit user-selected review evidence to the
Workspace-owned EvidenceSet. Command Palette actions add, remove, or clear the
selection; file-tab context actions use the same owner. Selected paths retain
stable user-selection provenance after passive open-file recomputation and tab
closure, are bounded at 128 with visible no-op/capacity feedback, and project
as Selected path in Review Briefs while suppressing a duplicate Open file row.
Focused model tests are authored. Formatting, diff, and identity checks pass;
no compile, bundle, launch, or rendered interaction claim is made here.

Commit `e101b63e43` makes those explicit selections restart-durable through an
additive Workspace database column. Add/remove/clear schedules the normal
Workspace serialization path; restore rehydrates only selected paths and gives
them the current Workspace Host classification. Passive roots, open tabs, and
terminal observations are not serialized as user choices. A focused database
round-trip test is authored. Formatting, diff, and identity checks pass; no
compiled restart claim is made here.

Commit `f535c5e6ae` makes the file-tab evidence action state-aware. The active
Workspace EvidenceSet selects exactly one Add or Remove menu action instead of
showing contradictory choices together; separate Command Palette actions keep
the keyboard path explicit. Formatting, diff, and identity checks pass;
rendered menu proof remains deferred.

Commit `0e6507756e` reconciles last-known terminal cwd evidence during saved
hosted-session restore. A failed attach records the cwd under the original
Session as Unresolved, Review Briefs state that reattachment is required, and
a successful attach replaces it with Current Host truth. A focused model test
is authored. Formatting, diff, and identity checks pass; transient in-place
transport-loss and compiled restart proof remain open.

Commit `ea2bb18453` observes the authoritative Host snapshot revision from each
hosted TerminalView. Attached, Starting, and Detached map to Current Workspace
evidence; Reconnecting, Missing, and Incompatible map to Unresolved; Exited
maps to Stale; and snapshot cwd changes update the same Session-owned record.
The policy does not equate transport loss with process exit. A focused mapping
test is authored. Formatting, diff, and identity checks pass; compiled
reconnect proof remains deferred.

Commit `0f8740b1a1` makes authoritative EvidenceSet mutation crate-private.
Downstream search, Git, settings, conversation, and review consumers retain
immutable record access but cannot attach roots, replace open files, reconcile
terminal scope, or invent user selections directly. Formatting, diff, and
identity checks pass; this is a source ownership gate rather than a runtime
claim.

Commit `af232402f5` replaces the shared provisional `workspace:pending`
evidence prefix with a stable UUID-backed namespace per Workspace EvidenceSet.
Two not-yet-persisted Workspaces showing the same path therefore cannot emit
colliding evidence record IDs. A focused same-path model test is authored;
formatting, diff, and identity checks pass.

Commit `a4047d95c0` adds an optional durable Workspace ID to terminal Host
snapshots and metadata updates. TerminalView associates both in-process and
helper-owned Sessions after they enter a durable Workspace; Session Rail
prefers the exact owner before cwd matching. Older snapshots deserialize with
no owner and retain the conservative fallback. Host-model, compatibility, and
integrated two-Workspace/same-cwd resolution tests are authored. Formatting,
diff, and identity checks pass; compiled detach/reattach proof remains open.

The corrected `Dez Dev.app` is now registered and launched as launchd child PID
`85053`, with `DEZ_EXPERIMENTAL_TERMINAL_HOST=1`, through its exact bundle path.
`lsof` resolves its text executable to
`/Users/test/Documents/zed 3.0/target/debug/bundle/osx/Dez Dev.app/Contents/MacOS/dez`.
The desktop is currently locked, and the approved accessibility controller
cannot unlock it automatically. A fresh rendered screenshot of the corrected
artifact therefore remains required before the visual matrix can be checked
complete.

Commits `a91b04809c`, `e4fbc22a3a`, `d9688490ad`, `704314cc92`,
`7a20dc1d19`, `962b611605`, `1ebb7c79d4`, `e9a595fcff`, `2334fbdcfc`,
`47e769da5d`, `d10d90648d`, `31cc1b1205`, `fbf8443359`, `c101fe6a43`,
`699cbd1bc8`, `869cddcce0`, `4e6292ff0a`, `a9b1a961c0`, `ff91b34a81`,
`4fc53b860f`, `8bcd11f4b6`, `ad59a60926`, `a20074de26`, `2435348289`, and
`f89f55868c`, `f40877d4ab`, `526218a972`, `bb0cf408b4`, `a8ce563373`,
`e101b63e43`, `f535c5e6ae`, `0e6507756e`, `ea2bb18453`, `0f8740b1a1`,
`af232402f5`, and `a4047d95c0` are newer than that running bundle. The first
passes all nine focused Session tests, including duplicate viewport
replacement without reordering or membership loss. The second makes Project
ready terminal-first, prevents New Window and startup fallback paths from
covering Dez's actionable launch surface with an unsolicited blank editor, and
cleans the public CLI help while keeping the legacy alias hidden for
compatibility. The full `zed --bin dez` source check and rebuilt CLI pass, but
these changes require the next consolidated app rebuild and rendered audit.
The later Session Rail slices replace the misleading zero-session caught-up
state and ambiguous creation copy, increase compact width, and suppress inert
zero-count scope and search controls. Their model assertions are authored. The
focused unit-test invocation remains unclaimed because its first clean-profile
attempt exhausted storage before reaching the test; after clearing only
regenerable Cargo caches, a clean focused sidebar library check passes.

The latest source also exposes Install CLI for Dez while retaining
`/usr/local/bin/dez` as the only global target. Its Linux guidance and CLI
launch-handshake errors identify Dez, and the official-Zed compatibility branch
refuses to manage the upstream command. Static and compile gates pass; the
installation interaction remains a manual release gate.

Commit `7a20dc1d19` additionally seeds a new terminal's initial working
directory into Workspace evidence before the first PTY event. This closes a
source-level gap where an idle new shell could open a review brief without cwd
scope. Full Dez source checks pass; runtime review and restoration proof remain
open.

Commit `962b611605` connects live background Workspace registration to the
durable viewport records without changing active selection. Repeat attachment
is idempotent and a second viewport does not duplicate global membership. All
ten focused Session tests and the workspace library compile gate pass; shared
live entity and consolidated restart proof remain open.

Commit `e9a595fcff` closes a separate empty-Workspace ownership gap. Removing
the final project group or closing the last project-backed Workspace previously
constructed its fallback with no database ID, making the visible empty state
ineligible for durable membership or serialization. Both paths now allocate an
ID before construction; normal registration makes the replacement a global App
Session member and activation selects it in the same viewport. The existing
persistence regression asserts identity, membership, and selection. The
production `cargo check --locked -p workspace --lib -j1` passes in 5m08s; the
expanded all-tests metadata check is not claimed because it was stopped at the
storage safety floor before returning a code result.

Commit `2334fbdcfc` upgrades the existing headless New Window regression to use
one real AppState for the original and newly opened MultiWorkspace. It asserts
distinct OS-window viewport IDs, distinct Workspace database IDs, membership
of both identities in the same App Session entity, and the correct active
selection per viewport. The same slice replaces a stale test-only bottom-dock
call with the supported flexible side-dock assertion. A full
`cargo check --locked -p workspace --tests -j1` passes with one unrelated
dead-code warning. Direct execution of the single test was attempted but
cancelled during sustained codegen/I/O saturation, so no executed-test or GUI
runtime claim is made.

Commit `47e769da5d` extracts the continuing open listener's restoration gate
into a named ordered dispatcher. Queued requests cannot dispatch until startup
signals completion, then retain arrival order and continue flowing. The signal
is completion-based because both successful restoration and visible
failure-fallback handling must release queued user intent. A focused regression
is authored for retention and ordering. Formatting, diff, and identity gates
pass; a cold `cargo check --locked -p zed --bin dez -j1` was stopped at the
3.4 GiB free-space safety floor before producing a code result. Only generated
artifacts from that attempt were removed. No compile, executed-test, bundle, or
runtime claim is inferred.

Commit `d10d90648d` makes failed restoration visible in durable ownership
state. When a database-backed Workspace window cannot materialize, startup
initially marks that identity unresolved while preserving its global order and
viewport placement for future retry or explicit removal. Commit `fbf8443359`
refines that state to the distinct durable `RestoreFailed` variant so an
identity merely skipped by the active restore policy never raises a false
failure warning.

Commit `31cc1b1205` makes the failed-restore notification persistent and
actionable. Its concise copy exposes **Open Dez log**, uses one stable
notification identity to avoid duplicate stacks, and falls back to the durable
empty recovery Workspace when the active-window toast update itself fails.
Formatting, diff, and identity gates pass. Full Dez compilation and rendered
interaction proof remain open.

Commit `fbf8443359` projects only `RestoreFailed` identities into a persistent
Session Rail warning with **Open Recent** and **Dismiss** actions. Dismiss removes
the unresolved App Session reference while leaving recent-workspace storage
untouched. The failure state survives reconciliation until real resolution or
explicit removal. All 12 focused Session tests pass in 4.89s; the offline
lockfile update adds only the direct Sidebar -> Session dependency. Formatting,
diff, and identity gates pass. A focused Sidebar library check was stopped at
the 3.4 GiB free-space floor while compiling inherited audio/WebRTC
dependencies, before the final crate returned a result; only artifacts from
that attempt were removed. Compiled UI and rendered interaction claims remain
open.

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
- [x] `./script/dez-identity-check`, including false cloud auto-connect
      defaults, the product-level automatic-connection gate, upstream
      prediction-provider gates, lazy Collab-panel construction, terminal-first
      empty-workspace seeding, and public CLI branding
- [x] `bash -n script/bundle-mac`
- [x] Focused Prettier checks for the canonical Dez documentation slices
- [x] Focused tests: 15 terminal client/model tests, 8 Host/helper tests, and
      three Session Rail terminal lifecycle tests
- [x] Ten focused App Session tests covering durable ordering, reconciliation,
      duplicate viewport replacement, multi-viewport membership, one-copy
      removal, live idempotent attachment, legacy migration, and serialization
      round trips
- [x] Full `cargo check -p zed --bin dez` with the recorded locked,
      storage-constrained dev profile after the terminal-first source slice
- [x] Rebuilt raw CLI help exposes `--dez <PATH>`, uses Dez product copy, and
      keeps the legacy `--zed` compatibility alias out of public help
- [x] Plain workspace-shell regression: the Session Rail includes a non-agent
      terminal as `Live`, leaves its agent classification empty, and selects its
      active row
- [x] Post-bundle Session Rail source check: commit `2dd523b6e9` adds the named
      accessibility landmark and narrow-width truncation, and the focused
      `sidebar` Cargo check passes with the recorded profile. The corrected
      signed bundle now contains the source edit; AX-tree inspection is open.
- [ ] Mode-resolved Session Rail width regression: commit `79f69b273c` makes
      rendering consume the same compact/icon/detailed width reserved by the
      workspace and contains assertions for all modes. Formatting and diff
      checks pass, but the focused test did not finish under the local storage
      ceiling. The post-fix complete app and bundle build pass; rendered and
      focused-test proof remain open.
- [x] Zero-session rail source regression: commit `1ebb7c79d4` expands compact
      geometry and removes inert zero-count scopes/search without hiding an
      existing clearable query. The model assertion is authored; formatting,
      diff, identity, and a clean focused `sidebar` Cargo check pass. Rendered
      proof remains in the manual gate.
- [x] Durable empty-fallback production regression: commit `e9a595fcff` gives
      both final-project replacement paths a database identity before
      construction. The persistence assertion covers global membership and
      active viewport selection; formatting, diff, identity, and focused
      production workspace-library checks pass. Execution of the expanded test
      remains open under the local storage ceiling.
- [x] Shared-App-Session New Window source regression: commit `2334fbdcfc`
      requires two durable viewports and Workspace IDs with one membership set
      and independent active selections. The complete workspace test
      configuration compiles. Direct test execution and packaged GUI proof
      remain open.
- [ ] Queued-open startup barrier regression: commit `47e769da5d` authors
      pre-barrier retention, ordered release, and post-barrier delivery
      coverage. Formatting, diff, and identity gates pass; the cold Dez target
      check reached the storage floor before compiling the regression, so its
      executed result and packaged GUI proof remain open.
- [x] Failed-restore durable resolution regression: commit `d10d90648d`
      retains ordering while changing only the failed identity; commit
      `fbf8443359` makes that state distinctly `RestoreFailed`. All 12 focused
      Session tests pass. Full startup integration remains open.
- [ ] Actionable failed-restore notice: commit `31cc1b1205` provides a stable
      non-autohiding toast and direct log action in source. Compiled and
      rendered click-through proof remains open.
- [ ] Persistent Session Rail restore-failure recovery: commit `fbf8443359`
      renders only actual failures with Open Recent and Dismiss actions in
      source. Its 12 owning state tests pass; the Sidebar library check hit the
      storage floor before returning a result, and rendered proof remains open.
- [ ] Restored empty-project launch regression: commit `4829f6b052` makes a
      loaded project render Project ready actions even when an old pane lacks
      the welcome flag. The assertion is authored and formatting passes; the
      complete app and bundle build pass, while the shared focused test and
      rendered interaction remain incomplete.
- [x] Bounded project-footer compile regression: commit `0d8496969f` gives project
      identity and Git controls explicit shrink/overflow contracts so their
      one-line labels can truncate instead of colliding. Formatting, diff, and
      complete app build checks pass; rendered narrow-width proof remains open.
- [x] Consolidated footer-utility compile regression: commit `abc4f8bedb` removes the
      dedicated Dez Command Search row while preserving the action, accessible
      tooltip, and on-demand prefix indicator. Formatting, diff, and complete
      app build checks pass; rendered proof remains open.
- [x] `cargo clippy -p dez_terminal_host --all-targets -- -D warnings` with the
      recorded storage-constrained dev profile
- [ ] App-facing modified-crate `cargo clippy` (the full app graph exceeds the
      remaining local storage budget)
- [x] Intended app and helper consolidated build
- [x] Intended CLI build
- [x] Debug bundle build and static bundle audit

## Runtime and manual gates {#runtime-and-manual-gates}

- [x] Intended raw-binary first launch and exact signed-bundle normal launch
- [ ] Restored and empty-workspace interaction audit (the first full-window
      overlay, compact-width clipping, and blank-center defects are fixed in the
      running bundle; the newer terminal-first action order and no-blank-editor
      startup behavior are source-only, so rebuild and fresh capture remain
      open)
- [ ] Offline, failed-Host, and incompatible-Host rendered states
- [ ] Quiet local-first launch with no Zed websocket, LiveKit room, or
      Zed-hosted edit-prediction request unless an explicit compatibility action
      is invoked
- [ ] Persistent terminal GUI-exit/restart/reattach proof
- [ ] Structured Codex attention/review/restart proof
- [ ] Visual state matrix and keyboard/pointer parity
- [ ] Accessibility tree, focus order, labels, contrast, and reduced-motion audit
- [ ] Official Zed installed coexistence without scheme, bundle, storage, CLI,
      or updater takeover. Commit `c101fe6a43` isolates macOS instance ports,
      Linux sockets, and CLI autodetection in source; installed proof remains
      open because official Zed is absent here.
- [x] Local bundle identity, helper inclusion, entitlements, and ad-hoc
      signature audit
- [ ] Developer ID signing, notarization, install, launch, and uninstall audit

The approved macOS UI-control path was retried after the exact packaged launch.
The application is targetable, but the desktop is locked and automatic unlock
fails. No alternate screenshot mechanism, AppleScript, or historical binary
path is used as a substitute. Unlock alone is no longer sufficient for final
visual evidence: the exact bundle must first be rebuilt from `a4047d95c0` or
later and re-audited.

## Known external release dependencies {#known-external-release-dependencies}

Public Developer ID signing and Apple notarization require Dez publisher
credentials. The ad-hoc local signature proves bundle structure, not public
notarization. Design-partner testing requires actual target users and remains
separate from local engineering verification. The exact packaged artifact is
running and contains the corrected shell source through `679cdc28445c`, but
predates `a91b04809c`, `e4fbc22a3a`, `d9688490ad`, `704314cc92`,
`7a20dc1d19`, `962b611605`, `1ebb7c79d4`, `e9a595fcff`, `2334fbdcfc`,
`47e769da5d`, `d10d90648d`, `31cc1b1205`, `fbf8443359`, `c101fe6a43`,
`699cbd1bc8`, `869cddcce0`, `4e6292ff0a`, `a9b1a961c0`, `ff91b34a81`,
`4fc53b860f`, `8bcd11f4b6`, `ad59a60926`, `a20074de26`, `2435348289`,
`f89f55868c`, `f40877d4ab`, `526218a972`, `bb0cf408b4`, `a8ce563373`,
`e101b63e43`, `f535c5e6ae`, `0e6507756e`, `ea2bb18453`, `0f8740b1a1`,
`af232402f5`, and `a4047d95c0`. A rebuild/re-audit and an unlocked
desktop are both prerequisites for the visual, interaction, accessibility, and
GUI-driven hosted-PTY recovery matrix.
