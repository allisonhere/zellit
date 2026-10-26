//! Derives zjstatus colors from a Zellij theme and rewrites them inside a layout file.
//!
//! zjstatus is styled with `#[fg=...,bg=...,bold]` segments inside the string properties of
//! its `plugin` block, optionally via `color_<name>` aliases referenced as `$name`. Only color
//! values are rewritten; text, modifiers and everything outside the plugin block are preserved.

use crate::theme::{RgbColor, Theme, ThemeComponent, ThemeComponentType};
use std::collections::HashMap;

/// zjstatus modes (other than `normal`, `locked`, `tmux`) cycle through the theme's
/// multiplayer colors in this order so each gets a distinct pill.
const CYCLED_MODES: [&str; 11] = [
    "resize",
    "pane",
    "tab",
    "scroll",
    "enter_search",
    "search",
    "rename_tab",
    "rename_pane",
    "session",
    "move",
    "prompt",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZjStatusStyle {
    pub bar_bg: RgbColor,
    pub bar_fg: RgbColor,
    pub muted: RgbColor,
    pub accent_bg: RgbColor,
    pub accent_fg: RgbColor,
    pub active_bg: RgbColor,
    pub active_fg: RgbColor,
    pub normal_bg: RgbColor,
    pub normal_fg: RgbColor,
    pub error: RgbColor,
    pub success: RgbColor,
    pub players: [RgbColor; 10],
}

const PLAYERS: [ThemeComponentType; 10] = [
    ThemeComponentType::Player1,
    ThemeComponentType::Player2,
    ThemeComponentType::Player3,
    ThemeComponentType::Player4,
    ThemeComponentType::Player5,
    ThemeComponentType::Player6,
    ThemeComponentType::Player7,
    ThemeComponentType::Player8,
    ThemeComponentType::Player9,
    ThemeComponentType::Player10,
];

fn mid(a: RgbColor, b: RgbColor) -> RgbColor {
    let avg = |x: u8, y: u8| ((u16::from(x) + u16::from(y)) / 2) as u8;
    RgbColor::new(avg(a.r, b.r), avg(a.g, b.g), avg(a.b, b.b))
}

impl Theme {
    /// Refreshes the zjstatus slots that follow the theme; hand-edited ones are kept.
    pub fn sync_zjstatus(&mut self) {
        let st = ZjStatusStyle::from_theme(self);
        let slots = [
            (ThemeComponentType::ZjBar, st.bar_fg, st.bar_bg),
            (ThemeComponentType::ZjAccent, st.accent_fg, st.accent_bg),
            (ThemeComponentType::ZjTabActive, st.active_fg, st.active_bg),
            (ThemeComponentType::ZjTabInactive, st.normal_fg, st.normal_bg),
            (ThemeComponentType::ZjMuted, st.muted, st.muted),
        ];
        for (slot, fg, bg) in slots {
            if !self.zj_overrides.contains(&slot) {
                self.components.insert(slot, ThemeComponent::new(fg, bg));
            }
        }
    }
}

impl ZjStatusStyle {
    /// Derived from the theme, then adjusted by any hand-edited zjstatus slots.
    pub fn from_theme(theme: &Theme) -> Self {
        let mut st = Self::derive(theme);
        let slot = |t: ThemeComponentType| {
            theme
                .zj_overrides
                .contains(&t)
                .then(|| theme.components.get(&t))
                .flatten()
        };
        if let Some(c) = slot(ThemeComponentType::ZjBar) {
            st.bar_fg = c.base;
            st.bar_bg = c.background;
            st.accent_fg = c.background;
            st.muted = mid(c.base, c.background);
        }
        if let Some(c) = slot(ThemeComponentType::ZjAccent) {
            st.accent_fg = c.base;
            st.accent_bg = c.background;
        }
        if let Some(c) = slot(ThemeComponentType::ZjTabActive) {
            st.active_fg = c.base;
            st.active_bg = c.background;
        }
        if let Some(c) = slot(ThemeComponentType::ZjTabInactive) {
            st.normal_fg = c.base;
            st.normal_bg = c.background;
        }
        if let Some(c) = slot(ThemeComponentType::ZjMuted) {
            st.muted = c.base;
        }
        st
    }

    fn derive(theme: &Theme) -> Self {
        let text = theme.get(ThemeComponentType::TextUnselected);
        let active = theme.get(ThemeComponentType::RibbonSelected);
        let normal = theme.get(ThemeComponentType::RibbonUnselected);
        let frame = theme.get(ThemeComponentType::FrameSelected);
        let accent = frame.base;
        let base_of = |t: ThemeComponentType, fallback: RgbColor| {
            theme.components.get(&t).map_or(fallback, |c| c.base)
        };
        Self {
            bar_bg: text.background,
            bar_fg: text.base,
            muted: mid(text.base, text.background),
            accent_bg: accent,
            accent_fg: text.background,
            active_bg: active.background,
            active_fg: active.base,
            normal_bg: normal.background,
            normal_fg: normal.base,
            error: base_of(ThemeComponentType::ExitCodeError, accent),
            success: base_of(ThemeComponentType::ExitCodeSuccess, accent),
            players: PLAYERS.map(|t| base_of(t, accent)),
        }
    }

    /// Pill color for a zjstatus mode name (`normal`, `locked`, `pane`, ...).
    pub fn mode_color(&self, mode: &str) -> RgbColor {
        match mode {
            "locked" => self.error,
            "tmux" => self.success,
            _ => CYCLED_MODES
                .iter()
                .position(|m| *m == mode)
                .map_or(self.accent_bg, |i| self.players[i % self.players.len()]),
        }
    }

    /// (background, foreground) of the main body segments of a property.
    fn role(&self, key: &str) -> (RgbColor, RgbColor) {
        if key.starts_with("tab_normal") {
            (self.normal_bg, self.normal_fg)
        } else if key.starts_with("tab_") {
            (self.active_bg, self.active_fg)
        } else if let Some(mode) = key.strip_prefix("mode_") {
            (self.mode_color(mode), self.bar_bg)
        } else if key == "notification_format_unread" {
            (self.error, self.bar_bg)
        } else {
            (self.accent_bg, self.accent_fg)
        }
    }

    /// Foreground for text drawn directly on the bar (no pill background).
    fn bar_text(&self, key: &str, on_bar: bool) -> RgbColor {
        if key == "border_format" || key == "notification_format_no_notifications" {
            self.muted
        } else if key == "notification_format_unread" {
            self.error
        } else if key.starts_with("tab_normal") {
            self.muted
        } else if key.starts_with("tab_") {
            self.bar_fg
        } else if key.starts_with("format_") {
            if on_bar { self.bar_fg } else { self.accent_bg }
        } else if on_bar {
            self.bar_fg
        } else {
            self.muted
        }
    }

    /// Color for a `color_<name>` alias definition, by conventional name; `None` leaves it alone.
    fn alias_color(&self, name: &str) -> Option<RgbColor> {
        let n = name.to_ascii_lowercase();
        Some(match n.as_str() {
            "bg" | "background" | "base" | "bar" | "crust" | "mantle" => self.bar_bg,
            "fg" | "foreground" | "text" => self.bar_fg,
            "fg_dim" | "dim" | "muted" | "subtext" | "overlay" | "surface" | "gray" | "grey" => {
                self.muted
            }
            "red" | "error" | "maroon" => self.error,
            "green" | "success" => self.success,
            "blue" | "accent" | "primary" | "lavender" => self.accent_bg,
            "yellow" | "orange" | "peach" => self.players[2],
            "purple" | "mauve" | "magenta" | "pink" => self.players[4],
            "cyan" | "teal" | "sky" => self.players[5],
            _ => return None,
        })
    }
}

struct Segment {
    attrs: Vec<String>,
    text: String,
}

type Aliases = HashMap<String, String>;

fn split_segments(value: &str) -> (String, Vec<Segment>) {
    let Some(first) = value.find("#[") else {
        return (value.to_string(), Vec::new());
    };
    let prefix = value[..first].to_string();
    let mut segments = Vec::new();
    let mut rest = &value[first..];
    while let Some(start) = rest.find("#[") {
        let after = &rest[start + 2..];
        let Some(end) = after.find(']') else {
            // Unterminated segment: keep the remainder verbatim as text.
            segments.push(Segment {
                attrs: Vec::new(),
                text: rest.to_string(),
            });
            return (prefix, segments);
        };
        let attrs = after[..end].split(',').map(|s| s.trim().to_string()).collect();
        let tail = &after[end + 1..];
        let next = tail.find("#[").unwrap_or(tail.len());
        segments.push(Segment {
            attrs,
            text: tail[..next].to_string(),
        });
        rest = &tail[next..];
    }
    (prefix, segments)
}

fn attr_value<'a>(attrs: &'a [String], name: &str) -> Option<&'a str> {
    attrs
        .iter()
        .find_map(|a| a.strip_prefix(name).and_then(|v| v.strip_prefix('=')))
}

