#![allow(dead_code, unused_imports)]

use anyhow::anyhow;

const STARTING_POSITION: i32 = 50;
const POSITIONS_IN_CIRCLE: i32 = 100;

struct Solver;

impl Solver {
    fn read() -> &'static str {
        include_str!("data/yr2025_d01.in")
    }

    fn readlines() -> Vec<&'static str> {
        Solver::read().lines().collect()
    }

    fn parse_instruction(line: &str) -> anyhow::Result<(char, i32)> {
        let direction = line.chars().next().ok_or_else(|| anyhow!("Empty line"))?;
        let clicks = line[1..].parse::<i32>()?;

        Ok((direction, clicks))
    }

    fn part1() -> anyhow::Result<i32> {
        let mut point = STARTING_POSITION;
        let mut result = 0;

        let lines = Solver::readlines();
        for line in lines.iter() {
            match Solver::parse_instruction(line) {
                Ok((direction, clicks)) => match direction {
                    'L' => {
                        point = (point - clicks + POSITIONS_IN_CIRCLE) % POSITIONS_IN_CIRCLE;
                    }
                    'R' => {
                        point = (point + clicks) % POSITIONS_IN_CIRCLE;
                    }
                    _ => {
                        return Err(anyhow!("Cannot properly parse the direction"));
                    }
                },
                Err(e) => return Err(e),
            }

            if point == 0 {
                result += 1;
            }
        }

        Ok(result)
    }

    fn part2() -> anyhow::Result<i32> {
        let mut point = STARTING_POSITION;
        let mut result = 0;

        let lines = Solver::readlines();
        for line in lines.iter() {
            match Solver::parse_instruction(line) {
                Ok((direction, clicks)) => {
                    let full_rotations = clicks / POSITIONS_IN_CIRCLE;
                    let remaining_clicks = clicks % POSITIONS_IN_CIRCLE;

                    result += full_rotations;
                    if remaining_clicks > 0 {
                        match direction {
                            'L' => {
                                if point - remaining_clicks <= 0 && point > 0 {
                                    result += 1;
                                }
                                point = (point - remaining_clicks + POSITIONS_IN_CIRCLE)
                                    % POSITIONS_IN_CIRCLE;
                            }
                            'R' => {
                                if point + remaining_clicks >= POSITIONS_IN_CIRCLE
                                    && point < POSITIONS_IN_CIRCLE
                                {
                                    result += 1;
                                }
                                point = (point + remaining_clicks) % POSITIONS_IN_CIRCLE;
                            }
                            _ => {
                                return Err(anyhow!("Cannot properly parse the direction"));
                            }
                        }
                    }
                }
                Err(e) => return Err(e),
            }
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
        assert_eq!(result, 1105);
    }

    #[test]
    pub fn test_part2() {
        let result = Solver::part2().expect("Part 2 should complete successfully");
        assert_eq!(result, 6599);
    }
}
