mod fmt;
mod r#impl;
mod inner;
pub mod mediator;
mod srank;

pub use super::Error;

// Note: Make Invalid States Unpresentable
/// This is
/// TODO: Document
///
/// Only exposed interface. Implements try from
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

#[test]
fn rank_from() {
    use crate::card::Card;
    use mediator as med;

    println!("{:?}", Rank::pair_try_from(&[Card::random(), Card::random()]))

}
