use super::{Challenge, ChallengePart, Result};

pub struct Day5<T: AsRef<str>> {
    r#in: Vec<T>,
}

impl<T: AsRef<str>> Day5<T> {
    pub fn new(r#in: Vec<T>) -> Self {
        Self { r#in }
    }

    fn run_first(&self) -> Result<String> {
        Ok(String::new())
    }

    fn run_second(&self) -> Result<String> {
        Ok(String::new())
    }
}

impl<T: AsRef<str>> Challenge<T> for Day5<T> {
    fn run(&self, part: ChallengePart) -> Result<String> {
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
                [D]    
            [N] [C]    
            [Z] [M] [P]
            1   2   3 

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
        "#
        .lines()
        .skip(1)
        .map(|line| line.trim())
        .collect::<Vec<_>>();

        let challenge = Day5::new(input);

        let result = challenge.run_first();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "CMZ");
    }
}
