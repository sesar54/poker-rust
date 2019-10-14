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
        pairs.push(vec!(last_card));

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
            },
            5 => Rank::Fives((pair[0], pair[1], pair[2], pair[3], pair[4])),
            4 => Rank::Quads((pair[0], pair[1], pair[2], pair[3])),
            3 => Rank::Trips((pair[0], pair[1], pair[2])),
            2 => Rank::Pair((pair[0], pair[1])),
            1 => Rank::High(pair[0]),
            _ => unreachable!(),
        }

    }

    fn straight_cards(cards: &[Card]) -> Vec<Vec<Card>> {

        let mut cards = cards.to_vec();
        cards.sort_by(|a, b| a.cmp_value_first(b));

        let mut iter = cards.iter().cloned();

        let mut straights: Vec<Vec<Card>> = Vec::new();
        let mut last_card = iter.next().unwrap();

        straights.push(vec!(last_card));

        while let Some(card) = iter.next() {

            if last_card.value as u8 == card.value as u8 + 1 {
                straights.last_mut().unwrap().push(card);

            } else {
                straights.push(vec![card]);
                last_card = card;

            }

        }

        straights.drain_filter(|s| s.len() > 5);
        straights

    }

    fn flushes_cards(cards: &[Card]) -> Vec<Vec<Card>> {

        let mut cards = cards.to_vec();
        cards.sort_by(|a, b| a.cmp_suit_first(b));

        let mut iter = cards.iter().cloned();

        let mut flushes: Vec<Vec<Card>> = Vec::new();
        let mut last_card = iter.next().unwrap();

        flushes.push(vec!(last_card));

        while let Some(card) = iter.next() {

            if last_card.suit == card.suit {
                flushes.last_mut().unwrap().push(card);

            } else {
                flushes.push(vec![card]);
                last_card = card;

            }

        }

        flushes.drain_filter(|f| f.len() > 5);
        flushes

    }

    pub fn update(&self, cards: Vec<Card>) {}
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.cards)
    }
}
