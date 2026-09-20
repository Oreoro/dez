---
title: dez on Windows
description: "Install the latest stable dez build on Windows."
---

# dez on Windows

## Installing dez

Get the latest stable build via the
[download page](https://dez.dev/download). After the first manual
installation, dez periodically checks for updates.

You can also build dez from source, see [these docs](https://dez.dev/docs/development/windows) for instructions.

### Package managers

Additionally, you can install dez using winget:

```sh
winget install -e --id dezIndustries.dez
```

## Uninstall

- Installed via installer: Use `Settings` → `Apps` → `Installed apps`, search for dez, and click Uninstall.
- Built from source: Remove the build output directory you created (e.g., your target/install folder).

Your settings and extensions live in your user profile. When uninstalling, you can choose to keep or remove them.

## Remote Development (SSH)

dez supports remote development on Windows through both SSH and WSL. You can connect to remote servers via SSH or work with files inside WSL distributions directly from dez.

For detailed instructions on setting up and using remote development features, including SSH configuration, WSL setup, and troubleshooting, see the [Remote Development documentation](./remote-development.md).

## Troubleshooting

### dez fails to start or shows a blank window

- Check that your hardware and operating system version are compatible with dez. See our [installation guide](./installation.md) for more information.
- Update your GPU drivers from your GPU vendor (Intel/AMD/NVIDIA/Qualcomm).
- Ensure hardware acceleration is enabled in Windows and not blocked by third‑party software.
- Try launching dez with no extensions or custom settings to isolate conflicts.

### Terminal issues

If activation scripts don’t run, update to the latest version and verify your shell profile files are not exiting early. For Git operations, confirm Git Bash or PowerShell is available and on PATH.

### SSH remoting problems

When prompted for credentials, use the graphical askpass dialog. If it doesn’t appear, check for credential manager conflicts and that GUI prompts aren’t blocked by your terminal.

### Graphics issues

#### dez fails to open / degraded performance

dez requires a DirectX 11 compatible GPU to run. If dez fails to open, your GPU may not meet the minimum requirements.

To check if your GPU supports DirectX 11, run the following command:

```
dxdiag
```

This will open the DirectX Diagnostic Tool, which shows the DirectX version your GPU supports under `System` → `System Information` → `DirectX Version`.

If you're running dez inside a virtual machine, it will use the emulated adapter provided by your VM. While dez will work in this environment, performance may be degraded.
