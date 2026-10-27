use core::fmt;
use std::collections::HashSet;

use strum::{EnumIter, IntoEnumIterator};

/// Available rules to evolve the alive cells. For the moment, just a couple
/// are available, instead of having more freedom to provide your own
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumIter)]
pub enum RuleName {
    #[default]
    GameOfLife,
    Highlife,
}

impl RuleName {
    pub fn count() -> usize {
        Self::iter().len()
    }

    pub fn by_index(n: usize) -> RuleName {
        Self::iter().nth(n.clamp(0, Self::count() - 1)).unwrap()
    }
}

impl fmt::Display for RuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleName::GameOfLife => write!(f, "Game Of Life"),
            RuleName::Highlife => write!(f, "Highlife"),
        }
    }
}

/// Implements the available set of rules (rule-string notation).
///
/// B3/S23: Original Game of Life
/// B36/S23: Highlife (https://en.wikipedia.org/wiki/Highlife_(cellular_automaton))
///
/// Note:
/// We could provide potentially more forms, but for the moment it
/// won't be be implemented.
#[derive(Debug, Clone)]
pub struct Rule {
    pub birth: HashSet<u8>,
    pub survival: HashSet<u8>,
}

impl Rule {
    /// Constructs a rule. Birth and survival at zero neighbors are not supported
    /// by the grid's evolution algorithm.
    pub fn new(birth: &[u8], survival: &[u8]) -> Self {
        Self {
            birth: HashSet::from_iter(birth.iter().cloned()),
            survival: HashSet::from_iter(survival.iter().cloned()),
        }
    }

    pub fn from(name: RuleName) -> Self {
        match name {
            RuleName::GameOfLife => Self::game_of_life(),
            RuleName::Highlife => Self::highlife(),
        }
    }

    /// B3/S23
    pub fn game_of_life() -> Self {
        Self::new(&[3], &[2, 3])
    }

    /// B36/S23
    pub fn highlife() -> Self {
        Self::new(&[3, 6], &[2, 3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_rules_and_removes_duplicates() {
        let rules = Rule::new(&[1, 3, 3], &[2, 4, 4]);

        assert_eq!(rules.birth, HashSet::from([1, 3]));
        assert_eq!(rules.survival, HashSet::from([2, 4]));
    }

    #[test]
    fn new_accepts_empty_rules() {
        let rules = Rule::new(&[], &[]);

        assert!(rules.birth.is_empty());
        assert!(rules.survival.is_empty());
    }

    #[test]
    fn game_of_life_uses_b3_s23() {
        let rules = Rule::game_of_life();

        assert_eq!(rules.birth, HashSet::from([3]));
        assert_eq!(rules.survival, HashSet::from([2, 3]));
    }

    #[test]
    fn highlife_uses_b36_s23() {
        let rules = Rule::highlife();

        assert_eq!(rules.birth, HashSet::from([3, 6]));
        assert_eq!(rules.survival, HashSet::from([2, 3]));
    }
}
