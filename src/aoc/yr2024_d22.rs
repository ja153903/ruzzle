#![allow(dead_code, unused_imports, unused)]

use anyhow::anyhow;

struct Solver;

impl Solver {
    fn read() -> &'static str {
        include_str!("data/yr2024_d22.in")
    }

    fn update(secret: i64) -> i64 {
        let phase1 = ((secret * 64) ^ secret) % 16777216;
        let phase2 = ((phase1 / 32) ^ phase1) % 16777216;

        ((phase2 * 2048) ^ phase2) % 16777216
    }

    fn part1() -> anyhow::Result<i64> {
        let secrets: Result<Vec<i64>, _> = Solver::read()
            .lines()
            .map(|line| line.parse::<i64>())
            .collect();

        let secrets = secrets.map_err(|_| anyhow!("Could not parse line into integer"))?;

        Ok(secrets
            .into_iter()
            .map(|secret| {
                let mut s = secret;
                for _ in 0..2000 {
                    s = Solver::update(s);
                }

                s
            })
            .sum())
    }

    fn part2() -> anyhow::Result<i64> {
        let secrets: Result<Vec<i64>, _> = Solver::read()
            .lines()
            .map(|line| line.parse::<i64>())
            .collect();

        let secrets = secrets.map_err(|_| anyhow!("Could not parse line into integer"))?;

        // for each secret, we can generate the 2000 updates and for each update, we can store the
        // prices along with the differentials leading up to the value (at most 4)
        // [ ("", 3), ("-3", 0), ("-3,6", 6), ("-3,6,-1", 5), ("-3,6,-1,-1", 4), ("6,-1,-1,0", 4),
        // ("-1,-1,0,2", 6), ()]

        todo!("don't quite know how to solve this one yet")
    }
}

#[cfg(test)]
mod test {
    use super::Solver;

    #[test]
    pub fn test_part1() {
        match Solver::part1() {
            Ok(res) => assert_eq!(res, 20441185092),
            Err(_) => panic!(),
        }
    }
}
