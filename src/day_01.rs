use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn day01_part1(inp: &str) -> i64 {
    let mut max = 0;
    let mut cur_max = 0;
    for ele in inp.lines() {
        if ele == "" {
            if cur_max > max {
                max = cur_max
            }
            cur_max = 0;
            continue;
        }
        cur_max += ele.parse::<i64>().unwrap();
    }
    max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            day01_part1(
                "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            ),
            24000
        )
    }
}
