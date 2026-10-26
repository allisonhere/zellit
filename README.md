# zellit

> **Previously known as `zellij-bar-theme-config` / `zellij-tab-config`.** Renamed to `zellit` starting with v0.7.0. The old names redirect to this repo. I got tired of remembering and typing zellij-tab-config ;)

A terminal UI for creating, editing, and applying [Zellij](https://zellij.dev) themes. Live preview of your changes across all UI components — tab bar, panes, status bar, tables, lists — before writing anything to disk.

![zellit screenshot](screen.png)

> **New — sidebar navigation.** The element selector is now a single **sidebar tree** on the left, grouped by section (TabBar / Panes / Content / Status). Move through every element with `↑ ↓` (or `j` / `k`), press `1`–`4` to jump straight to a group, and `/` to fuzzy-search any element. Press `A` for an About screen with project links.

## Features

- **Live preview** — see every color change reflected instantly across a full Zellij-layout mockup
- **41 built-in themes** — all official Zellij themes bundled, no extra files needed
- **Per-component theming** — control foreground and background for Zellij UI elements independently
- **Multiplayer colors** — edit the 10 `player_1`–`player_10` cursor/pane-border colors Zellij shows for other clients in a shared session
- **Dual-mode color picker** — RGB sliders plus an HSL field picker with live HEX/RGB/HSL values
- **Yank / paste / undo** — copy a color, paste it elsewhere, or undo the last change (`y` / `p` / `u`)
- **Theme loader** — fuzzy search, filter by built-in or saved, live preview on scroll
- **Rename / delete** — manage saved themes directly from the loader (`r` / `d`)
- **Save themes** — writes to `~/.config/zellij/themes/` in the correct KDL format
- **Apply to Zellij** — sets `theme "<name>"` in `~/.config/zellij/config.kdl` so Zellij picks it up on next launch
- **Preview backdrop** — pane interiors use the active theme's main UI background as an editor aid; actual terminal pane background is separate Zellij pane/layout state
- **In-app self-update** — checks GitHub releases on startup and can replace the binary in place on Linux x86_64 (`U` when an update is available)
- **Help overlay** — press `?` for a full keybinding reference

## Installation

### Prebuilt binary (Linux x86_64)

```sh
curl -fsSL https://raw.githubusercontent.com/allisonhere/zellit/main/install-binary.sh | sh
```

### Build from source (all platforms)

Requires `git` and `cargo` ([rustup.rs](https://rustup.rs)).

```sh
curl -fsSL https://raw.githubusercontent.com/allisonhere/zellit/main/install-source.sh | sh
```

Both scripts install to `~/.local/bin` by default. Override with `INSTALL_DIR`:

```sh
curl -fsSL ... | INSTALL_DIR=/usr/local/bin sh
```

### Manual install

```sh
git clone https://github.com/allisonhere/zellit.git
cd zellit
cargo build --release
cp target/release/zellit ~/.local/bin/
```

## Usage

```
zellit
```

The app opens a full-terminal preview of a Zellij layout. Use the keyboard to navigate and edit. Pane interiors are painted with the active theme's main UI background so contrast is visible while editing; this is a preview backdrop, not a saved terminal pane background.

### Keybindings

| Key | Action |
|-----|--------|
| `↑ ↓` / `j k` | Move through the sidebar tree of preview elements |
| `1`–`5` | Jump to a group (TabBar / StatusBar / Panes / Content / Multiplayer) |
| `/` | Fuzzy-search and jump to any element |
| `Tab` | Toggle FG / BG (not available on pane borders) |
| `c` / `Enter` | Open color picker for the selected color |
| `y` | Yank (copy) current color |
| `p` | Paste yanked color |
| `u` | Undo last color change |
| `s` | Save theme as… (prompts for a name) |
| `l` | Open theme loader |
| `a` | Apply current theme to Zellij (also recolors any [zjstatus](https://github.com/dj95/zjstatus) plugin in `~/.config/zellij/layouts/*.kdl`; originals saved once as `.kdl.bak`) |
| `z` | Toggle a zjstatus-style bar in the preview |
| `6` | Jump to the zjstatus group (only when a zjstatus plugin is found in `~/.config/zellij/layouts/*.kdl`; otherwise shown greyed as "not detected") |
| `x` | Reset the selected zjstatus slot so it follows the theme again |
| `U` | Install the latest released binary when an update is available on Linux x86_64 |
| `?` | Toggle help overlay |
| `A` | About — project credits and links |
| `q` / `Esc` | Quit |

**Color picker:**

| Key | Action |
|-----|--------|
| `Tab` / `Shift + Tab` | Move focus between picker controls |
| `m` | Switch RGB sliders / HSL field |
| `f` | Toggle FG / BG (non-pane elements only) |
| `↑ ↓ ← →` | Nudge the focused control |
| `Shift` / `Alt` with arrows | Coarse / fine nudging |
| `#` | Jump straight to hex editing |
| `Enter` | Edit the focused value field or confirm |
| `Esc` | Cancel |
| `mouse drag` | Drag in the HSL field or lightness slider |

**Theme loader:**

| Key | Action |
|-----|--------|
| `type` | Search — fuzzy filter by name; `Enter` or `↓` to commit and navigate results |
| `↑ ↓` | Navigate themes |
| `Enter` | Load selected theme into editor |
| `a` | Apply selected theme directly to Zellij |
| `b` | Filter: built-in themes only |
| `s` | Filter: saved themes only |
| `r` | Rename selected saved theme |
| `d` | Delete selected saved theme |
| `Esc` | Clear search / cancel |

### zjstatus

On apply, zellit recolors every [zjstatus](https://github.com/dj95/zjstatus) plugin block in `~/.config/zellij/layouts/*.kdl` from the theme (a one-time `.kdl.bak` is kept). Only colors change: `fg=`/`bg=` in `#[...]` formats and `color_<name>` alias definitions (matched by conventional names such as `bg`, `fg`, `fg_dim`, `red`, `green`, `blue`). `$alias` references and all non-color options are left as written. The zjstatus group lets you override the Bar, Accent, Tab (Active/Inactive) and Muted slots by hand; untouched slots keep following the theme, and overrides are saved next to the theme as `<name>.zjstatus.json`. Each mode pill gets its own color: `locked` uses the theme's error color, `tmux` its success color, and the other modes cycle through the multiplayer colors. Inside Zellij, the default layout is reloaded into the current tab; if the layout points at a missing `zjstatus.wasm`, zellit skips the reload and says so.

## Built-in themes

All 41 official Zellij themes are bundled in the binary and available immediately from the theme loader (`l`):

ansi · ao · atelier · ayu-dark · ayu-light · ayu-mirage · blade-runner · catppuccin-frappe · catppuccin-latte · catppuccin-macchiato · catppuccin-mocha · cyber-noir · dayfox · dracula · everforest-dark · everforest-light · flexoki-dark · gruber-darker · gruvbox-dark · gruvbox-light · iceberg-dark · iceberg-light · kanagawa · lucario · menace · molokai-dark · night-owl · nightfox · nord · one-half-dark · onedark · pencil-light · retro-wave · solarized-dark · solarized-light · terafox · tokyo-night · tokyo-night-dark · tokyo-night-light · tokyo-night-storm · vesper

## Theme format

Themes are saved to `~/.config/zellij/themes/<name>.kdl`:

```kdl
themes {
    my-theme {
        ribbon_selected {
            base 30 30 46
            background 137 180 250
            emphasis_0 255 255 255
            emphasis_1 200 200 200
            emphasis_2 150 150 150
            emphasis_3 100 100 100
        }
        // text_unselected, ribbon_unselected, frame_selected, … etc.
        multiplayer_user_colors {
            player_1 255 121 198
            // … player_2 through player_10
        }
    }
}
```

Standard Zellij palette themes (using `fg`, `bg`, `black`, `red`, … keys) are also supported — the loader maps palette colors to components automatically.

### Terminal pane background

Zellit edits Zellij theme data. Theme `background` values style Zellij-owned UI and plugin components, and the preview pane backdrop uses the theme's main UI background so light/dark variants are easy to judge.

The actual default background of terminal applications inside panes is not saved in theme KDL. In Zellij, that belongs to pane/layout state such as layout `default_bg` or runtime pane color actions, so Zellit treats it as a separate future feature rather than mixing it into theme files.

## Themeable components

| Component | What it styles |
|-----------|---------------|
| `text_unselected` / `text_selected` | Status-bar text — bare UI text / selection text |
| `ribbon_unselected` / `ribbon_selected` | Tabs and status-bar mode pills |
| `frame_unselected` / `frame_selected` / `frame_highlight` | Pane borders (FG only — border color) |
| `table_title` / `table_cell_unselected` / `table_cell_selected` | Table widgets |
| `list_unselected` / `list_selected` | List widgets |
| `exit_code_success` / `exit_code_error` | Command-pane exit code indicators |
| `multiplayer_user_colors` (`player_1`–`player_10`) | Pane border / cursor color shown for each other connected client in a shared session |

## Requirements

- A terminal with true color support
- Rust stable (only needed if building from source)
