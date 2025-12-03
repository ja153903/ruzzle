#![allow(dead_code, unused_imports)]

use anyhow::anyhow;
use itertools::Itertools;
use std::collections::VecDeque;

struct Solver;

impl Solver {
    fn read() -> &'static str {
        include_str!("data/yr2025_d03.in")
    }

    fn readlines<'a>() -> Vec<&'a str> {
        Self::read().lines().collect()
    }

    fn part1() -> anyhow::Result<i64> {
        Ok(Self::readlines()
            .iter()
            .map(|pack| {
                // for each pack, we want to keep track of a queue of size 2
                let mut largest_items: VecDeque<char> = VecDeque::new();
                pack.chars().for_each(|ch| {
                    // if the second value in the queue is larger than the
                    // first value in the queue, then we'll bump it whenever we see
                    // a larger item

                    if largest_items.len() <= 1 {
                        largest_items.push_back(ch);
                    } else {
                        // remove the first two elements to compare
                        let fst = largest_items.pop_front();
                        let snd = largest_items.pop_front();

                        if let (Some(fst), Some(snd)) = (fst, snd) {
                            // if fst is less than snd, then we can just accept
                            // anything that comes
                            if fst < snd {
                                largest_items.push_front(snd);
                                largest_items.push_back(ch);
                            // if fst is greater than snd but snd is less than ch
                            // then we just add ch
                            } else if snd < ch {
                                largest_items.push_front(fst);
                                largest_items.push_back(ch);
                            // if fst is greater than snd and snd is greater than ch
                            } else {
                                largest_items.push_front(fst);
                                largest_items.push_back(snd);
                            }
                        }
                    }
                });

                let fst = largest_items.pop_front();
                let snd = largest_items.pop_front();

                if let (Some(fst), Some(snd)) = (fst, snd) {
                    let combined: String = vec![fst, snd].into_iter().collect();
                    if let Ok(parsed) = combined.parse::<i64>() {
                        return parsed;
                    }
                }

                0
            })
            .sum())
    }

    fn part2() -> anyhow::Result<i64> {
        Ok(Self::readlines()
            .iter()
            .map(|pack| {
                let mut largest_items: VecDeque<char> = VecDeque::new();
                let mut deletes_remaining = pack.len() - 12;
                pack.chars().for_each(|ch| {
                    while let Some(&back) = largest_items.back()
                        && back < ch
                        && deletes_remaining > 0
                    {
                        largest_items.pop_back();
                        deletes_remaining -= 1;
                    }

                    largest_items.push_back(ch);
                });

                largest_items.truncate(12);

                let combined: String = largest_items.into_iter().collect();
                combined.parse::<i64>().unwrap_or(0)
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    pub fn test_part1() {
        let result = Solver::part1().expect("Part 1 should complete successfully");
        assert_eq!(result, 17766);
    }

    #[test]
    pub fn test_part2() {
        let result = Solver::part2().expect("Part 2 should complete successfully");
        assert_eq!(result, 176582889354075);
    }
}
