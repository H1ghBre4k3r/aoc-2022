use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

/// A struct representing a rucksack with two compartements.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Rucksack(String, String);

#[derive(Debug, Clone, PartialEq, Eq)]
struct ElveGroup(String, String, String);

trait Priority {
    /// Get the priority of this item for the elves.
    fn to_prio(self) -> u32;
}

impl Priority for char {
    fn to_prio(self) -> u32 {
        if ('a'..='z').contains(&self) {
            self as u32 - 'a' as u32 + 1
        } else {
            self as u32 - 'A' as u32 + 27
        }
    }
}

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

impl ElveGroup {
    /// Find all elements which are in the rucksacks of all three elves.
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

        chars_set = duplicates;
        duplicates = HashSet::new();
        for c in self.2.chars() {
            if chars_set.get(&c).is_some() {
                duplicates.insert(c);
            }
        }

        duplicates.into_iter().collect()
    }
}

/// Generator for part 1 of day 3.
/// It packs the items of all elves into their respective compartements.
#[aoc_generator(day3, part1)]
fn generator_day03_part1(inp: &str) -> Vec<Rucksack> {
    let mut rucksacks = vec![];
    for line in inp.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        rucksacks.push(Rucksack(left.to_string(), right.to_string()));
    }
    rucksacks
}

/// Generator for part 2 of day 3.
/// It groups three elves as one "ElveGroup".
#[aoc_generator(day3, part2)]
fn generator_day03_part2(inp: &str) -> Vec<ElveGroup> {
    let mut elve_groups = vec![];

    let mut current_elves = vec![];

    for line in inp.lines() {
        current_elves.push(line);
        if current_elves.len() == 3 {
            elve_groups.push(ElveGroup(
                current_elves[0].to_string(),
                current_elves[1].to_string(),
                current_elves[2].to_string(),
            ));
            current_elves.clear();
        }
    }

    elve_groups
}

#[aoc(day3, part1)]
fn day03_part1(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .iter()
        .map(|rucksack| {
            rucksack
                .find_duplicates()
                .iter()
                .map(|elem| elem.to_prio())
                .sum::<u32>()
        })
        .sum()
}

#[aoc(day3, part2)]
fn day03_part2(elve_groups: &[ElveGroup]) -> u32 {
    elve_groups
        .iter()
        .map(|elve_group| {
            elve_group
                .find_duplicates()
                .iter()
                .map(|elem| elem.to_prio())
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
    fn test_generator_day03_part1() {
        let rucksacks = generator_day03_part1(INPUT);
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
    fn test_generator_day03_part2() {
        let elve_groups = generator_day03_part2(INPUT);
        assert_eq!(
            elve_groups,
            vec![
                ElveGroup(
                    "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
                    "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
                    "PmmdzqPrVvPwwTWBwg".to_string()
                ),
                ElveGroup(
                    "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
                    "ttgJtRGJQctTZtZT".to_string(),
                    "CrZsJsPPZsGzwwsLwLmpwMDw".to_string()
                )
            ]
        )
    }

    #[test]
    fn test_rucksack_find_duplicates() {
        assert_eq!(
            Rucksack("abcde".to_string(), "ABcDE".to_string()).find_duplicates(),
            vec!['c']
        );
        assert_eq!(
            Rucksack("abcdaaa".to_string(), "ABCD".to_string()).find_duplicates(),
            vec![]
        );
    }

    #[test]
    fn test_elve_group_find_duplicates() {
        let elve_groups = generator_day03_part2(INPUT);
        assert_eq!(
            elve_groups
                .iter()
                .map(|group| group
                    .find_duplicates()
                    .iter()
                    .map(|elem| elem.to_prio())
                    .sum::<u32>())
                .collect::<Vec<_>>(),
            vec![18, 52]
        );
    }

    #[test]
    fn test_day03_part1() {
        let rucksacks = generator_day03_part1(INPUT);
        assert_eq!(day03_part1(&rucksacks), 157);
    }

    #[test]
    fn test_day03_part2() {
        let elve_groups = generator_day03_part2(INPUT);
        assert_eq!(day03_part2(&elve_groups), 70);
    }
}
