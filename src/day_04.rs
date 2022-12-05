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
}
