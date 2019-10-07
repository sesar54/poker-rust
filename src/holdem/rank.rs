use std::fmt;
use std::borrow::Cow;

use crate::card::*;
use crate::holdem::{Rank, Rank::*};

impl fmt::Display for Rank {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match *self {

            High(..) =>             write!(f, "Highcard"),
            Pair(..) =>             write!(f, "Pair"),
            TwoPair(..) =>          write!(f, "Two pairs"),
            Trips(..) =>            write!(f, "Three of a kind"),
            Straight(..) =>         write!(f, "Straight"),
            Flush(..) =>            write!(f, "Flush"),
            House(..) =>            write!(f, "Full house"),
            Quads(..) =>            write!(f, "Four of a kind"),
            StraightFlush(card, ..) => match card.value {
                _ =>                write!(f, "Straight flush"),
                Ace =>              write!(f, "Royal flush"),
            }
            FivePair(..) =>         write!(f, "Five of a kind"),

        }
    }
}

impl Rank {

    fn new(cards: &mut [Card]) -> Result<Rank, &str> {

        let err = "Can't create a rank of {:?}";

        match cards.len() {
            
            0 =>
                Err("Can't c")

            1 => 
                Ok(High(cards[0])),

            2 => 
                if cards[0].value == cards[1].value {
                    cards.sort_by_key(|c| c.suit);
                    Ok(Pair(cards[0],cards[1]))
                } else {
                    Err("test")
                }

            3 => 
                // Check if Trips compatable
                if cards[0].value == cards[1].value 
                && cards[1].value == cards[2].value {
                    cards.sort_by_key(|c| c.suit);
                    Ok(Trips(cards[0],cards[1],cards[3]))
                } else {
                    Err("")
                }
                
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