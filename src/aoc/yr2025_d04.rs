#![allow(dead_code, unused_imports)]

use anyhow::anyhow;

use crate::aoc::grid;

struct Solver;

impl Solver {
    fn read() -> &'static str {
        include_str!("data/yr2025_d04.in")
    }

    fn readlines() -> Vec<Vec<char>> {
        Self::read()
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect()
    }

    fn part1() -> anyhow::Result<i32> {
        let grid = Self::readlines();
        if grid.is_empty() || grid[0].is_empty() {
            return Err(anyhow!("Grid is empty"));
        }

        let rows = grid.len();
        let cols = grid[0].len();
        let mut count = 0;

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == '@' {
                    let neighbor_count = grid::count_neighbors(&grid, row, col, '@', true);
                    if neighbor_count < 4 {
                        count += 1;
                    }
                }
            }
        }

        Ok(count)
    }

    fn part2() -> anyhow::Result<i32> {
        let mut grid = Self::readlines();
        if grid.is_empty() || grid[0].is_empty() {
            return Err(anyhow!("Grid is empty"));
        }

        let rows = grid.len();
        let cols = grid[0].len();
        let mut total_removed = 0;

        loop {
            let mut changed = false;

            for row in 0..rows {
                for col in 0..cols {
                    if grid[row][col] == '@' {
                        let neighbor_count = grid::count_neighbors(&grid, row, col, '@', true);
                        if neighbor_count < 4 {
                            grid[row][col] = '.';
                            total_removed += 1;
                            changed = true;
                        }
                    }
                }
            }

            if !changed {
                break;
            }
        }

        Ok(total_removed)
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    pub fn test_part1() {
        let result = Solver::part1().expect("Part 1 should complete successfully");
        assert_eq!(result, 1349);
    }

    #[test]
    pub fn test_part2() {
        let result = Solver::part2().expect("Part 2 should complete successfully");
        assert_eq!(result, 8277);
    }
}
