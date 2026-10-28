use std::fmt;

use ratatui::style::{Color, Style};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumIter)]
pub enum ThemeName {
    Matrix,
    Cyberpunk,
    RetroAmber,
    Nord,
    #[default]
    VioletHaze,
    RosePineDawn,
    BlackAndWhite,
}

impl ThemeName {
    pub fn count() -> usize {
        Self::iter().len()
    }

    pub fn by_index(n: usize) -> ThemeName {
        Self::iter().nth(n.clamp(0, Self::count() - 1)).unwrap()
    }
}

impl fmt::Display for ThemeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeName::Matrix => write!(f, "Matrix"),
            ThemeName::Cyberpunk => write!(f, "Cyberpunk"),
            ThemeName::RetroAmber => write!(f, "Retro Amber"),
            ThemeName::Nord => write!(f, "Nord"),
            ThemeName::VioletHaze => write!(f, "Violet Haze"),
            ThemeName::RosePineDawn => write!(f, "Rose Pine Dawn"),
            ThemeName::BlackAndWhite => write!(f, "Black and white"),
        }
    }
}

/// Stores the theme for the App.
///
/// There are series of presets to choose from
#[derive(Default, Clone)]
pub struct Theme {
    pub background: Style,
    pub text: Style,
    pub accent: Style,
    pub alive_cell: Style,
    /// Muted trail for previous-generation cells, with lower contrast than live cells.
    pub shadow_cell: Style,
    pub dead_cell: Style,
}

impl Theme {
    pub fn from(theme_name: ThemeName) -> Self {
        match theme_name {
            ThemeName::Matrix => Theme::matrix(),
            ThemeName::Cyberpunk => Theme::cyberpunk(),
            ThemeName::RetroAmber => Theme::retro_amber(),
            ThemeName::Nord => Theme::nord(),
            ThemeName::VioletHaze => Theme::violet_haze(),
            ThemeName::RosePineDawn => Theme::rose_pine_dawn(),
            ThemeName::BlackAndWhite => Theme::og(),
        }
    }

    pub fn matrix() -> Self {
        Self {
            background: Style::default().bg(Color::Black),
            text: Style::default().fg(Color::Green).bg(Color::Black),
            accent: Style::default().fg(Color::LightGreen).bg(Color::Black),
            alive_cell: Style::default().fg(Color::Green).bg(Color::Black),
            shadow_cell: Style::default().fg(Color::Rgb(0, 70, 0)).bg(Color::Black),
            dead_cell: Style::default().bg(Color::Black),
        }
    }

    pub fn cyberpunk() -> Self {
        Self {
            background: Style::default().bg(Color::Rgb(13, 2, 33)), // Deep night-city purple/black
            text: Style::default()
                .fg(Color::Rgb(0, 240, 255))
                .bg(Color::Rgb(13, 2, 33)), // Neon Cyan
            accent: Style::default()
                .fg(Color::Rgb(255, 232, 26))
                .bg(Color::Rgb(13, 2, 33)), // Cyber Yellow
            alive_cell: Style::default()
                .fg(Color::Rgb(255, 0, 85))
                .bg(Color::Rgb(13, 2, 33)), // Hot Neon Pink
            shadow_cell: Style::default()
                .fg(Color::Rgb(98, 1, 51))
                .bg(Color::Rgb(13, 2, 33)),
            dead_cell: Style::default().bg(Color::Rgb(13, 2, 33)),
        }
    }

    pub fn retro_amber() -> Self {
        Self {
            background: Style::default().bg(Color::Rgb(20, 10, 0)), // Deep dark brown / black
            text: Style::default()
                .fg(Color::Rgb(255, 176, 0))
                .bg(Color::Rgb(20, 10, 0)), // VT100 Amber
            accent: Style::default()
                .fg(Color::Rgb(255, 210, 80))
                .bg(Color::Rgb(20, 10, 0)), // Bright Gold
            alive_cell: Style::default()
                .fg(Color::Rgb(255, 140, 0))
                .bg(Color::Rgb(20, 10, 0)), // Deep Warm Amber
            shadow_cell: Style::default()
                .fg(Color::Rgb(102, 56, 0))
                .bg(Color::Rgb(20, 10, 0)),
            dead_cell: Style::default().bg(Color::Rgb(20, 10, 0)),
        }
    }

    pub fn nord() -> Self {
        Self {
            background: Style::default().bg(Color::Rgb(46, 52, 64)), // Polar Night (Dark Slate)
            text: Style::default()
                .fg(Color::Rgb(236, 239, 244))
                .bg(Color::Rgb(46, 52, 64)), // Snow Storm White
            accent: Style::default()
                .fg(Color::Rgb(143, 188, 187))
                .bg(Color::Rgb(46, 52, 64)), // Frost Teal
            alive_cell: Style::default()
                .fg(Color::Rgb(136, 192, 208))
                .bg(Color::Rgb(46, 52, 64)), // Ice Blue
            shadow_cell: Style::default()
                .fg(Color::Rgb(78, 101, 114))
                .bg(Color::Rgb(46, 52, 64)),
            dead_cell: Style::default().bg(Color::Rgb(46, 52, 64)),
        }
    }

