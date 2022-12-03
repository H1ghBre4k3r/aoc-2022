use aoc_runner_derive::{aoc, aoc_generator};

/// Enum representing a hand in rock-paper-scissors
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Loose = 0,
    Draw = 3,
    Win = 6,
}

impl Hand {
    /// Parse the input into a valid hand, or None if input is not a valid hand.
    fn from_code(code: &str) -> Option<Hand> {
        match code {
            "A" | "X" => Some(Self::Rock),
            "B" | "Y" => Some(Self::Paper),
            "C" | "Z" => Some(Self::Scissor),
            _ => None,
        }
    }

    /// Calculate the outcome of a game of rock-paper-scissors.
    fn beats(&self, other: Hand) -> Outcome {
        match (*self, other) {
            (Hand::Rock, Hand::Paper)
            | (Hand::Paper, Hand::Scissor)
            | (Hand::Scissor, Hand::Rock) => Outcome::Loose,
            (Hand::Paper, Hand::Rock)
            | (Hand::Scissor, Hand::Paper)
            | (Hand::Rock, Hand::Scissor) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }

    fn val(self) -> i64 {
        self as i64
    }
}

impl Outcome {
    fn val(self) -> i64 {
        self as i64
    }
}

/// Convert the sample input into a valid vector of hands.
#[aoc_generator(day2)]
fn generator_day02(inp: &str) -> Vec<(Hand, Hand)> {
    let mut hands = vec![];

    for line in inp.lines() {
        let mut coded_hands = line.split_whitespace();
        // Note: This is highly unsafe and not recommended in production
        // But since I do not care enough for safety in AOC...LUL
        let enemy = coded_hands.next().unwrap();
        let we = coded_hands.next().unwrap();
        hands.push((
            Hand::from_code(enemy).unwrap(),
            Hand::from_code(we).unwrap(),
        ));
    }

    hands
}

#[aoc(day2, part1)]
fn day02_part1(hands: &[(Hand, Hand)]) -> i64 {
    hands.iter().fold(0, |score, hands| {
        let (enemy, we) = hands;
        score + we.val() + we.beats(*enemy).val()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_hand_beats() {
        assert_eq!(Hand::Rock.beats(Hand::Scissor), Outcome::Win);
        assert_eq!(Hand::Paper.beats(Hand::Rock), Outcome::Win);
        assert_eq!(Hand::Scissor.beats(Hand::Paper), Outcome::Win);

        assert_eq!(Hand::Scissor.beats(Hand::Rock), Outcome::Loose);
        assert_eq!(Hand::Rock.beats(Hand::Paper), Outcome::Loose);
        assert_eq!(Hand::Paper.beats(Hand::Scissor), Outcome::Loose);

        assert_eq!(Hand::Rock.beats(Hand::Rock), Outcome::Draw);
        assert_eq!(Hand::Paper.beats(Hand::Paper), Outcome::Draw);
        assert_eq!(Hand::Scissor.beats(Hand::Scissor), Outcome::Draw);
    }

    #[test]
    fn test_generator() {
        let hands = generator_day02(INPUT);
        assert_eq!(
            hands,
            vec![
                (Hand::Rock, Hand::Paper),
                (Hand::Paper, Hand::Rock),
                (Hand::Scissor, Hand::Scissor)
            ]
        );
    }

    #[test]
    fn test_day01_part1() {
        let hands = generator_day02(INPUT);
        assert_eq!(day02_part1(&hands), 15);
    }
}
