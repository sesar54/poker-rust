use super::{ConvertRankError, Rank};
use std::error::Error;
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

impl<E> Error for ConvertRankError<E>
where
    E: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

impl<E> fmt::Display for ConvertRankError<E>
where
    E: Error,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(TODO: Error at hand/rank/mod.rs, {})", self.0)
    }
}
