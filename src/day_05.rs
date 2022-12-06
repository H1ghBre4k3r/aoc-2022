use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
struct ParseError;

/// Struct representing an item within a stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item(char);

impl FromStr for Item {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.trim().chars().nth(1).expect("AoC lied to me")))
    }
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
}
