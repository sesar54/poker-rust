use super::{inner::srank, mediator, Rank, TryFromMediatorError};
use std::fmt;

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

macro_rules! display_mediator {
    ($type:ident) => {
        impl fmt::Display for mediator::$type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
    ($type0:ident, $($type1:ident),+) => {
        display_mediator!($type0);
        $(display_mediator!($type1);)*
    };
}

display_mediator!(
    High,
    Pair,
    TwoPair,
    Trips,
    Straight,
    Flush,
    House,
    Quads,
    StraightFlush,
    Fives
);

impl fmt::Display for srank::TryFromRankError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rank: {:?}, lacks mapping to any SRank", self.0)
    }
}

impl fmt::Display for TryFromMediatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Could not convert from Mediator to Rank. Reason: {}",
            self.0
        )
    }
}
