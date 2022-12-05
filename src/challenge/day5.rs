use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use super::error::*;
use super::{Challenge, ChallengePart, Result};

#[derive(Debug)]
struct Crate(char);

impl Crate {
    fn batch<'a>(iter: &'a mut impl Iterator<Item = char>) -> Option<Option<Crate>> {
        let chars: [char; 3];
        if let (Some(x), Some(y), Some(z)) = (iter.next(), iter.next(), iter.next()) {
            chars = [x, y, z];
        } else {
            return None;
        }

        // Skip space between crates
        iter.next();

        match chars {
            [' ', ' ', ' '] => Some(None),
            ['[', c, ']'] => Some(Some(Crate(c))),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
struct Move {
    count: usize,
    src: String,
    dst: String,
}

impl Move {
    fn from_commands(line: &str) -> Result<Self> {
        let mut r#move: Move = Default::default();

        let mut words = line.split_ascii_whitespace();
        for _ in 0..3 {
            r#move
                .read_command(&mut words)
                .map_err(|_| Error::invalid_command_error(5, line))?;
        }

        Ok(r#move)
    }

    fn read_command<'a>(&mut self, iter: &mut impl Iterator<Item = &'a str>) -> Result<()> {
        let (command, argument) = (iter.next(), iter.next());

        match (command, argument) {
            (Some("move"), Some(n)) => self.count = n.parse().expect("expected a number"),
            (Some("from"), Some(n)) => self.src = n.to_string(),
            (Some("to"), Some(n)) => self.dst = n.to_string(),
            _ => return Err(Default::default()),
        }

        Ok(())
    }
}

pub struct Day5<T: AsRef<str>> {
    r#in: Vec<T>,
}

impl<T: AsRef<str>> Day5<T> {
    pub fn new(r#in: Vec<T>) -> Self {
        Self { r#in }
    }

    fn run(&self, multi_move: bool) -> Result<String> {
        let mut iter = self.r#in.iter().map(|line| line.as_ref());

        let (mut stacks, names) = Self::setup_stacks(&mut iter)?;
        let moves = Self::setup_moves(&mut iter)?;

        for r#move in moves {
            Self::move_crates(&mut stacks, r#move, multi_move)?;
        }

        let results = names
            .iter()
            .map(|name| {
                let c = stacks
                    .get_mut(name)
                    .ok_or_else(|| Error::missing_data_error(5, "crate stack"))?
                    .pop_front()
                    .ok_or_else(|| Error::missing_data_error(5, "stack is empty"))?
                    .0;

                Ok::<char, Error>(c)
            })
            .collect::<Result<String>>()?;

        Ok(results)
    }

    fn setup_stacks<'a>(
        iter: &mut impl Iterator<Item = &'a str>,
    ) -> Result<(HashMap<String, VecDeque<Crate>>, Vec<String>)> {
        let crate_data = iter.take_while(|line| line.len() > 0).collect_vec();

        let mut stacks = HashMap::new();
        let names = crate_data
            .get(crate_data.len() - 1)
            .ok_or(Error::missing_data_error(5, "crate stack names"))?
            .split_whitespace()
            .map(|name| {
                stacks.insert(String::from(name), VecDeque::new());
                String::from(name)
            })
            .collect_vec();

        let crates = crate_data
            .iter()
            .take(crate_data.len() - 1)
            .map(|line| line.chars().batching(Crate::batch).collect_vec())
            .collect_vec();

        for mut level in crates {
            for (idx, slot) in level.iter_mut().enumerate() {
                if slot.is_none() {
                    continue;
                }

                let name = names
                    .get(idx)
                    .ok_or(Error::missing_data_error(5, "name for stack"))?;

                let queue = stacks
                    .get_mut(name)
                    .ok_or(Error::missing_data_error(5, "deque for stack"))?;

                // take the crate out of the level vector, leaving None in its place, so no copy
                // is created
                queue.push_back(slot.take().unwrap());
            }
        }

        Ok((stacks, names))
    }

    fn setup_moves<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Result<Vec<Move>> {
        let moves = iter
            .filter(|line| line.len() > 0)
            .map(|line| Move::from_commands(line))
            .collect::<Result<Vec<_>>>()?;

        Ok(moves)
    }

    fn move_crates(
        stacks: &mut HashMap<String, VecDeque<Crate>>,
        r#move: Move,
        multi_move: bool,
    ) -> Result<()> {
        let mut intermediate = VecDeque::new();

        let src = stacks
            .get_mut(&r#move.src)
            .ok_or_else(|| Error::missing_data_error(5, "crate stack"))?;

        for _ in 0..r#move.count {
            intermediate.push_front(
                src.pop_front()
                    .ok_or_else(|| Error::missing_data_error(5, "crate"))?,
            )
        }

        let dst = stacks
            .get_mut(&r#move.dst)
            .ok_or_else(|| Error::missing_data_error(5, "crate stack"))?;

        for _ in 0..r#move.count {
            // multi_move causes the crates to be moved as-is such that they end up in the same
            // order. Pushing/popping from the same end reverses the order; so:
            // to multi_move, crates come out in the reverse order, and go into/out of the same
            // end of the intermediate queue to flip the order, and go into dst in the same order
            // as they were in src. To not multi_move, crates have to pass through the dequeue
            // without reversing. The crates were pushed in the front end of the intermediate queue
            let ct = if multi_move {
                intermediate.pop_front()
            } else {
                intermediate.pop_back()
            };

            dst.push_front(ct.unwrap());
        }

        Ok(())
    }
}

impl<T: AsRef<str>> Challenge<T> for Day5<T> {
    fn run(&self, part: ChallengePart) -> Result<String> {
        match part {
            ChallengePart::First => self.run(false),
            ChallengePart::Second => self.run(true),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        // the # characters prevent the significant spaces in the crate structure from being
        // trimmed; they are themselves trimmed after whitespace
        let input = r#"
           #    [D]    
           #[N] [C]    
           #[Z] [M] [P]
           #1   2   3 

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
        "#
        .lines()
        .skip(1)
        .map(|line| line.trim())
        .map(|line| line.trim_start_matches(|c| c == '#'))
        .collect::<Vec<_>>();

        let challenge = Day5::new(input);

        let result = challenge.run(false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "CMZ");

        let result = challenge.run(true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "MCD");
    }
}