/// Resolves a `$alias` to its original definition; plain values pass through. Lowercased.
fn resolve(value: &str, aliases: &Aliases) -> String {
    let v = match value.strip_prefix('$') {
        Some(name) => aliases.get(name).map_or(value, String::as_str),
        None => value,
    };
    v.to_ascii_lowercase()
}

/// Rewrites an existing `fg=`/`bg=` attribute; `$alias` references are left to the alias.
fn set_attr(attrs: &mut [String], name: &str, color: RgbColor) {
    let prefix = format!("{name}=");
    if let Some(a) = attrs.iter_mut().find(|a| a.starts_with(&prefix)) {
        if !a[prefix.len()..].starts_with('$') {
            *a = format!("{prefix}{}", color.to_hex());
        }
    }
}

/// A segment that only draws a separator: blank, or starting with a non-ASCII glyph.
fn is_edge_text(text: &str) -> bool {
    !text.contains('{')
        && (text.trim().is_empty() || text.trim_start().chars().next().is_some_and(|c| !c.is_ascii()))
}

fn recolor_value(
    key: &str,
    value: &str,
    style: &ZjStatusStyle,
    bar_old: Option<&str>,
    aliases: &Aliases,
) -> String {
    let (prefix, mut segments) = split_segments(value);
    let (role_bg, role_fg) = style.role(key);
    let is_bar = |v: Option<&str>| matches!((v, bar_old), (Some(v), Some(b)) if resolve(v, aliases) == b);

    for seg in &mut segments {
        let bg_attr = attr_value(&seg.attrs, "bg");
        let fg_attr = attr_value(&seg.attrs, "fg");
        let on_bar = is_bar(bg_attr);
        let edge = is_edge_text(&seg.text);

        let (fg, bg) = if key == "format_space" {
            (None, Some(style.bar_bg))
        } else if edge && on_bar {
            (Some(role_bg), Some(style.bar_bg))
        } else if edge && is_bar(fg_attr) {
            (Some(style.bar_bg), Some(role_bg))
        } else if bg_attr.is_none() && fg_attr.is_some() {
            (Some(style.bar_text(key, false)), None)
        } else if on_bar {
            (Some(style.bar_text(key, true)), Some(style.bar_bg))
        } else {
            (Some(role_fg), Some(role_bg))
        };
        if let Some(fg) = fg {
            set_attr(&mut seg.attrs, "fg", fg);
        }
        if let Some(bg) = bg {
            set_attr(&mut seg.attrs, "bg", bg);
        }
    }

    let mut out = prefix;
    for seg in segments {
        out.push_str("#[");
        out.push_str(&seg.attrs.join(","));
        out.push(']');
        out.push_str(&seg.text);
    }
    out
}

