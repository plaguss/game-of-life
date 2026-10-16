use std::collections::{HashMap, HashSet};
use std::fmt;

pub type AliveCells = HashSet<(i32, i32)>;

#[derive(Debug, Clone)]
pub struct PatternError {
    name: String,
}

impl std::error::Error for PatternError {}

impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        write!(f, "Invalid pattern '{name}'")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PatternName {
    Blinker,
    Toad,
    Beacon,
    #[default]
    Rpentomino,
    Pulsar,
    PentaDecathlon,
    Glider,
    GliderGun,
    Bunnies,
}

// TODO: Nothing ensures completeness here, it should be validated in some form, the same applies to the themes
pub const ALL_PATTERNS: [PatternName; 9] = [
    PatternName::Blinker,
    PatternName::Toad,
    PatternName::Beacon,
    PatternName::Rpentomino,
    PatternName::Pulsar,
    PatternName::PentaDecathlon,
    PatternName::Glider,
    PatternName::GliderGun,
    PatternName::Bunnies,
];

impl fmt::Display for PatternName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatternName::Blinker => write!(f, "Blinker"),
            PatternName::Toad => write!(f, "Toad"),
            PatternName::Beacon => write!(f, "Beacon"),
            PatternName::Rpentomino => write!(f, "R-pentomino"),
            PatternName::Pulsar => write!(f, "Pulsar"),
            PatternName::PentaDecathlon => write!(f, "Penta Decathlon"),
            PatternName::Glider => write!(f, "Glider"),
            PatternName::GliderGun => write!(f, "Glider Gun"),
            PatternName::Bunnies => write!(f, "Bunnies"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Pattern {
    /// Name of the pattern
    name: &'static str,
    /// Position of the alive cells in the grid
    pub alive_cells: AliveCells,
}

impl Pattern {
    pub fn new(name: &'static str, alive_cells: AliveCells) -> Self {
        Self { name, alive_cells }
    }

    pub fn default() -> Self {
        Self::from(PatternName::Rpentomino)
    }

    pub fn from(pattern_name: PatternName) -> Self {
        match pattern_name {
            PatternName::Blinker => Self::from_name("Blinker").unwrap(),
            PatternName::Toad => Self::from_name("Toad").unwrap(),
            PatternName::Beacon => Self::from_name("Beacon").unwrap(),
            PatternName::Rpentomino => Self::from_name("R-pentomino").unwrap(),
            PatternName::Pulsar => Self::from_name("Pulsar").unwrap(),
            PatternName::PentaDecathlon => Self::from_name("PentaDecathlon").unwrap(),
            PatternName::Glider => Self::from_name("Glider").unwrap(),
            PatternName::GliderGun => Self::from_name("GliderGun").unwrap(),
            PatternName::Bunnies => Self::from_name("Bunnies").unwrap(),
        }
    }

    pub fn from_name(name: &'static str) -> Result<Self, PatternError> {
        let available_cells = available_patterns();
        let alive_cells = available_cells.get(name).ok_or_else(|| PatternError {
            name: name.to_string(),
        })?;

        Ok(Self {
            name,
            alive_cells: alive_cells.iter().copied().collect(),
        })
    }
}

pub fn available_patterns() -> HashMap<&'static str, &'static [(i32, i32)]> {
    let mut patterns: HashMap<&'static str, &'static [(i32, i32)]> = HashMap::new();

    patterns.insert("Blinker", &[(2, 1), (2, 2), (2, 3)]);
    patterns.insert("Toad", &[(2, 2), (2, 3), (2, 4), (3, 1), (3, 2), (3, 3)]);

    patterns.insert("Beacon", &[(1, 1), (1, 2), (2, 1), (4, 3), (4, 4), (3, 4)]);

    patterns.insert(
        "Pulsar",
        &[
            (2, 4),
            (2, 5),
            (2, 6),
            (2, 10),
            (2, 11),
            (2, 12),
            (4, 2),
            (5, 2),
            (6, 2),
            (4, 7),
            (5, 7),
            (6, 7),
            (4, 9),
            (5, 9),
            (6, 9),
            (4, 14),
            (5, 14),
            (6, 14),
            (7, 4),
            (7, 5),
            (7, 6),
            (7, 10),
            (7, 11),
            (7, 12),
            (9, 4),
            (9, 5),
            (9, 6),
            (9, 10),
            (9, 11),
            (9, 12),
            (10, 2),
            (11, 2),
            (12, 2),
            (10, 7),
            (11, 7),
            (12, 7),
            (10, 9),
            (11, 9),
            (12, 9),
            (10, 14),
            (11, 14),
            (12, 14),
            (14, 4),
            (14, 5),
            (14, 6),
            (14, 10),
            (14, 11),
            (14, 12),
        ],
    );

    patterns.insert(
        "Penta Decathlon",
        &[
            (5, 4),
            (6, 4),
            (7, 4),
            (8, 4),
            (9, 4),
            (10, 4),
            (11, 4),
            (12, 4),
            (5, 5),
            (7, 5),
            (8, 5),
            (9, 5),
            (10, 5),
            (12, 5),
            (5, 6),
            (6, 6),
            (7, 6),
            (8, 6),
            (9, 6),
            (10, 6),
            (11, 6),
            (12, 6),
        ],
    );

    patterns.insert("Glider", &[(0, 2), (1, 0), (1, 2), (2, 1), (2, 2)]);

    patterns.insert(
        "Glider Gun",
        &[
            (0, 24),
            (1, 22),
            (1, 24),
            (2, 12),
            (2, 13),
            (2, 20),
            (2, 21),
            (2, 34),
            (2, 35),
            (3, 11),
            (3, 15),
            (3, 20),
            (3, 21),
            (3, 34),
            (3, 35),
            (4, 0),
            (4, 1),
            (4, 10),
            (4, 16),
            (4, 20),
            (4, 21),
            (5, 0),
            (5, 1),
            (5, 10),
            (5, 14),
            (5, 16),
            (5, 17),
            (5, 22),
            (5, 24),
            (6, 10),
            (6, 16),
            (6, 24),
            (7, 11),
            (7, 15),
            (8, 12),
            (8, 13),
        ],
    );

    patterns.insert(
        "Bunnies",
        &[
            (10, 10),
            (10, 16),
            (11, 12),
            (11, 16),
            (12, 12),
            (12, 15),
            (12, 17),
            (13, 11),
            (13, 13),
        ],
    );

    patterns.insert("R-pentomino", &[(0, 1), (0, 2), (1, 0), (1, 1), (2, 1)]);

    patterns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_name_and_cells() {
        let cells = HashSet::from([(0, 0), (1, 1)]);
        let pattern = Pattern::new("Custom", cells.clone());

        assert_eq!(pattern.name, "Custom");
        assert_eq!(pattern.alive_cells, cells);
    }

    #[test]
    fn from_name_loads_known_pattern() {
        let pattern = Pattern::from_name("Blinker").unwrap();

        assert_eq!(pattern.name, "Blinker");
        assert_eq!(pattern.alive_cells, HashSet::from([(2, 1), (2, 2), (2, 3)]));
    }

    #[test]
    fn from_name_rejects_unknown_pattern() {
        let error = Pattern::from_name("Unknown").unwrap_err();

        assert_eq!(error.to_string(), "Invalid pattern 'Unknown'");
    }
}
