use aoc_runner_derive::aoc;

/// Find the position where N distinct characters occur in the string.
fn find_first_n_distinct_characters<const N: usize>(inp: &str) -> usize {
    let mut current_window: [char; N] = [0 as char; N];

    'outter: for (i, c) in inp.chars().enumerate() {
        for j in 0..current_window.len() - 1 {
            current_window[j] = current_window[j + 1];
        }
        current_window[current_window.len() - 1] = c;

        for (j, a) in current_window.iter().enumerate() {
            for (k, b) in current_window.iter().enumerate() {
                if a == b && j != k || i < N {
                    continue 'outter;
                }
            }
        }
        return i + 1;
    }
    unreachable!()
}

#[aoc(day6, part1)]
fn day06_part1(inp: &str) -> usize {
    find_first_n_distinct_characters::<4>(inp)
}

#[aoc(day6, part2)]
fn day06_part2(inp: &str) -> usize {
    find_first_n_distinct_characters::<14>(inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day05_part1() {
        assert_eq!(day06_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(day06_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(day06_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(day06_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(day06_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_day05_part2() {
        assert_eq!(day06_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(day06_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(day06_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(day06_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(day06_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
