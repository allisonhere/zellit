use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Self { r, g, b })
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn saturating_add(self, delta: i8) -> u8 {
        let val = i32::from(self) + i32::from(delta);
        val.clamp(0, 255) as u8
    }

    pub fn saturating_add_unsigned(self, delta: u8) -> u8 {
        let val = i32::from(self) + i32::from(delta);
        val.clamp(0, 255) as u8
    }
}

impl From<RgbColor> for i32 {
    fn from(color: RgbColor) -> Self {
        i32::from(color.r) * 256 * 256 + i32::from(color.g) * 256 + i32::from(color.b)
    }
}

impl From<RgbColor> for u8 {
    fn from(color: RgbColor) -> Self {
        color.r
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ThemeComponent {
    pub base: RgbColor,
    pub background: RgbColor,
    pub emphasis_0: RgbColor,
    pub emphasis_1: RgbColor,
    pub emphasis_2: RgbColor,
    pub emphasis_3: RgbColor,
}

impl ThemeComponent {
    pub fn new(base: RgbColor, background: RgbColor) -> Self {
        Self {
            base,
            background,
            emphasis_0: RgbColor::new(255, 255, 255),
            emphasis_1: RgbColor::new(200, 200, 200),
            emphasis_2: RgbColor::new(150, 150, 150),
            emphasis_3: RgbColor::new(100, 100, 100),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub components: HashMap<ThemeComponentType, ThemeComponent>,
}

impl Theme {
    pub fn get(&self, t: ThemeComponentType) -> &ThemeComponent {
        self.components
            .get(&t)
            .unwrap_or_else(|| panic!("missing component {:?}", t))
    }

    pub fn get_mut(&mut self, t: ThemeComponentType) -> &mut ThemeComponent {
        self.components.entry(t).or_insert_with(ThemeComponent::default)
    }
}

impl Default for Theme {
    fn default() -> Self {
        let mut components = HashMap::new();
        components.insert(
            ThemeComponentType::TextUnselected,
            ThemeComponent::new(RgbColor::new(200, 200, 200), RgbColor::new(30, 30, 30)),
        );
        components.insert(
            ThemeComponentType::TextSelected,
            ThemeComponent::new(RgbColor::new(255, 255, 255), RgbColor::new(60, 60, 60)),
        );
        components.insert(
            ThemeComponentType::RibbonUnselected,
            ThemeComponent::new(RgbColor::new(180, 180, 180), RgbColor::new(40, 40, 40)),
        );
        components.insert(
            ThemeComponentType::RibbonSelected,
            ThemeComponent::new(RgbColor::new(255, 255, 255), RgbColor::new(80, 80, 80)),
        );
        components.insert(
            ThemeComponentType::TableTitle,
            ThemeComponent::new(RgbColor::new(200, 200, 200), RgbColor::new(50, 50, 50)),
        );
        components.insert(
            ThemeComponentType::TableCellUnselected,
            ThemeComponent::new(RgbColor::new(180, 180, 180), RgbColor::new(35, 35, 35)),
        );
        components.insert(
            ThemeComponentType::TableCellSelected,
            ThemeComponent::new(RgbColor::new(255, 255, 255), RgbColor::new(60, 60, 60)),
        );
        components.insert(
            ThemeComponentType::ListUnselected,
            ThemeComponent::new(RgbColor::new(180, 180, 180), RgbColor::new(30, 30, 30)),
        );
        components.insert(
            ThemeComponentType::ListSelected,
            ThemeComponent::new(RgbColor::new(255, 255, 255), RgbColor::new(70, 70, 70)),
        );
        components.insert(
            ThemeComponentType::FrameUnselected,
            ThemeComponent::new(RgbColor::new(100, 100, 100), RgbColor::new(30, 30, 30)),
        );
        components.insert(
            ThemeComponentType::FrameSelected,
            ThemeComponent::new(RgbColor::new(255, 255, 255), RgbColor::new(50, 50, 50)),
        );
        components.insert(
            ThemeComponentType::FrameHighlight,
            ThemeComponent::new(RgbColor::new(255, 200, 100), RgbColor::new(60, 50, 30)),
        );
        components.insert(
            ThemeComponentType::ExitCodeSuccess,
            ThemeComponent::new(RgbColor::new(100, 255, 100), RgbColor::new(30, 30, 30)),
        );
        components.insert(
            ThemeComponentType::ExitCodeError,
            ThemeComponent::new(RgbColor::new(255, 100, 100), RgbColor::new(30, 30, 30)),
        );
        for (player, color) in ThemeComponentType::players().iter().zip(DEFAULT_PLAYER_COLORS) {
            components.insert(*player, ThemeComponent::new(color, RgbColor::new(0, 0, 0)));
        }
        Self {
            name: String::from("default"),
            components,
        }
    }
}

/// Default colors for the 10 multiplayer cursor/border slots, used when a
/// theme file has no `multiplayer_user_colors` block of its own.
const DEFAULT_PLAYER_COLORS: [RgbColor; 10] = [
    RgbColor { r: 220, g: 80, b: 200 },  // player_1 — magenta
    RgbColor { r: 80, g: 140, b: 220 },  // player_2 — blue
    RgbColor { r: 150, g: 90, b: 210 },  // player_3 — purple
    RgbColor { r: 220, g: 200, b: 80 },  // player_4 — yellow
    RgbColor { r: 80, g: 200, b: 200 },  // player_5 — cyan
    RgbColor { r: 210, g: 180, b: 70 },  // player_6 — gold
    RgbColor { r: 220, g: 80, b: 80 },   // player_7 — red
    RgbColor { r: 190, g: 190, b: 200 }, // player_8 — silver
    RgbColor { r: 230, g: 150, b: 190 }, // player_9 — pink
    RgbColor { r: 150, g: 100, b: 70 },  // player_10 — brown
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThemeComponentType {
    TextUnselected,
    TextSelected,
    RibbonUnselected,
    RibbonSelected,
    TableTitle,
    TableCellUnselected,
    TableCellSelected,
    ListUnselected,
    ListSelected,
    FrameUnselected,
    FrameSelected,
    FrameHighlight,
    ExitCodeSuccess,
    ExitCodeError,
    // Multiplayer cursor / pane-border colors for other connected clients.
    // Only `ThemeComponent::base` is used for these — they are single
    // colors, not FG/BG pairs — and they're kept out of `all()` because
    // they serialize to their own `multiplayer_user_colors` KDL block
    // instead of a `{ base; background; emphasis_0..3 }` declaration.
    Player1,
    Player2,
    Player3,
    Player4,
    Player5,
    Player6,
    Player7,
    Player8,
    Player9,
    Player10,
}

impl ThemeComponentType {
    #[allow(dead_code)]
    pub fn label(&self) -> &'static str {
        match self {
            Self::TextUnselected => "Text (Unselected)",
            Self::TextSelected => "Text (Selected)",
            Self::RibbonUnselected => "Ribbon/Tab (Unselected)",
            Self::RibbonSelected => "Ribbon/Tab (Selected)",
            Self::TableTitle => "Table Title",
            Self::TableCellUnselected => "Table Cell (Unselected)",
            Self::TableCellSelected => "Table Cell (Selected)",
            Self::ListUnselected => "List Item (Unselected)",
            Self::ListSelected => "List Item (Selected)",
            Self::FrameUnselected => "Frame (Unselected)",
            Self::FrameSelected => "Frame (Selected)",
            Self::FrameHighlight => "Frame Highlight",
            Self::ExitCodeSuccess => "Exit Code (Success)",
            Self::ExitCodeError => "Exit Code (Error)",
            Self::Player1 => "Player 1",
            Self::Player2 => "Player 2",
            Self::Player3 => "Player 3",
            Self::Player4 => "Player 4",
            Self::Player5 => "Player 5",
            Self::Player6 => "Player 6",
            Self::Player7 => "Player 7",
            Self::Player8 => "Player 8",
            Self::Player9 => "Player 9",
            Self::Player10 => "Player 10",
        }
    }

    pub fn component_key(&self) -> &'static str {
        match self {
            Self::TextUnselected => "text_unselected",
            Self::TextSelected => "text_selected",
            Self::RibbonUnselected => "ribbon_unselected",
            Self::RibbonSelected => "ribbon_selected",
            Self::TableTitle => "table_title",
            Self::TableCellUnselected => "table_cell_unselected",
            Self::TableCellSelected => "table_cell_selected",
            Self::ListUnselected => "list_unselected",
            Self::ListSelected => "list_selected",
            Self::FrameUnselected => "frame_unselected",
            Self::FrameSelected => "frame_selected",
            Self::FrameHighlight => "frame_highlight",
            Self::ExitCodeSuccess => "exit_code_success",
            Self::ExitCodeError => "exit_code_error",
            Self::Player1 => "player_1",
            Self::Player2 => "player_2",
            Self::Player3 => "player_3",
            Self::Player4 => "player_4",
            Self::Player5 => "player_5",
            Self::Player6 => "player_6",
            Self::Player7 => "player_7",
            Self::Player8 => "player_8",
            Self::Player9 => "player_9",
            Self::Player10 => "player_10",
        }
    }

    /// The 14 standard FG/BG style declarations. Excludes the multiplayer
    /// player colors, which have their own KDL shape — see [`Self::players`].
    pub fn all() -> &'static [Self] {
        &[
            Self::TextUnselected,
            Self::TextSelected,
            Self::RibbonUnselected,
            Self::RibbonSelected,
            Self::TableTitle,
            Self::TableCellUnselected,
            Self::TableCellSelected,
            Self::ListUnselected,
            Self::ListSelected,
            Self::FrameUnselected,
            Self::FrameSelected,
            Self::FrameHighlight,
            Self::ExitCodeSuccess,
            Self::ExitCodeError,
        ]
    }

    /// The 10 multiplayer cursor/border color slots, in `player_1..player_10` order.
    pub fn players() -> &'static [Self; 10] {
        &[
            Self::Player1,
            Self::Player2,
            Self::Player3,
            Self::Player4,
            Self::Player5,
            Self::Player6,
            Self::Player7,
            Self::Player8,
            Self::Player9,
            Self::Player10,
        ]
    }
}

