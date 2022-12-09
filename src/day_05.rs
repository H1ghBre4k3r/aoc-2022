use std::{error::Error, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug)]
struct ParseError;

/// Struct representing an item within a stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item(char);

impl FromStr for Item {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().chars().nth(1) {
            Some(val) => Ok(Self(val)),
            None => Err(ParseError),
        }
    }
}

/// Struct for representing a stack of crates.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct Stack {
    items: Vec<Item>,
}

impl Stack {
    /// Push an element onto this stack.
    pub fn push(&mut self, item: Item) {
        self.items.push(item);
    }

    /// Pop an element from this stack.
    pub fn pop(&mut self) -> Option<Item> {
        self.items.pop()
    }

    /// Pop n elements of this stack.
    pub fn pop_n(&mut self, n: usize) -> Vec<Option<Item>> {
        let mut items = vec![];

        for _ in 0..n {
            items.insert(0, self.items.pop());
        }

        items
    }

    /// Push all provided elements into this stack.
    pub fn push_all(&mut self, items: Vec<Item>) {
        self.items.append(&mut items.clone());
    }
}

/// Struct for representing an instruction to move a specified amount of crates from one stack to
/// the other.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct Instruction(usize, usize, usize);

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut whitespaces = s.split_whitespace();

        // the instructions usually have the form of:
        // 'move X from Y to Z'
        //
        // so we need the 1., 3. and 5. index, therefore always taking the 1. index, since
        // `nth(1)` advances the iterator by 2 elements.
        let amount = whitespaces.nth(1);
        let source = whitespaces.nth(1);
        let destination = whitespaces.nth(1);

        let (Some(amount), Some(source), Some(destination)) = (amount, source, destination) else {
            panic!("AoC betrayed us! ({:?} {:?} {:?})", amount, source, destination);
        };

        let amount = amount.parse::<usize>()?;
        let source = source.parse::<usize>()?;
        let destination = destination.parse::<usize>()?;
        Ok(Self(amount, source, destination))
    }
}

/// Split a row of the stack inputs into chunks.
fn split_row_input_into_chunks(inp: &str) -> Vec<Option<Item>> {
    inp.chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk| {
            chunk
                .iter()
                .collect::<String>()
                .trim()
                .parse::<Item>()
                .map_or(None, |val| Some(val))
        })
        .collect::<Vec<_>>()
}

/// Parse input into stacks and instructions.
#[aoc_generator(day5)]
fn generator_day5(inp: &str) -> (Vec<Stack>, Vec<Instruction>) {
    let Some((crates, instructions)) = inp.split_once("\n\n") else {
        panic!("AoC betrayed us!")
    };

    // determine the amount of stacks we need to fill
    let mut stack_lines = crates.lines().rev();
    let num_stacks =
        stack_lines
            .next()
            .unwrap()
            .chars()
            .fold(0, |memo, c| if c.is_numeric() { memo + 1 } else { memo });
    let mut stacks = vec![Stack::default(); num_stacks];

    // ...and then fill them :D
    for line in stack_lines {
        for (i, ele) in split_row_input_into_chunks(line).iter().enumerate() {
            if let Some(item) = *ele {
                stacks[i].push(item);
            }
        }
    }

    (
        stacks,
        // some fancy string parsing etc.
        instructions
            .lines()
            .map(|line| {
                line.parse::<Instruction>()
                    .expect("Parsing not successful :(")
            })
            .collect::<Vec<_>>(),
    )
}

#[aoc(day5, part1)]
fn day05_part1(input: &(Vec<Stack>, Vec<Instruction>)) -> String {
    let (mut stacks, instructions) = input.clone();

    // move crates around
    for Instruction(amount, source, target) in instructions {
        for _ in 0..amount {
            let item = stacks[source - 1].pop().expect("we are empty....");
            stacks[target - 1].push(item);
        }
    }

    // combine top elements
    stacks
        .iter_mut()
        .map(|stack| stack.pop())
        .fold("".to_owned(), |mut memo, current| {
            if let Some(c) = current {
                memo.push(c.0);
            }
            memo
        })
}

#[aoc(day5, part2)]
fn day05_part2(input: &(Vec<Stack>, Vec<Instruction>)) -> String {
    let (mut stacks, instructions) = input.clone();

    // move crates around
    for Instruction(amount, source, target) in instructions {
        let items = stacks[source - 1].pop_n(amount);
        stacks[target - 1].push_all(
            items
                .into_iter()
                .filter(|item| item.is_some())
                .map(|item| item.unwrap())
                .collect(),
        );
    }

    // combine top elements
    stacks
        .iter_mut()
        .map(|stack| stack.pop())
        .fold("".to_owned(), |mut memo, current| {
            if let Some(c) = current {
                memo.push(c.0);
            }
            memo
        })
}

#[cfg(test)]
mod tests {

    use crate::day_05::*;

    const INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_parse_item() {
        assert_eq!("[D]".parse::<Item>().unwrap(), Item('D'));
    }

    #[test]
    fn test_split_input_into_chunks() {
        assert_eq!(split_row_input_into_chunks("[A]"), vec![Some(Item('A'))]);
        assert_eq!(
            split_row_input_into_chunks("[A]    "),
            vec![Some(Item('A')), None]
        );
        assert_eq!(
            split_row_input_into_chunks("[A]     [B]"),
            vec![Some(Item('A')), None, Some(Item('B'))]
        );
        assert_eq!(
            split_row_input_into_chunks("    [A]"),
            vec![None, Some(Item('A'))]
        );
        assert_eq!(
            split_row_input_into_chunks("    [A]    "),
            vec![None, Some(Item('A')), None]
        )
    }

    #[test]
    fn test_pop_n() {
        let mut stack = Stack {
            items: vec![Item('A'), Item('B')],
        };
        let items = stack.pop_n(2);
        assert_eq!(items, vec![Some(Item('A')), Some(Item('B'))]);
        assert!(stack.items.len() == 0);
    }

    #[test]
    fn test_push_n() {
        let mut stack = Stack::default();
        stack.push_all(vec![Item('A'), Item('B')]);
        assert_eq!(stack.items, vec![Item('A'), Item('B')]);
    }

    #[test]
    fn test_generator_day5() {
        let stacks = generator_day5(INPUT);
        assert_eq!(
            stacks,
            (
                vec![
                    Stack {
                        items: vec![Item('Z'), Item('N')]
                    },
                    Stack {
                        items: vec![Item('M'), Item('C'), Item('D')]
                    },
                    Stack {
                        items: vec![Item('P')]
                    }
                ],
                vec![
                    Instruction(1, 2, 1),
                    Instruction(3, 1, 3),
                    Instruction(2, 2, 1),
                    Instruction(1, 1, 2)
                ]
            )
        )
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            "move 1 from 2 to 3".parse::<Instruction>().unwrap(),
            Instruction(1, 2, 3)
        );
    }

    #[test]
    fn test_day05_part1() {
        let generated = generator_day5(INPUT);
        assert_eq!(day05_part1(&generated), "CMZ".to_string());
    }

    #[test]
    fn test_day06_part2() {
        let generated = generator_day5(INPUT);
        assert_eq!(day05_part2(&generated), "MCD".to_string());
    }
}
