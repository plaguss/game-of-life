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
    Rpentomino,
    Pulsar,
    PentaDecathlon,
    Glider,
    LightweightSpaceship,
    MiddleweightSpaceship,
    HeavyweightSpaceship,
    Loafer,
    GliderGun,
    Bunnies,
    Diehard,
    Acorn,
    BHeptomino,
    Herschel,
    SwitchEngine,
    Pufferfish,
    #[default]
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
            PatternName::LightweightSpaceship => write!(f, "Lightweight spaceship"),
            PatternName::MiddleweightSpaceship => write!(f, "Middleweight spaceship"),
            PatternName::HeavyweightSpaceship => write!(f, "Heavyweight spaceship"),
            PatternName::Loafer => write!(f, "Loafer"),
            PatternName::GliderGun => write!(f, "Glider Gun"),
            PatternName::Bunnies => write!(f, "Bunnies"),
            PatternName::Diehard => write!(f, "Diehard"),
            PatternName::Acorn => write!(f, "Acorn"),
            PatternName::BHeptomino => write!(f, "B-heptomino"),
            PatternName::Herschel => write!(f, "Herschel"),
            PatternName::SwitchEngine => write!(f, "Switch engine"),
            PatternName::Pufferfish => write!(f, "Pufferfish"),
            PatternName::RandomSoup => write!(f, "Random Soup"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Fixed { alive_cells: AliveCells },
    RandomSoup { density: f32 },
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

        Self::Fixed { alive_cells }
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
        PatternName::LightweightSpaceship,
        &[
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (1, 0),
            (1, 4),
            (2, 4),
            (3, 0),
            (3, 3),
        ],
    );
    patterns.insert(
        PatternName::MiddleweightSpaceship,
        &[
            (0, 3),
            (1, 1),
            (1, 5),
            (2, 0),
            (3, 0),
            (3, 5),
            (4, 0),
            (4, 1),
            (4, 2),
            (4, 3),
            (4, 4),
        ],
    );
    patterns.insert(
        PatternName::HeavyweightSpaceship,
        &[
            (0, 3),
            (0, 4),
            (1, 1),
            (1, 6),
            (2, 0),
            (3, 0),
            (3, 6),
            (4, 0),
            (4, 1),
            (4, 2),
            (4, 3),
            (4, 4),
            (4, 5),
        ],
    );
    patterns.insert(
        PatternName::Loafer,
        &[
            (0, 1),
            (0, 2),
            (0, 5),
            (0, 7),
            (0, 8),
            (1, 0),
            (1, 3),
            (1, 6),
            (1, 7),
            (2, 1),
            (2, 3),
            (3, 2),
            (4, 8),
            (5, 6),
            (5, 7),
            (5, 8),
            (6, 5),
            (7, 6),
            (8, 7),
            (8, 8),
        ],
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

    patterns.insert(
        PatternName::Diehard,
        &[(0, 6), (1, 0), (1, 1), (2, 1), (2, 5), (2, 6), (2, 7)],
    );
    patterns.insert(
        PatternName::Acorn,
        &[(0, 1), (1, 3), (2, 0), (2, 1), (2, 4), (2, 5), (2, 6)],
    );
    patterns.insert(
        PatternName::BHeptomino,
        &[(0, 1), (0, 2), (0, 3), (1, 0), (1, 1), (1, 2), (2, 1)],
    );
    patterns.insert(
        PatternName::Herschel,
        &[(0, 0), (1, 0), (1, 1), (1, 2), (2, 0), (2, 2), (3, 2)],
    );
    patterns.insert(
        PatternName::SwitchEngine,
        &[
            (0, 1),
            (0, 3),
            (1, 0),
            (2, 1),
            (2, 4),
            (3, 3),
            (3, 4),
            (3, 5),
        ],
    );
    patterns.insert(
        PatternName::Pufferfish,
        &[
            (0, 3),
            (0, 11),
            (1, 2),
            (1, 3),
            (1, 4),
            (1, 10),
            (1, 11),
            (1, 12),
            (2, 1),
            (2, 2),
            (2, 5),
            (2, 9),
            (2, 12),
            (2, 13),
            (3, 3),
            (3, 4),
            (3, 5),
            (3, 9),
            (3, 10),
            (3, 11),
            (5, 4),
            (5, 10),
            (6, 2),
            (6, 5),
            (6, 9),
            (6, 12),
            (7, 0),
            (7, 6),
            (7, 8),
            (7, 14),
            (8, 0),
            (8, 1),
            (8, 6),
            (8, 8),
            (8, 13),
            (8, 14),
            (9, 6),
            (9, 8),
            (10, 3),
            (10, 5),
            (10, 9),
            (10, 11),
            (11, 4),
            (11, 10),
        ],
    );

    patterns
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{grid::Grid, rules::Rule};

    fn centered_grid(name: PatternName) -> Grid {
        let mut grid = Grid::new(Rule::game_of_life());
        grid.resize(400, 400);
        grid.set_pattern(&Pattern::from(name));
        grid.center_alive_cells();
        grid
    }

    #[test]
    fn new_patterns_match_catalog_layouts() {
        let cases: &[(PatternName, &[&str])] = &[
            (
                PatternName::LightweightSpaceship,
                &[".OOOO", "O...O", "....O", "O..O."],
            ),
            (
                PatternName::MiddleweightSpaceship,
                &["...O..", ".O...O", "O.....", "O....O", "OOOOO."],
            ),
            (
                PatternName::HeavyweightSpaceship,
                &["...OO..", ".O....O", "O......", "O.....O", "OOOOOO."],
            ),
            (
                PatternName::Loafer,
                &[
                    ".OO..O.OO",
                    "O..O..OO.",
                    ".O.O.....",
                    "..O......",
                    "........O",
                    "......OOO",
                    ".....O...",
                    "......O..",
                    ".......OO",
                ],
            ),
            (PatternName::Diehard, &["......O.", "OO......", ".O...OOO"]),
            (PatternName::Acorn, &[".O.....", "...O...", "OO..OOO"]),
            (PatternName::BHeptomino, &[".OOO", "OOO.", ".O.."]),
            (PatternName::Herschel, &["O..", "OOO", "O.O", "..O"]),
            (
                PatternName::SwitchEngine,
                &[".O.O..", "O.....", ".O..O.", "...OOO"],
            ),
            (
                PatternName::Pufferfish,
                &[
                    "...O.......O...",
                    "..OOO.....OOO..",
                    ".OO..O...O..OO.",
                    "...OOO...OOO...",
                    "...............",
                    "....O.....O....",
                    "..O..O...O..O..",
                    "O.....O.O.....O",
                    "OO....O.O....OO",
                    "......O.O......",
                    "...O.O...O.O...",
                    "....O.....O....",
                ],
            ),
        ];
        let available = available_patterns();
        for &(name, rows) in cases {
            let expected: AliveCells = rows
                .iter()
                .enumerate()
                .flat_map(|(row, cells)| {
                    cells.bytes().enumerate().filter_map(move |(col, cell)| {
                        (cell == b'O').then_some((row as i16, col as i16))
                    })
                })
                .collect();
            let actual = available[&name];
            assert_eq!(actual.len(), expected.len(), "{name}");
            assert_eq!(
                actual.iter().copied().collect::<AliveCells>(),
                expected,
                "{name}"
            );
        }
    }

    #[test]
    fn spaceships_translate_after_their_period() {
        for (name, period, delta) in [
            (PatternName::Glider, 4, (1, 1)),
            (PatternName::LightweightSpaceship, 4, (0, 2)),
            (PatternName::MiddleweightSpaceship, 4, (0, -2)),
            (PatternName::HeavyweightSpaceship, 4, (0, -2)),
            (PatternName::Loafer, 7, (0, -1)),
        ] {
            let mut grid = centered_grid(name);
            let expected: AliveCells = grid
                .get_alive_cells()
                .iter()
                .map(|&(row, col)| (row + delta.0, col + delta.1))
                .collect();
            for _ in 0..period {
                grid.evolve();
            }
            assert_eq!(grid.get_alive_cells(), &expected, "{name}");
        }
    }

    #[test]
    fn oscillators_return_after_their_period() {
        for (name, period) in [
            (PatternName::Blinker, 2),
            (PatternName::Toad, 2),
            (PatternName::Beacon, 2),
            (PatternName::Pulsar, 3),
            (PatternName::PentaDecathlon, 15),
        ] {
            let mut grid = centered_grid(name);
            let initial = grid.get_alive_cells().clone();
            for generation in 1..=period {
                grid.evolve();
                if generation < period {
                    assert_ne!(
                        grid.get_alive_cells(),
                        &initial,
                        "{name}, generation {generation}"
                    );
                } else {
                    assert_eq!(grid.get_alive_cells(), &initial, "{name}");
                }
            }
        }
    }

    #[test]
    fn diehard_dies_at_generation_130() {
        let mut grid = centered_grid(PatternName::Diehard);
        for _ in 0..129 {
            grid.evolve();
        }
        assert!(grid.population() > 0);
        grid.evolve();
        assert_eq!(grid.population(), 0);
    }

    #[test]
    fn growth_patterns_evolve_on_a_large_board() {
        for (name, population) in [
            (PatternName::SwitchEngine, 66),
            (PatternName::Pufferfish, 122),
        ] {
            let mut grid = centered_grid(name);
            for _ in 0..100 {
                grid.evolve();
            }
            assert_eq!(grid.population(), population, "{name}");
        }
    }

    #[test]
    fn from_loads_expected_pattern() {
        let available = available_patterns();
        assert_eq!(available.len(), PatternName::count() - 1);
        assert!(!available.contains_key(&PatternName::RandomSoup));
        for pattern_name in PatternName::iter() {
            match (pattern_name, Pattern::from(pattern_name)) {
                (PatternName::RandomSoup, Pattern::RandomSoup { density }) => {
                    assert_eq!(density, 0.3);
                }
                (PatternName::RandomSoup, Pattern::Fixed { .. }) => {
                    panic!("Random Soup should not be a fixed pattern");
                }
                (name, Pattern::Fixed { alive_cells }) => {
                    let cells = available.get(&name).unwrap();
                    assert!(!cells.is_empty(), "{name}");
                    assert_eq!(alive_cells.len(), cells.len(), "{name}: duplicate cells");
                    assert_eq!(alive_cells, cells.iter().copied().collect(), "{name}");
                }
                (name, Pattern::RandomSoup { .. }) => {
                    panic!("{name} should be a fixed pattern");
                }
            }
        }
    }

    #[test]
    fn pattern_indices_match_menu_order_and_clamp_at_end() {
        for (index, name) in PatternName::iter().enumerate() {
            assert_eq!(PatternName::by_index(index), name);
        }
        assert_eq!(
            PatternName::by_index(PatternName::count() - 1),
            PatternName::RandomSoup
        );
        assert_eq!(PatternName::by_index(usize::MAX), PatternName::RandomSoup);
    }

    #[test]
    fn random_pattern_never_returns_random_soup() {
        for _ in 0..1000 {
            assert!(matches!(random_pattern(), Pattern::Fixed { .. }));
        }
    }
}
