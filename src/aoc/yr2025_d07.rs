#![allow(dead_code, unused_imports)]

use anyhow::anyhow;

struct Solver;

impl Solver {
    fn read() -> &'static str {
        include_str!("data/yr2025_d07.in")
    }

    fn part1() -> anyhow::Result<i32> {
        todo!()
    }

    fn part2() -> anyhow::Result<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    pub fn test_part1() {
        let result = Solver::part1().expect("Part 1 should complete successfully");
        assert_eq!(result, 0);
    }

    #[test]
    pub fn test_part2() {
        let result = Solver::part2().expect("Part 2 should complete successfully");
        assert_eq!(result, 0);
    }
}
