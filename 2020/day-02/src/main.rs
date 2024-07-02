use std::{env, fs};

use crate::policy::{parse_line, Part1PasswordPolicy, Part2PasswordPolicy, PasswordPolicy};

mod policy;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    let content = fs::read_to_string(filepath).expect("Cannot open file.");

    let part1_result = content
        .lines()
        .map(parse_line::<Part1PasswordPolicy>)
        .map(Result::unwrap)
        .filter(|(policy, password)| policy.is_valid(password))
        .count();
    println!(
        "Part 1: The total number of valid passwords is {}",
        part1_result
    );

    let part2_result = content
        .lines()
        .map(parse_line::<Part2PasswordPolicy>)
        .map(Result::unwrap)
        .filter(|(policy, password)| policy.is_valid(password))
        .count();
    println!(
        "Part 2: The total number of valid passwords is {}",
        part2_result
    );
}
