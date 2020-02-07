use inner::srank;
use std::error;

mod fmt;
mod r#impl;
mod inner;
pub mod mediator;

// Note: Make Invalid States Unrepresentable
/// This is
/// TODO: Document
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    High(inner::High),
    Pair(inner::Pair),
    TwoPair(inner::TwoPair),
    Trips(inner::Trips),
    Straight(inner::Straight),
    Flush(inner::Flush),
    House(inner::House),
    Quads(inner::Quads),
    StraightFlush(inner::StraightFlush),
    Fives(inner::Fives),
}

#[derive(Debug)]
pub struct TryFromMediatorError(pub Box<dyn error::Error>);

#[derive(Debug)] // TEMPORARY
pub struct InvalidStraightError<E>(pub srank::TryFromRankError, pub E);