/// Parses a `key "value"` property line, returning the key and the value's byte range.
fn parse_property(line: &str) -> Option<(&str, std::ops::Range<usize>)> {
    let trimmed = line.trim_start();
    let indent = line.len() - trimmed.len();
    let key_end = trimmed.find(char::is_whitespace)?;
    let key = &trimmed[..key_end];
    let after = trimmed[key_end..].trim_start();
    if !after.starts_with('"') || !after.trim_end().ends_with('"') || after.trim_end().len() < 2 {
        return None;
    }
    let value_start = line.len() - after.len() + 1;
    let value_end = indent + trimmed.trim_end().len() - 1;
    (value_end >= value_start).then_some((key, value_start..value_end))
}

fn brace_delta(line: &str) -> i32 {
    let mut delta = 0;
    let mut in_str = false;
    for c in line.chars() {
        match c {
            '"' => in_str = !in_str,
            '{' if !in_str => delta += 1,
            '}' if !in_str => delta -= 1,
            _ => {}
        }
    }
    delta
}

/// The `location="file:..."` path of every zjstatus plugin in `src`, `~` left unexpanded.
pub fn plugin_locations(src: &str) -> Vec<String> {
    src.lines()
        .filter(|l| l.contains("plugin") && l.contains("zjstatus"))
        .filter_map(|l| {
            let rest = &l[l.find("location=\"")? + "location=\"".len()..];
            let loc = &rest[..rest.find('"')?];
            Some(loc.strip_prefix("file:").unwrap_or(loc).to_string())
        })
        .collect()
}

