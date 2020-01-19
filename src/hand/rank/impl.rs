use super::{inner, medier, ConvertRankError, Rank};

use std::convert::{From, TryFrom};
use std::fmt;

macro_rules! derive_try_from {
    ($rank:tt) => {
        impl TryFrom<medier::$rank> for Rank {
            type Error = ConvertRankError<()>; // TODO

            fn try_from(cards: medier::$rank) -> Result<Self, Self::Error> {
                let raw_rank = inner::$rank::from(cards);

                if cards == medier::$rank::from(raw_rank) {
                    Ok(Rank::$rank(raw_rank))
                } else {
                    Err(ConvertRankError(()))
                }
            }
        }
    };
}

/// Constructors
impl From<medier::High> for Rank {
    fn from(card: medier::High) -> Self {
        Rank::High(card.into())
    }
}

derive_try_from!(Pair);
derive_try_from!(TwoPair);
derive_try_from!(Trips);

impl TryFrom<medier::Straight> for Rank {
    type Error = ConvertRankError<String>;
    fn try_from(cards: medier::Straight) -> Result<Self, Self::Error> {
        Ok(Self::Straight(
            inner::Straight::try_from(cards).map_err(ConvertRankError)?,
        ))
    }
}

derive_try_from!(Flush);
derive_try_from!(House);
derive_try_from!(Quads);

impl TryFrom<medier::StraightFlush> for Rank {
    type Error = ConvertRankError<String>;
    fn try_from(cards: medier::StraightFlush) -> Result<Self, Self::Error> {
        Ok(Self::StraightFlush(
            inner::StraightFlush::try_from(cards).map_err(ConvertRankError)?,
        ))
    }
}

derive_try_from!(Fives);

impl Rank {
    
    /// Will return number of cards in Rank. This is constant.
    /// * High            => 1
    /// * Pair            => 2
    /// * TwoPair         => 4
    /// * Trips           => 3
    /// * Straight        => 5
    /// * Flush           => 5
    /// * House           => 5
    /// * Quads           => 4
    /// * StraightFlush   => 5
    /// * Fives           => 5
    pub fn len(&self) -> usize {
        match &self {
            High => 1,
            Pair => 2,
            TwoPair => 4,
            Trips => 3,
            Straight => 5,
            Flush => 5,
            House => 5,
            Quads => 4,
            StraightFlush => 5,
            Fives => 5,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            High => write!(f, "High card"),
            Pair => write!(f, "Pair"),
            TwoPair => write!(f, "Two pairs"),
            Trips => write!(f, "Three of a kind"),
            Straight => write!(f, "Straight"),
            Flush => write!(f, "Flush"),
            House => write!(f, "Full house"),
            Quads => write!(f, "Four of a kind"),
            /*
            StraightFlush(straight_flush) => match straight_flush.srank {
                SRank::Ten => write!(f, "Royal flush"),
                _ => write!(f, "Straight flush"),
            },*/
            Fives => write!(f, "Five of a kind"),
        }
    }
}

impl Rank {
    /// Consumes the rank
    pub fn to_boxed_slice(self) -> Box<[medier::RCard]> {
        use medier::*;
        match self {
            Self::High(high) => Box::new([High::from(high).0]),
            Self::Pair(pair) => Box::new(Pair::from(pair).0),
            Self::TwoPair(two_pair) => {
                let TwoPair(arr0, arr1) = TwoPair::from(two_pair);
                arr0.0.iter().chain(arr1.0.iter()).map(|&c| c).collect()
            }
            Self::Trips(trips) => Box::new(Trips::from(trips).0),
            Self::Straight(straight) => Box::new(Straight::from(straight).0),
            Self::Flush(flush) => Box::new(Flush::from(flush).0),
            Self::House(house) => {
                let House { trips, pair } = House::from(house);
                trips.0.iter().chain(pair.0.iter()).map(|&c| c).collect()
            }
            Self::Quads(quads) => Box::new(Quads::from(quads).0),
            Self::StraightFlush(sf) => Box::new(StraightFlush::from(sf).0),
            Self::Fives(fives) => Box::new(Fives::from(fives).0),
        }
    }
}
