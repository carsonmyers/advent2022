use std::collections::HashSet;
use std::iter::FromIterator;

use itertools::Itertools;

use super::{Challenge, ChallengePart, Result};

pub struct Day3<T: AsRef<str>> {
    r#in: Vec<T>,
    item_values: Vec<char>,
}

impl<T: AsRef<str>> Day3<T> {
    pub fn new(r#in: Vec<T>) -> Self {
        let item_values = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .collect::<Vec<_>>();

        Self { r#in, item_values }
    }

    fn run_first(&self) -> Result<i64> {
        let result = self
            .r#in
            .iter()
            .map(|data| {
                let sack = data.as_ref();
                let half = sack.len() / 2;
                let (left, right) = (
                    HashSet::<char>::from_iter(sack.chars().take(half)),
                    HashSet::<char>::from_iter(sack.chars().skip(half)),
                );

                left.intersection(&right)
                    .map(|c| self.priority_of(*c))
                    .sum::<i64>()
            })
            .sum::<i64>();

        Ok(result)
    }

    fn run_second(&self) -> Result<i64> {
        let result = self
            .r#in
            .chunks_exact(3)
            .map(|chunk| {
                if chunk.len() != 3 {
                    panic!("invalid window size {}", chunk.len());
                }

                let (first, second, third) = (
                    HashSet::<char>::from_iter(chunk[0].as_ref().chars()),
                    HashSet::<char>::from_iter(chunk[1].as_ref().chars()),
                    HashSet::<char>::from_iter(chunk[2].as_ref().chars()),
                );

                first
                    .intersection(&second)
                    .map(|c| *c)
                    .collect::<HashSet<_, _>>()
                    .intersection(&third)
                    .map(|c| self.priority_of(*c))
                    .sum::<i64>()
            })
            .sum::<i64>();

        Ok(result)
    }

    fn priority_of(&self, item: char) -> i64 {
        let index = self
            .item_values
            .iter()
            .find_position(|c| **c == item)
            .expect("invalid item found")
            .0;

        // score is 1-based, index is 0-based
        index as i64 + 1
    }
}

impl<T: AsRef<str>> Challenge<T> for Day3<T> {
    fn run(&self, part: ChallengePart) -> Result<i64> {
        match part {
            ChallengePart::First => self.run_first(),
            ChallengePart::Second => self.run_second(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = r#"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "#
        .lines()
        .skip(1)
        .map(|line| line.trim())
        .collect::<Vec<_>>();

        let challenge = Day3::new(input);

        let result = challenge.run_first();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 157);

        let result = challenge.run_second();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 70);
    }
}
