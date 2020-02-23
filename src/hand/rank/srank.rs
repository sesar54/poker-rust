use crate::card::Rank;
use mimpl::mimpl;
use num_traits::FromPrimitive;
use std::convert::{From, TryFrom};
use std::error;
use variant_count::VariantCount;

#[derive(Debug)] // TEMPORARY
pub struct TryFromRankError(pub Rank);

impl error::Error for TryFromRankError {} // TODO?

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

mimpl!(Default; SRank, || SRank::Ace);

impl From<SRank> for Rank {
    fn from(srank: SRank) -> Self {
        match srank {
            SRank::Ace => Rank::Ace,
            SRank::Two => Rank::Two,
            SRank::Three => Rank::Three,
            SRank::Four => Rank::Four,
            SRank::Five => Rank::Five,
            SRank::Six => Rank::Six,
            SRank::Seven => Rank::Seven,
            SRank::Eight => Rank::Eight,
            SRank::Nine => Rank::Nine,
            SRank::Ten => Rank::Ten,
        }
    }
}

impl TryFrom<Rank> for SRank {
    type Error = TryFromRankError;

    fn try_from(rank: Rank) -> Result<Self, Self::Error> {
        match rank {
            Rank::Ace => Ok(SRank::Ace),
            Rank::Two => Ok(SRank::Two),
            Rank::Three => Ok(SRank::Three),
            Rank::Four => Ok(SRank::Four),
            Rank::Five => Ok(SRank::Five),
            Rank::Six => Ok(SRank::Six),
            Rank::Seven => Ok(SRank::Seven),
            Rank::Eight => Ok(SRank::Eight),
            Rank::Nine => Ok(SRank::Nine),
            Rank::Ten => Ok(SRank::Ten),
            rank => Err(TryFromRankError(rank)),
        }
    }
}

impl Iterator for SRank {
    type Item = SRank;
    fn next(&mut self) -> Option<Self::Item> {
        Self::from_u32(*self as u32 + 1 % 10)
    }
}
