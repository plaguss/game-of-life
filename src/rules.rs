use std::collections::HashSet;

/// TODO: Allow having a more general set, and generate from parsing the string rules
///
///
/// Implements the available set of rules (rule-string notation).
///
/// B3/S23: Original Game of Life
/// B36/S23: Highlife (https://en.wikipedia.org/wiki/Highlife_(cellular_automaton))
#[derive(Debug, Clone)]
pub struct Rules {
    pub birth: HashSet<u8>,
    pub survival: HashSet<u8>,
}

impl Rules {
    pub fn new(birth: &[u8], survival: &[u8]) -> Self {
        Self {
            birth: HashSet::from_iter(birth.iter().cloned()),
            survival: HashSet::from_iter(survival.iter().cloned()),
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
        let rules = Rules::new(&[1, 3, 3], &[2, 4, 4]);

        assert_eq!(rules.birth, HashSet::from([1, 3]));
        assert_eq!(rules.survival, HashSet::from([2, 4]));
    }

    #[test]
    fn new_accepts_empty_rules() {
        let rules = Rules::new(&[], &[]);

        assert!(rules.birth.is_empty());
        assert!(rules.survival.is_empty());
    }

    #[test]
    fn game_of_life_uses_b3_s23() {
        let rules = Rules::game_of_life();

        assert_eq!(rules.birth, HashSet::from([3]));
        assert_eq!(rules.survival, HashSet::from([2, 3]));
    }

    #[test]
    fn highlife_uses_b36_s23() {
        let rules = Rules::highlife();

        assert_eq!(rules.birth, HashSet::from([3, 6]));
        assert_eq!(rules.survival, HashSet::from([2, 3]));
    }
}
