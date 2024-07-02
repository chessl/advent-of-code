use std::ops::RangeInclusive;

use super::PasswordPolicy;

#[derive(Debug, PartialEq)]
pub struct Part1PasswordPolicy {
    pub range: RangeInclusive<usize>,
    pub letter: char,
}

impl PasswordPolicy for Part1PasswordPolicy {
    fn new(positions: (usize, usize), letter: char) -> Self {
        let (low, high) = positions;
        Part1PasswordPolicy {
            range: low..=high,
            letter,
        }
    }

    fn is_valid(&self, password: &str) -> bool {
        self.range
            .contains(&password.chars().filter(|&c| c == self.letter).count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid() {
        assert!(Part1PasswordPolicy {
            range: 1..=3,
            letter: 'a'
        }
        .is_valid("abcde"));
        assert!(!Part1PasswordPolicy {
            range: 1..=3,
            letter: 'b'
        }
        .is_valid("cdefg"));
        assert!(Part1PasswordPolicy {
            range: 2..=9,
            letter: 'c'
        }
        .is_valid("ccccccccc"));
    }
}
