use std::collections::{HashMap, HashSet};

use crate::patterns::{AliveCells, Pattern};
use crate::rules::Rules;

fn get_num_neighbours(
    alive_cells: &AliveCells,
    height: i32,
    width: i32,
) -> HashMap<(i32, i32), u8> {
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

// TODO: Decrease the height/width to i16, there's no more capacity anyway
#[derive(Debug, Clone)]
pub struct Grid {
    alive_cells: AliveCells,
    rules: Rules,
    /// Step number. Shouldn't reach such a high number
    /// for this case.
    step: u32,
    // grid dimensions, in the same coordinates as the cells
    height: i32,
    width: i32,
}

impl Grid {
    pub const MIN_DIMENSION: i32 = 40;

    pub fn new(pattern: Pattern, rules: Rules) -> Self {
        // TODO: Store the name or something so it can be shown later
        Self {
            alive_cells: pattern.alive_cells,
            rules,
            step: 0,
            height: Self::MIN_DIMENSION,
            width: Self::MIN_DIMENSION,
        }
    }

    pub fn get_alive_cells(&self) -> &AliveCells {
        &self.alive_cells
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    /// Number of cells alive. Shouldn't reach such a high number
    /// for this case.
    pub fn population(&self) -> usize {
        self.alive_cells.len()
    }

    /// Stage of the evolution. Increments on each call to evolve.
    pub fn generation(&self) -> u32 {
        self.step
    }

    /// Resizes the board and recenters the cells within the space when the board
    /// shrinks and there's not enough space.
    ///
    /// - When expanding, nothing is done, just let the cells evolve as usual.
    /// - Shrinking without collisions, nothing changes
    /// - Shrinking with collisions: the whole pattern is recentered. It may look
    /// like a jump, but the pattern isn't split.
    pub fn resize(&mut self, height: i32, width: i32) {
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
                if self.rules.survival.contains(num) {
                    next_alive_cells.insert(cell.clone());
                }
            } else {
                // If it's dead
                if self.rules.birth.contains(num) {
                    next_alive_cells.insert(cell.clone());
                }
            }
        }
        self.alive_cells = next_alive_cells;
        self.step += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_initial_cells() {
        let cells = HashSet::from([(0, 0), (1, 1)]);
        let grid = Grid::new(Pattern::new("Custom", cells.clone()), Rules::game_of_life());

        assert_eq!(grid.get_alive_cells(), &cells);
    }

    #[test]
    fn resize_uses_logical_grid_dimensions_and_enforces_minimum() {
        let mut grid = Grid::new(
            Pattern::new("Custom", HashSet::new()),
            Rules::game_of_life(),
        );

        grid.resize(53, 79);
        assert_eq!((grid.height(), grid.width()), (53, 79));

        grid.resize(2, 18);
        assert_eq!((grid.height(), grid.width()), (40, 40));
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
    fn empty_and_isolated_cells_evolve_to_empty() {
        for cells in [HashSet::new(), HashSet::from([(0, 0)])] {
            let mut grid = Grid::new(Pattern::new("Custom", cells), Rules::game_of_life());

            grid.evolve();

            assert!(grid.get_alive_cells().is_empty());
        }
    }

    #[test]
    fn blinker_returns_to_initial_state_after_two_steps() {
        let initial = HashSet::from([(0, -1), (0, 0), (0, 1)]);
        let mut grid = Grid::new(
            Pattern::new("Blinker", initial.clone()),
            Rules::game_of_life(),
        );

        grid.evolve();
        assert_eq!(
            grid.get_alive_cells(),
            &HashSet::from([(39, 0), (0, 0), (1, 0)])
        );

        grid.evolve();
        assert_eq!(
            grid.get_alive_cells(),
            &HashSet::from([(0, 39), (0, 0), (0, 1)])
        );
    }

    #[test]
    fn evolve_uses_custom_birth_and_survival_rules() {
        let mut grid = Grid::new(
            Pattern::new("Custom", HashSet::from([(0, 0), (0, 1)])),
            Rules::new(&[2], &[1]),
        );

        grid.evolve();

        assert_eq!(
            grid.get_alive_cells(),
            &HashSet::from([(0, 0), (0, 1), (39, 0), (39, 1), (1, 0), (1, 1),])
        );
    }
}
