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
    /// Parse the input into a valid hand.
    fn from_code(code: &str) -> Hand {
        match code {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => unreachable!(),
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

    /// Get the hand that would result in the specified outcome if played against this hand.
    fn with_coutcome_against_this(&self, outcome: Outcome) -> Hand {
        match (*self, outcome) {
            (Hand::Rock, Outcome::Draw)
            | (Hand::Scissor, Outcome::Win)
            | (Hand::Paper, Outcome::Loose) => Self::Rock,
            (Hand::Paper, Outcome::Draw)
            | (Hand::Rock, Outcome::Win)
            | (Hand::Scissor, Outcome::Loose) => Self::Paper,
            (Hand::Scissor, Outcome::Draw)
            | (Hand::Paper, Outcome::Win)
            | (Hand::Rock, Outcome::Loose) => Self::Scissor,
        }
    }

    fn val(self) -> i64 {
        self as i64
    }
}

impl Outcome {
    /// Parse the input into a valid outcome.
    fn from_code(code: &str) -> Self {
        match code {
            "X" => Self::Loose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }

    fn val(self) -> i64 {
        self as i64
    }
}

/// Convert the sample input into a valid vector of hands.
#[aoc_generator(day2, part1)]
fn generator_day02_part1(inp: &str) -> Vec<(Hand, Hand)> {
    let mut hands = vec![];

    for line in inp.lines() {
        let mut coded_hands = line.split_whitespace();
        // Note: This is highly unsafe and not recommended in production
        // But since I do not care enough for safety in AOC...LUL
        let enemy = coded_hands.next().unwrap();
        let we = coded_hands.next().unwrap();
        hands.push((Hand::from_code(enemy), Hand::from_code(we)));
    }

    hands
}

/// Conver the sample input into a vector containing the hand of the enemy and the desired outcome.
#[aoc_generator(day2, part2)]
fn generator_day02_part2(inp: &str) -> Vec<(Hand, Outcome)> {
    let mut rounds = vec![];

    for line in inp.lines() {
        let mut coded_rounds = line.split_whitespace();
        // Note: This is highly unsafe and not recommended in production
        // But since I do not care enough for safety in AOC...LUL
        let enemy = coded_rounds.next().unwrap();
        let outcome = coded_rounds.next().unwrap();
        rounds.push((Hand::from_code(enemy), Outcome::from_code(outcome)));
    }

    rounds
}

#[aoc(day2, part1)]
fn day02_part1(hands: &[(Hand, Hand)]) -> i64 {
    hands.iter().fold(0, |score, hands| {
        let (enemy, we) = hands;
        score + we.val() + we.beats(*enemy).val()
    })
}

#[aoc(day2, part2)]
fn day02_part2(rounds: &[(Hand, Outcome)]) -> i64 {
    rounds
        .iter()
        .map(|round| {
            let (enemy, outcome) = round;
            (enemy, enemy.with_coutcome_against_this(*outcome))
        })
        .fold(0, |score, hands| {
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
    fn test_hand_with_outcome_against_this() {
        assert_eq!(
            Hand::Rock.with_coutcome_against_this(Outcome::Draw),
            Hand::Rock
        );
        assert_eq!(
            Hand::Paper.with_coutcome_against_this(Outcome::Draw),
            Hand::Paper
        );
        assert_eq!(
            Hand::Scissor.with_coutcome_against_this(Outcome::Draw),
            Hand::Scissor
        );

        assert_eq!(
            Hand::Rock.with_coutcome_against_this(Outcome::Win),
            Hand::Paper
        );
        assert_eq!(
            Hand::Paper.with_coutcome_against_this(Outcome::Win),
            Hand::Scissor
        );
        assert_eq!(
            Hand::Scissor.with_coutcome_against_this(Outcome::Win),
            Hand::Rock
        );

        assert_eq!(
            Hand::Rock.with_coutcome_against_this(Outcome::Loose),
            Hand::Scissor
        );
        assert_eq!(
            Hand::Scissor.with_coutcome_against_this(Outcome::Loose),
            Hand::Paper
        );
        assert_eq!(
            Hand::Paper.with_coutcome_against_this(Outcome::Loose),
            Hand::Rock
        );
    }

    #[test]
    fn test_generator_part1() {
        let hands = generator_day02_part1(INPUT);
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
    fn test_generator_part2() {
        let rounds = generator_day02_part2(INPUT);
        assert_eq!(
            rounds,
            vec![
                (Hand::Rock, Outcome::Draw),
                (Hand::Paper, Outcome::Loose),
                (Hand::Scissor, Outcome::Win)
            ]
        );
    }

    #[test]
    fn test_day02_part1() {
        let hands = generator_day02_part1(INPUT);
        assert_eq!(day02_part1(&hands), 15);
    }

    #[test]
    fn test_day02_part2() {
        let rounds = generator_day02_part2(INPUT);
        assert_eq!(day02_part2(&rounds), 12);
    }
}
