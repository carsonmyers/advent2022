use thiserror::Error;

use crate::input;

#[derive(Error, Debug, Default)]
pub enum Error {
    #[error("invalid day `{0}`")]
    InvalidDay(usize),
    #[error("day `{0}` not implemented")]
    DayNotImplemented(usize),
    #[error("input error: {0}")]
    InputError(#[from] input::error::Error),
    #[error("error parsing int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("missing data in challenge: {0}")]
    MissingDataError(String),
    #[error("invalid command in challenge: {0}")]
    InvalidCommandError(String),
    #[error("too many lines of input")]
    TooManyLinesError(),
    #[error("no solution found")]
    NoSolutionError(),
    #[error("unknown error")]
    #[default]
    UnknownError,
}

impl Error {
    pub fn missing_data(name: &str) -> Self {
        Error::MissingDataError(name.to_string())
    }
}

/*#[derive(Error, Debug, Default)]
#[error("{source}")]
pub struct Error {
    pub(crate) day: usize,
    #[source]
    pub(crate) source: ErrorKind,
}

impl Error {
    pub(crate) fn invalid_day(day: usize) -> Self {
        Error {
            day,
            source: ErrorKind::InvalidDay(day),
        }
    }

    pub(crate) fn not_implemented(day: usize) -> Self {
        Error {
            day,
            source: ErrorKind::DayNotImplemented(day),
        }
    }

    pub(crate) fn input_error(day: usize, err: input::error::Error) -> Self {
        Error {
            day,
            source: ErrorKind::InputError(err),
        }
    }

    pub(crate) fn parse_int_error(day: usize, err: std::num::ParseIntError, num: String) -> Self {
        Error {
            day,
            source: ErrorKind::ParseIntError(err, num),
        }
    }

    pub(crate) fn missing_data_error(day: usize, data: &str) -> Self {
        Error {
            day,
            source: ErrorKind::MissingDataError(data.to_owned()),
        }
    }

    pub(crate) fn invalid_command_error(day: usize, command: &str) -> Self {
        Error {
            day,
            source: ErrorKind::InvalidCommandError(command.to_owned()),
        }
    }
}
*/
