use super::Error;
use crate::card::{self, Card};
use mimpl::mimpl;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::convert::{From, TryFrom};
use std::error;
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

mimpl!(Default; SRank, || SRank::Ace);

impl From<SRank> for card::Rank {
    fn from(srank: SRank) -> Self {
        match srank {
            SRank::Ace => card::Rank::Ace,
            SRank::Two => card::Rank::Two,
            SRank::Three => card::Rank::Three,
            SRank::Four => card::Rank::Four,
            SRank::Five => card::Rank::Five,
            SRank::Six => card::Rank::Six,
            SRank::Seven => card::Rank::Seven,
            SRank::Eight => card::Rank::Eight,
            SRank::Nine => card::Rank::Nine,
            SRank::Ten => card::Rank::Ten,
        }
    }
}

impl TryFrom<Card> for SRank {
    type Error = Error;

    fn try_from(card: Card) -> Result<Self, Self::Error> {
        match card.rank {
            card::Rank::Ace => Ok(SRank::Ace),
            card::Rank::Two => Ok(SRank::Two),
            card::Rank::Three => Ok(SRank::Three),
            card::Rank::Four => Ok(SRank::Four),
            card::Rank::Five => Ok(SRank::Five),
            card::Rank::Six => Ok(SRank::Six),
            card::Rank::Seven => Ok(SRank::Seven),
            card::Rank::Eight => Ok(SRank::Eight),
            card::Rank::Nine => Ok(SRank::Nine),
            card::Rank::Ten => Ok(SRank::Ten),
            rank => Err(Error::TryFromRank(box Error::InvalidSRank(card, rank))),
        }
    }
}

impl TryFrom<card::Rank> for SRank {
    type Error = ();

    fn try_from(rank: card::Rank) -> Result<Self, Self::Error> {
        match rank {
            card::Rank::Ace => Ok(SRank::Ace),
            card::Rank::Two => Ok(SRank::Two),
            card::Rank::Three => Ok(SRank::Three),
            card::Rank::Four => Ok(SRank::Four),
            card::Rank::Five => Ok(SRank::Five),
            card::Rank::Six => Ok(SRank::Six),
            card::Rank::Seven => Ok(SRank::Seven),
            card::Rank::Eight => Ok(SRank::Eight),
            card::Rank::Nine => Ok(SRank::Nine),
            card::Rank::Ten => Ok(SRank::Ten),
            _ => Err(()),
        }
    }
}

impl Iterator for SRank {
    type Item = SRank;
    fn next(&mut self) -> Option<Self::Item> {
        Self::from_u32(*self as u32 + 1 % 10)
    }
}
