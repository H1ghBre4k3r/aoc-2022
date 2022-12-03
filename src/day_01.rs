use aoc_runner_derive::{aoc, aoc_generator};

type Elve = i64;

#[aoc_generator(day01)]
fn generator_day01(inp: &str) -> Vec<Elve> {
    let mut elves = vec![];
    let mut current_elve = 0;
    for val in inp.lines() {
        if val.trim() == "" {
            elves.push(current_elve);
            current_elve = 0;
            continue;
        }
        current_elve += val.parse::<i64>().unwrap();
    }
    elves.push(current_elve);
    elves
}

#[aoc(day1, part1)]
pub fn day01_part1(elves: &[Elve]) -> i64 {
    let mut elves = elves.to_vec();
    elves.sort();
    *elves.last().unwrap()
}

#[aoc(day1, part2)]
pub fn day01_part2(elves: &[Elve]) -> i64 {
    let mut elves = elves.to_vec();
    elves.sort();
    let last_three_elves = elves.split_off(elves.len() - 3);
    last_three_elves.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let elves = generator_day01(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        );
        assert_eq!(day01_part1(&elves), 24000)
    }

    #[test]
    fn test_part_2() {
        let elves = generator_day01(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        );
        assert_eq!(day01_part2(&elves), 45000)
    }
}
