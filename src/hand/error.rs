use crate::card::{self, Card};
use std::collections::LinkedList;
use std::fmt;

#[derive(Debug)] // TEMPORARY
pub enum Error {
    EmptyHand,
    TryFromMediator(Box<dyn std::error::Error>),
    TryFromSlice(Box<dyn std::error::Error>),
    TryFromRank(Box<dyn std::error::Error>),
    InvalidStraight(Box<[Card]>, Box<dyn std::error::Error>),
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
    PairError([Box<dyn std::error::Error>; 2]),
}

impl std::error::Error for Error {}
unsafe impl Send for Error {}

impl fmt::Display for Error {
    // TODO
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
