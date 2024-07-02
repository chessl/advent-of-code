mod parser;
mod part1;
mod part2;

pub use parser::*;
pub use part1::*;
pub use part2::*;

pub trait PasswordPolicy {
    fn new(positions: (usize, usize), letter: char) -> Self;
    fn is_valid(&self, password: &str) -> bool;
}
