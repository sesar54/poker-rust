use std::fmt;

use crate::*;
use holdem::*;

impl Hand {
    /**
     * Creating a new hand will cause all given cards to be automatically
     * evaluated into a rank
     */
    pub fn new(cards: &[Card]) -> Hand {
        let rank = Hand::ranking(cards);

        match rank {
            Ok(rank) => {
                return Hand {
                    cards: cards.to_owned(),
                    rank: rank,
                }
            }
            Err(e) => panic!(e),
        }
    }

    /// Takes a sorted slice of any number of cards and return the best card rank
    /// hand.
    ///
    /// If given a slice of length 0, there wont be a rank to return so None is
    /// returned. This is considered very erroneous.
    ///
    /// If given a unsorted slice, the function will not work properly

    fn ranking(cards: &[Card]) -> Result<Rank, &'static str> {
        if cards.is_empty() {
            panic!();
            //Err("No cards were given");
        }

        let some_pair = Hand::pair_rank(cards);

        let straight_cards = Hand::straight_cards(cards);

        let flush_cards = Hand::flushes_cards(cards);

        unimplemented!();

        /* Compare and return a rank *
        if let Some(straight_flush) = straight_flush {
            return Some(std::cmp::max(pair, straight_flush));
        } else {
            return Some(pair);
        }*/
    }

    fn pair_rank(cards: &[Card]) -> Result<Rank, &'static str> {
        // cards.sort_by(|a: Card, b| a.)

        let mut pairs: Vec<Vec<Card>> = Vec::new();
        let mut iter = cards.iter().cloned();

        let mut last_card = iter.next().unwrap();
        pairs.push(vec![last_card]);

        while let Some(card) = iter.next() {
            if last_card.value == card.value {
                pairs.last_mut().unwrap().push(card);
            } else {
                pairs.push(vec![card]);
                last_card = card;
            }
        }

        pairs.sort_by_key(|v| v.len());

        let pair: Vec<Card> = pairs.pop().unwrap();

        match pair.len() {
            len @ 6..0xFF => {
                let pair = &pair[len..];
                Rank::Fives((pair[0], pair[1], pair[2], pair[3], pair[4]))
            }
            5 => Rank::Fives((pair[0], pair[1], pair[2], pair[3], pair[4])),
            4 => Rank::Quads((pair[0], pair[1], pair[2], pair[3])),
            3 => Rank::Trips((pair[0], pair[1], pair[2])),
            2 => Rank::Pair((pair[0], pair[1])),
            1 => Rank::High(pair[0]),
            _ => unreachable!(),
        }
    }


    /// Returns cards grouped together by neighbors
    pub fn pair_cards(cards: &[Card]) -> Vec<Vec<&Card>> {
        // Creates a sorted vector of references to cards
        let cards = {
            let mut vec: Vec<&Card> = Vec::new();

            for card in cards {
                vec.push(&card);
            }

            vec.sort_by_key(|c| c.value);
            vec
        };

        println!("Cards sorted: {:#?}", cards);

        // Value to be returned
        let mut pairs: Vec<Vec<&Card>> = Vec::new();
        // Main Sequence Generator
        let mut iter = cards.iter().peekable();
        let mut temp_vec: Vec<&Card> = Vec::new();
        let mut prev_value = iter.peek().unwrap().value;

        while let Some(card) = iter.next() {
            if prev_value == card.value {
                temp_vec.push(card);
               
            } else {
                pairs.push(temp_vec);
                temp_vec = vec![card];
                prev_value = card.value;
            }
        }

        pairs.push(temp_vec);
        pairs


    }

    /// Returns cards grouped together by neighbors, in any clump size
    /// If a King and an Ace is present in ```cards```
    pub fn straight_cards(cards: &[Card]) -> Vec<Vec<&Card>> {
        // Creates a sorted vector of references to cards
        let cards = {
            let mut vec: Vec<&Card> = Vec::new();

            for card in cards {
                vec.push(&card);
            }

            vec.sort();
            //vec.sort_by_key(|c| c.value);
            vec
        };

        println!("Cards sorted: {:#?}", cards);

        // Value to be returned
        let mut straights: Vec<Vec<&Card>> = Vec::new();
        // Main Sequence Generator
        let mut iter = cards.iter().peekable();
        let mut temp_vec: Vec<&Card> = Vec::new();
        let mut prev_value= iter.peek().unwrap().value;

        while let Some(card) = iter.next() {
            if prev_value == card.value 
            || card.value as u8 == prev_value as u8 + 1 {
                temp_vec.push(card);
            } else {
                straights.push(temp_vec);
                temp_vec = vec![card];
            }

            prev_value = card.value;
        }

        straights.push(temp_vec);

        // Ace rule (Not proven broadway)
        if cards.last().unwrap().value == King {
            if let Some(ace_maybe) = cards.first() {
                if ace_maybe.value == Ace {
                    let mut broadway = straights.last().unwrap().clone();

                    broadway.push(ace_maybe);
                    straights.push(broadway);
                }
            }
        }

        straights

    }

    fn flushes_cards(cards: &[Card]) -> Vec<Vec<&Card>> {

        // Creates a sorted vector of references to cards
        let cards = {
            let mut vec: Vec<&Card> = Vec::new();

            for card in cards {
                vec.push(&card);
            }

            vec.sort_by_key(|c| c.suit);
            vec
        };

        println!("Cards sorted: {:#?}", cards);

        // Value to be returned
        let mut flushes: Vec<Vec<&Card>> = Vec::new();
        // Main Sequence Generator
        let mut iter = cards.iter().peekable();
        let mut temp_vec: Vec<&Card> = Vec::new();
        let mut prev_suit = iter.peek().unwrap().suit;

        while let Some(card) = iter.next() {
            if prev_suit == card.suit {
                temp_vec.push(card);
               
            } else {
                flushes.push(temp_vec);
                temp_vec = vec![card];
                prev_suit = card.suit;
            }
        }

        flushes.push(temp_vec);
        flushes
    }

    //pub fn update(&self, cards: Vec<Card>) {}

    pub fn test() {
        let cards = cards!(Ace, Spades; King, Spades; Queen, Diamonds; Jack, Clubs; Ten, Clubs; Nine, Spades; Eight, Spades; Seven, Spades);
        println!("Cards in {:#?}", cards);
        println!("Straight out: {:#?}", Hand::straight_cards(&cards));
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.cards)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        use crate::holdem::Hand;

        Hand::test();
    }
}
