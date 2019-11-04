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
            Err(e) => panic!("{:?}", e),
        }
    }

    /// Takes a sorted slice of any number of cards and return the best card rank
    /// hand.
    ///
    /// If given a slice of length 0, there wont be a rank to return so None is
    /// returned. This is considered very erroneous.
    ///
    /// If given a unsorted slice, the function will not work properly

    fn ranking(cards: &[Card]) -> Result<Rank, RankErr> {
        if cards.is_empty() {
            panic!();
            //Err("No cards were given");
        }

        let pair = Hand::pair_rank(cards)?;

        let straight_flush = Hand::straight_flush_rank(cards);

        /* Compare and return a rank */
        if let Some(straight_flush) = straight_flush {
            Ok(std::cmp::max(pair, straight_flush))
        } else {
            Ok(pair)
        }
    }

    fn pair_rank(cards: &[Card]) -> Result<Rank, RankErr> {
        let pair = Hand::pair_groups(cards).pop().unwrap();

        match pair.len() {
            len @ 5..0xFF => {
                let pair = &pair[len - 5..];
                Rank::Fives((pair[0], pair[1], pair[2], pair[3], pair[4]))
            }
            4 => Rank::Quads((pair[0], pair[1], pair[2], pair[3])),
            3 => Rank::Trips((pair[0], pair[1], pair[2])),
            2 => Rank::Pair((pair[0], pair[1])),
            1 => Rank::High(pair[0]),
            _ => unreachable!(),
        }
    }

    /// Returns Some rank that is either Straight, Flush, StraightFlush
    /// Else None
    fn straight_flush_rank(cards: &[Card]) -> Option<Rank> {
        // Copy sort and const
        let mut cards = cards.to_vec();
        cards.sort();

        // Iterate over all flushes for all flushes that has 5 or more cards,
        // Then for the flush cards run it through the straight function
        // and iterate over straights that has 5 or more cards.
        // Return the 5 last cards from a valid straight flush.
        let flush_group = Hand::flush_groups(cards.as_slice());
        let mut flush_iter = flush_group.iter().rev().filter(|v| v.len() >= 5);

        while let Some(flush) = flush_iter.next() {
            // Check the same cards that was in flush
            // (Might be more than 5, this is important)
            let straight_group = Hand::straight_groups(flush.as_slice());
            let mut straight_iter = straight_group.iter().rev().filter(|v| v.len() >= 5);

            while let Some(cards) = straight_iter.next() {
                // Slice to correct range and make tuple
                let cards = &cards[cards.len() - 5..];
                let cards = (cards[0], cards[1], cards[2], cards[3], cards[4]);

                return Some(Rank::StraightFlush(cards).unwrap());
            }
        }

        // Get the last flush that has 5 or more cards,
        // tuple the last cards in range
        // and return as Some(Hand::Flush)
        let mut flush_iter = flush_group.iter().filter(|v| v.len() >= 5);

        if let Some(flush) = flush_iter.next() {
            let flush = (flush[0], flush[1], flush[2], flush[3], flush[4]);
            return Some(Rank::Flush(flush).unwrap());
        }

        // Get the last straight that has 5 or more cards,
        // tuple the last cards in range
        // and return as Some(Hand::Straight)
        let straight_group = Hand::straight_groups(cards.as_slice());
        let mut straight_iter = straight_group.iter().rev().filter(|v| v.len() >= 5);

        if let Some(cards) = straight_iter.next() {
            let cards = (cards[0], cards[1], cards[2], cards[3], cards[4]);
            return Some(Rank::Straight(cards).unwrap());
        }

        // Well I suppose nothing worked out huh?
        None
    }

    /// Returns cards grouped together by these rules:
    /// 1. Cards are sorted by it's value first.
    /// 2. Cards are grouped together if their neighbor has the same value.
    fn pair_groups(cards: &[Card]) -> Vec<Vec<Card>> {
        let mut cards = cards.to_vec();
        cards.sort();

        // Value to be returned
        let mut pairs: Vec<Vec<Card>> = Vec::new();
        // Main Sequence Generator
        let mut iter = cards.iter().cloned().peekable();
        let mut temp_vec: Vec<Card> = Vec::new();
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

    /// Returns cards grouped together by these rules:
    /// 1. Cards are sorted by it's value first.
    /// 2. Cards are grouped together if their neighbor has the same value
    ///     OR value + 1
    /// 3. If the absolute last card is a King
    ///     and the absolute first card is an Ace,
    ///     make a copy of the last group and append it with the Ace card.
    fn straight_groups(cards: &[Card]) -> Vec<Vec<Card>> {
        let mut cards = cards.to_vec();
        cards.sort();

        // Value to be returned
        let mut straights: Vec<Vec<Card>> = Vec::new();
        // Main Sequence Generator
        let mut iter = cards.iter().cloned().peekable();
        let mut temp_vec: Vec<Card> = Vec::new();
        let mut prev_value = iter.peek().unwrap().value;

        while let Some(card) = iter.next() {
            if prev_value == card.value || card.value as u8 == prev_value as u8 + 1 {
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

                    broadway.push(*ace_maybe);
                    straights.push(broadway);
                }
            }
        }

        straights
    }

    /// Returns cards grouped together by these rules:
    /// 1. Cards are sorted by it's suit first.
    /// 2. Cards are grouped together if their neighbor has the same suit.
    fn flush_groups(cards: &[Card]) -> Vec<Vec<Card>> {
        let mut cards = cards.to_vec();
        cards.sort_by_key(|c| c.suit);

        // Value to be returned
        let mut flushes: Vec<Vec<Card>> = Vec::new();
        // Main Sequence Generator
        let mut iter = cards.iter().cloned().peekable();
        let mut temp_vec: Vec<Card> = Vec::new();
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
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.cards)
    }
}
