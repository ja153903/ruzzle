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
    fn part1() -> anyhow::Result<i32> {
        let grid = Self::readlines();
        let rows = grid.len();
        let cols = grid[0].len();

        let mut res = 0;

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '@' {
                    let mut count = 0;

                    // check the 8 directions
                    if i > 0 && grid[i - 1][j] == '@' {
                        count += 1;
                    }
                    if j > 0 && grid[i][j - 1] == '@' {
                        count += 1;
                    }
                    if i < rows - 1 && grid[i + 1][j] == '@' {
                        count += 1;
                    }
                    if j < cols - 1 && grid[i][j + 1] == '@' {
                        count += 1;
                    }

                    // get diagonals as well
                    if i > 0 && j > 0 && grid[i - 1][j - 1] == '@' {
                        count += 1;
                    }
                    if i < rows - 1 && j < cols - 1 && grid[i + 1][j + 1] == '@' {
                        count += 1;
                    }

                    if i > 0 && j < cols - 1 && grid[i - 1][j + 1] == '@' {
                        count += 1;
                    }

                    if i < rows - 1 && j > 0 && grid[i + 1][j - 1] == '@' {
                        count += 1;
                    }

                    if count < 4 {
                        res += 1;
                    }
                }
            }
        }

        Ok(res)
    }

    fn part2() -> anyhow::Result<i32> {
        let mut grid = Self::readlines();
        let rows = grid.len();
        let cols = grid[0].len();

        let mut res = 0;

        loop {
            let mut has_mutated_state = false;

            for i in 0..rows {
                for j in 0..cols {
                    if grid[i][j] == '@' {
                        let mut count = 0;

                        // check the 8 directions
                        if i > 0 && grid[i - 1][j] == '@' {
                            count += 1;
                        }
                        if j > 0 && grid[i][j - 1] == '@' {
                            count += 1;
                        }
                        if i < rows - 1 && grid[i + 1][j] == '@' {
                            count += 1;
                        }
                        if j < cols - 1 && grid[i][j + 1] == '@' {
                            count += 1;
                        }

                        // get diagonals as well
                        if i > 0 && j > 0 && grid[i - 1][j - 1] == '@' {
                            count += 1;
                        }
                        if i < rows - 1 && j < cols - 1 && grid[i + 1][j + 1] == '@' {
                            count += 1;
                        }

                        if i > 0 && j < cols - 1 && grid[i - 1][j + 1] == '@' {
                            count += 1;
                        }

                        if i < rows - 1 && j > 0 && grid[i + 1][j - 1] == '@' {
                            count += 1;
                        }

                        if count < 4 {
                            has_mutated_state = true;
                            grid[i][j] = '.';
                            res += 1;
                        }
                    }
                }
            }

            if !has_mutated_state {
                break;
            }
        }

        Ok(res)
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
        assert_eq!(result, 0);
    }
}
