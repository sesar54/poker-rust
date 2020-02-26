use super::{inner, Error, Rank};
use crate::card::Card;
use mimpl::mimpl;
use seq_macro::seq;
use std::convert::{TryFrom, TryInto};

/* -------------------------------------------------------------------------- */
/*                        Declaration of rank Mediators                       */
/* -------------------------------------------------------------------------- */

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct High(pub [Card; 1]);
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Pair(pub [Card; 2]);
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct TwoPair(pub Pair, pub Pair);
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Trips(pub [Card; 3]);
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Straight(pub [Card; 5]);
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Flush(pub [Card; 5]);
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct House {
    pub trips: Trips,
    pub pair: Pair,
}
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Quads(pub [Card; 4]);
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct StraightFlush(pub [Card; 5]);
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Fives(pub [Card; 5]);

/* -------------------------------------------------------------------------- */
/*                            Implement from inner                            */
/* -------------------------------------------------------------------------- */

mimpl!(From; inner::High, High, |high: inner::High|
    Self([Card {rank: high.rank, suit: high.suit}])
);

mimpl!(From; inner::Pair, Pair, |pair: inner::Pair|
    Self(seq!(n in 0..2{[#(Card{rank: pair.crank, suit: pair.suits[n]},)*]}))
);

mimpl!(From; inner::TwoPair, TwoPair, |two_pair: inner::TwoPair|
    Self(two_pair.pair0.into(), two_pair.pair1.into())
);

mimpl!(From; inner::Trips, Trips, |trips: inner::Trips|
    Self(seq!(n in 0..3{[#(Card{rank: trips.crank,  suit: trips.suits[n]},)*]}))
);

mimpl!(From; inner::Straight, Straight, |straight: inner::Straight| {
        let mut straight = straight.clone();
        Self(seq!(n in 0..4{[
            // Dont iterate on first
            Card{rank: straight.srank.into(), suit: straight.suits[0]},
            #(Card{rank: straight.srank.next().unwrap().into(), suit:  straight.suits[n]},)*
        ]}))
    }
);

mimpl!(From; inner::Flush, Flush, |flush: inner::Flush|
    Self(seq!(n in 0..5{[#(Card{rank: flush.ranks[n], suit:  flush.csuit},)*]}))
);

mimpl!(From; inner::House, House, |house: inner::House|
    Self {trips: house.trips.into(), pair: house.pair.into()}
);

mimpl!(From; inner::Quads, Quads, |quads: inner::Quads|
    Self(seq!(n in 0..4{[#(Card{rank: quads.crank,  suit: quads.suits[n]},)*]}))
);

mimpl!(From; inner::StraightFlush, StraightFlush, |sf: inner::StraightFlush| {
        let mut sf = sf.clone();
        Self(seq!(n in 0..4{[
            Card{rank: sf.srank.into(), suit: sf.csuit}, // Dont iterate on first
            #(Card{rank: sf.srank.next().unwrap().into(), suit: sf.csuit},)*
        ]}))
    }
);

mimpl!(From; inner::Fives, Fives, |fives: inner::Fives|
    Self(seq!(n in 0..5{[#(Card{rank: fives.crank, suit:  fives.suits[n]},)*]}))
);

/* -------------------------------------------------------------------------- */
/*                             Implement from Rank                            */
/* -------------------------------------------------------------------------- */

macro_rules! from_Rank {
    ($type:ident) => {
        mimpl!(From; Rank, Option<$type>, |rank: Rank|
            if let Rank::$type(inner) = rank {Some(inner.into())} else {None}
        );
    };
    ($type0:ident, $($type1:ident),+) => {
        from_Rank!($type0);
        $(from_Rank!($type1);)*
    }
}

from_Rank!(
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

/* -------------------------------------------------------------------------- */
/*                         Implement try_from &[Card]                         */
/* -------------------------------------------------------------------------- */

macro_rules! impl_TryFrom_slice {
    ($type:tt, $len:expr) => {
        impl TryFrom<&[Card]> for $type {
            type Error = Error;

            /// Tries to check at the last $len cards and put them into Self.
            /// Else if there is not enough cards, return an error.
            fn try_from(cards: &[Card]) -> Result<Self, Self::Error> {
                cards.rchunks($len)
                    .next()
                    .map(|cards| Self(cards.try_into().unwrap()))
                    .ok_or(Error::TryFromSlice(box Error::InvalidLength {
                        actual: cards.len(),
                        contents: cards.to_vec().into_boxed_slice(),
                        expected: $len,
                    }))
                }
        }
    };
    ($type0:tt, $len0:expr; $($type1:tt, $len1:expr);+ $(;)*) => {
        impl_TryFrom_slice!($type0, $len0);
        $(impl_TryFrom_slice!($type1, $len1);)*
    }
}

// Cannot and will not implement for splitted ranks.
impl_TryFrom_slice!(
    High, 1;
    Pair, 2;
    //TwoPair, 4;
    Trips, 3;
    Straight, 5;
    Flush, 5;
    //House, 5;
    Quads, 4;
    StraightFlush, 5;
    Fives, 5;
);

macro_rules! impl_to_vec {
    ($type:tt) => {
        impl $type {
            pub fn to_vec(self) -> Vec<Card> {
                self.0.to_vec()
            }
        }
    };
    ($type0:tt, $($type1:tt),+ ) => {
        impl_to_vec!($type0);
        $(impl_to_vec!($type1);)*
    }
}

impl_to_vec!(
    High,
    Pair,
    //TwoPair,
    Trips,
    Straight,
    Flush,
    //House,
    Quads,
    StraightFlush,
    Fives
);

impl TwoPair {
    pub fn to_vec(&self) -> Vec<Card> {
        let TwoPair(arr0, arr1) = self;
        arr0.0.iter().chain(arr1.0.iter()).copied().collect()
    }
}

impl House {
    pub fn to_vec(&self) -> Vec<Card> {
        let House { trips, pair } = self;
        trips.0.iter().chain(pair.0.iter()).copied().collect()
    }
}
