#![allow(dead_code, unused_imports)]

use anyhow::anyhow;

#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "*" => Ok(Self::Multiply),
            "+" => Ok(Self::Add),
            _ => Err(anyhow!("Unknown operation: {}", s)),
        }
    }

    fn identity(&self) -> i64 {
        match self {
            Self::Multiply => 1,
            Self::Add => 0,
        }
    }

    fn apply(&self, values: &[i64]) -> i64 {
        match self {
            Self::Multiply => values.iter().product(),
            Self::Add => values.iter().sum(),
        }
    }
}

struct Solver;

impl Solver {
    fn read() -> &'static str {
        include_str!("data/yr2025_d06.in")
    }

    fn parse_grid() -> anyhow::Result<(Vec<Vec<i64>>, Vec<Operation>)> {
        let mut lines: Vec<&str> = Self::read().lines().collect();
        let operations_line = lines
            .pop()
            .ok_or_else(|| anyhow!("Missing operations line"))?;

        let operations: Result<Vec<Operation>, _> = operations_line
            .split_whitespace()
            .map(Operation::from_str)
            .collect();

        let grid: Result<Vec<Vec<i64>>, _> = lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|part| {
                        part.parse::<i64>()
                            .map_err(|e| anyhow!("Parse error: {}", e))
                    })
                    .collect()
            })
            .collect();

        Ok((grid?, operations?))
    }

    fn part1() -> anyhow::Result<i64> {
        let (grid, operations) = Self::parse_grid()?;

        let mut total = 0;
        for (col, operation) in operations.iter().enumerate() {
            let column_values: Vec<i64> = grid.iter().map(|row| row[col]).collect();

            total += operation.apply(&column_values);
        }

        Ok(total)
    }

    fn part2() -> anyhow::Result<i64> {
        let input = Self::read();
        let mut lines: Vec<&str> = input.lines().collect();
        let operations_line = lines
            .pop()
            .ok_or_else(|| anyhow!("Missing operations line"))?;

        let operations: Result<Vec<Operation>, _> = operations_line
            .split_whitespace()
            .map(Operation::from_str)
            .collect();
        let operations = operations?;

        let mut total = 0;
        let mut groups = Vec::new();

        let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
        let mut current_group = Vec::new();

        for col_idx in (0..max_width).rev() {
            let mut column_chars = String::new();
            let mut has_content = false;

            for line in &lines {
                if let Some(ch) = line.chars().nth(col_idx)
                    && !ch.is_whitespace()
                {
                    has_content = true;
                    column_chars.push(ch);
                }
            }

            if has_content {
                if let Ok(num) = column_chars.parse::<i64>() {
                    current_group.push(num);
                }
            } else if !current_group.is_empty() {
                groups.push(current_group);
                current_group = Vec::new();
            }
        }

        if !current_group.is_empty() {
            groups.push(current_group);
        }

        for (group, operation) in groups.iter().zip(operations.iter().rev()) {
            total += operation.apply(group);
        }

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    pub fn test_part1() {
        let result = Solver::part1().expect("Part 1 should complete successfully");
        assert_eq!(result, 5877594983578);
    }

    #[test]
    pub fn test_part2() {
        let result = Solver::part2().expect("Part 2 should complete successfully");
        assert_eq!(result, 11159825706149);
    }
}
