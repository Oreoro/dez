## MODIFIED Requirements

### Requirement: Core review and writing surfaces remain available
The application SHALL retain terminal, project navigation, editor, git review,
diff, search, settings, theme, keymap, Markdown, and title bar surfaces.
The title bar SHALL render with visible content (at minimum the project name)
for every open project. The terminal panel icon SHALL be visible in the
status bar by default. The application SHALL identify as "dez" in all
user-visible surfaces (title bar, menus, about dialog, URL scheme) while
preserving extension compatibility with the upstream Zed extension ecosystem.

#### Scenario: Core workspace opens with retained surfaces
- **WHEN** a user opens a project
- **THEN** the user can open terminal sessions, browse project files, view diffs,
  inspect git status, search content, edit files, and open Markdown files

#### Scenario: App identity shows dez branding
- **WHEN** a user opens the application
- **THEN** the app menu shows "About dez", "Quit dez", and "Hide dez"
- **AND** the binary is named `dez`
- **AND** config is stored in platform-appropriate dez directories

#### Scenario: URL scheme uses dez://
- **WHEN** a `dez://` URL is opened by the OS
- **THEN** the dez application handles it and opens the corresponding resource

#### Scenario: Existing Zed extensions load without modification
- **WHEN** a user installs an extension from the Zed extension registry
- **THEN** the extension loads and functions correctly without recompilation
- **AND** the WIT namespace `zed:extension` is preserved unchanged
