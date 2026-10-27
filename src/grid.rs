use std::collections::{HashMap, HashSet};

use crate::history::History;
use crate::patterns::{AliveCells, Cell, Pattern};
use crate::rules::Rule;

use rand::RngExt;

fn get_num_neighbours(alive_cells: &AliveCells, height: i16, width: i16) -> HashMap<Cell, u8> {
    // The neighbors are the deltas to check
    let neighbours = [
        (-1, -1), // Above left
        (-1, 0),  // Above
        (-1, 1),  // Above right
        (0, -1),  // Left
        (0, 1),   // Right
        (1, -1),  // Below left
        (1, 0),   // Below
        (1, 1),   // Below right
    ];
    let mut num_neighbours = HashMap::new();
    for (row, col) in alive_cells.iter() {
        for (drow, dcol) in neighbours.iter() {
            // Apply here the rem_euclid to do a toroidal wrap
            let neighbour_row = (row + drow).rem_euclid(height);
            let neighbour_col = (col + dcol).rem_euclid(width);
            num_neighbours
                .entry((neighbour_row, neighbour_col))
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }
    num_neighbours
}

#[derive(Debug, Clone)]
pub struct Grid {
    /// Current cells in the board
    alive_cells: AliveCells,
    /// The cells that were alive in the previous generation, so we can draw the
    /// shadow if requested
    previous_cells: AliveCells,
    rule: Rule,
    /// Step number. Shouldn't reach such a high number
    /// for this case.
    step: u32,
    /// grid dimensions, in the same coordinates as the cells
    height: i16,
    width: i16,
    /// History tracker of the board
    history: History,
}

impl Grid {
    pub const MIN_DIMENSION: i16 = 40;
    /// Number of generations kept in the history. It also serves as the
    /// number of unchanged generations after which the board is stagnant.
    pub const HISTORY_WINDOW: usize = 50;

    /// On creation, the alive_cells aren't populated, those come later
    /// when the pattern is set.
    pub fn new(rule: Rule) -> Self {
        Self {
            alive_cells: HashSet::new(),
            previous_cells: HashSet::new(),
            rule,
            step: 0,
            height: Self::MIN_DIMENSION,
            width: Self::MIN_DIMENSION,
            history: History::new(Self::HISTORY_WINDOW),
        }
    }

    pub fn get_alive_cells(&self) -> &AliveCells {
        &self.alive_cells
    }

    pub fn get_previous_cells(&self) -> &AliveCells {
        &self.previous_cells
    }

    pub fn height(&self) -> i16 {
        self.height
    }

    pub fn width(&self) -> i16 {
        self.width
    }

    /// Number of cells alive. Shouldn't reach such a high number
    /// for this case.
    pub fn population(&self) -> usize {
        self.alive_cells.len()
    }

    pub fn density(&self) -> f32 {
        self.population() as f32 / (self.height() as f32 * self.width() as f32)
    }

    /// Stage of the evolution. Increments on each call to evolve.
    pub fn generation(&self) -> u32 {
        self.step
    }

    /// Sets the pattern on the grid on a second stage. We need
    /// to do it like this to ensure the random soup has knowledge of the
    /// size to fill the space properly.
    pub fn set_pattern(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::Fixed { alive_cells } => {
                self.alive_cells = alive_cells.clone();
            }
            Pattern::RandomSoup { density } => {
                self.build_random_soup(*density);
            }
        }
    }

    /// Adds a pattern to the grid during the evolution.
    /// This method is mainly relevant for the "auto" mode, to ensure
    /// the grid doesn't get stale.
    ///
    /// It adds a Pattern::Fixed to a random position within
    /// the available board. Does nothing if the pattern has no cells
    /// or doesn't fit within the grid.
    pub fn add_pattern(&mut self, pattern: &Pattern) {
        if let Pattern::Fixed { alive_cells } = pattern {
            if alive_cells.is_empty() {
                return;
            }
            let mut rng = rand::rng();

            // Get the pattern's bounding box to ensure it fits within the grid.
            // The cells are non-empty, so the min/max always exist.
            let min_row = alive_cells.iter().map(|&(r, _)| r).min().unwrap();
            let min_col = alive_cells.iter().map(|&(_, c)| c).min().unwrap();
            let max_row = alive_cells.iter().map(|&(r, _)| r).max().unwrap();
            let max_col = alive_cells.iter().map(|&(_, c)| c).max().unwrap();
            let pattern_height = max_row - min_row + 1;
            let pattern_width = max_col - min_col + 1;
            if pattern_height > self.height || pattern_width > self.width {
                return;
            }

            let offset_row = rng.random_range(0..=(self.height - pattern_height));
            let offset_col = rng.random_range(0..=(self.width - pattern_width));

            // Reposition the alive cells
            let new_alive_cells: AliveCells = alive_cells
                .iter()
                .map(|&(r, c)| (r - min_row + offset_row, c - min_col + offset_col))
                .collect();

            self.alive_cells = self.alive_cells.union(&new_alive_cells).copied().collect();
        }
    }

    fn build_random_soup(&mut self, density: f32) {
        let mut rng = rand::rng();
        self.alive_cells = HashSet::new();

        for r in 0..self.height {
            for c in 0..self.width {
                let p: f32 = rng.random();
                if p < density {
                    self.alive_cells.insert((r, c));
                }
            }
        }
    }

    /// Resizes the board and recenters the cells within the space when the board
    /// shrinks and there's not enough space.
    ///
    /// - When expanding, nothing is done, just let the cells evolve as usual.
    /// - Shrinking without collisions, nothing changes
    /// - Shrinking with collisions: the whole pattern is recentered. It may look
    ///   like a jump, but the pattern isn't split.
    pub fn resize(&mut self, height: i16, width: i16) {
        self.height = height.max(Self::MIN_DIMENSION);
        self.width = width.max(Self::MIN_DIMENSION);

        // Determine when to recenter the board by checking the possible collisions.
        // The negative positions for row/col covers possible offsets that centering
        // the cells can produce if the pattern is bigger than the board.
        let collides = self
            .alive_cells
            .iter()
            .any(|&(r, c)| r < 0 || c < 0 || r >= self.height || c >= self.width);

        if collides {
            self.center_alive_cells();
        }
    }

    /// Places the pattern in the center of the board.
    /// Initially they are defined to start at the top left, but to make it simpler
    /// to reason (and nicer), it's better to just start with room to evolve in every
    /// direction.
    pub fn center_alive_cells(&mut self) {
        if self.alive_cells.is_empty() {
            return;
        }
        let min_row = self.alive_cells.iter().map(|&(r, _)| r).min().unwrap();
        let max_row = self.alive_cells.iter().map(|&(r, _)| r).max().unwrap();
        let min_col = self.alive_cells.iter().map(|&(_, c)| c).min().unwrap();
        let max_col = self.alive_cells.iter().map(|&(_, c)| c).max().unwrap();

        // Get the limits of the pattern selected
        let pattern_height = max_row - min_row + 1;
        let pattern_width = max_col - min_col + 1;
        // Compute the offset to apply to each of the cells
        let offset_row = (self.height - pattern_height) / 2 - min_row;
        let offset_col = (self.width - pattern_width) / 2 - min_col;

        self.alive_cells = self
            .alive_cells
            .iter()
            .map(|&(r, c)| (r + offset_row, c + offset_col))
            .collect();
    }

    /// Evolve a single step in the grid
    pub fn evolve(&mut self) {
        let num_neighbors = get_num_neighbours(&self.alive_cells, self.height, self.width);

        let mut next_alive_cells = HashSet::new();

        for (cell, num) in num_neighbors.iter() {
            if self.alive_cells.contains(cell) {
                // If it's alive
                if self.rule.survival.contains(num) {
                    next_alive_cells.insert(*cell);
                }
            } else {
                // If it's dead
                if self.rule.birth.contains(num) {
                    next_alive_cells.insert(*cell);
                }
            }
        }
        // Store them in case we want to plot the shadow
        self.previous_cells = self.alive_cells.clone();
        self.alive_cells = next_alive_cells;
        // It shouldn't reach the max, but who knows...
        self.step = self.step.saturating_add(1);
        self.history.add_point(&self.alive_cells);
    }

    /// Checks whether the board is stagnant within the last `generations`.
    pub fn is_stagnant(&self, generations: usize) -> bool {
        self.history.is_stagnant(generations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::PatternName;

    #[test]
    fn new_stores_pattern_cells() {
        let mut grid = Grid::new(Rule::game_of_life());
        grid.set_pattern(&Pattern::from(PatternName::Glider));

        assert_eq!(
            grid.get_alive_cells(),
            &HashSet::from([(0, 2), (1, 0), (1, 2), (2, 1), (2, 2)])
        );
    }

    #[test]
    fn resize_uses_logical_grid_dimensions_and_enforces_minimum() {
        let mut grid = Grid::new(Rule::game_of_life());
        grid.set_pattern(&Pattern::from(PatternName::Blinker));

        grid.resize(53, 79);
        assert_eq!((grid.height(), grid.width()), (53, 79));

        grid.resize(2, 18);
        assert_eq!((grid.height(), grid.width()), (40, 40));
    }

    #[test]
    fn resize_recenters_only_when_cells_do_not_fit() {
        let mut grid = Grid::new(Rule::game_of_life());
        grid.resize(60, 60);
        grid.set_pattern(&Pattern::Fixed {
            alive_cells: HashSet::from([(45, 45), (45, 46)]),
        });

        grid.resize(50, 50);
        assert_eq!(grid.get_alive_cells(), &HashSet::from([(45, 45), (45, 46)]));

        grid.resize(40, 40);
        assert_eq!(grid.get_alive_cells(), &HashSet::from([(19, 19), (19, 20)]));
    }

    #[test]
    fn neighbors_exclude_self_and_count_shared_neighbors() {
        let counts = get_num_neighbours(&HashSet::from([(0, 0), (0, 1)]), 60, 40);

        assert_eq!(counts.len(), 12);
        assert_eq!(counts[&(0, 0)], 1);
        assert_eq!(counts[&(0, 1)], 1);
        assert_eq!(counts[&(59, 0)], 2);
        assert_eq!(counts[&(1, 39)], 1);
    }

    #[test]
    fn pattern_dies_out_without_birth_or_survival_rules() {
        let mut grid = Grid::new(Rule::new(&[], &[]));
        grid.set_pattern(&Pattern::from(PatternName::Blinker));

        grid.evolve();

        assert!(grid.get_alive_cells().is_empty());
    }

    #[test]
    fn blinker_returns_to_initial_state_after_two_steps() {
        let mut grid = Grid::new(Rule::game_of_life());
        grid.set_pattern(&Pattern::from(PatternName::Blinker));

        let initial = grid.get_alive_cells().clone();

        grid.evolve();
        assert_eq!(
            grid.get_alive_cells(),
            &HashSet::from([(1, 2), (2, 2), (3, 2)])
        );

        grid.evolve();
        assert_eq!(grid.get_alive_cells(), &initial);
    }

    #[test]
    fn blinker_evolves_across_grid_edge() {
        let mut grid = Grid::new(Rule::game_of_life());
        grid.set_pattern(&Pattern::Fixed {
            alive_cells: HashSet::from([(39, 10), (0, 10), (1, 10)]),
        });

        grid.evolve();

        assert_eq!(
            grid.get_alive_cells(),
            &HashSet::from([(0, 9), (0, 10), (0, 11)])
        );
    }

    #[test]
    fn evolve_updates_generation_and_previous_cells() {
        let mut grid = Grid::new(Rule::game_of_life());
        grid.set_pattern(&Pattern::from(PatternName::Blinker));
        let initial = grid.get_alive_cells().clone();

        assert_eq!(grid.generation(), 0);
        assert!(grid.get_previous_cells().is_empty());

        grid.evolve();
        let first_generation = grid.get_alive_cells().clone();
        assert_eq!(grid.generation(), 1);
        assert_eq!(grid.get_previous_cells(), &initial);

        grid.evolve();
        assert_eq!(grid.generation(), 2);
        assert_eq!(grid.get_previous_cells(), &first_generation);
        assert_eq!(grid.get_alive_cells(), &initial);
    }

    #[test]
    fn evolve_uses_custom_birth_and_survival_rules() {
        let mut grid = Grid::new(Rule::new(&[2], &[1]));
        grid.set_pattern(&Pattern::from(PatternName::Blinker));

        grid.evolve();

        // Blinker cells (2, 1), (2, 2), (2, 3): the middle cell dies (2 neighbours),
        // the ends survive (1 neighbour), and the dead cells above and below
        // the middle are born (2 neighbours).
        assert_eq!(
            grid.get_alive_cells(),
            &HashSet::from([(2, 1), (2, 3), (1, 1), (1, 3), (3, 1), (3, 3)])
        );
    }

    #[test]
    fn highlife_gives_birth_with_six_neighbors() {
        let pattern = Pattern::Fixed {
            alive_cells: HashSet::from([(9, 9), (9, 10), (9, 11), (10, 9), (10, 11), (11, 9)]),
        };
        let mut highlife = Grid::new(Rule::highlife());
        let mut game_of_life = Grid::new(Rule::game_of_life());
        highlife.set_pattern(&pattern);
        game_of_life.set_pattern(&pattern);

        highlife.evolve();
        game_of_life.evolve();

        assert!(highlife.get_alive_cells().contains(&(10, 10)));
        assert!(!game_of_life.get_alive_cells().contains(&(10, 10)));
    }

    #[test]
    fn random_soup_extreme_densities_replace_existing_cells() {
        let mut grid = Grid::new(Rule::game_of_life());
        grid.set_pattern(&Pattern::from(PatternName::Glider));

        grid.set_pattern(&Pattern::RandomSoup { density: 1.0 });
        assert_eq!(
            grid.population(),
            (grid.height() as usize) * (grid.width() as usize)
        );
        assert!(
            grid.get_alive_cells()
                .iter()
                .all(|&(r, c)| (0..grid.height()).contains(&r) && (0..grid.width()).contains(&c))
        );

        grid.set_pattern(&Pattern::RandomSoup { density: 0.0 });
        assert!(grid.get_alive_cells().is_empty());
    }

    #[test]
    fn add_pattern_is_within_bounds() {
        let mut grid = Grid::new(Rule::game_of_life());
        grid.add_pattern(&Pattern::from(PatternName::Toad));

        let min_row = grid
            .get_alive_cells()
            .iter()
            .map(|&(r, _)| r)
            .min()
            .unwrap();
        let max_row = grid
            .get_alive_cells()
            .iter()
            .map(|&(r, _)| r)
            .max()
            .unwrap();
        let min_col = grid
            .get_alive_cells()
            .iter()
            .map(|&(_, c)| c)
            .min()
            .unwrap();
        let max_col = grid
            .get_alive_cells()
            .iter()
            .map(|&(_, c)| c)
            .max()
            .unwrap();

        assert_eq!(max_row - min_row + 1, 2);
        assert_eq!(max_col - min_col + 1, 4);
        assert!(
            grid.get_alive_cells()
                .iter()
                .all(|&(r, c)| (0..grid.height()).contains(&r) && (0..grid.width()).contains(&c))
        );
    }

    #[test]
    fn add_pattern_preserves_existing_cells() {
        let mut grid = Grid::new(Rule::game_of_life());
        grid.set_pattern(&Pattern::from(PatternName::Glider));
        let existing_cells = grid.get_alive_cells().clone();

        grid.add_pattern(&Pattern::from(PatternName::Toad));

        assert!(existing_cells.is_subset(grid.get_alive_cells()));
    }

    #[test]
    fn add_pattern_translates_exact_fit_with_shifted_coordinates() {
        let mut grid = Grid::new(Rule::game_of_life());
        grid.add_pattern(&Pattern::Fixed {
            alive_cells: HashSet::from([(10, -7), (49, 32), (20, 9)]),
        });

        assert_eq!(
            grid.get_alive_cells(),
            &HashSet::from([(0, 0), (39, 39), (10, 16)])
        );
    }

    #[test]
    fn add_pattern_ignores_empty_patterns() {
        let mut grid = Grid::new(Rule::game_of_life());
        grid.add_pattern(&Pattern::Fixed {
            alive_cells: HashSet::new(),
        });

        assert!(grid.get_alive_cells().is_empty());
    }

    #[test]
    fn add_pattern_ignores_patterns_larger_than_grid() {
        let mut grid = Grid::new(Rule::game_of_life());
        grid.add_pattern(&Pattern::Fixed {
            alive_cells: HashSet::from([(0, 0), (Grid::MIN_DIMENSION, 0)]),
        });

        assert!(grid.get_alive_cells().is_empty());
    }
}
