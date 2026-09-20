---
title: Uninstall
description: "This guide covers how to uninstall dez on different operating systems."
---

# Uninstall

This guide covers how to uninstall dez on different operating systems.

## macOS

### Standard Installation

If you installed dez by downloading it from the website:

1. Quit dez if it's running
2. Open Finder and go to your Applications folder
3. Drag dez to the Trash (or right-click and select "Move to Trash")
4. Empty the Trash

### Homebrew Installation

If you installed dez using Homebrew, use the following command:

```sh
brew uninstall --cask dez
```

### Removing User Data (Optional)

To completely remove all dez configuration files and data:

1. Open Finder
2. Press `Cmd + Shift + G` to open "Go to Folder"
3. Delete the following directories if they exist:
   - `~/Library/Application Support/dez`
   - `~/Library/Saved Application State/dev.dez.dez.savedState`
   - `~/Library/Logs/dez`
   - `~/Library/Caches/dev.dez.dez`
   - `~/Library/Caches/dez`
   - `~/.config/dez`
   - `~/.local/state/dez`

## Linux

### Standard Uninstall

If dez was installed using the default installation script, run:

```sh
dez --uninstall
```

You'll be prompted whether to keep or delete your preferences. After making a choice, you should see a message that dez was successfully uninstalled.

If the `dez` command is not found in your PATH, try:

```sh
$HOME/.local/bin/dez --uninstall
```

or:

```sh
$HOME/.local/dez.app/bin/dez --uninstall
```

### Package Manager

If you installed dez using a package manager (such as Flatpak, Snap, or a distribution-specific package manager), consult that package manager's documentation for uninstallation instructions.

### Manual Removal

If the uninstall command fails or dez was installed to a custom location, you can manually remove:

- Installation directory: `~/.local/dez.app` (or your custom installation path)
- Binary symlink: `~/.local/bin/dez`
- Configuration and data: `~/.config/dez`

## Windows

### Standard Installation

1. Quit dez if it's running
2. Open Settings (Windows key + I)
3. Go to "Apps" > "Installed apps" (or "Apps & features" on Windows 10)
4. Search for "dez"
5. Click the three dots menu next to dez and select "Uninstall"
6. Follow the prompts to complete the uninstallation

Alternatively, you can:

1. Open the Start menu
2. Right-click on dez
3. Select "Uninstall"

### Removing User Data (Optional)

To completely remove all dez configuration files and data:

1. Press `Windows key + R` to open Run
2. Type `%APPDATA%` and press Enter
3. Delete the `dez` folder if it exists
4. Press `Windows key + R` again, type `%LOCALAPPDATA%` and press Enter
5. Delete the `dez` folder if it exists

## Troubleshooting

If you encounter issues during uninstallation:

- **macOS/Windows**: Ensure dez is completely quit before attempting to uninstall. Check Activity Manager (macOS) or Task Manager (Windows) for any running dez processes.
- **Linux**: If the uninstall script fails, check the error message and consider manual removal of the directories listed above.
- **All platforms**: If you want to start fresh while keeping dez installed, you can delete the configuration directories instead of uninstalling the application entirely.

For additional help, see our [Linux-specific documentation](./linux.md) or visit the [dez community](https://dez.dev/community-links).
