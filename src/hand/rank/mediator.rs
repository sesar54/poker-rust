use super::inner::{self, SRank};
use crate::card::{Card, Rank, Suit};
use crate::r#trait::Circular;

use seq_macro::seq;
use std::convert::TryFrom;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct High(pub Card);
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Pair(pub [Card; 2]);
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct TwoPair(pub Pair, pub Pair);
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Trips(pub [Card; 3]);
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Straight(pub [Card; 5]);
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Flush(pub [Card; 5]);
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct House {
    pub trips: Trips,
    pub pair: Pair,
}
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Quads(pub [Card; 4]);
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct StraightFlush(pub [Card; 5]);
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Fives(pub [Card; 5]);

impl From<inner::High> for High {
    fn from(high: inner::High) -> Self {
        Self(Card {
            rank: high.rank,
            suit: high.suit,
        })
    }
}

impl From<inner::Pair> for Pair {
    fn from(pair: inner::Pair) -> Self {
        Self(seq!(n in 0..2{[#(Card{rank: pair.crank, suit: pair.suits[n]},)*]}))
    }
}

impl From<inner::TwoPair> for TwoPair {
    fn from(twoPair: inner::TwoPair) -> Self {
        Self(twoPair.pair0.into(), twoPair.pair1.into())
    }
}

impl From<inner::Trips> for Trips {
    fn from(trips: inner::Trips) -> Self {
        Self(seq!(n in 0..3{[#(Card{rank: trips.crank,  suit: trips.suits[n]},)*]}))
    }
}

impl From<inner::Straight> for Straight {
    fn from(straight: inner::Straight) -> Self {
        Self(
            seq!(n in 0..5{[#(Card{rank: straight.srank.step(n).into(), suit:  straight.suits[n]},)*]}),
        )
    }
}

impl From<inner::Flush> for Flush {
    fn from(flush: inner::Flush) -> Self {
        Self(seq!(n in 0..5{[#(Card{rank: flush.ranks[n], suit:  flush.csuit},)*]}))
    }
}

impl From<inner::House> for House {
    fn from(house: inner::House) -> Self {
        Self {
            trips: house.trips.into(),
            pair: house.pair.into(),
        }
    }
}

impl From<inner::Quads> for Quads {
    fn from(quads: inner::Quads) -> Self {
        Self(seq!(n in 0..4{[#(Card{rank: quads.crank,  suit: quads.suits[n]},)*]}))
    }
}

impl From<inner::StraightFlush> for StraightFlush {
    fn from(sf: inner::StraightFlush) -> Self {
        Self(seq!(n in 0..5{[#(Card{rank: sf.srank.step(n).into(), suit: sf.csuit},)*]}))
    }
}

impl From<inner::Fives> for Fives {
    fn from(fives: inner::Fives) -> Self {
        Self(seq!(n in 0..5{[#(Card{rank: fives.crank, suit:  fives.suits[n]},)*]}))
    }
}
