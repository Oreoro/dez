---
title: Appearance and Visual Customization - Dez
description: Customize Dez's themes, fonts, icons, UI density, and other visual settings to match your preferences.
---

# Appearance

Dez starts with the translucent Lumin theme, JetBrains Mono for code and
terminals, and a readable sans-serif interface. This guide shows how to
customize those visual defaults.

For information on how the settings system works, see [All Settings](./reference/all-settings.md).

## Customize Dez in 5 Minutes

Here's how to make Dez feel like home:

1. **Pick a theme**: Press {#kb theme_selector::Toggle} to open the Theme Selector. Arrow through the list to preview themes in real time, and press Enter to apply.

2. **Toggle light/dark mode quickly**: Press {#kb theme::ToggleMode}. If you currently use a static `"theme": "..."` value, the first toggle converts it to dynamic mode settings with default themes.

3. **Choose an icon theme**: Run {#action icon_theme_selector::Toggle} from the command palette to browse icon themes.

4. **Set your font**: Open the Settings Editor with {#kb zed::OpenSettings} and search for `buffer_font_family`. Set it to your preferred coding font.

5. **Adjust font size**: In the same Settings Editor, search for `buffer_font_size` and `ui_font_size` to tweak the editor and interface text sizes.

That's it. You now have a personalized Dez setup.

## Themes

Install themes from the Extensions page ({#action zed::Extensions}), then switch between them with the Theme Selector ({#kb theme_selector::Toggle}).

Dez bundles Lumin and follows your system appearance by default:

```json [settings]
{
  "theme": {
    "mode": "system",
    "light": "Lumin Light",
    "dark": "Lumin Blur"
  }
}
```

You can also override specific theme attributes for fine-grained control.

→ [Themes documentation](./themes.md)

## Icon Themes

Customize file and folder icons in Files and tabs. Browse available icon themes with the Icon Theme Selector ({#action icon_theme_selector::Toggle} in the command palette).

Like color themes, icon themes support separate light and dark variants:

```json [settings]
{
  "icon_theme": {
    "mode": "system",
    "light": "Zed (Default)",
    "dark": "Zed (Default)"
  }
}
```

→ [Icon Themes documentation](./icon-themes.md)

## Fonts

Dez uses a sans-serif face for interface chrome and JetBrains Mono for code.
The bundled font means editor and terminal typography is consistent even on a
new machine.

| Setting                | Used for                  |
| ---------------------- | ------------------------- |
| `buffer_font_family`   | Editor text               |
| `ui_font_family`       | Interface elements        |
| `terminal.font_family` | [Terminal](./terminal.md) |

Example configuration:

```json [settings]
{
  "buffer_font_family": "JetBrains Mono",
  "buffer_font_size": 14,
  "ui_font_family": ".ZedSans",
  "ui_font_size": 14,
  "terminal": {
    "font_family": "JetBrains Mono",
    "font_size": 14
  }
}
```

### Font Ligatures

To disable font ligatures:

```json [settings]
{
  "buffer_font_features": {
    "calt": false
  }
}
```

### Line Height

Adjust line spacing with `buffer_line_height`:

- `"comfortable"` — 1.618 ratio (default)
- `"standard"` — 1.3 ratio
- `{ "custom": 1.5 }` — Custom ratio

## UI Elements

Dez provides extensive control over UI elements including:

- **Tab bar** — Show/hide, navigation buttons, file icons, git status
- **Status bar** — Language selector, cursor position, line endings
- **Scrollbar** — Visibility, git diff indicators, search results
- **Minimap** — Code overview display
- **Gutter** — Line numbers, fold indicators, breakpoints
- **Workspace Tools and Agent** — Tool-pane sizing, visibility, and placement

→ [Visual Customization documentation](./visual-customization.md) for all UI element settings

## What's Next

- [All Settings](./reference/all-settings.md) — Complete settings reference
- [Key bindings](./key-bindings.md) — Customize keyboard shortcuts
- [Vim Mode](./vim.md) — Enable modal editing
