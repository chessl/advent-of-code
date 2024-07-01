use std::{collections::HashMap, env, fs, time::Instant};

use itertools::Itertools;

fn method1(lines: &[u32]) -> Option<(u32, u32)> {
    let mut map: HashMap<u32, bool> = HashMap::new();
    for num in lines.iter() {
        match map.get(&(2020 - num)) {
            Some(true) => {
                return Some((*num, 2020 - num));
            }
            _ => {
                map.insert(*num, true);
                continue;
            }
        }
    }

    None
}

fn method2(lines: &[u32], size: usize) -> Option<Vec<&u32>> {
    return lines
        .iter()
        .combinations(size)
        .find(|c| c.iter().copied().sum::<u32>() == 2020);
}

fn method3_part1(lines: &[u32]) -> Option<(u32, u32)> {
    return lines
        .iter()
        .copied()
        .tuple_combinations()
        .find(|(n1, n2)| n1 + n2 == 2020);
}

fn method3_part2(lines: &[u32]) -> Option<(u32, u32, u32)> {
    return lines
        .iter()
        .copied()
        .tuple_combinations()
        .find(|(n1, n2, n3)| n1 + n2 + n3 == 2020);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];

    let mut now = Instant::now();

    let contents = fs::read_to_string(filepath).expect("Cannot open file.");
    let lines = contents
        .lines()
        .map(str::parse::<u32>)
        .filter(|r| match r {
            Ok(num) => *num <= 2020,
            _ => false,
        })
        .map(Result::unwrap)
        .collect::<Vec<u32>>();

    let mut elapsed = now.elapsed();
    println!("load fild elapsed: {:.2?}", elapsed);

    println!("\n============================method1===========================");
    now = Instant::now();
    let (num1, num2) = method1(&lines).unwrap();
    elapsed = now.elapsed();
    println!(
        "Find the result {} x {} = {} with method1 elapsed: {:.2?}",
        num1,
        num2,
        num1 * num2,
        elapsed
    );

    println!("\n============================method2===========================");
    now = Instant::now();
    let mut result2 = method2(&lines, 2).unwrap();
    elapsed = now.elapsed();
    println!(
        "Find the result {} x {} = {} with method1 elapsed: {:.2?}",
        *result2.first().unwrap(),
        *result2.get(1).unwrap(),
        result2.iter().copied().product::<u32>(),
        elapsed,
    );

    now = Instant::now();
    result2 = method2(&lines, 3).unwrap();
    elapsed = now.elapsed();
    println!(
        "Find the result {} x {} x {} = {} with method2: {:.2?}",
        *result2.first().unwrap(),
        *result2.get(1).unwrap(),
        *result2.get(2).unwrap(),
        result2.iter().copied().product::<u32>(),
        elapsed
    );

    println!("\n============================method3===========================");
    now = Instant::now();
    let (num1, num2) = method3_part1(&lines).unwrap();
    elapsed = now.elapsed();
    println!(
        "Find the result {} x {} = {} with method3 elapsed: {:.2?}",
        num1,
        num2,
        num1 * num2,
        elapsed
    );

    now = Instant::now();
    let (num1, num2, num3) = method3_part2(&lines).unwrap();
    elapsed = now.elapsed();
    println!(
        "Find the result {} x {} x {} = {} with method3 elapsed: {:.2?}",
        num1,
        num2,
        num3,
        num1 * num2 * num3,
        elapsed
    );
}
