use std::collections::{HashMap, HashSet};
use std::fmt;

use rand::RngExt;
use strum::{EnumIter, IntoEnumIterator};

pub type Cell = (i16, i16);
pub type AliveCells = HashSet<Cell>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumIter, Hash)]
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
    RandomSoup,
}

impl PatternName {
    pub fn count() -> usize {
        Self::iter().len()
    }

    pub fn by_index(n: usize) -> PatternName {
        Self::iter().nth(n.clamp(0, Self::count() - 1)).unwrap()
    }
}

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
            PatternName::RandomSoup => write!(f, "Random Soup"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Fixed {
        name: PatternName,
        alive_cells: AliveCells,
    },
    RandomSoup {
        density: f32,
    },
}

impl Pattern {
    /// Creates the recipe for a named pattern
    pub fn from(pattern_name: PatternName) -> Self {
        if pattern_name.eq(&PatternName::RandomSoup) {
            return Self::random_soup(0.3);
        }

        let available_cells = available_patterns();

        let alive_cells = available_cells
            .get(&pattern_name)
            .unwrap()
            .iter()
            .copied()
            .collect();

        Self::Fixed {
            name: pattern_name,
            alive_cells,
        }
    }

    /// Special type of pattern that determines the alive cells based on a density pattern.
    /// Sets a cell to alive with a given probability. For this pattern to work
    /// we need grid size, and we don't know about it on creation time, so we just
    /// use a predetermined density.
    ///
    /// Note: For the moment it won't be configurable by the app.
    pub fn random_soup(density: f32) -> Self {
        Self::RandomSoup { density }
    }
}

/// Select a random pattern from the enum of PatternName, excluding the RandomSoup (which
/// is always the last)
pub fn random_pattern() -> Pattern {
    let mut rng = rand::rng();
    let idx = rng.random_range(0..(PatternName::count() - 1));
    Pattern::from(PatternName::by_index(idx))
}

pub fn available_patterns() -> HashMap<PatternName, &'static [Cell]> {
    let mut patterns: HashMap<PatternName, &'static [Cell]> = HashMap::new();

    patterns.insert(PatternName::Blinker, &[(2, 1), (2, 2), (2, 3)]);
    patterns.insert(
        PatternName::Toad,
        &[(2, 2), (2, 3), (2, 4), (3, 1), (3, 2), (3, 3)],
    );

    patterns.insert(
        PatternName::Beacon,
        &[(1, 1), (1, 2), (2, 1), (4, 3), (4, 4), (3, 4)],
    );

    patterns.insert(
        PatternName::Pulsar,
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
        PatternName::PentaDecathlon,
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

    patterns.insert(
        PatternName::Glider,
        &[(0, 2), (1, 0), (1, 2), (2, 1), (2, 2)],
    );

    patterns.insert(
        PatternName::GliderGun,
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
        PatternName::Bunnies,
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

    patterns.insert(
        PatternName::Rpentomino,
        &[(0, 1), (0, 2), (1, 0), (1, 1), (2, 1)],
    );

    patterns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_loads_expected_pattern() {
        let available = available_patterns();
        for pattern_name in PatternName::iter() {
            let pattern = Pattern::from(pattern_name);
            match pattern {
                Pattern::Fixed { name, alive_cells } => {
                    assert_eq!(name, pattern_name);
                    assert_eq!(
                        alive_cells,
                        available
                            .get(&pattern_name)
                            .unwrap()
                            .iter()
                            .copied()
                            .collect()
                    );
                }
                _ => {} // don't check anything for the moment
            }
        }
    }

    #[test]
    fn random_pattern_never_returns_random_soup() {
        for _ in 0..1000 {
            assert!(matches!(random_pattern(), Pattern::Fixed { .. }));
        }
    }
}
