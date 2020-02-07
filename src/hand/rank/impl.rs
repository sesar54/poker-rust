use super::{
    inner,
    mediator, InvalidStraightError, Rank, TryFromMediatorError,
};
use crate::card::Card;
use std::convert::{From, TryFrom};
use std::error;
use std::fmt;

impl Rank {
    /// Will return number of cards in Rank. These are constant.
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
        match self {
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

    pub fn to_boxed_slice(self) -> Box<[Card]> {
        use mediator::*;
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

macro_rules! try_from_mediator {
    ($type:ident) => {

        impl error::Error for mediator::$type {
            // TODO https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
        }

        impl TryFrom<mediator::$type> for Rank {
            type Error = TryFromMediatorError;

            fn try_from(cards: mediator::$type) -> Result<Self, Self::Error> {
                let inner_rank = inner::$type::from(cards);

                if cards == mediator::$type::from(inner_rank) {
                    Ok(Rank::$type(inner_rank))
                } else {
                    Err(TryFromMediatorError(cards.into()))
                }
            }
        }
    };
    ($type0:ident, $($type1:ident),+) => {
        try_from_mediator!($type0);
        $(try_from_mediator!($type1);)*
    };
}

#[rustfmt::skip]
try_from_mediator!(
    /* High */ 
    Pair, 
    TwoPair, 
    Trips, 
    /* Straight */ 
    Flush, 
    House, 
    Quads,
    /* StraightFlush */ 
    Fives
);

impl From<mediator::High> for Rank {
    fn from(card: mediator::High) -> Self {
        Rank::High(card.into())
    }
}

impl<T> fmt::Display for InvalidStraightError<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T> error::Error for InvalidStraightError<T> where T: error::Error {}

impl TryFrom<mediator::Straight> for Rank {
    type Error = InvalidStraightError<mediator::Straight>;
    fn try_from(cards: mediator::Straight) -> Result<Self, Self::Error> {
        Ok(Self::Straight(
            inner::Straight::try_from(cards).map_err(|e| InvalidStraightError(e, cards))?,
        ))
    }
}

impl TryFrom<mediator::StraightFlush> for Rank {
    type Error = InvalidStraightError<mediator::StraightFlush>;
    fn try_from(cards: mediator::StraightFlush) -> Result<Self, Self::Error> {
        Ok(Self::StraightFlush(
            inner::StraightFlush::try_from(cards).map_err(|e| InvalidStraightError(e, cards))?,
        ))
    }
}

impl error::Error for TryFromMediatorError {}
