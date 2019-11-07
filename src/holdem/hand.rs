extern crate log;

use crate::*;
use holdem::*;

use std::convert::TryFrom;
use std::fmt;

use log::error;

impl Hand {
    /// Creating a new hand will cause all given cards to be automatically
    /// evaluated into a rank
    pub fn new(cards: Vec<Card>) -> Hand {
        match Hand::ranking(&cards) {
            Ok(rank) => Hand { cards, rank },
            Err(e) => {
                error!("{:#?}", e);
                panic!("{:#?}", e)
            }
        }
    }

    /// Takes a slice of cards and return the best card rank.
    ///
    /// If given a slice of length 0, return immediately with an error.
    fn ranking(cards: &[Card]) -> Result<Rank, RankErr> {
        if cards.is_empty() {
            Err(RankErr::Explained(format!(
                "No cards were given. Cards: {:?}",
                cards
            )))
        } else {
            let pair_rank = Hand::pair_rank(cards)?;
            let option_sf = Hand::straight_flush_rank(cards);

            // Compare and return the biggest rank
            match option_sf {
                Some(sf) => Ok(std::cmp::max(pair_rank, sf?)),
                None => Ok(pair_rank),
            }
        }
    }

    /// Return one of the pair ranks.
    /// TODO
    fn pair_rank(cards: &[Card]) -> Result<Rank, RankErr> {
        let pair_groups = Hand::pair_groups(cards);
        let mut pair_iter = pair_groups.iter().rev();
        let mut largest_pair: &Vec<Card>;

        if let Some(pairs) = pair_iter.next() {
            largest_pair = pairs;

            let mut fives: Option<[Card; 5]> = None;
            let mut quads: Option<[Card; 4]> = None;
            let mut trips: Option<[Card; 3]> = None;
            let mut pair: Option<[Card; 2]> = None;
            let mut high: Option<[Card; 1]> = None;

            macro_rules! try_from {
                ($from:expr; $to:ty) => {$to::try_from($from)}
            }

            for pairs in pair_iter {
                if largest_pair.len() < pairs.len() {
                    largest_pair = pairs;
                }

                match pairs.len() {
                    4 if quads.is_none() => 
                        quads = Some(try_from!(<[Card; 4]>; pairs).unwrap()),
                    3 if trips.is_none() => 
                        trips = Some(<[Card; 3]>::try_from(pairs.as_slice()).unwrap()),
                    2 if pair.is_none() => 
                        pair = Some(<[Card; 2]>::try_from(pairs.as_slice()).unwrap()),
                    _ => (),
                }
            }

            let pair = largest_pair;
            match largest_pair.len() {
                len @ 5..0xFF => {
                    let pair = &pair[len - 5..];
                    Rank::Fives([pair[0], pair[1], pair[2], pair[3], pair[4]])
                }
                4 => Rank::Quads([pair[0], pair[1], pair[2], pair[3]]),
                3 => Rank::Trips([pair[0], pair[1], pair[2]]),
                2 => Rank::Pair([pair[0], pair[1]]),
                1 => Rank::High(pair[0]),
                0 => Err(RankErr::Explained(format!("TODO Error: {:#?}", cards))),
                _ => unreachable!(),
            }
        } else {
            Err(RankErr::Explained(format!("TODO Error: {:#?}", cards)))
        }
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

        for card in iter {
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

    /// Maybe returns one rank after checking in order:
    /// **[StraightFlush, Flush, Straight]**
    ///
    /// This is done by
    fn straight_flush_rank(cards: &[Card]) -> Option<Result<Rank, RankErr>> {
        // Copy, sort and const.
        let mut cards = cards.to_vec();
        cards.sort();
        let cards = cards;

        // With the flush gro
        let flush_grouping = Hand::flush_groups(cards.as_slice());

        let result = if let Some(straight_flush) = Hand::straight_flush_cards(&flush_grouping) {
            match Rank::StraightFlush(straight_flush) {
                Err(e) => Err(RankErr::Explained(format!("Function straight_cards() with grouping from flush_groups() generated a false positive. Error: {:?}", e))),
                sf => sf
            }
        } else if let Some(flush) = Hand::extract_5_cards(&flush_grouping) {
            match Rank::Flush(flush) {
                Err(e) => Err(RankErr::Explained(format!("Function flush_cards() with grouping from flush_groups() generated a false positive. Error: {:?}", e))),
                flush => flush
            }
        } else if let Some(straight) =
            Hand::extract_5_cards(&Hand::straight_groups(cards.as_slice()))
        {
            match Rank::Straight(straight) {
                Err(e) => Err(RankErr::Explained(format!("Function straight_cards() with grouping from straight_groups() generated a false positive. Error: {:?}", e))),
                straight => straight
            }
        } else {
            return None;
        };

        Some(result)
    }

    /// Returns cards grouped together by these rules:
    /// 1. Cards are sorted by it's value first.
    /// 2. Cards are grouped together if their neighbor has the same value + 1
    /// 3. If the absolute last card is a King
    ///     and the absolute first card is an Ace,
    ///     make a copy of the last grouping, append it with the Ace card and save
    ///     it along the other groups.
    ///     This is done to simulating the ace rule in straights.
    ///
    fn straight_groups(cards: &[Card]) -> Vec<Vec<Card>> {
        let mut cards = cards.to_vec();
        cards.sort_by_key(|c| c.value);

        // Value to be returned
        let mut straight_groupings = Vec::<Vec<Card>>::new();
        // Main Sequence Generator
        let mut iter = cards.iter().cloned();
        let mut temp_vec;
        let mut prev_value;

        // First card initiates things
        if let Some(first_card) = iter.next() {
            prev_value = first_card.value;
            temp_vec = vec![first_card];
        } else {
            // No cards in cards
            return straight_groupings;
        }

        // Iterate over rest of cards
        for card in iter {
            if card.value as u8 == prev_value as u8 + 1 {
                temp_vec.push(card);
            // Drop temp_vec into straight_groupings and start a new one
            } else {
                straight_groupings.push(temp_vec);
                temp_vec = vec![card];
            }

            prev_value = card.value;
        }

        straight_groupings.push(temp_vec);

        // Ace rule
        match (cards.first(), cards.last(), straight_groupings.last()) {
            (Some(ace), Some(king), Some(broadway)) if ace.value == Ace && king.value == King => {
                let mut broadway = broadway.clone();
                broadway.push(*ace);
                straight_groupings.push(broadway);
            }
            (Some(_), Some(_), None) => {
                error!(
                    "Card Ace and King found, but no element was appended to straight_groupings"
                );
                unreachable!();
            }
            _ => {}
        }

        straight_groupings
    }

    /// Iterate, in reverse, over groupings until one group has size 5 or over.
    /// Extract it's 5 most valuable cards (last cards).
    fn extract_5_cards(groupings: &[Vec<Card>]) -> Option<[Card; 5]> {
        if let Some(cards) = groupings.iter().rev().find(|v| v.len() >= 5) {
            let cards = &cards[cards.len() - 5..];
            let cards = [cards[0], cards[1], cards[2], cards[3], cards[4]];
            Some(cards)
        } else {
            None
        }
    }

    /// Returns cards grouped together by these rules:
    /// 1. Cards are sorted by it's suit first.
    /// 2. Cards are grouped together if their neighbor has the same suit.
    fn flush_groups(cards: &[Card]) -> Vec<Vec<Card>> {
        let mut cards = cards.to_vec();
        cards.sort_by_key(|c| c.suit);

        // Value to be returned
        let mut flush_groupings: Vec<Vec<Card>> = Vec::new();
        // Main Sequence Generator
        let mut iter = cards.iter().cloned();
        let mut temp_vec;
        let mut prev_suit;

        // First card initiates things
        if let Some(first_card) = iter.next() {
            temp_vec = vec![first_card];
            prev_suit = first_card.suit;

        // No cards in cards
        } else {
            return flush_groupings;
        }

        // Iterate over rest of cards
        for card in iter {
            if prev_suit == card.suit {
                temp_vec.push(card);
            } else {
                flush_groupings.push(temp_vec);
                temp_vec = vec![card];
                prev_suit = card.suit;
            }
        }

        flush_groupings.push(temp_vec);
        flush_groupings
    }

    fn straight_flush_cards(flush_grouping: &[Vec<Card>]) -> Option<[Card; 5]> {
        for group in flush_grouping.iter().rev().filter(|v| v.len() >= 5) {
            if let Some(cards) = Hand::extract_5_cards(&Hand::straight_groups(&group)) {
                return Some(cards);
            }
        }

        None
    }

    //pub fn update(&self, cards: Vec<Card>) {}
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.cards)
    }
}
