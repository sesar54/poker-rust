use std::fmt;

use crate::card::*;
use crate::holdem::{Rank, RankInner::*};

impl fmt::Display for Rank {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

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

        ok_rank!(High(card))

    }

    pub fn Pair(card: (Card, Card)) -> ResRank {

        if card.0.value != card.1.value {
            Err("Not pair")

        } else if card.0 > card.1 {
            Err("Unordered")

        } else {
            ok_rank!(Pair(card.0, card.1))

        }
    }

    /**
     * This one will go with a bang
     */
    pub fn TwoPair(card: (Card, Card, Card, Card)) -> ResRank {

        let pairTup = |p| match p {
            Pair(x, y) => (x, y)
        };

        let pair0 = pairTup(Pair(card.0, card.1));
        let pair1 = pairTup(Pair(card.2, card.3));

        if pair0 > pair1 {
            Err("Unordered")

        } else {
            ok_rank!(TwoPair((pair0.0, pair0.1), (pair1.0, pair1.1)))

        }


    }

    pub fn Trips(card: (Card, Card, Card)) -> ResRank {

        if card.0.value != card.1.value || card.1.value != card.2.value {
            Err("Not Trips")

        } else if card.0 > card.1 || card.1 > card.2 {
            Err("Unordered")

        } else {
            ok_rank!(Trips(card.0, card.1, card.2))

        }

    }


    pub fn Straight(card: (Card, Card, Card, Card, Card)) -> ResRank {

        #![feature(is_sorted)]
        let arr = [
            card.0.value as u8,
            card.1.value as u8,
            card.2.value as u8,
            card.3.value as u8,
            card.4.value as u8,
        ];


        if arr[0] != arr[4] + 4 {
            Err("Not Straight")

        } else {
            ok_rank!(Straight(card.0, card.1, card.2, card.3, card.4))

        }

    }

    pub fn Flush(card: (Card, Card, Card, Card, Card))
    -> ResRank {

        let arr = [
            card.0.suit as u8,
            card.1.suit as u8,
            card.2.suit as u8,
            card.3.suit as u8,
            card.4.suit as u8,
        ];

        /* See if they are all the same */
        if arr.iter().min() != arr.iter().max() {
            Err("Not Flush")

        //} else if arr.is_sorted() { //TODO Wait until stable
        //    Err("Unordered")

        } else {
            ok_rank!(Flush(card.0, card.1, card.2, card.3, card.4))

        }

    }

    pub fn House(card: ((Card, Card), (Card, Card, Card))) -> ResRank {

        let pairTup = |p| match p {
            Pair(x, y) => (x, y),
        };

        let tripTup = |t| match t {
            Trips(x, y, z) => (x, y, z)
        };

        let pair = pairTup(Pair((card.0).0, (card.0).1));
        let trips = tripTup(Trips((card.1).0, (card.1).1, (card.1).2));

        ok_rank!(House((pair.0, pair.1), (trips.0, trips.1, trips.2)))

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
            ok_rank!(Quads(card.0, card.1, card.2, card.3))

        }

    }

    pub fn StraightFlush(card: (Card, Card, Card, Card, Card)) -> ResRank {

        if let Straight(..) = Straight(card.0, card.1, card.2, card.3, card.4)
        }


    }

    fn new(cards: &mut [Card]) -> Result<Rank, &str> {

        let err = "Can't create a rank of {:?}";

        match cards.len() {

            4 =>
                // Check if TwoPair compatable
                if cards[0].value == cards[1].value
                && cards[2].value == cards[3].value {

                    // Check if Quad compatable
                    if cards[1].value == cards[2].value {
                        cards.sort_by_key(|c| c.suit);
                        Ok(Quads(cards[0], cards[1], cards[2], cards[3]))

                    // Check if TwoPair is formatted correctly
                    } else if cards[0].value < cards[2].value {
                        let (left, right) = cards.split_at_mut(2);
                        left.sort_by_key(|c| c.suit);
                        right.sort_by_key(|c| c.suit);
                        Ok(TwoPair((left[0], left[1]), (right[0], right[1])))

                    } else {
                        Err("")
                    }


                } else {
                    Err("")
                }

            5 =>

            _ => Err(""),
        }



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
