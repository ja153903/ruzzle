#![allow(dead_code, unused_imports)]

use anyhow::anyhow;

struct Solver;

struct Dimensions {
    length: i32,
    width: i32,
    height: i32,
}

impl Dimensions {
    fn from(line: &str) -> anyhow::Result<Self> {
        let parts: Result<Vec<i32>, _> = line
            .split("x")
            .map(|part| part.parse::<i32>())
            .take(3)
            .collect();

        let parts =
            parts.map_err(|_| anyhow!("Could not parse numeric value in line: {}", line))?;

        match parts[..] {
            [fst, snd, thd] => Ok(Self {
                length: fst,
                width: snd,
                height: thd,
            }),
            _ => Err(anyhow!(
                "Expected exactly 3 dimensions, got {} in line: {}",
                parts.len(),
                line
            )),
        }
    }

    fn surface_area(&self) -> i32 {
        2 * self.length * self.width + 2 * self.length * self.height + 2 * self.width * self.height
    }

    fn smallest_area(&self) -> i32 {
        (self.length * self.width)
            .min(self.length * self.height)
            .min(self.width * self.height)
    }

    fn volume(&self) -> i32 {
        self.length * self.width * self.height
    }

    fn smallest_perimeter(&self) -> i32 {
        (2 * self.length + 2 * self.width)
            .min(2 * self.length + 2 * self.height)
            .min(2 * self.width + 2 * self.height)
    }
}

impl Solver {
    fn read() -> &'static str {
        include_str!("data/yr2015_d02.in")
    }

    fn part1() -> anyhow::Result<i32> {
        Solver::read()
            .lines()
            .map(|line| {
                let dim = Dimensions::from(line)?;
                Ok(dim.surface_area() + dim.smallest_area())
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map(|values| values.into_iter().sum())
    }

    fn part2() -> anyhow::Result<i32> {
        Solver::read()
            .lines()
            .map(|line| {
                let dim = Dimensions::from(line)?;
                Ok(dim.volume() + dim.smallest_perimeter())
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .map(|values| values.into_iter().sum())
    }
}

#[cfg(test)]
mod test {
    use super::Solver;

    #[test]
    pub fn test_part1() {
        let result = Solver::part1().expect("Part 1 should complete successfully");
        assert_eq!(result, 1606483);
    }

    #[test]
    pub fn test_part2() {
        let result = Solver::part2().expect("Part 2 should complete successfully");
        assert_eq!(result, 3842356);
    }
}
