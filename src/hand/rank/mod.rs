use crate::hand::CardRef;

mod r#impl;
/**
 * A Rank consist of a number of cards in a specific configuration. They are
 * sorted by the lowest value first and greatest value last (actually in what
 * order they are written).
 */
#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Rank(RankInner);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum RankInner {
    High(r#type::High),
    Pair(r#type::Pair),
    TwoPair([CardRef; 2], [CardRef; 2]),
    Trips(r#type::Trips),
    Straight(r#type::Straight),
    Flush(r#type::Flush),
    House([CardRef; 3], [CardRef; 2]),
    Quads(r#type::Quads),
    StraightFlush(r#type::StraightFlush),
    Fives(r#type::Fives),
}

#[derive(Debug)]
pub enum Error {
    Explained(String),
    Invalid(Rank),
    Unsorted(Rank),
}

pub mod r#type {
    use super::CardRef;

    pub type High = [CardRef; 1];
    pub type Pair = [CardRef; 2];
    pub type TwoPair = (Pair, Pair);
    pub type Trips = [CardRef; 3];
    pub type Straight = [CardRef; 5];
    pub type Flush = [CardRef; 5];
    pub type House = (Trips, Pair);
    pub type Quads = [CardRef; 4];
    pub type StraightFlush = [CardRef; 5];
    pub type Fives = [CardRef; 5];
}
