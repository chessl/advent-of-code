use super::PasswordPolicy;
use peg::error::ParseError;

pub fn parse_line<T: PasswordPolicy>(
    line: &str,
) -> Result<(T, &str), ParseError<peg::str::LineCol>> {
    peg::parser! {
        grammar policy_parser() for str {
          rule number() -> usize = n:$(['0'..='9']+) { n.parse().unwrap() }

          rule positions() -> (usize, usize) = first:number() "-" second:number() { (first, second) }

          rule letter() -> char = n:(['a'..='z']) { n }

          rule policy<T: PasswordPolicy>() -> T = p:positions() " " l:letter() { T::new(p, l) }

          rule password() -> &'input str = n:$([_]*) { n }

          pub rule line<T: PasswordPolicy>() -> (T, &'input str) = left:policy() ": " right:password() { (left, right)}
        }

    }
    policy_parser::line(line)
}

#[cfg(test)]
mod tests {
    use crate::policy::{Part1PasswordPolicy, Part2PasswordPolicy};

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            parse_line::<Part1PasswordPolicy>("1-3 a: abcde").unwrap(),
            (
                Part1PasswordPolicy {
                    range: 1..=3,
                    letter: 'a'
                },
                "abcde"
            )
        );

        assert_eq!(
            parse_line::<Part1PasswordPolicy>("1-3 b: cdefg").unwrap(),
            (
                Part1PasswordPolicy {
                    range: 1..=3,
                    letter: 'b'
                },
                "cdefg"
            )
        );

        assert_eq!(
            parse_line::<Part1PasswordPolicy>("2-9 c: ccccccccc").unwrap(),
            (
                Part1PasswordPolicy {
                    range: 2..=9,
                    letter: 'c'
                },
                "ccccccccc"
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            parse_line::<Part2PasswordPolicy>("1-3 a: abcde").unwrap(),
            (
                Part2PasswordPolicy {
                    first: 1,
                    second: 3,
                    letter: 'a'
                },
                "abcde"
            )
        );

        assert_eq!(
            parse_line::<Part2PasswordPolicy>("1-3 b: cdefg").unwrap(),
            (
                Part2PasswordPolicy {
                    first: 1,
                    second: 3,
                    letter: 'b'
                },
                "cdefg"
            )
        );

        assert_eq!(
            parse_line::<Part2PasswordPolicy>("2-9 c: ccccccccc").unwrap(),
            (
                Part2PasswordPolicy {
                    first: 2,
                    second: 9,
                    letter: 'c'
                },
                "ccccccccc"
            )
        );
    }
}
