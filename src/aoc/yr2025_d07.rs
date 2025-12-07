#![allow(dead_code, unused_imports)]

use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::anyhow;
use itertools::Itertools;

struct Solver;

impl Solver {
    fn read() -> &'static str {
        include_str!("data/yr2025_d07.in")
    }

    fn readlines() -> Vec<Vec<char>> {
        Self::read()
            .lines()
            .map(|line| line.chars().collect())
            .collect()
    }

    // NOTE: BFS example
    fn part1() -> anyhow::Result<i32> {
        let grid = Self::readlines();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        let mut result = 0;

        let start_col = grid[0]
            .iter()
            .find_position(|&&ch| ch == 'S')
            .ok_or_else(|| anyhow!("Cannot find start"))?;

        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

        queue.push_back((0, start_col.0 as i32));
        visited.insert((0, start_col.0 as i32));

        while !queue.is_empty() {
            match queue.pop_front() {
                Some((r, c)) => {
                    let nr = r + 1;
                    if (nr as usize) >= grid.len() {
                        continue;
                    }

                    if grid[nr as usize][c as usize] == '^' {
                        result += 1;

                        if c > 0 {
                            let nc = c - 1;
                            if !visited.contains(&(nr, nc)) {
                                queue.push_back((nr, nc));
                                visited.insert((nr, nc));
                            }
                        }

                        if (c as usize) < grid[0].len() - 1 {
                            let nc = c + 1;
                            if !visited.contains(&(nr, nc)) {
                                queue.push_back((nr, nc));
                                visited.insert((nr, nc));
                            }
                        }
                    } else if !visited.contains(&(nr, c)) {
                        queue.push_back((nr, c));
                        visited.insert((nr, c));
                    }
                }
                None => return Err(anyhow!("Empty item popped from queue")),
            }
        }

        Ok(result)
    }

    // NOTE: DFS example
    fn part2() -> anyhow::Result<i64> {
        let grid = Self::readlines();
        let start_col = grid[0]
            .iter()
            .find_position(|&&ch| ch == 'S')
            .ok_or_else(|| anyhow!("Cannot find start"))?;

        let mut memo: HashMap<(i64, i64, usize), i64> = HashMap::new();
        Ok(Self::dfs_memoized(
            &grid,
            (0, start_col.0 as i64),
            0,
            &mut memo,
        ))
    }

    fn dfs_memoized(
        grid: &Vec<Vec<char>>,
        start: (i64, i64),
        level: usize,
        memo: &mut HashMap<(i64, i64, usize), i64>,
    ) -> i64 {
        let key = (start.0, start.1, level);

        if let Some(&cached) = memo.get(&key) {
            return cached;
        }

        let (row, col) = start;
        let rows = grid.len();
        let cols = grid[0].len();

        let result = if row < 0 || col < 0 || (row as usize) >= rows || (col as usize) >= cols {
            0
        } else if level == rows - 1 {
            1
        } else {
            let new_row = row + 1;
            if (new_row as usize) >= rows {
                0
            } else if grid[new_row as usize][col as usize] == '^' {
                Self::dfs_memoized(grid, (new_row, col - 1), level + 1, memo)
                    + Self::dfs_memoized(grid, (new_row, col + 1), level + 1, memo)
            } else {
                Self::dfs_memoized(grid, (new_row, col), level + 1, memo)
            }
        };

        memo.insert(key, result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    pub fn test_part1() {
        let result = Solver::part1().expect("Part 1 should complete successfully");
        assert_eq!(result, 1678);
    }

    #[test]
    pub fn test_part2() {
        let result = Solver::part2().expect("Part 2 should complete successfully");
        assert_eq!(result, 0);
    }
}
