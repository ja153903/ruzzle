#![allow(dead_code, unused_imports)]

use anyhow::anyhow;

struct Solver;

impl Solver {
    fn read() -> &'static str {
        include_str!("data/yr2025_d02.in")
    }

    fn parse_ranges() -> Vec<Vec<String>> {
        Self::read()
            .trim()
            .split(",")
            .map(|it| it.split("-").map(|it| it.to_string()).collect())
            .collect()
    }

    fn process_ranges<F>(mut processor: F) -> anyhow::Result<i64>
    where
        F: FnMut(i64) -> bool,
    {
        let mut result = 0;
        for range in Self::parse_ranges().iter() {
            let start = range[0].parse::<i64>()?;
            let end = range[1].parse::<i64>()?;

            for i in start..=end {
                if processor(i) {
                    result += i;
                }
            }
        }
        Ok(result)
    }

    fn is_mirrored(n: i64) -> bool {
        let s = n.to_string();
        let mid = s.len() / 2;
        s.len().is_multiple_of(2) && s[0..mid] == s[mid..]
    }

    fn is_repeating_pattern(n: i64) -> bool {
        let s = n.to_string();
        let mid = s.len() / 2;

        for j in 0..mid {
            let substr = &s[0..=j];
            if substr.is_empty() {
                continue;
            }

            if s.len().is_multiple_of(substr.len()) {
                let expected = substr.repeat(s.len() / substr.len());
                if expected == s {
                    return true;
                }
            }
        }
        false
    }

    fn part1() -> anyhow::Result<i64> {
        Self::process_ranges(Self::is_mirrored)
    }

    fn part2() -> anyhow::Result<i64> {
        Self::process_ranges(Self::is_repeating_pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    pub fn test_part1() {
        let result = Solver::part1().expect("Part 1 should complete successfully");
        assert_eq!(result, 19386344315);
    }

    #[test]
    pub fn test_part2() {
        let result = Solver::part2().expect("Part 2 should complete successfully");
        assert_eq!(result, 34421651192);
    }
}
