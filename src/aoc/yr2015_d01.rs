#![allow(dead_code, unused_imports)]

use anyhow::anyhow;

struct Solver;

impl Solver {
    fn read() -> &'static str {
        include_str!("data/yr2015_d01.in")
    }

    fn part1() -> anyhow::Result<i32> {
        let result = Solver::read()
            .chars()
            .map(|ch| match ch {
                '(' => 1,
                ')' => -1,
                _ => 0,
            })
            .sum();
        Ok(result)
    }

    fn part2() -> anyhow::Result<usize> {
        let contents = Solver::read();
        let mut level = 0;

        for (i, ch) in contents.char_indices() {
            if ch == '(' {
                level += 1;
            } else if ch == ')' {
                level -= 1;
            }

            if level < 0 {
                return Ok(i + 1);
            }
        }

        Err(anyhow!("Could not find correct level"))
    }
}

#[cfg(test)]
mod test {
    use super::Solver;

    #[test]
    pub fn test_part1() {
        let result = Solver::part1().expect("Part 1 should complete successfully");
        assert_eq!(result, 74);
    }

    #[test]
    pub fn test_part2() {
        let result = Solver::part2().expect("Part 2 should complete successfully");
        assert_eq!(result, 1795);
    }
}
