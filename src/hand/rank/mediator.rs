use super::{inner, Rank};
use crate::card::Card;
use seq_macro::seq;

/* -------------------------------------------------------------------------- */
/*                        Declaration of rank Mediators                       */
/* -------------------------------------------------------------------------- */

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct High(pub Card);
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
    Self(Card {rank: high.rank, suit: high.suit})
);

mimpl!(From; inner::Pair, Pair, |pair: inner::Pair|
    Self(seq!(n in 0..2{[#(Card{rank: pair.crank, suit: pair.suits[n]},)*]}))
);

mimpl!(From; inner::TwoPair, TwoPair, |twoPair: inner::TwoPair|
    Self(twoPair.pair0.into(), twoPair.pair1.into())  
);

mimpl!(From; inner::Trips, Trips, |trips: inner::Trips|
    Self(seq!(n in 0..3{[#(Card{rank: trips.crank,  suit: trips.suits[n]},)*]}))
);

mimpl!(From; inner::Straight, Straight, |straight: inner::Straight|
    Self(seq!(n in 0..4{[
        // Dont iterate on first
        Card{rank: straight.srank.into(), suit: straight.suits[0]},
        #(Card{rank: straight.srank.next().unwrap().into(), suit:  straight.suits[n]},)*
    ]}))  
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

mimpl!(From; inner::StraightFlush, StraightFlush, |sf: inner::StraightFlush|
    Self(seq!(n in 0..4{[
        Card{rank: sf.srank.into(), suit: sf.csuit}, // Dont iterate on first
        #(Card{rank: sf.srank.next().unwrap().into(), suit: sf.csuit},)*
    ]}))    
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
