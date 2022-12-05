use aoc_runner_derive::{aoc, aoc_generator};

/// Struct representing a section for elves to clean.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Section(usize, usize);

impl Section {
    fn from_string(inp: &str) -> Section {
        let Some((left, right)) = inp.split_once("-") else {
            panic!("Malformed input");
        };

        Section(
            left.parse::<usize>().expect("Input is no real number"),
            right.parse::<usize>().expect("Input is no real number"),
        )
    }

    /// Check, whether this section contains another specified section.
    fn contains_section(&self, other: &Section) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
}

/// Parse input into pairs of sections.
#[aoc_generator(day4)]
fn generator_day04(inp: &str) -> Vec<(Section, Section)> {
    inp.lines()
        .map(|line| {
            let Some((left, right)) = line.split_once(",") else {
            panic!("Malformed input (no ',' present)");
        };
            (Section::from_string(left), Section::from_string(right))
        })
        .collect()
}

#[aoc(day4, part1)]
fn day04_part1(sections: &[(Section, Section)]) -> usize {
    sections.iter().fold(0, |acc, (left, right)| {
        acc + if left.contains_section(&right) || right.contains_section(&left) {
            1
        } else {
            0
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_section_from_string() {
        assert_eq!(Section::from_string("1-2"), Section(1, 2));
    }

    #[test]
    #[should_panic]
    fn test_section_from_string_no_dash() {
        Section::from_string("123");
    }

    #[test]
    #[should_panic]
    fn test_section_from_string_no_number() {
        Section::from_string("123-abc");
    }

    #[test]
    fn test_generator_day04() {
        assert_eq!(
            generator_day04(INPUT),
            vec![
                (Section(2, 4), Section(6, 8)),
                (Section(2, 3), Section(4, 5)),
                (Section(5, 7), Section(7, 9)),
                (Section(2, 8), Section(3, 7)),
                (Section(6, 6), Section(4, 6)),
                (Section(2, 6), Section(4, 8))
            ]
        );
    }

    #[test]
    fn test_section_contains_section() {
        assert!(Section(2, 8).contains_section(&Section(3, 5)));
        assert!(!Section(3, 6).contains_section(&Section(3, 7)));
    }

    #[test]
    fn test_day04_part1() {
        let sections = generator_day04(INPUT);
        assert_eq!(day04_part1(&sections), 2);
    }
}
