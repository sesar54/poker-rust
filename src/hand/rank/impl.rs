use super::{inner, mediator as med, Error as E, Rank};
use crate::card::Card;
use std::convert::{From, TryFrom};
use std::fmt;

/* -------------------------------------------------------------------------- */

macro_rules! impl_try_from_mediator {
    ($type:ident) => {
        impl TryFrom<med::$type> for Rank {
            type Error = E;

            fn try_from(cards: med::$type) -> Result<Self, Self::Error> {
                let inner_rank = inner::$type::from(cards);

                println!("{}", cards);
                println!("{:?}",cards);
                println!("{:?}", cards.to_vec().into_boxed_slice());

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
        impl_try_from_mediator!($type0);
        $(impl_try_from_mediator!($type1);)*
    };
}

#[rustfmt::skip]
impl_try_from_mediator!(
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

/* -------------------------------------------------------------------------- */

macro_rules! wrapper_mediator_try_from {
    ($type:ident, $fn_name:ident) => {
        impl Rank {
            pub fn $fn_name(cards: &[Card]) -> Result<Self, super::Error> {
                med::$type::try_from(cards).map(Rank::try_from)?
            }
        }
    };
    ($type0:ident, $fn_name0:ident; $($type1:ident, $fn_name1:ident;)* $(;)*) => {
        wrapper_mediator_try_from!($type0, $fn_name0);
        $(wrapper_mediator_try_from!($type1, $fn_name1);)*
    };
}

// Complex try_from (or try) is implemented in the block below.
wrapper_mediator_try_from!(
    //High, high_from; // Huehuehuehue
    Pair, pair_try_from;
    Trips, trips_try_from;
    // TwoPair, two_pair_try_from;
    Straight, straight_try_from;
    Flush, flush_try_from;
    Quads, quads_try_from;
    // House, house_try_from;
    StraightFlush, straight_flush_try_from;
    Fives, fives_try_from;
);


impl Rank {
    pub fn high_from(card: &Card) -> Self {
        Rank::from(med::High([*card]))
    }

    pub fn two_pair_try_from(pair0: &[Card], pair1: &[Card]) -> Result<Self, super::Error> {
        match (med::Pair::try_from(pair0), med::Pair::try_from(pair1)) {
            (Ok(pair0), Ok(pair1)) => Ok((pair0, pair1)),
            (Err(e0), Err(e1)) => Err(E::PairError([box e0, box e1])),
            (pair0, pair1) => {
                if let Err(pair0) = pair0 {
                    Err(pair0)
                } else if let Err(pair1) = pair1 {
                    Err(pair1)
                } else {
                    unreachable!();
                }
            }
        }
        .map_or_else(Err, |(p0, p1)| Rank::try_from(med::TwoPair(p0, p1)))
    }

    pub fn house_try_from(trips: &[Card], pair: &[Card]) -> Result<Self, super::Error> {
        match (med::Trips::try_from(trips), med::Pair::try_from(pair)) {
            (Ok(trips), Ok(pair)) => Ok((trips, pair)),
            (Err(e0), Err(e1)) => Err(E::PairError([box e0, box e1])),
            (trips, pair) => {
                if let Err(trips) = trips {
                    Err(trips)
                } else if let Err(pair) = pair {
                    Err(pair)
                } else {
                    unreachable!();
                }
            }
        }
        .map_or_else(Err, |(trips, pair)| {
            Rank::try_from(med::House { trips, pair })
        })
    }
}

/* -------------------------------------------------------------------------- */

impl Rank {
    pub fn to_vec(self) -> Vec<Card> {
        match self {
            Self::High(inner)           => med::High::from(inner).to_vec(),
            Self::Pair(inner)           => med::Pair::from(inner).to_vec(),
            Self::TwoPair(inner)        => med::TwoPair::from(inner).to_vec(),
            Self::Trips(inner)          => med::Trips::from(inner).to_vec(),
            Self::Straight(inner)       => med::Straight::from(inner).to_vec(),
            Self::Flush(inner)          => med::Flush::from(inner).to_vec(),
            Self::House(inner)          => med::House::from(inner).to_vec(),
            Self::Quads(inner)          => med::Quads::from(inner).to_vec(),
            Self::StraightFlush(inner)  => med::StraightFlush::from(inner).to_vec(),
            Self::Fives(inner)          => med::Fives::from(inner).to_vec(),
        }
    }
}
