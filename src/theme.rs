use std::fmt;

use ratatui::style::{Color, Style};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumIter)]
pub enum ThemeName {
    Matrix,
    Cyberpunk,
    RetroAmber,
    Ocean,
    Synthwave,
    #[default]
    Nord,
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
            ThemeName::Ocean => write!(f, "Ocean"),
            ThemeName::Synthwave => write!(f, "Synthwave"),
            ThemeName::Nord => write!(f, "Nord"),
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
    pub dead_cell: Style,
}

impl Theme {
    pub fn new(
        background: Style,
        text: Style,
        accent: Style,
        alive_cell: Style,
        dead_cell: Style,
    ) -> Self {
        Self {
            background,
            text,
            accent,
            alive_cell,
            dead_cell,
        }
    }

    pub fn from(theme_name: ThemeName) -> Self {
        match theme_name {
            ThemeName::Matrix => Theme::matrix(),
            ThemeName::Cyberpunk => Theme::cyberpunk(),
            ThemeName::RetroAmber => Theme::retro_amber(),
            ThemeName::Ocean => Theme::ocean(),
            ThemeName::Synthwave => Theme::synthwave(),
            ThemeName::Nord => Theme::nord(),
            ThemeName::BlackAndWhite => Theme::og(),
        }
    }

    pub fn matrix() -> Self {
        Self {
            background: Style::default().bg(Color::Black),
            text: Style::default().fg(Color::Green).bg(Color::Black),
            accent: Style::default().fg(Color::LightGreen).bg(Color::Black),
            alive_cell: Style::default().fg(Color::Green).bg(Color::Black),
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
            dead_cell: Style::default().bg(Color::Rgb(20, 10, 0)),
        }
    }

    pub fn ocean() -> Self {
        Self {
            background: Style::default().bg(Color::Rgb(10, 25, 47)), // Deep Abyss Blue
            text: Style::default()
                .fg(Color::Rgb(100, 225, 225))
                .bg(Color::Rgb(10, 25, 47)), // Seafoam / Aquamarine
            accent: Style::default()
                .fg(Color::Rgb(136, 146, 176))
                .bg(Color::Rgb(10, 25, 47)), // Ocean Slate Gray
            alive_cell: Style::default()
                .fg(Color::Rgb(0, 180, 216))
                .bg(Color::Rgb(10, 25, 47)), // Bioluminescent Blue
            dead_cell: Style::default().bg(Color::Rgb(10, 25, 47)),
        }
    }

    pub fn synthwave() -> Self {
        Self {
            background: Style::default().bg(Color::Rgb(26, 11, 46)), // Deep synthwave dark purple
            text: Style::default()
                .fg(Color::Rgb(0, 245, 255))
                .bg(Color::Rgb(26, 11, 46)), // Electric Cyan
            accent: Style::default()
                .fg(Color::Rgb(255, 144, 31))
                .bg(Color::Rgb(26, 11, 46)), // Sunset Orange
            alive_cell: Style::default()
                .fg(Color::Rgb(255, 41, 117))
                .bg(Color::Rgb(26, 11, 46)), // Hot Neon Magenta/Pink
            dead_cell: Style::default().bg(Color::Rgb(26, 11, 46)),
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
            dead_cell: Style::default().bg(Color::Rgb(46, 52, 64)),
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
            dead_cell: Style::default().bg(Color::Rgb(245, 245, 245)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_themes_dont_overflow() {
        // Grab first, last, and above the last
        assert_eq!(ThemeName::by_index(0), ThemeName::Matrix);
        assert_eq!(
            ThemeName::by_index(ThemeName::count() - 1),
            ThemeName::BlackAndWhite
        );
        assert_eq!(
            ThemeName::by_index(ThemeName::count()),
            ThemeName::BlackAndWhite
        );
    }
}
