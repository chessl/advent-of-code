use std::{env, fs, time::Instant};

#[derive(Clone, Copy)]
struct Slope {
    right: usize,
    down: usize,
    count: u64,
    x: usize,
}

impl Slope {
    fn new(right: usize, down: usize) -> Self {
        Slope {
            right,
            down,
            count: 0,
            x: 0,
        }
    }

    fn found_tree(&mut self) {
        self.count += 1;
    }

    fn slide(&mut self) {
        self.x += self.right;
    }

    fn skip(&self, y: usize) -> bool {
        y % self.down != 0
    }
}

fn count_trees_1(map: &str, mut slope: Slope) -> u64 {
    let lines = map.lines().step_by(slope.down);

    for line in lines {
        let relative_x = slope.x % line.len();

        if line.chars().nth(relative_x) == Some('#') {
            slope.found_tree()
        }

        slope.slide();
    }

    slope.count
}

fn count_all_1(map: &str, slopes: &[Slope]) -> u64 {
    slopes.iter().map(|s| count_trees_1(map, *s)).product()
}

fn count_all_2(map: &str, slopes: &mut [Slope]) -> u64 {
    let lines = map.lines().enumerate();

    for (index, line) in lines {
        for slope in slopes.iter_mut() {
            if slope.skip(index) {
                continue;
            }
            let relative_x = slope.x % line.len();

            if line.chars().nth(relative_x) == Some('#') {
                slope.found_tree();
            }

            slope.slide();
        }
    }

    slopes.iter().map(|s| s.count).product()
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let pathname = &args[1];
    let content = fs::read_to_string(pathname).expect("cannot open file.");

    println!("\n============================part1===========================");
    let mut now = Instant::now();
    let mut count = count_trees_1(&content, Slope::new(3, 1));
    let mut elapsed = now.elapsed();
    println!("the number of trees is {}, elapsed: {:.2?}", count, elapsed);

    println!("\n============================part2===========================");
    now = Instant::now();
    count = count_all_1(
        &content,
        &[
            Slope::new(1, 1),
            Slope::new(3, 1),
            Slope::new(5, 1),
            Slope::new(7, 1),
            Slope::new(1, 2),
        ],
    );
    elapsed = now.elapsed();
    println!(
        "method1: the product is {}, elapsed: {:.2?}",
        count, elapsed
    );

    now = Instant::now();
    count = count_all_2(
        &content,
        &mut [
            Slope::new(1, 1),
            Slope::new(3, 1),
            Slope::new(5, 1),
            Slope::new(7, 1),
            Slope::new(1, 2),
        ],
    );
    elapsed = now.elapsed();
    println!(
        "method2: the product is {}, elapsed: {:.2?}",
        count, elapsed
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_1() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

        assert_eq!(count_trees_1(input, Slope::new(1, 1)), 2);
        assert_eq!(count_trees_1(input, Slope::new(3, 1)), 7);
        assert_eq!(count_trees_1(input, Slope::new(5, 1)), 3);
        assert_eq!(count_trees_1(input, Slope::new(7, 1)), 4);
        assert_eq!(count_trees_1(input, Slope::new(1, 2)), 2);
        assert_eq!(
            count_all_1(
                input,
                &[
                    Slope::new(1, 1),
                    Slope::new(3, 1),
                    Slope::new(5, 1),
                    Slope::new(7, 1),
                    Slope::new(1, 2),
                ]
            ),
            336
        );
    }
    #[test]
    fn test_count_2() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

        assert_eq!(
            count_all_2(
                input,
                &mut [
                    Slope::new(1, 1),
                    Slope::new(3, 1),
                    Slope::new(5, 1),
                    Slope::new(7, 1),
                    Slope::new(1, 2),
                ]
            ),
            336
        );
    }
}
