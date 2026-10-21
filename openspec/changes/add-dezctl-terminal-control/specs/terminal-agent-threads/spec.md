## ADDED Requirements

### Requirement: Agent Thread control uses the dezctl thread group
The application SHALL provide `dezctl thread retie --worktree <path>` and `dezctl thread create --worktree <current|new> --agent <agent> --prompt <prompt>` as the only Agent Thread control command forms, with the existing Agent Thread-only authorization and behavior.

#### Scenario: Agent Thread moves to an existing worktree
- **WHEN** a live Agent Thread invokes `dezctl thread retie --worktree <path>`
- **THEN** the application moves that calling Agent Thread to the specified existing worktree under the same authorization rules as the former command

#### Scenario: Agent Thread creates a sibling thread
- **WHEN** a live Agent Thread invokes `dezctl thread create` with a valid worktree mode, agent, and prompt
- **THEN** the application creates the requested sibling Agent Thread with the existing create-thread behavior

#### Scenario: Ordinary terminal invokes a thread command
- **WHEN** a recognized ordinary dez terminal that is not a registered Agent Thread invokes a thread command
- **THEN** dez rejects the request with `caller-not-agent-thread`

#### Scenario: Agent Thread uses an old flat command
- **WHEN** an Agent Thread invokes `retie-thread` or `create-thread` without the `thread` command group
- **THEN** `dezctl` rejects the unsupported command form

### Requirement: dez provides an opt-in Agent Thread control skill
dez SHALL bundle a release-matched `dezctl` skill and SHALL let a user inspect, install, update, and uninstall it for each supported agent. dez SHALL NOT install or update the skill before the user opts in, and SHALL NOT add new dez text to global `AGENTS.md`, `CLAUDE.md`, or other general instruction files.

#### Scenario: User inspects the skill
- **WHEN** a user runs `dezctl skill print`
- **THEN** standard output contains the complete skill bundled with that `dezctl` executable without connecting to dez

#### Scenario: User installs the skill
- **WHEN** a user confirms installation for a supported agent
- **THEN** dez writes the bundled skill to that agent's verified skill directory and records dez ownership and the installed content digest

#### Scenario: User has not installed the skill
- **WHEN** dez launches and no dez-owned skill record exists for an agent
- **THEN** dez does not create or change that agent's skill or general instruction files

#### Scenario: User uninstalls the skill
- **WHEN** a user uninstalls an unchanged dez-owned skill
- **THEN** dez removes the installed skill and its ownership record without changing other skills or general instructions

### Requirement: dez updates only skills it owns
On every application launch, dez SHALL compare each recorded dez-owned skill with the release-matched bundled skill. dez SHALL atomically replace an unchanged older dez-owned skill. dez SHALL preserve a user-modified installed skill and report a conflict instead of overwriting or deleting it.

#### Scenario: A newer dez version launches
- **WHEN** a recorded skill still matches the digest that dez installed and the bundled skill has changed
- **THEN** dez atomically replaces the skill and records the new version and digest

#### Scenario: User modified an installed skill
- **WHEN** the installed skill no longer matches its recorded digest
- **THEN** dez preserves the file and reports a visible conflict with keep and replace choices

#### Scenario: Installed skill is current
- **WHEN** the installed skill and ownership record match the current bundled skill
- **THEN** dez makes no file change

### Requirement: The skill detects dez Agent Threads without affecting other sessions
The skill metadata SHALL trigger for worktree, Agent Thread, and terminal-control tasks. Its body SHALL use the release-channel marker and matching control endpoint as a cheap availability check, then use `dezctl terminal current --json` as the authoritative caller probe. It SHALL NOT use a terminal environment variable as caller identity.

#### Scenario: Agent creates a worktree in a dez Agent Thread
- **WHEN** an installed skill loads for a worktree task and the caller probe reports `is_agent_thread: true`
- **THEN** the agent follows the skill's marker discovery and `dezctl thread retie` instructions

#### Scenario: Skill loads in an ordinary dez terminal
- **WHEN** the caller probe succeeds and reports `is_agent_thread: false`
- **THEN** the skill permits terminal commands and does not permit Agent Thread commands

#### Scenario: Control endpoint is absent
- **WHEN** the release marker or matching control endpoint does not exist
- **THEN** the skill does not invoke `dezctl` and continues the user's task normally

#### Scenario: Caller probe fails
- **WHEN** the connection fails, the protocol is incompatible, or dez reports `caller-not-recognized`
- **THEN** the skill continues the user's task without dez control commands

#### Scenario: Current marker refers to an older installation
- **WHEN** dez launches and the release-channel-scoped marker has an older executable path or content
- **THEN** dez replaces the marker with the running version's current `dezctl` location and metadata

### Requirement: Agent Thread remote route boundaries remain unchanged
The new command name and protocol surface SHALL NOT change the Direct or Tunneled remote launch, executable, credential, or traffic boundaries for Agent Threads.

#### Scenario: Direct remote Agent Thread uses control commands
- **WHEN** a Direct remote Agent Thread is configured
- **THEN** it uses only the configured ambient remote executable and gains no dez-managed launch or credential control

#### Scenario: Tunneled remote Agent Thread uses control commands
- **WHEN** a Tunneled remote Agent Thread is configured
- **THEN** it uses only the pinned dez-managed remote executable and routes its traffic through local dez under the existing boundary
