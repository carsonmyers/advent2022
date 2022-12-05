use super::{Challenge, ChallengePart, Result};

#[derive(Debug)]
pub struct Assignment(i64, i64);

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        self.0 <= other.0 && self.0 <= other.1 && self.1 >= other.1 && self.1 >= other.0
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.0 <= other.1 && self.0 >= other.0 || self.1 >= other.0 && self.1 <= other.1
    }
}

impl From<&str> for Assignment {
    fn from(d: &str) -> Self {
        let mut parts = d.split('-');
        Assignment(
            parts
                .next()
                .expect("invalid assignment data")
                .parse::<i64>()
                .expect("invalid assignment number"),
            parts
                .next()
                .expect("invalid assignment data")
                .parse::<i64>()
                .expect("invalid assignment number"),
        )
    }
}

pub struct Day4<T: AsRef<str>> {
    r#in: Vec<T>,
}

impl<T: AsRef<str>> Day4<T> {
    pub fn new(r#in: Vec<T>) -> Self {
        Self { r#in }
    }

    fn run_first(&self) -> Result<i64> {
        Ok(self
            .get_assignments()
            .into_iter()
            .filter(|(left, right)| left.contains(right) || right.contains(left))
            .count() as i64)
    }

    fn run_second(&self) -> Result<i64> {
        Ok(self
            .get_assignments()
            .into_iter()
            .filter(|(left, right)| left.overlaps(right) || right.overlaps(left))
            .count() as i64)
    }

    fn get_assignments(&self) -> Vec<(Assignment, Assignment)> {
        self.r#in
            .iter()
            .filter_map(|line| match line.as_ref() {
                "" => None,
                l => Some(l),
            })
            .map(|line| {
                let mut parts = line.split(',');

                (
                    Assignment::from(parts.next().expect("invalid assignment data")),
                    Assignment::from(parts.next().expect("invalid assignment data")),
                )
            })
            .collect()
    }
}

impl<T: AsRef<str>> Challenge<T> for Day4<T> {
    fn run(&self, part: ChallengePart) -> Result<String> {
        match part {
            ChallengePart::First => self.run_first().map(|res| res.to_string()),
            ChallengePart::Second => self.run_second().map(|res| res.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = r#"
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        "#
        .lines()
        .skip(1)
        .map(|line| line.trim())
        .collect::<Vec<_>>();

        let challenge = Day4::new(input);

        let result = challenge.run_first();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);

        let result = challenge.run_second();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);
    }
}
