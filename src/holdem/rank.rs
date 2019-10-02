use std::fmt;
use std::borrow::Cow;

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

// TODO Write error messages
impl Rank {
    
    pub fn High(card: Card) -> Result<Rank, &'static str> {
        
        if card == card {
            Ok(Rank(High(card)))
        
        } else {
            Err("")

        }
        
    }

    pub fn Pair(card0: Card, card1: Card) -> Result<Rank, &'static str> {

        if card0.value == card1.value {
            // Quicksort so first card is the smaller card 
            let (card0,card1) = {
                if card0.suit > card1.suit {
                    (card1, card0)
                } else {
                    (card0, card1)
                }
            };

            Ok(Rank(Pair(card0,card1)))

        } else {
            Err("")

        }
    }

    pub fn Trips(card0: Card, card1: Card, card2: Card) -> Result<Rank, &'static str> {

        if card0.value == card1.value && card1.value == card2.value {

            (card0, card1, card2).sort();

            Ok(Rank(Trips(card0,card1,card3)))

        } else {
            Err("")
        }

    }

    fn tuple_sort_2<T>(e0: T, e1: T) -> (T, T)
    where T: std::cmp::PartialOrd {

        if e0 > e1 {
            (e1, e0)
        } else {
            (e0, e1)
        }

    }

    fn tuple_sort_3<T>(e0: T, e1: T, e2: T) -> (T, T, T)
    where T: std::cmp::PartialOrd {

        let (e0, e1) = Rank::tuple_sort_2(e0, e1);
        let (e1, e2) = Rank::tuple_sort_2(e1, e2);
        let (e2, e0) = Rank::tuple_sort_2(e2, e0);

        (e0, e1, e2)

    }

    fn _tuple_sort<T>()

    fn new(cards: &mut [Card]) -> Result<Rank, &str> {

        let err = "Can't create a rank of {:?}";

        match cards.len() {

            3 => 
                // Check if Trips compatable
                
                
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