/// Rewrites the colors of every zjstatus plugin block in `src`. Returns `src` unchanged when
/// it contains none.
pub fn recolor_layout(src: &str, style: &ZjStatusStyle) -> String {
    let lines: Vec<&str> = src.split('\n').collect();
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        let opens_block = line.contains("plugin") && line.contains("zjstatus") && line.contains('{');
        if !opens_block {
            out.push(line.to_string());
            i += 1;
            continue;
        }

        // Collect the whole plugin block.
        let mut depth = 0;
        let mut end = i;
        for (j, l) in lines.iter().enumerate().skip(i) {
            depth += brace_delta(l);
            end = j;
            if depth <= 0 {
                break;
            }
        }
        let block = &lines[i + 1..end];

        // Original alias definitions (`color_<name> "<value>"`), used to classify `$name` uses.
        let aliases: Aliases = block
            .iter()
            .filter_map(|l| {
                let (key, range) = parse_property(l)?;
                let name = key.strip_prefix("color_")?;
                Some((name.to_string(), l[range].to_string()))
            })
            .collect();

        let bar_old = block.iter().find_map(|l| {
            let (key, range) = parse_property(l)?;
            if key != "format_space" {
                return None;
            }
            let (_, segs) = split_segments(&l[range]);
            segs.iter()
                .find_map(|s| attr_value(&s.attrs, "bg").map(|v| resolve(v, &aliases)))
        });

        out.push(line.to_string());
        for l in block {
            let Some((key, range)) = parse_property(l) else {
                out.push((*l).to_string());
                continue;
            };
            let value = &l[range.clone()];
            let new = if let Some(name) = key.strip_prefix("color_") {
                style.alias_color(name).map(|c| c.to_hex())
            } else if value.contains("#[") {
                Some(recolor_value(key, value, style, bar_old.as_deref(), &aliases))
            } else {
                None
            };
            match new {
                Some(new) => out.push(format!("{}{}{}", &l[..range.start], new, &l[range.end..])),
                None => out.push((*l).to_string()),
            }
        }
        if end > i {
            out.push(lines[end].to_string());
        }
        i = end + 1;
    }

    out.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const LAYOUT: &str = r##"layout {
    default_tab_template {
        children

        pane size=1 borderless=true {
            plugin location="file:~/.config/zellij/plugins/zjstatus.wasm" {
                format_left  "{mode}#[fg=black,bg=blue,bold]{session}  #[fg=blue,bg=#181825]{tabs}"
                format_right "#[fg=#181825,bg=#b1bbfa]{datetime}"
                format_space "#[bg=#181825]"

                hide_frame_for_single_pane "true"

                mode_normal  "#[bg=blue] "

                tab_normal              "#[fg=#181825,bg=#4C4C59] #[fg=#000000,bg=#4C4C59]{index}  {name} #[fg=#4C4C59,bg=#181825]"
                tab_normal_fullscreen   "#[fg=#6C7086,bg=#181825] {index} {name} [] "
                tab_active              "#[fg=#181825,bg=#ffffff,bold,italic] {index}  {name} #[fg=#ffffff,bg=#181825]"

                datetime          "#[fg=#6C7086,bg=#b1bbfa,bold] {format} "
                datetime_format   "%A, %d %b %Y %H:%M"
            }
        }
    }
}
"##;

    fn style() -> ZjStatusStyle {
        ZjStatusStyle::from_theme(&Theme::default())
    }

    #[test]
    fn recolors_roles_and_keeps_structure() {
        let s = style();
        let out = recolor_layout(LAYOUT, &s);
        let bar = s.bar_bg.to_hex();
        assert!(out.contains(&format!("format_space \"#[bg={bar}]\"")));
        assert!(out.contains(&format!(
            "tab_active              \"#[fg={},bg={},bold,italic] {{index}}  {{name}} #[fg={},bg={bar}]\"",
            s.active_fg.to_hex(),
            s.active_bg.to_hex(),
            s.active_bg.to_hex()
        )));
        assert!(out.contains(&format!("mode_normal  \"#[bg={}] \"", s.accent_bg.to_hex())));
        assert!(out.contains("hide_frame_for_single_pane \"true\""));
        assert!(out.contains("datetime_format   \"%A, %d %b %Y %H:%M\""));
        assert!(!out.contains("#b1bbfa") && !out.contains("#4C4C59") && !out.contains("#181825"));
    }

    #[test]
    fn is_idempotent() {
        let s = style();
        let once = recolor_layout(LAYOUT, &s);
        assert_eq!(recolor_layout(&once, &s), once);
    }

    #[test]
    fn leaves_layouts_without_zjstatus_untouched() {
        let src = "layout {\n    pane\n    pane size=1 {\n        plugin location=\"zellij:status-bar\"\n    }\n}\n";
        assert_eq!(recolor_layout(src, &style()), src);
    }

    #[test]
    fn only_color_values_change() {
        let out = recolor_layout(LAYOUT, &style());
        let strip = |s: &str| {
            let mut r = String::new();
            let mut it = s.chars().peekable();
            let mut in_hex = false;
            while let Some(c) = it.next() {
                if c == '#' && it.peek().is_some_and(|n| n.is_ascii_hexdigit()) {
                    in_hex = true;
                    continue;
                }
                if in_hex && c.is_ascii_hexdigit() {
                    continue;
                }
                in_hex = false;
                r.push(c);
            }
            r
        };
        // Named colors (black/blue) become hex, so compare line counts and non-color keys.
        assert_eq!(out.lines().count(), LAYOUT.lines().count());
        assert!(strip(&out).contains("{session}") && strip(&out).contains("{tabs}"));
    }

    const ALIAS_LAYOUT: &str = r##"layout {
    pane size=1 borderless=true {
        plugin location="file:~/.config/zellij/plugins/zjstatus.wasm" {
            color_bg     "#181825"
            color_fg_dim "#6C7086"
            color_blue   "#89b4fa"

            format_left   "{mode} #[fg=$blue,bold]{session}"
            format_space  "#[bg=$bg]"
            border_format "#[fg=#6C7086]{char}"

            mode_normal  "#[bg=$blue] {name} "
            mode_locked  "#[bg=#f38ba8] {name} "
            mode_pane    "#[bg=#a6e3a1] {name} "

            tab_normal   "#[fg=#6C7086] {name} "
            datetime     "#[fg=#6C7086,bold] {format} "
        }
    }
}
"##;

    #[test]
    fn aliases_are_redefined_and_references_preserved() {
        let s = style();
        let out = recolor_layout(ALIAS_LAYOUT, &s);
        assert!(out.contains(&format!("color_bg     \"{}\"", s.bar_bg.to_hex())));
        assert!(out.contains(&format!("color_fg_dim \"{}\"", s.muted.to_hex())));
        assert!(out.contains(&format!("color_blue   \"{}\"", s.accent_bg.to_hex())));
        assert!(out.contains("#[fg=$blue,bold]{session}"));
        assert!(out.contains("format_space  \"#[bg=$bg]\""));
        assert!(out.contains("mode_normal  \"#[bg=$blue] {name} \""));
        assert_eq!(recolor_layout(&out, &s), out);
    }

    #[test]
    fn modes_get_distinct_colors() {
        let mut theme = Theme::default();
        theme.get_mut(ThemeComponentType::ExitCodeError).base = RgbColor::new(255, 0, 0);
        theme.get_mut(ThemeComponentType::Player2).base = RgbColor::new(0, 0, 255);
        let s = ZjStatusStyle::from_theme(&theme);
        let out = recolor_layout(ALIAS_LAYOUT, &s);
        assert!(out.contains("mode_locked  \"#[bg=#ff0000] {name} \""));
        assert!(out.contains("mode_pane    \"#[bg=#0000ff] {name} \""));
    }

    #[test]
    fn border_and_bare_text_are_visible_on_the_bar() {
        let s = style();
        let out = recolor_layout(ALIAS_LAYOUT, &s);
        let muted = s.muted.to_hex();
        assert!(out.contains(&format!("border_format \"#[fg={muted}]{{char}}\"")));
        assert!(out.contains(&format!("datetime     \"#[fg={muted},bold] {{format}} \"")));
        assert!(!out.contains(&format!("fg={}", s.bar_bg.to_hex())));
    }

    #[test]
    fn finds_plugin_locations() {
        assert_eq!(
            plugin_locations(ALIAS_LAYOUT),
            vec!["~/.config/zellij/plugins/zjstatus.wasm".to_string()]
        );
    }

    #[test]
    fn overridden_slots_win_and_others_follow_the_theme() {
        let mut theme = Theme::default();
        theme.get_mut(ThemeComponentType::ZjTabActive).background = RgbColor::new(1, 2, 3);
        theme.zj_overrides.insert(ThemeComponentType::ZjTabActive);
        theme.sync_zjstatus();
        let before = ZjStatusStyle::from_theme(&theme);
        assert_eq!(before.active_bg, RgbColor::new(1, 2, 3));

        theme.get_mut(ThemeComponentType::TextUnselected).background = RgbColor::new(9, 9, 9);
        theme.sync_zjstatus();
        let after = ZjStatusStyle::from_theme(&theme);
        assert_eq!(after.active_bg, RgbColor::new(1, 2, 3));
        assert_eq!(after.bar_bg, RgbColor::new(9, 9, 9));
        assert_eq!(theme.get(ThemeComponentType::ZjBar).background, RgbColor::new(9, 9, 9));
    }
}
