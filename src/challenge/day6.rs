use itertools::Itertools;

use super::error::Error;
use super::{Challenge, ChallengePart, Result};

pub struct Day6<T: AsRef<str>> {
    r#in: Vec<T>,
}

impl<T: AsRef<str>> Day6<T> {
    pub fn new(r#in: Vec<T>) -> Self {
        Self { r#in }
    }

    fn run_first(&self) -> Result<usize> {
        let result = self
            .get_signal()?
            .windows(4)
            .find_position(unique)
            .map(|(pos, _)| pos + 4)
            .ok_or_else(|| Error::NoSolutionError())?;

        Ok(result)
    }

    fn run_second(&self) -> Result<usize> {
        let result = self
            .get_signal()?
            .windows(14)
            .find_position(unique)
            .map(|(pos, _)| pos + 14)
            .ok_or_else(|| Error::NoSolutionError())?;

        Ok(result)
    }

    fn get_signal(&self) -> Result<Vec<char>> {
        let mut signal = self
            .r#in
            .iter()
            .map(|line| line.as_ref().chars().collect_vec())
            .collect_vec();

        if signal.len() > 1 {
            Err(Error::TooManyLinesError())
        } else if signal.get(0).is_none() {
            Err(Error::missing_data("input data"))
        } else {
            Ok(signal.swap_remove(0))
        }
    }
}

impl<T: AsRef<str>> Challenge<T> for Day6<T> {
    fn run(&self, part: ChallengePart) -> Result<String> {
        match part {
            ChallengePart::First => self.run_first().map(|res| res.to_string()),
            ChallengePart::Second => self.run_second().map(|res| res.to_string()),
        }
    }
}

fn unique(chars: &&[char]) -> bool {
    let bits = to_bits(chars);
    let count = count_bits(bits);

    count == chars.len()
}

fn to_bits(chars: &[char]) -> u64 {
    let mut bits = 0_u64;
    for c in chars {
        let offset = u64::from(*c) - u64::from('a');
        let bit = 1_u64 << offset;
        bits |= bit;
    }

    bits
}

fn count_bits(bits: u64) -> usize {
    let mut count = 0;
    let mut n = bits;
    while n > 0 {
        count += 1;
        n = n & (n - 1);
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let table: &[(&str, usize, usize); 5] = &[
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
        ];

        for (input, expected_1, expected_2) in table {
            let challenge = Day6::new(vec![input]);

            let result = challenge.run_first();
            assert!(result.is_ok());
            assert_eq!(&result.unwrap(), expected_1);

            let result = challenge.run_second();
            assert!(result.is_ok());
            assert_eq!(&result.unwrap(), expected_2);
        }
    }
}
