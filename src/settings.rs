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
            min_density: 0.002,
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
