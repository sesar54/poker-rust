use super::{inner, mediator as med, Error as E, Rank};
use crate::card::Card;
use std::convert::{From, TryFrom};
use std::fmt;

impl Rank {
    fn to_vec(self) -> Vec<Card> {
        match self {
            Self::High(high) => med::High::from(high).to_vec(),
            Self::Pair(pair) => med::Pair::from(pair).to_vec(),
            Self::TwoPair(two_pair) => med::TwoPair::from(two_pair).to_vec(),
            Self::Trips(trips) => med::Trips::from(trips).to_vec(),
            Self::Straight(straight) => med::Straight::from(straight).to_vec(),
            Self::Flush(flush) => med::Flush::from(flush).to_vec(),
            Self::House(house) => med::House::from(house).to_vec(),
            Self::Quads(quads) => med::Quads::from(quads).to_vec(),
            Self::StraightFlush(sf) => med::StraightFlush::from(sf).to_vec(),
            Self::Fives(fives) => med::Fives::from(fives).to_vec(),
        }
    }
}

macro_rules! impl_TryFrom_mediator {
    ($type:ident) => {
        impl TryFrom<med::$type> for Rank {
            type Error = E;

            fn try_from(cards: med::$type) -> Result<Self, Self::Error> {
                let inner_rank = inner::$type::from(cards);

                match med::$type::from(inner_rank) {

                    built if cards == built => Ok(Rank::$type(inner_rank)),
                    built => Err(E::TryFromMediator(box E::BuildForgery{
                        original: cards.to_vec().into_boxed_slice(),
                        forged:  built.to_vec().into_boxed_slice(),
                        components: format!("{:?}", inner_rank),
                    })),
                }
            }
        }
    };
    ($type0:ident, $($type1:ident),+ $(,)*) => {
        impl_TryFrom_mediator!($type0);
        $(impl_TryFrom_mediator!($type1);)*
    };
}

#[rustfmt::skip]
impl_TryFrom_mediator!(
    // High
    Pair,
    TwoPair,
    Trips, 
    // Straight 
    Flush, 
    House, 
    Quads, 
    // StraightFlush
    Fives,
);

impl From<med::High> for Rank {
    fn from(card: med::High) -> Self {
        Rank::High(card.into())
    }
}

impl TryFrom<med::Straight> for Rank {
    type Error = E;
    fn try_from(cards: med::Straight) -> Result<Self, Self::Error> {
        inner::Straight::try_from(cards)
            .map(Self::Straight)
            .map_err(|err| E::TryFromMediator(box E::InvalidStraight(box cards.0, box err)))
    }
}

impl TryFrom<med::StraightFlush> for Rank {
    type Error = E;
    fn try_from(cards: med::StraightFlush) -> Result<Self, Self::Error> {
        inner::StraightFlush::try_from(cards)
            .map(Self::StraightFlush)
            .map_err(|err| E::TryFromMediator(box E::InvalidStraight(box cards.0, box err)))
    }
}

macro_rules! try_macro {
    ($type:ident, $fn_name:ident) => {
        impl Rank {
            pub fn $fn_name(cards: &[Card]) -> Result<Self, super::Error> {
                med::$type::try_from(cards).map(Rank::try_from)?
            }
        }
    };
    ($type0:ident, $fn_name0:ident; $($type1:ident, $fn_name1:ident;)* $(;)*) => {
        try_macro!($type0, $fn_name0);
        $(try_macro!($type1, $fn_name1);)*
    };
}

try_macro!(
    Pair, try_from_pair;
    Trips, try_from_trips;
    Straight, try_from_straight;
    Flush, try_from_flush;
    Quads, try_from_quads;
    StraightFlush, try_from_straight_flush;
    Fives, try_from_fives;
);

impl Rank {
    pub fn from_high(card: &Card) -> Self {
        Rank::from(med::High([*card]))
    }

    pub fn try_from_two_pair(pair0: [Card; 2], pair1: [Card; 2]) -> Result<Self, super::Error> {
        Rank::try_from(med::TwoPair(med::Pair(pair0), med::Pair(pair1)))
    }

    pub fn try_from_house(trips: [Card; 3], pair: [Card; 2]) -> Result<Self, super::Error> {
        Rank::try_from(med::House {
            trips: med::Trips(trips),
            pair: med::Pair(pair),
        })
    }
}
