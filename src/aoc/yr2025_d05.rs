#![allow(dead_code, unused_imports)]

use anyhow::anyhow;
use itertools::Itertools;

struct Solver;

impl Solver {
    fn read() -> &'static str {
        include_str!("data/yr2025_d05.in")
    }

    fn readlines() -> Vec<String> {
        Self::read().lines().map(|line| line.to_string()).collect()
    }

    fn parse_ranges_from_ids() -> anyhow::Result<(Vec<Vec<i64>>, Vec<i64>)> {
        let lines = Self::readlines();
        let empty_line = lines.iter().find_position(|line| line.is_empty());
        if let Some((position, _value)) = empty_line {
            let ranges = &lines[0..position];
            let ids = lines[position + 1..]
                .iter()
                .map(|id| id.parse::<i64>().unwrap_or(0))
                .collect::<Vec<i64>>();

            let ranges: Vec<Vec<i64>> = ranges
                .iter()
                .map(|range| {
                    range
                        .split("-")
                        .map(|part| part.parse::<i64>().unwrap_or(0))
                        .collect()
                })
                .collect();

            Ok((ranges, ids))
        } else {
            Err(anyhow!("Could not find the proper position"))
        }
    }

    fn merge_intervals(intervals: &mut [Vec<i64>]) -> Vec<Vec<i64>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut result: Vec<Vec<i64>> = Vec::new();

        for interval in intervals.iter() {
            if let Some(last) = result.last_mut()
                && last[1] >= interval[0]
            {
                *last = vec![last[0].min(interval[0]), last[1].max(interval[1])];
            } else {
                result.push(interval.to_vec());
            }
        }

        result
    }

    fn part1() -> anyhow::Result<i32> {
        let mut result = 0;
        let (mut ranges, ids) = Self::parse_ranges_from_ids()?;
        let intervals = Self::merge_intervals(&mut ranges);

        for &id in ids.iter() {
            for interval in intervals.iter() {
                if interval[0] <= id && id <= interval[1] {
                    result += 1;
                    break;
                }
            }
        }

        Ok(result)
    }

    fn part2() -> anyhow::Result<i64> {
        let mut result = 0;
        let (mut ranges, _ids) = Self::parse_ranges_from_ids()?;
        let intervals = Self::merge_intervals(&mut ranges);

        for interval in intervals.iter() {
            result += interval[1] - interval[0] + 1;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    pub fn test_part1() {
        let result = Solver::part1().expect("Part 1 should complete successfully");
        assert_eq!(result, 513);
    }

    #[test]
    pub fn test_part2() {
        let result = Solver::part2().expect("Part 2 should complete successfully");
        assert_eq!(result, 339668510830757);
    }
}
