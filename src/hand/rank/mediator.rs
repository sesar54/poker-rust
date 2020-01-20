use super::inner::{self, SRank};
use crate::card::{Rank, Suit};
use crate::r#trait::Circular;

use seq_macro::seq;
use std::convert::TryFrom;

/// # RRank (Real Rank)
/// Consist of all real ranks which can be used for comparison, (All exept for Wild)
/// Order has changed from normal card::Rank.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RRank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Default for RRank {
    fn default() -> Self {
        Self::Two
    }
}

impl From<RRank> for Rank {
    fn from(rank: RRank) -> Rank {
        match rank {
            Ace => Rank::Ace,
            Two => Rank::Two,
            Three => Rank::Three,
            Four => Rank::Four,
            Five => Rank::Five,
            Six => Rank::Six,
            Seven => Rank::Seven,
            Eight => Rank::Eight,
            Nine => Rank::Nine,
            Ten => Rank::Ten,
            Jack => Rank::Jack,
            Queen => Rank::Queen,
            King => Rank::King,
        }
    }
}

impl RRank {
    pub fn into_rank(self) -> Rank {
        self.into()
    }
}

impl TryFrom<Rank> for RRank {
    type Error = &'static str;

    fn try_from(rank: Rank) -> Result<Self, Self::Error> {
        match rank {
            Wildcard => Err("Wildcards are unambigious"),
            Ace => Ok(RRank::Ace),
            Two => Ok(RRank::Two),
            Two => Ok(RRank::Two),
            Three => Ok(RRank::Three),
            Four => Ok(RRank::Four),
            Five => Ok(RRank::Five),
            Six => Ok(RRank::Six),
            Seven => Ok(RRank::Seven),
            Eight => Ok(RRank::Eight),
            Nine => Ok(RRank::Nine),
            Ten => Ok(RRank::Ten),
            Jack => Ok(RRank::Jack),
            Queen => Ok(RRank::Queen),
            King => Ok(RRank::King),
        }
    }
}

impl From<SRank> for RRank {
    fn from(rank: SRank) -> Self {
        match rank {
            Ace => Self::Ace,
            Two => Self::Two,
            Two => Self::Two,
            Three => Self::Three,
            Four => Self::Four,
            Five => Self::Five,
            Six => Self::Six,
            Seven => Self::Seven,
            Eight => Self::Eight,
            Nine => Self::Nine,
            Ten => Self::Ten,
        }
    }
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct RCard {
    pub rank: RRank,
    pub suit: Suit,
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct High(pub RCard);
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Pair(pub [RCard; 2]);
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct TwoPair(pub Pair, pub Pair);
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Trips(pub [RCard; 3]);
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Straight(pub [RCard; 5]);
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Flush(pub [RCard; 5]);
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct House {
    pub trips: Trips,
    pub pair: Pair,
}
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Quads(pub [RCard; 4]);
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct StraightFlush(pub [RCard; 5]);
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Fives(pub [RCard; 5]);

impl From<inner::High> for High {
    fn from(high: inner::High) -> Self {
        Self(RCard{rank: high.rank, suit: high.suit})
    }
}

impl From<inner::Pair> for Pair {
    fn from(pair: inner::Pair) -> Self {
        Self(seq!(n in 0..2{[#(RCard{rank: pair.crank, suit: pair.suits[n]},)*]}))
    }
}

impl From<inner::TwoPair> for TwoPair {
    fn from(twoPair: inner::TwoPair) -> Self {
        Self(twoPair.pair0.into(), twoPair.pair1.into())
    }
}

impl From<inner::Trips> for Trips {
    fn from(trips: inner::Trips) -> Self {
        Self(seq!(n in 0..3{[#(RCard{rank: trips.crank,  suit: trips.suits[n]},)*]}))
    }
}

impl From<inner::Straight> for Straight {
    fn from(straight: inner::Straight) -> Self {
        Self(seq!(n in 0..5{[#(RCard{rank: straight.srank.step(n).into(), suit:  straight.suits[n]},)*]}))
    }
}

impl From<inner::Flush> for Flush {
    fn from(flush: inner::Flush) -> Self {
        Self(seq!(n in 0..5{[#(RCard{rank: flush.ranks[n], suit:  flush.csuit},)*]}))
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
        Self(seq!(n in 0..4{[#(RCard{rank: quads.crank,  suit: quads.suits[n]},)*]}))
    }
}

impl From<inner::StraightFlush> for StraightFlush {
    fn from(sf: inner::StraightFlush) -> Self {
        Self(seq!(n in 0..5{[#(RCard{rank: sf.srank.step(n).into(),  suit: sf.csuit},)*]}))
    }
}

impl From<inner::Fives> for Fives {
    fn from(fives: inner::Fives) -> Self {
        Self(seq!(n in 0..5{[#(RCard{rank: fives.crank, suit:  fives.suits[n]},)*]}))
    }
}
