use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::patterns::{AliveCells, Cell};

/// A single history point.
/// The first element is the population size,
/// the second the hashed board state.
#[derive(Debug, Clone)]
pub struct Point {
    pub population_size: usize,
    pub fingerprint: u64,
}

/// TODO: It will need
/// - rolling history of board fingerprints (last N generations)
/// - Check board states (access fingerprints)
/// Tracks the evolution of the board, to detect stagnation and such.
#[derive(Debug, Clone)]
pub struct History {
    /// Number of steps to store at max
    window: usize,
    /// Stores the population and the fingerprints
    /// of the board state.
    points: VecDeque<Point>,
}

impl History {
    pub fn new(window: usize) -> Self {
        Self {
            window,
            points: VecDeque::with_capacity(window),
        }
    }

    /// Adds a single point to the history (to be called on each step of the board)
    /// to the back of the deque, and when the capacity is reached, starts popping
    /// elements from the front.
    pub fn add_point(&mut self, board: &AliveCells) {
        let point = Point {
            population_size: board.len(),
            fingerprint: Self::get_fingerprint(board),
        };
        self.points.push_back(point);

        if self.points.len() > self.window {
            self.points.pop_front();
        }
    }

    /// Obtains the fingerprint of the board as a hash of the sorted cells.
    fn get_fingerprint(cells: &AliveCells) -> u64 {
        let mut hasher = DefaultHasher::new();
        let mut sorted_cells = cells.iter().copied().collect::<Vec<Cell>>();
        sorted_cells.sort_unstable();
        sorted_cells.hash(&mut hasher);
        hasher.finish()
    }

    /// Check whether the history is stagnated by looking whether the board
    /// hasn't seen any change after `generations` steps, or the population
    /// hasn't changed during the registered history.
    pub fn is_stagnant(&self, generations: usize) -> bool {
        if self.points.len() < generations {
            return false;
        }
        // TODO: ADD ANOTHER CHECK FOR THE DENSITY, IF IT'S TOO LOW AFTER N GENERATIONS,
        // RETURN TRUE
        self.board_unchanged_for(generations) || self.population_unchanged()
    }

    /// Check whether the fingerprint hasn't changed after `generations` steps
    /// have passed.
    fn board_unchanged_for(&self, generations: usize) -> bool {
        match self.points.back() {
            Some(first) => self.points.iter().rev().skip(1).take(generations - 1).any(
                |&Point {
                     population_size: _,
                     fingerprint,
                 }| fingerprint == first.fingerprint,
            ),
            None => false,
        }
    }

    /// Check if the population size has changed (uses all the history for it).
    /// It's a looser signal, so even if there's movement (due to repetitive
    /// patterns such as the blinker), we can still decide to add more "life"
    /// to the board.
    fn population_unchanged(&self) -> bool {
        match self.points.front() {
            Some(first) => self.points.iter().all(
                |&Point {
                     population_size,
                     fingerprint: _,
                 }| population_size == first.population_size,
            ),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BLOCK: &[Cell] = &[(0, 0), (0, 1), (1, 0), (1, 1)];
    const BLINKER_H: &[Cell] = &[(0, 1), (1, 1), (2, 1)];
    const BLINKER_V: &[Cell] = &[(1, 0), (1, 1), (1, 2)];

    fn board(cells: &[Cell]) -> AliveCells {
        cells.iter().copied().collect()
    }

    #[test]
    fn new_history_is_empty_and_not_stagnant() {
        let history = History::new(5);
        assert!(history.points.is_empty());
        assert!(!history.is_stagnant(0));
        assert!(!history.is_stagnant(1));
    }

    #[test]
    fn add_point_keeps_at_most_window_points() {
        let mut history = History::new(3);
        for i in 0..5 {
            history.add_point(&board(&[(i, 0)]));
        }

        assert_eq!(history.points.len(), 3);
        assert_eq!(
            history.points.front().unwrap().fingerprint,
            History::get_fingerprint(&board(&[(2, 0)]))
        );
        assert_eq!(
            history.points.back().unwrap().fingerprint,
            History::get_fingerprint(&board(&[(4, 0)]))
        );
    }

    #[test]
    fn window_of_one_keeps_latest_point() {
        let mut history = History::new(1);
        history.add_point(&board(BLOCK));
        history.add_point(&board(&[(0, 0)]));

        assert_eq!(history.points.len(), 1);
        assert_eq!(history.points[0].population_size, 1);
    }

    #[test]
    fn zero_window_stores_nothing() {
        let mut history = History::new(0);
        history.add_point(&board(BLOCK));

        assert!(history.points.is_empty());
        assert!(!history.is_stagnant(1));
    }

    #[test]
    fn still_board_is_stagnant_once_enough_points_exist() {
        let mut history = History::new(5);
        for _ in 0..3 {
            history.add_point(&board(BLOCK));
        }

        assert!(!history.is_stagnant(4));
        assert!(history.is_stagnant(3));
    }

    #[test]
    fn generations_longer_than_window_are_never_stagnant() {
        let mut history = History::new(3);
        for _ in 0..10 {
            history.add_point(&board(BLOCK));
        }

        assert!(!history.is_stagnant(4));
    }

    #[test]
    fn board_unchanged_only_in_recent_points_is_stagnant() {
        let mut history = History::new(5);
        history.add_point(&board(&[(0, 0)]));
        history.add_point(&board(&[(0, 0), (0, 1)]));
        history.add_point(&board(BLOCK));
        history.add_point(&board(BLOCK));
        history.add_point(&board(BLOCK));

        assert!(history.is_stagnant(3));
    }

    #[test]
    fn recent_board_change_breaks_board_stagnation() {
        let mut history = History::new(5);
        history.add_point(&board(BLOCK));
        history.add_point(&board(BLOCK));
        history.add_point(&board(&[(5, 5), (5, 6), (6, 5), (6, 6), (7, 7)]));

        assert!(!history.is_stagnant(3));
    }

    #[test]
    fn oscillator_is_flagged_by_unchanged_population() {
        let mut history = History::new(10);
        for i in 0..6 {
            let cells = if i % 2 == 0 { BLINKER_H } else { BLINKER_V };
            history.add_point(&board(cells));
        }

        assert!(!history.board_unchanged_for(2));
        assert!(history.is_stagnant(6));
    }

    #[test]
    fn growing_population_with_new_boards_is_not_stagnant() {
        let mut history = History::new(5);
        for n in 1..=5 {
            let cells: Vec<Cell> = (0..n).map(|i| (i, 0)).collect();
            history.add_point(&board(&cells));
        }

        assert!(!history.is_stagnant(5));
    }

    #[test]
    fn empty_board_is_stagnant_when_it_stays_empty() {
        let mut history = History::new(5);
        for _ in 0..3 {
            history.add_point(&board(&[]));
        }

        assert!(history.is_stagnant(3));
    }

    #[test]
    fn fingerprint_ignores_insertion_order() {
        let forward = board(&[(0, 0), (0, 1), (1, 0), (1, 1), (5, 3)]);
        let backward = board(&[(5, 3), (1, 1), (1, 0), (0, 1), (0, 0)]);

        assert_eq!(
            History::get_fingerprint(&forward),
            History::get_fingerprint(&backward)
        );
    }

    #[test]
    fn fingerprint_differs_for_different_boards() {
        assert_ne!(
            History::get_fingerprint(&board(BLINKER_H)),
            History::get_fingerprint(&board(BLINKER_V))
        );
    }
}
