use std::fmt;

use crate::card::*;
use crate::holdem::{Rank, RankInner};

impl fmt::Display for Rank {



    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        use RankInner::*;

        match self.0 {

            High(..) =>             write!(f, "Highcard"),
            Pair(..) =>             write!(f, "Pair"),
            TwoPair(..) =>          write!(f, "Two pairs"),
            Trips(..) =>            write!(f, "Three of a kind"),
            Straight(..) =>         write!(f, "Straight"),
            Flush(..) =>            write!(f, "Flush"),
            House(..) =>            write!(f, "Full house"),
            Quads(..) =>            write!(f, "Four of a kind"),
            StraightFlush(.., card) => match card.value {
                _ =>                write!(f, "Straight flush"),
                Ace =>              write!(f, "Royal flush"),
            }
            FivePair(..) =>         write!(f, "Five of a kind"),

        }
    }
}

macro_rules! ok_rank {
    ($rank:expr) => {
        return Ok(Rank($rank))
    };
}

type ResRank = Result<Rank, &'static str>;


impl Rank {

    pub fn High(card: Card) -> ResRank {

        ok_rank!(RankInner::High(card))

    }

    pub fn Pair(card0: Card, card1: Card) -> ResRank {

        if card0.value != card1.value {
            Err("Not pair")

        } else if card0 > card1 {
            Err("Unordered")

        } else {
            ok_rank!(RankInner::Pair(card0, card1))

        }
    }

    /**
     * This one will go with a bang
     */
    pub fn TwoPair(card0: Card, card1: Card, card2: Card, card3: Card)
        -> ResRank {

        let pairTup = |p| match p {RankInner::Pair(x, y) => (x, y)};

        let pair0 = pairTup(RankInner::Pair(card0, card1));
        let pair1 = pairTup(RankInner::Pair(card2, card3));

        if pair0 > pair1 {
            Err("Unordered")

        } else {
            ok_rank!(RankInner::TwoPair((pair0.0, pair0.1), (pair1.0, pair1.1)))

        }


    }

    pub fn Trips(card0: Card, card1: Card, card2: Card) -> ResRank {

        if card0.value != card1.value || card1.value != card2.value {
            Err("Not Trips")

        } else if card0 > card1 || card1 > card2 {
            Err("Unordered")

        } else {
            ok_rank!(RankInner::Trips(card0, card1, card2))

        }

    }


    pub fn Straight
        (card0: Card, card1: Card, card2: Card, card3: Card, card4: Card) ->
        ResRank {

        #![feature(is_sorted)]
        let arr = [
            card0.value as u8,
            card1.value as u8,
            card2.value as u8,
            card3.value as u8,
            card4.value as u8,
        ];


        if arr[0] != arr[4] + 4 {
            Err("Not Straight")

        //} else if arr.is_sorted() { //TODO Wait until stable
        //    Err("Unordered")

        } else {
            ok_rank!(RankInner::Straight(card0, card1, card2, card3, card4))

        }

    }

    pub fn Flush
        (card0: Card, card1: Card, card2: Card, card3: Card, card4: Card) ->
        ResRank {

        let arr = [
            card0.suit as u8,
            card1.suit as u8,
            card2.suit as u8,
            card3.suit as u8,
            card4.suit as u8,
        ];

        /* See if they are all the same */
        if arr.iter().min() != arr.iter().max() {
            Err("Not Flush")

        //} else if arr.is_sorted() { //TODO Wait until stable
        //    Err("Unordered")

        } else {
            ok_rank!(RankInner::Flush(card0, card1, card2, card3, card4))

        }

    }

    pub fn House
        (card0: Card, card1: Card, card2: Card, card3: Card, card4: Card) ->
        ResRank {

        let pairTup = |p| match p {RankInner::Pair(x, y) => (x, y)};
        let tripTup = |t| match t {RankInner::Trips(x, y, z) => (x, y, z)};

        let pair = pairTup(RankInner::Pair(card0, card1));
        let trips = tripTup(RankInner::Trips(card2, card3, card4));

        ok_rank!(RankInner::House((pair.0, pair.1), (trips.0, trips.1, trips.2)))

    }

    pub fn Quads(card: (Card, Card, Card, Card)) -> ResRank {

        let arr = [
            card.0.value as u8,
            card.1.value as u8,
            card.2.value as u8,
            card.3.value as u8,
        ];

        if arr.iter().min() != arr.iter().max() {
            Err("Quads")

        //} else if arr.is_sorted(){
        //    Err("Unordered")

        } else {
            ok_rank!(RankInner::Quads(card.0, card.1, card.2, card.3))

        }

    }

    pub fn StraightFlush
        (card0: Card, card1: Card, card2: Card, card3: Card, card4: Card) ->
        ResRank {

        let tup = (
            RankInner::Straight(card0, card1, card2, card3, card4),
            RankInner::Flush(card0, card1, card2, card3, card4),
        );

        Err("Not")

    }

}

#[cfg(test)]
mod tests {

    use crate::*;
    use crate::holdem::{ Rank, Rank::* };

    #[test]
    fn display() {
        println!("{}", High(card!()));
        println!("{}", Rank::Pair(card!(),card!()))

    }

}
