use super::medier::{self, RRank};
use crate::card::Suit;
use mimpl::*;
use num_traits::FromPrimitive;
use seq_macro::seq;
use std::cmp::Ordering;
use std::convert::{From, Into, TryFrom};
use variant_count::VariantCount;

/// Consist of Ranks which can be the first card in a straight.
/// Example:
///     Ace can be both first and last Rank in a straight.
///         [Ace, Two, Three, Four, Five]
///         [Ten, Jack, Queen, King, Ace]     
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, PartialOrd, Ord, VariantCount)]
pub enum SRank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

impl crate::r#trait::Circular<isize> for SRank {
    /// Cycles over the elements of `SRank`, starting at `self`.
    /// Returns the n'th neighbor.
    /// # Examples
    /// ```rust
    ///
    /// ```
    fn step(self, i: isize) -> Self {
        FromPrimitive::from_isize(self as isize + i % SRank::VARIANT_COUNT as isize).unwrap()
    }
}

impl Default for SRank {
    fn default() -> Self {
        Self::Ace
    }
}

impl TryFrom<RRank> for SRank {
    type Error = String;

    fn try_from(rank: RRank) -> Result<Self, Self::Error> {
        match rank {
            Wildcard => Err("Wildcards are unambigious".into()),
            Ace => Ok(SRank::Ace),
            Two => Ok(SRank::Two),
            Two => Ok(SRank::Two),
            Three => Ok(SRank::Three),
            Four => Ok(SRank::Four),
            Five => Ok(SRank::Five),
            Six => Ok(SRank::Six),
            Seven => Ok(SRank::Seven),
            Eight => Ok(SRank::Eight),
            Nine => Ok(SRank::Nine),
            Ten => Ok(SRank::Ten),
            rank => Err(format!("A Straight can't start on rank {:?}.", rank)),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct High {
    pub rank: RRank,
    pub suit: Suit,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pair {
    pub crank: RRank,
    pub suits: [Suit; 2],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TwoPair {
    pub pair0: Pair,
    pub pair1: Pair,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Trips {
    pub crank: RRank,
    pub suits: [Suit; 3],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Straight {
    pub srank: SRank,
    pub suits: [Suit; 5],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Flush {
    pub ranks: [RRank; 5],
    pub csuit: Suit,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct House {
    pub trips: Trips,
    pub pair: Pair,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Quads {
    pub crank: RRank,
    pub suits: [Suit; 4],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StraightFlush {
    pub srank: SRank,
    pub csuit: Suit,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Fives {
    pub crank: RRank,
    pub suits: [Suit; 5],
}

impl From<medier::High> for High {
    fn from(card: medier::High) -> Self {
        High {
            rank: card.0.rank,
            suit: card.0.suit,
        }
    }
}

impl From<medier::Pair> for Pair {
    fn from(cards: medier::Pair) -> Self {
        Pair {
            crank: cards.0[0].rank,
            suits: seq!(n in 0..2{[#(cards.0[n].suit,)*]}),
        }
    }
}

impl From<medier::TwoPair> for TwoPair {
    fn from(cards: medier::TwoPair) -> Self {
        TwoPair {
            pair0: Pair::from(cards.0),
            pair1: Pair::from(cards.1),
        }
    }
}

///
impl From<medier::Trips> for Trips {
    fn from(cards: medier::Trips) -> Self {
        Trips {
            crank: cards.0[0].rank,
            suits: seq!(n in 0..3{[#(cards.0[n].suit,)*]}),
        }
    }
}
///
impl TryFrom<medier::Straight> for Straight {
    type Error = String;

    fn try_from(cards: medier::Straight) -> Result<Self, Self::Error> {
        Ok(Straight {
            srank: SRank::try_from(cards.0[0].rank)?,
            suits: seq!(n in 0..5{[#(cards.0[n].suit,)*]}),
        })
    }
}

///
impl From<medier::Flush> for Flush {
    fn from(cards: medier::Flush) -> Self {
        Flush {
            csuit: cards.0[0].suit,
            ranks: seq!(n in 0..5{[#(cards.0[n].rank,)*]}),
        }
    }
}

impl From<medier::House> for House {
    fn from(house: medier::House) -> Self {
        House {
            trips: Trips::from(house.trips),
            pair: Pair::from(house.pair),
        }
    }
}

impl From<medier::Quads> for Quads {
    fn from(cards: medier::Quads) -> Self {
        Quads {
            crank: cards.0[0].rank,
            suits: seq!(n in 0..4{[#(cards.0[n].suit,)*]}),
        }
    }
}

impl TryFrom<medier::StraightFlush> for StraightFlush {
    type Error = String;
    fn try_from(cards: medier::StraightFlush) -> Result<Self, Self::Error> {
        Ok(StraightFlush {
            srank: SRank::try_from(cards.0[0].rank)?,
            csuit: cards.0[0].suit,
        })
    }
}

impl From<medier::Fives> for Fives {
    fn from(cards: medier::Fives) -> Self {
        Fives {
            crank: cards.0[0].rank,
            suits: seq!(n in 0..5{[#(cards.0[n].suit,)*]}),
        }
    }
}

mimpl!(PartialOrd; High, USE_CMP);
mimpl!(Ord; High, Box::new(|this: &High, that: &High| this.rank.cmp(&that.rank)));

mimpl!(PartialOrd; Pair, USE_CMP);
mimpl!(Ord; Pair, Box::new(|this: &Pair, that: &Pair| this.crank.cmp(&that.crank)));

mimpl!(PartialOrd; TwoPair, USE_CMP);
mimpl!(Ord; TwoPair, Box::new(|this: &TwoPair, that: &TwoPair| {
    let order = this.pair0.cmp(&that.pair0);
    if order == Ordering::Equal {
        this.pair1.cmp(&that.pair1)
    } else {
        order
    }
}));

mimpl!(PartialOrd; Trips, USE_CMP);
mimpl!(Ord; Trips, Box::new(|this: &Trips, that: &Trips| this.crank.cmp(&that.crank)));

mimpl!(PartialOrd; Straight, USE_CMP);
mimpl!(Ord; Straight, Box::new(|this: &Straight, that: &Straight| this.srank.cmp(&that.srank)));

mimpl!(PartialOrd; Flush, USE_CMP);
mimpl!(Ord; Flush, Box::new(|this: &Flush, that: &Flush| this.ranks[0].cmp(&that.ranks[0])));

mimpl!(PartialOrd; House, USE_CMP);
mimpl!(Ord; House, Box::new(|this: &House, that: &House| {
    let order = this.trips.cmp(&that.trips);
    if order == Ordering::Equal {
        this.pair.cmp(&that.pair)
    } else {
        order
    }
}));

mimpl!(PartialOrd; Quads, USE_CMP);
mimpl!(Ord; Quads, Box::new(|this: &Quads, that: &Quads| this.crank.cmp(&that.crank)));

mimpl!(PartialOrd; StraightFlush, USE_CMP);
mimpl!(Ord; StraightFlush, Box::new(|this: &StraightFlush, that: &StraightFlush| this.srank.cmp(&that.srank)));

mimpl!(PartialOrd; Fives, USE_CMP);
mimpl!(Ord; Fives, Box::new(|this: &Fives, that: &Fives| this.crank.cmp(&that.crank)));
