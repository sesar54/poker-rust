use super::mediator;
use crate::card::{Rank, Suit};

use mimpl::mimpl;
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

fn test() {
    SRank::default();
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
    type Error = String;

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
            rank => Err(format!("A Straight can't start on rank {:?}.", rank)),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct High {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pair {
    pub crank: Rank,
    pub suits: [Suit; 2],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TwoPair {
    pub pair0: Pair,
    pub pair1: Pair,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Trips {
    pub crank: Rank,
    pub suits: [Suit; 3],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Straight {
    pub srank: SRank,
    pub suits: [Suit; 5],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Flush {
    pub ranks: [Rank; 5],
    pub csuit: Suit,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct House {
    pub trips: Trips,
    pub pair: Pair,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Quads {
    pub crank: Rank,
    pub suits: [Suit; 4],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StraightFlush {
    pub srank: SRank,
    pub csuit: Suit,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Fives {
    pub crank: Rank,
    pub suits: [Suit; 5],
}

mimpl!(From; mediator::High, High, |card: mediator::High| 
    High {rank: card.0.rank, suit: card.0.suit}
);
mimpl!(From; mediator::Pair, Pair, |pair: mediator::Pair| 
    Pair {crank: pair.0[0].rank, suits: seq!(n in 0..2{[#(pair.0[n].suit,)*]})}
);
mimpl!(From; mediator::TwoPair, TwoPair, |cards: mediator::TwoPair| 
    TwoPair {pair0: Pair::from(cards.0), pair1: Pair::from(cards.1)}
);
mimpl!(From; mediator::Trips, Trips, |cards: mediator::Trips| 
    Trips {crank: cards.0[0].rank, suits: seq!(n in 0..3{[#(cards.0[n].suit,)*]})}
);
mimpl!(TryFrom; mediator::Straight, Straight, String, |cards: mediator::Straight| 
    Ok(Straight {srank: SRank::try_from(cards.0[0].rank)?,suits: seq!(n in 0..5{[#(cards.0[n].suit,)*]})})
);
mimpl!(From; mediator::Flush, Flush, |cards: mediator::Flush| 
    Flush {csuit: cards.0[0].suit, ranks: seq!(n in 0..5{[#(cards.0[n].rank,)*]})}
);
mimpl!(From; mediator::House, House, |house: mediator::House|
    House {trips: Trips::from(house.trips), pair: Pair::from(house.pair)}
);
mimpl!(From; mediator::Quads, Quads, |cards: mediator::Quads|
    Quads {crank: cards.0[0].rank,suits: seq!(n in 0..4{[#(cards.0[n].suit,)*]})}  
);
mimpl!(TryFrom; mediator::StraightFlush, StraightFlush, String, |cards: mediator::StraightFlush|
    Ok(StraightFlush {srank: SRank::try_from(cards.0[0].rank)?, csuit: cards.0[0].suit})
);
mimpl!(From; mediator::Fives, Fives, |cards: mediator::Fives|
    Fives {crank: cards.0[0].rank, suits: seq!(n in 0..5{[#(cards.0[n].suit,)*]})}
);

mimpl!(PartialOrd; High, USE_CMP);
mimpl!(PartialOrd; Pair, USE_CMP);
mimpl!(PartialOrd; TwoPair, USE_CMP);
mimpl!(PartialOrd; Trips, USE_CMP);
mimpl!(PartialOrd; Straight, USE_CMP);
mimpl!(PartialOrd; Flush, USE_CMP);
mimpl!(PartialOrd; House, USE_CMP);
mimpl!(PartialOrd; Quads, USE_CMP);
mimpl!(PartialOrd; StraightFlush, USE_CMP);
mimpl!(PartialOrd; Fives, USE_CMP);

mimpl!(Ord; High, |this: &High, that: &High| this.rank.cmp(&that.rank));
mimpl!(Ord; Pair, |this: &Pair, that: &Pair| this.crank.cmp(&that.crank));
mimpl!(Ord; TwoPair, |this: &TwoPair, that: &TwoPair| {
    let order = this.pair0.cmp(&that.pair0);
    if order == Ordering::Equal {
        this.pair1.cmp(&that.pair1)
    } else {
        order
    }
});
mimpl!(Ord; Trips, |this: &Trips, that: &Trips| this.crank.cmp(&that.crank));
mimpl!(Ord; Straight, |this: &Straight, that: &Straight| this.srank.cmp(&that.srank));
mimpl!(Ord; Flush, |this: &Flush, that: &Flush| this.ranks[0].cmp(&that.ranks[0]));
mimpl!(Ord; House, |this: &House, that: &House| {
    let order = this.trips.cmp(&that.trips);
    if order == Ordering::Equal {
        this.pair.cmp(&that.pair)
    } else {
        order
    }
});
mimpl!(Ord; Quads, |this: &Quads, that: &Quads| this.crank.cmp(&that.crank));
mimpl!(Ord; StraightFlush, |this: &StraightFlush, that: &StraightFlush| this.srank.cmp(&that.srank));
mimpl!(Ord; Fives, |this: &Fives, that: &Fives| this.crank.cmp(&that.crank));