    pub fn violet_haze() -> Self {
        Self {
            background: Style::default().bg(Color::Rgb(23, 19, 31)),
            text: Style::default()
                .fg(Color::Rgb(224, 216, 234))
                .bg(Color::Rgb(23, 19, 31)),
            accent: Style::default()
                .fg(Color::Rgb(230, 169, 204))
                .bg(Color::Rgb(23, 19, 31)),
            alive_cell: Style::default()
                .fg(Color::Rgb(196, 167, 231))
                .bg(Color::Rgb(23, 19, 31)),
            shadow_cell: Style::default()
                .fg(Color::Rgb(83, 68, 95))
                .bg(Color::Rgb(23, 19, 31)),
            dead_cell: Style::default().bg(Color::Rgb(23, 19, 31)),
        }
    }

    pub fn rose_pine_dawn() -> Self {
        Self {
            background: Style::default().bg(Color::Rgb(250, 244, 237)),
            text: Style::default()
                .fg(Color::Rgb(87, 82, 121))
                .bg(Color::Rgb(250, 244, 237)),
            accent: Style::default()
                .fg(Color::Rgb(40, 105, 131))
                .bg(Color::Rgb(250, 244, 237)),
            alive_cell: Style::default()
                .fg(Color::Rgb(180, 99, 122))
                .bg(Color::Rgb(250, 244, 237)),
            shadow_cell: Style::default()
                .fg(Color::Rgb(223, 193, 197))
                .bg(Color::Rgb(250, 244, 237)),
            dead_cell: Style::default().bg(Color::Rgb(250, 244, 237)),
        }
    }

    pub fn og() -> Self {
        Self {
            background: Style::default().bg(Color::Rgb(245, 245, 245)), // Off-white / Soft paper
            text: Style::default()
                .fg(Color::Rgb(20, 20, 20))
                .bg(Color::Rgb(245, 245, 245)), // Ink Black
            accent: Style::default()
                .fg(Color::Rgb(100, 100, 100))
                .bg(Color::Rgb(245, 245, 245)), // Slate Gray
            alive_cell: Style::default()
                .fg(Color::Rgb(0, 0, 0))
                .bg(Color::Rgb(245, 245, 245)), // Pure Black
            shadow_cell: Style::default()
                .fg(Color::Rgb(159, 159, 159))
                .bg(Color::Rgb(245, 245, 245)),
            dead_cell: Style::default().bg(Color::Rgb(245, 245, 245)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_names_select_their_presets() {
        for (name, expected) in [
            (ThemeName::Matrix, Theme::matrix()),
            (ThemeName::Cyberpunk, Theme::cyberpunk()),
            (ThemeName::RetroAmber, Theme::retro_amber()),
            (ThemeName::Nord, Theme::nord()),
            (ThemeName::VioletHaze, Theme::violet_haze()),
            (ThemeName::RosePineDawn, Theme::rose_pine_dawn()),
            (ThemeName::BlackAndWhite, Theme::og()),
        ] {
            let actual = Theme::from(name);
            assert_eq!(actual.background, expected.background, "{name}");
            assert_eq!(actual.text, expected.text, "{name}");
            assert_eq!(actual.accent, expected.accent, "{name}");
            assert_eq!(actual.alive_cell, expected.alive_cell, "{name}");
            assert_eq!(actual.shadow_cell, expected.shadow_cell, "{name}");
            assert_eq!(actual.dead_cell, expected.dead_cell, "{name}");
        }
    }

    #[test]
    fn themes_have_visible_cells_and_consistent_board_backgrounds() {
        for name in ThemeName::iter() {
            let theme = Theme::from(name);
            assert!(theme.background.bg.is_some(), "{name}");
            assert!(theme.alive_cell.fg.is_some(), "{name}");
            assert_ne!(theme.alive_cell.fg, theme.background.bg, "{name}");
            assert_eq!(theme.alive_cell.bg, theme.background.bg, "{name}");
            assert_eq!(theme.dead_cell.bg, theme.background.bg, "{name}");
        }
    }

    #[test]
    fn shadow_cells_have_distinct_colors_and_match_the_background() {
        for name in ThemeName::iter() {
            let theme = Theme::from(name);
            assert!(theme.shadow_cell.fg.is_some(), "{name}");
            assert_ne!(theme.shadow_cell.fg, theme.alive_cell.fg, "{name}");
            assert_ne!(theme.shadow_cell.fg, theme.background.bg, "{name}");
            assert_eq!(theme.shadow_cell.bg, theme.background.bg, "{name}");
        }
    }

    #[test]
    fn ensure_themes_dont_overflow() {
        for (index, name) in ThemeName::iter().enumerate() {
            assert_eq!(ThemeName::by_index(index), name);
        }
        assert_eq!(
            ThemeName::by_index(ThemeName::count() - 1),
            ThemeName::BlackAndWhite
        );
        assert_eq!(
            ThemeName::by_index(ThemeName::count()),
            ThemeName::BlackAndWhite
        );
        assert_eq!(ThemeName::by_index(usize::MAX), ThemeName::BlackAndWhite);
    }
}
