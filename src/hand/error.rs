use crate::card::{self, Card};
use std::error;
use std::fmt;

#[derive(Debug)] // TEMPORARY
pub enum Error {
    EmptyHand,
    TryFromMediator(Box<dyn error::Error + Send>),
    TryFromSlice(Box<dyn error::Error + Send>),
    TryFromRank(Box<dyn error::Error + Send>),
    InvalidStraight(Box<[Card]>, Box<dyn error::Error + Send>),
    InvalidSRank(Card, card::Rank),
    BuildForgery {
        original: Box<[Card]>,
        components: String,
        forged: Box<[Card]>,
    },
    InvalidLength {
        expected: usize,
        actual: usize,
        contents: Box<[Card]>,
    },
}

impl error::Error for Error {}
unsafe impl Send for Error {}

impl fmt::Display for Error {
    // TODO
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
