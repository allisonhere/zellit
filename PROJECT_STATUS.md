# zellit

A terminal UI application for creating, editing, and applying Zellij themes.

> Formerly `zellij-tab-config` / `zellij-bar-theme-config`. See README.md for
> install instructions, full keybindings, and the theme file format — this
> doc is a quick internal map of the source layout and current feature set.

## Files

- `src/main.rs` — entry point, wires up `theme` / `config` / `ui` / `update` / `bundled_themes`
- `src/theme/mod.rs` — theme data structures (`RgbColor`, `ThemeComponent`, `Theme`, `ThemeComponentType`)
- `src/config/mod.rs` — `ConfigManager` for KDL theme file parsing/saving, plus `config.kdl` apply logic
- `src/ui/state.rs` — `App` state, `PreviewGroup`/`PreviewElement` navigation model, color editing/undo/yank
- `src/ui/render.rs` — all ratatui rendering: sidebar tree, live preview, color picker overlay, theme loader, help/about screens
- `src/ui/events.rs` — keyboard/mouse input handling per `InputMode`
- `src/ui/color_picker.rs` — `ColorEditor` (RGB sliders + HSL field picker) and its layout math
- `src/update.rs` — in-app self-update against GitHub releases (Linux x86_64)
- `src/bundled_themes.rs` + `src/bundled_themes/*.kdl` — all 41 official Zellij themes, embedded via `include_str!`

## Dependencies

```toml
ratatui = { version = "0.30", features = ["all-widgets"] }
crossterm = "0.29"
kdl = "4"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
dirs = "5"
thiserror = "2"
ureq = { version = "2", features = ["json"] }
palette = { version = "0.7", default-features = false, features = ["std"] }
```

## Features

### Navigation

- **↑ ↓ / j k** — move through the sidebar tree of preview elements
- **1–5** — jump straight to a group (TabBar / StatusBar / Panes / Content / Multiplayer)
- **/** — fuzzy-search and jump to any element
- **Tab** — toggle FG/BG (not available on pane borders or multiplayer player colors — those are single-color)
- **c** / **Enter** — open the color picker for the selected color
- **y / p / u** — yank, paste, undo a color
- **s** — save-as prompt for a named theme
- **l** — open the theme loader (fuzzy search, filter by built-in/saved, rename `r`, delete `d`)
- **a** — apply the current theme to Zellij
- **U** — install the latest release when available (Linux x86_64)
- **?** — help overlay; **A** — about screen; **q / Esc** — quit

### Color Picker

Dual-mode: RGB sliders (pik-style, `█`/`░` bars) or an HSL field with live HEX/RGB/HSL values.
`m` toggles mode, `f` toggles FG/BG (non-single-color elements only), `#` jumps to hex entry,
mouse drag works in the HSL field and lightness slider.

### Editable Elements

| Group | Elements |
|-------|----------|
| Tab Bar | Tab (Selected), Tab (Unselected) |
| Status Bar | Text (Unselected), Text (Selected) |
| Panes | Pane (Selected), Pane (Unselected), Pane (Highlight) — **FG only**, border color |
| Content | Table Title, Table Cell (Selected/Unselected), List (Selected/Unselected) |
| Exit Codes | Exit (Success), Exit (Error) |
| Multiplayer | Player 1 – Player 10 — **FG only**, single color each |

## Theme Components

Each of the 14 standard components (`text_unselected`, `text_selected`, `ribbon_unselected`,
`ribbon_selected`, `table_title`, `table_cell_unselected`, `table_cell_selected`,
`list_unselected`, `list_selected`, `frame_unselected`, `frame_selected`, `frame_highlight`,
`exit_code_success`, `exit_code_error`) has:

- **base** — foreground/primary color
- **background**
- **emphasis_0..3** — additional emphasis levels (not currently editable in this UI)

`multiplayer_user_colors` is a separate 10-entry block (`player_1`..`player_10`), each a single
RGB color — no FG/BG pair, no emphasis levels. It's the pane-border/cursor color Zellij shows for
other clients connected to the same shared session.

## Running

```bash
cargo run
```

Themes are saved to `~/.config/zellij/themes/` as named `.kdl` files.

## KDL Format

```kdl
themes {
    theme_name {
        ribbon_selected {
            base 255 255 255
            background 80 80 80
            emphasis_0 255 255 255
            emphasis_1 200 200 200
            emphasis_2 150 150 150
            emphasis_3 100 100 100
        }
        // ... other 13 standard components
        multiplayer_user_colors {
            player_1 255 121 198
            // ... player_2 through player_10
        }
    }
}
```

Palette-style themes (`fg`, `bg`, `black`, `red`, … keys instead of explicit component blocks) are
also supported — `theme_from_palette` derives all 14 components and the 10 player colors from the
named palette entries.

## TODO / Known Issues

- [ ] No "reset whole theme to default" action (per-edit cancel via `Esc` in the color picker
      already reverts a single in-progress change)
- [ ] `emphasis_0..3` levels remain unexposed in the UI (parsed/preserved, not editable)
- [ ] Single-value (8-bit ANSI index 0–15) color entries in KDL are supported when *loading*
      themes, but zellit always *writes* full RGB triples

## Inspiration

- Color picker UI inspired by [pik](https://github.com/immanelg/pik)
- pik uses horizontal sliders with visual progress bars
- Keyboard-driven navigation similar to vim-style editors
