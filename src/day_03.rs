use aoc_runner_derive::aoc_generator;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rucksack(String, String);

#[aoc_generator(day3)]
fn generator_day03(inp: &str) -> Vec<Rucksack> {
    let mut rucksacks = vec![];
    for line in inp.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        rucksacks.push(Rucksack(left.to_string(), right.to_string()));
    }
    rucksacks
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_generator_day03() {
        let rucksacks = generator_day03(INPUT);
        assert_eq!(
            rucksacks,
            vec![
                Rucksack("vJrwpWtwJgWr".to_string(), "hcsFMMfFFhFp".to_string()),
                Rucksack(
                    "jqHRNqRjqzjGDLGL".to_string(),
                    "rsFMfFZSrLrFZsSL".to_string()
                ),
                Rucksack("PmmdzqPrV".to_string(), "vPwwTWBwg".to_string()),
                Rucksack("wMqvLMZHhHMvwLH".to_string(), "jbvcjnnSBnvTQFn".to_string()),
                Rucksack("ttgJtRGJ".to_string(), "QctTZtZT".to_string()),
                Rucksack("CrZsJsPPZsGz".to_string(), "wwsLwLmpwMDw".to_string())
            ]
        )
    }
}
