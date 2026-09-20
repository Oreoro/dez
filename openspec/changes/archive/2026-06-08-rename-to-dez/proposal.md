## Why

The terminal-first fork needs its own identity separate from Zed. The name "dez" — strike the prompt, spark the agent — captures the project's philosophy: the editor is the stone, the terminal is the steel, the agents are the spark. A distinct name avoids user data collisions (separate config/data directories), enables independent branding and distribution, and signals that this is a different product with different goals.

## What Changes

- Change `APP_NAME` from `"Zed"` to `"dez"` in `crates/paths/src/paths.rs`, which cascades to all platform-specific config, data, cache, and state directory paths.
- Rename the binary from `zed` to `dez` in `crates/zed/Cargo.toml`.
- Update macOS bundle identifiers (`dev.zed.Zed*` → unique dez identifiers) and display names.
- Update Windows app identifiers and installer.
- Update Linux desktop entry, Flatpak, Snap, and packaging files.
- Change the URL scheme from `zed://` to `dez://` across CLI, open listener, and scheme registration.
- Update app menus ("About Zed" → "About dez", "Quit Zed" → "Quit dez", etc.).
- Update release channel `app_id()`, docs URLs, and feedback/report URLs.
- **Keep `package zed:extension` in all WIT files unchanged** so existing Zed extensions continue to work without recompilation.
- **Keep the `.zed/` project settings folder name** for backward compatibility with existing projects.
- Keep `ZED_SERVER_URL`, `ZED_RELEASE_CHANNEL`, and similar env vars unchanged (they are internal and not user-facing).
- Disable auto-update (point to nowhere or remove it) since this is an independent fork.
- Update app icons with dez branding.

## Capabilities

### New Capabilities

_None_

### Modified Capabilities

- `focused-product-surface`: The application identity is changing from Zed to dez, affecting the title bar, menus, about dialog, and URL scheme. Extension loading must remain compatible with existing Zed extensions.

## Impact

- **User data**: Config moves from `~/.config/zed/` to `~/.config/dez/`. Existing users need to migrate or reconfigure.
- **Binary**: The CLI changes from `zed` to `dez`. Scripts and aliases need updating.
- **URL scheme**: `zed://` links become `dez://` links. External tools using `zed://` need updating.
- **Extensions**: Unaffected — WIT namespace stays `zed:extension`, extension registry stays at Zed's API.
- **Project settings**: `.zed/` folder name unchanged for compatibility.
