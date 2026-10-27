use std::time::Duration;

use crate::patterns::{Pattern, PatternName};
use crate::rules::{Rule, RuleName};
use crate::theme::{Theme, ThemeName};

#[derive(Clone, Debug)]
pub struct Settings {
    pub theme: ThemeName,
    pub fps: u8,
    pub pattern: PatternName,
    pub rule: RuleName,
    /// Allow the grid to evolve by itself, and add patterns when it's getting stale.
    pub auto: bool,
    pub min_density: f32,
    /// Whether to render the shadow of the cells, that's the previous alive cells,
    /// which looks like an afterglow.
    pub shadow: bool,
}

/// Builds a elements on demand, fresh and from the corresponding names.
/// This keeps the settings lightweight to move around
impl Settings {
    pub fn default() -> Self {
        Self {
            theme: ThemeName::default(),
            fps: 12,
            pattern: PatternName::default(),
            rule: RuleName::default(),
            auto: false,
            min_density: 0.005,
            shadow: false,
        }
    }

    pub fn get_theme(&self) -> Theme {
        Theme::from(self.theme)
    }

    pub fn get_pattern(&self) -> Pattern {
        Pattern::from(self.pattern)
    }

    pub fn get_rule(&self) -> Rule {
        Rule::from(self.rule)
    }

    /// The tick interval actually translates the FPS to how often to update the app.
    pub fn tick_interval(&self) -> Duration {
        Duration::from_secs_f64(1.0 / f64::from(self.fps.max(1)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn defaults_match_initial_app_settings() {
        let settings = Settings::default();

        assert_eq!(settings.theme, ThemeName::Nord);
        assert_eq!(settings.fps, 12);
        assert_eq!(settings.pattern, PatternName::Rpentomino);
        assert_eq!(settings.rule, RuleName::GameOfLife);
        assert!(!settings.auto);
        assert_eq!(settings.min_density, 0.005);
        assert!(!settings.shadow);
    }

    #[test]
    fn tick_interval_converts_fps_and_handles_zero() {
        let mut settings = Settings::default();
        assert_eq!(settings.tick_interval(), Duration::from_nanos(83_333_333));

        settings.fps = 60;
        assert_eq!(settings.tick_interval(), Duration::from_nanos(16_666_667));

        settings.fps = 1;
        assert_eq!(settings.tick_interval(), Duration::from_secs(1));
        settings.fps = 0;
        assert_eq!(settings.tick_interval(), Duration::from_secs(1));
    }

    #[test]
    fn getters_use_current_selections() {
        let mut settings = Settings::default();
        settings.theme = ThemeName::Matrix;
        settings.pattern = PatternName::Glider;
        settings.rule = RuleName::Highlife;

        assert_eq!(settings.get_theme().alive_cell, Theme::matrix().alive_cell);
        match settings.get_pattern() {
            Pattern::Fixed { alive_cells } => assert_eq!(
                alive_cells,
                HashSet::from([(0, 2), (1, 0), (1, 2), (2, 1), (2, 2)])
            ),
            Pattern::RandomSoup { .. } => panic!("Glider should be a fixed pattern"),
        }
        assert_eq!(settings.get_rule().birth, HashSet::from([3, 6]));

        settings.theme = ThemeName::Nord;
        settings.pattern = PatternName::RandomSoup;
        settings.rule = RuleName::GameOfLife;

        assert_eq!(settings.get_theme().alive_cell, Theme::nord().alive_cell);
        match settings.get_pattern() {
            Pattern::RandomSoup { density } => assert_eq!(density, 0.3),
            Pattern::Fixed { .. } => panic!("Random Soup should not be a fixed pattern"),
        }
        assert_eq!(settings.get_rule().birth, HashSet::from([3]));
    }
}
