use std::time::Duration;

use crate::patterns::{Pattern, PatternName};
use crate::theme::{Theme, ThemeName};

#[derive(Clone, Debug)]
pub struct Settings {
    pub theme: ThemeName,
    pub fps: u8,
    pub pattern: PatternName,
}

/// Builds a elements on demand, fresh and from the corresponding names.
/// This keeps the settings lightweight to move around
impl Settings {
    pub fn default() -> Self {
        Self {
            theme: ThemeName::default(),
            fps: 12,
            pattern: PatternName::default(),
        }
    }

    pub fn get_theme(&self) -> Theme {
        Theme::from(self.theme)
    }

    pub fn get_pattern(&self) -> Pattern {
        Pattern::from(self.pattern)
    }

    /// The tick interval actually translates the FPS to how often to update the app.
    pub fn tick_interval(&self) -> Duration {
        Duration::from_secs_f64(1.0 / f64::from(self.fps.max(1)))
    }
}
