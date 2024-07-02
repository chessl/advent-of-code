use super::PasswordPolicy;

#[derive(Debug, PartialEq)]
pub struct Part2PasswordPolicy {
    pub first: usize,
    pub second: usize,
    pub letter: char,
}

impl PasswordPolicy for Part2PasswordPolicy {
    fn new(positions: (usize, usize), letter: char) -> Self {
        let (first, second) = positions;
        Part2PasswordPolicy {
            first,
            second,
            letter,
        }
    }

    fn is_valid(&self, password: &str) -> bool {
        let chars = password.chars().collect::<Vec<char>>();

        let l1 = chars.get(self.first - 1);
        let l2 = chars.get(self.second - 1);

        (l1 == Some(&self.letter) || l2 == Some(&self.letter)) && l1 != l2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid() {
        assert!(Part2PasswordPolicy {
            first: 1,
            second: 3,
            letter: 'a'
        }
        .is_valid("abcde"));
        assert!(!Part2PasswordPolicy {
            first: 1,
            second: 3,
            letter: 'b'
        }
        .is_valid("cdefg"));
        assert!(!Part2PasswordPolicy {
            first: 2,
            second: 9,
            letter: 'c'
        }
        .is_valid("ccccccccc"));
    }
}
