use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

/// A struct representing a rucksack with two compartements.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Rucksack(String, String);

impl Rucksack {
    /// Find all elements which are in both compartements.
    pub fn find_duplicates(&self) -> Vec<char> {
        let mut duplicates = HashSet::new();

        let mut chars_set = HashSet::new();
        for c in self.0.chars() {
            chars_set.insert(c);
        }

        for c in self.1.chars() {
            if chars_set.get(&c).is_some() {
                duplicates.insert(c);
            }
        }
        duplicates.into_iter().collect()
    }
}

#[aoc_generator(day3)]
fn generator_day03(inp: &str) -> Vec<Rucksack> {
    let mut rucksacks = vec![];
    for line in inp.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        rucksacks.push(Rucksack(left.to_string(), right.to_string()));
    }
    rucksacks
}

#[aoc(day3, part1)]
fn day03_part1(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .iter()
        .map(|rucksack| {
            rucksack
                .find_duplicates()
                .iter()
                .map(|elem| {
                    if *elem >= 'a' && *elem <= 'z' {
                        *elem as u32 - 'a' as u32 + 1
                    } else {
                        *elem as u32 - 'A' as u32 + 27
                    }
                })
                .sum::<u32>()
        })
        .sum()
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
        );
    }

    #[test]
    fn test_find_duplicates() {
        assert_eq!(
            Rucksack("abcde".to_string(), "ABcDe".to_string()).find_duplicates(),
            vec!['c', 'e']
        );
        assert_eq!(
            Rucksack("abcdaaa".to_string(), "ABCD".to_string()).find_duplicates(),
            vec![]
        );
    }

    #[test]
    fn test_day03_part1() {
        let rucksacks = generator_day03(INPUT);
        assert_eq!(day03_part1(&rucksacks), 157);
    }
}
