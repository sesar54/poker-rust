use super::{Hand, Rank, RankErr, RankInner};
use crate::card::{self, Card, Circular};

use std::convert::TryFrom;
use std::fmt;
use std::cmp::Ordering;

extern crate log;
use log::error;

impl Hand {
    /// Creating a new hand will cause all given cards to be automatically
    /// evaluated into a rank
    pub fn new(cards: Vec<Card>) -> Result<Hand, RankErr> {
        match Hand::ranking(&cards) {
            Ok(rank) => Ok(Hand { cards, rank }),
            Err(e) => Err(e),
        }
    }

    pub fn discard(self) -> Vec<Card> {
        self.cards
    }

    /// Takes a slice of cards and return the best card rank.
    ///
    /// If given a slice of length 0, return immediately with an error.
    pub fn ranking(cards: &[Card]) -> Result<Rank, RankErr> {
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

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn take(&mut self, mut cards: Vec<Card>) -> Result<(), RankErr> {
        match Hand::ranking(&self.cards) {
            Ok(rank) => self.rank = rank,
            Err(err) => return Err(err),
        }

        self.cards.append(&mut cards);

        Ok(())
    }

    /// Return the best ranking pair found in `cards`.
    /// Pairs are:
    /// `High`, `Pair`, `TwoPair`, `Trips`, `House`, `Quads` and `Fives`
    ///
    /// This function will always return High, if no other pair was found.
    /// The exception to the rule is if the slice of cards have a size of 0
    /// or something internally went wrong.
    pub fn pair_rank(cards: &[Card]) -> Result<Rank, RankErr> {
        let pair_groups = Hand::pair_groups(cards);
        let pair_iter = pair_groups.iter().rev().map(|p| p.as_slice());

        // Define some
        let mut quads = None;
        let mut trips = None;
        let mut pairs = (None, None);
        let mut high = None;

        // Build some
        for cards in pair_iter {
            match cards.len() {
                // Return immediately since Fives can't be beaten
                5 => return Rank::Fives(<[Card; 5]>::try_from(cards).unwrap()),
                4 if quads.is_none() => quads = Some(<[Card; 4]>::try_from(cards).unwrap()),
                3 if trips.is_none() => trips = Some(<[Card; 3]>::try_from(cards).unwrap()),
                2 => {
                    if pairs.0.is_none() {
                        pairs.0 = Some(<[Card; 2]>::try_from(cards).unwrap())
                    } else if pairs.1.is_none() {
                        pairs.1 = Some(<[Card; 2]>::try_from(cards).unwrap())
                    }
                }
                1 if high.is_none() => high = Some(<[Card; 1]>::try_from(cards).unwrap()),
                _ => (),
            }
        }

        // Get some
        if let Some(quads) = quads {
            Rank::Quads(quads)
        } else if let (Some(trips), Some(pair)) = (trips, pairs.0) {
            Rank::House(trips, pair)
        } else if let Some(trips) = trips {
            Rank::Trips(trips)
        } else if let (Some(pair0), Some(pair1)) = (pairs.0, pairs.1) {
            Rank::TwoPair(pair1, pair0) // Notice that pair1 comes before pair0
        } else if let Some(pair) = pairs.0 {
            Rank::Pair(pair)
        } else if let Some(high) = high {
            Rank::High(high)
        } else {
            Err(RankErr::Explained(format!("TODO Error: {:#?}", cards)))
        }
    }

    /// Maybe returns one rank after checking in order:
    /// **[StraightFlush, Flush, Straight]**
    pub fn straight_flush_rank(cards: &[Card]) -> Option<Result<Rank, RankErr>> {
        // Copy, sort and const.
        let mut cards = cards.to_vec();
        cards.sort();
        let cards = cards;

        // Look where flush_grouping is used and draw some conclusions on what
        // is happening and in which order.
        let flush_grouping = Hand::flush_groups(cards.as_slice());

        let result = if let Some(straight_flush) = Hand::straight_flush_cards(&flush_grouping) {
            match Rank::StraightFlush(straight_flush) {
                Err(e) => Err(RankErr::Explained(format!("Function straight_cards() with grouping from flush_groups() generated a false positive. Error: {:?}", e))),
                sf => sf
            }
        } else if let Some(flush) = Hand::extract_last_cards(&flush_grouping) {
            match Rank::Flush(flush) {
                Err(e) => Err(RankErr::Explained(format!("Function flush_cards() with grouping from flush_groups() generated a false positive. Error: {:?}", e))),
                flush => flush
            }
        } else if let Some(straight) =
            Hand::extract_last_cards(&Hand::straight_groups(cards.as_slice()))
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
    /// 1. Cards are sorted by it's rank first.
    /// 1.1 Ace Cards are sorted last (more valuable)
    /// 2. Cards are grouped together if their neighbor has the same rank.
    pub fn pair_groups(cards: &[Card]) -> Vec<Vec<Card>> {
        let mut cards = cards.to_vec();
        cards.sort_by(|a, b| a.cmp_rank_first(*b));

        // Ace rule
        let rotate = cards.len() - cards.iter().filter(|c| c.rank == card::Rank::Ace).count();
        cards.rotate_right(rotate);

        // Value to be returned
        let mut pairs: Vec<Vec<Card>> = Vec::new();
        // Main Sequence Generator
        let mut iter = cards.iter().cloned().peekable();
        let mut temp_vec: Vec<Card> = Vec::new();
        let mut prev_rank = iter.peek().unwrap().rank;

        for card in iter {
            if prev_rank == card.rank {
                temp_vec.push(card);
            } else {
                pairs.push(temp_vec);
                temp_vec = vec![card];
                prev_rank = card.rank;
            }
        }

        pairs.push(temp_vec);
        pairs
    }

    /// Returns cards grouped together by these rules:
    /// 1. Cards are sorted by it's suit first.
    /// 2. Cards are grouped together if their neighbor has the same suit.
    pub fn flush_groups(cards: &[Card]) -> Vec<Vec<Card>> {
        let mut cards = cards.to_vec();
        cards.sort_by(|a, b| a.cmp_suit_first(*b));

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

    /// Returns cards grouped together by these rules:
    /// 1. Cards are sorted by it's rank first.
    /// 2. Cards are grouped together if their neighbor has the same rank + 1
    /// 3. If the absolute last card is a King
    ///     and the absolute first card is an Ace,
    ///     make a copy of the last grouping, append it with the Ace card and save
    ///     it along the other groups.
    ///     This is done to simulating the ace rule in straights.
    ///
    pub fn straight_groups(cards: &[Card]) -> Vec<Vec<Card>> {
        let mut cards = cards.to_vec();
        cards.sort_by(|a, b| a.cmp_rank_first(*b));

        // Value to be returned
        let mut straight_groupings = Vec::<Vec<Card>>::new();
        // Main Sequence Generator
        let mut iter = cards.iter().cloned();
        let mut temp_vec;
        let mut prev_rank;

        // First card initiates things
        if let Some(first_card) = iter.next() {
            prev_rank = first_card.rank;
            temp_vec = vec![first_card];
        } else {
            // No cards in cards
            return straight_groupings;
        }

        // Iterate over rest of cards
        for card in iter {
            if card.rank == prev_rank.step(1) {
                temp_vec.push(card);
            // Drop temp_vec into straight_groupings and start a new one
            } else {
                straight_groupings.push(temp_vec);
                temp_vec = vec![card];
            }

            prev_rank = card.rank;
        }

        straight_groupings.push(temp_vec);

        // Ace rule
        match (cards.first(), cards.last(), straight_groupings.last()) {
            (Some(ace), Some(king), Some(broadway))
                if ace.rank == card::Rank::Ace && king.rank == card::Rank::King =>
            {
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

    /// Returns 5 cards that was successfully filtered through both
    /// `flush_groups(..)` and `straight_groups(..)`.
    ///
    /// This function extends `flush_groups(..)` as it's output is assumed to be
    /// this functions input.
    fn straight_flush_cards(flush_grouping: &[Vec<Card>]) -> Option<[Card; 5]> {
        for group in flush_grouping.iter().rev().filter(|v| v.len() >= 5) {
            if let Some(cards) = Hand::extract_last_cards(&Hand::straight_groups(&group)) {
                return Some(cards);
            }
        }

        None
    }

    /// Iterate, in reverse, over groupings that has size 5 or over.
    /// Extract it's 5 most valuable cards (last cards).
    fn extract_last_cards(groupings: &[Vec<Card>]) -> Option<[Card; 5]> {
        if let Some(cards) = groupings.iter().rev().find(|v| v.len() >= 5) {
            let cards = &cards[cards.len() - 5..];
            let cards = [cards[0], cards[1], cards[2], cards[3], cards[4]];
            Some(cards)
        } else {
            None
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.cards)
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {

        let default = || self.cmp(other);

        let ord: Option<Ordering> = match (self.0, other.0) {
            (RankInner::High(this), RankInner::High(other)) => {
                if this[0].rank == other[0].rank {
                    Some(default())
                } else if this[0].rank == card::Rank::Ace {
                    Some(Ordering::Greater)
                } else if other[0].rank == card::Rank::Ace {
                    Some(Ordering::Less)
                } else {
                    Some(default())
                }
            }
            _ => Some(default()),
        };

        if let Some(Ordering) = ord {
            Ordering
        } else {
            default()
        }

    }

}

