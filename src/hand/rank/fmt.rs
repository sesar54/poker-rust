use super::{mediator, srank::SRank, Rank};
use std::fmt;

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Rank::High(..) => write!(f, "High card"),
            Rank::Pair(..) => write!(f, "Pair"),
            Rank::TwoPair(..) => write!(f, "Two pairs"),
            Rank::Trips(..) => write!(f, "Three of a kind"),
            Rank::Straight(..) => write!(f, "Straight"),
            Rank::Flush(..) => write!(f, "Flush"),
            Rank::House(..) => write!(f, "Full house"),
            Rank::Quads(..) => write!(f, "Four of a kind"),
            Rank::StraightFlush(straight_flush) => match straight_flush.srank {
                SRank::Ten => write!(f, "Royal flush"),
                _ => write!(f, "Straight flush"),
            },
            Rank::Fives(..) => write!(f, "Five of a kind"),
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
