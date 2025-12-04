#![allow(dead_code, unused_imports)]

use anyhow::anyhow;

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

    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    fn count_neighbors(grid: &[Vec<char>], row: usize, col: usize, target: char) -> usize {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        let (r, c) = (row as i32, col as i32);

        Self::DIRECTIONS
            .iter()
            .filter(|(dr, dc)| {
                let (nr, nc) = (r + dr, c + dc);
                nr >= 0
                    && nr < rows
                    && nc >= 0
                    && nc < cols
                    && grid[nr as usize][nc as usize] == target
            })
            .count()
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
                    let neighbor_count = Self::count_neighbors(&grid, row, col, '@');
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
                        let neighbor_count = Self::count_neighbors(&grid, row, col, '@');
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
