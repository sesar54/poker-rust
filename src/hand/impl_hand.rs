use super::{Hand, Rank, RankErr, RankInner};
use crate::card::{self, Card, Circular};

use std::fmt;
use std::rc::Rc;

extern crate log;
use log::error;

type CardRef = Rc<Card>;

impl Hand {
    /// Iron rule: Map Vec<Card> to Vec<CardRef>
    /// Creating a new hand will cause all given cards to be automatically
    /// evaluated into a rank
    pub fn new(cards: Vec<Card>) -> Result<Hand, RankErr> {
        let cards = cards.into_iter().map(Rc::new).collect::<Vec<CardRef>>();
        match Hand::ranking(&cards, &Vec::new()) {
            // 2nd arg is a placeholder
            Ok((rank, kickers)) => Ok(Hand {
                cards,
                rank,
                kickers,
            }),
            Err(e) => Err(e),
        }
    }

    // Iron rule: Map cards to Vec<Card>
    pub fn discard(self) -> Vec<Card> {
        self.cards.iter().map(|c| **c).collect()
    }

    /// Takes a slice of community cards and return the best card rank
    ///
    /// If given a slice of length 0, return immediately with an error.
    pub fn ranking(
        cards: &[CardRef],
        community: &[CardRef],
    ) -> Result<(Rank, Vec<CardRef>), RankErr> {
        let cards: Vec<CardRef> = cards.iter().chain(community.iter()).cloned().collect();

        if cards.is_empty() {
            Err(RankErr::Explained(format!(
                "No cards were given. Cards: {:?}",
                cards
            )))
        } else {
            let pair_rank = Hand::pair_rank(&cards)?;
            let option_sf = Hand::straight_flush_rank(&cards);

            // Compare and return the biggest rank
            let rank = match option_sf {
                Some(sf) => std::cmp::max(pair_rank, sf?),
                None => pair_rank,
            };

            // TODO
            let mut cards = cards;
            let kickers = cards.drain_filter(|c| rank.as_slice().contains(c)).collect();

            Ok((rank, kickers))
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn update(&mut self, community: &[CardRef]) -> Result<(), RankErr> {
        let (rank, kickers) = Hand::ranking(&self.cards, community)?;

        self.rank = rank;
        self.kickers = kickers;
        Ok(())
    }

    /// Return the best ranking pair found in `cards`.
    /// Pairs are:
    /// `High`, `Pair`, `TwoPair`, `Trips`, `House`, `Quads` and `Fives`
    ///
    /// This function will always return High, if no other pair was found.
    /// The exception to the rule is if the slice of cards have a size of 0
    /// or something internally went wrong.
    pub fn pair_rank(cards: &[CardRef]) -> Result<Rank, RankErr> {
        let mut pair_groups = Hand::pair_groups(cards);
        let pair_iter = pair_groups.iter_mut().rev();

        // Define some
        let mut quads = None;
        let mut trips = None;
        let mut pairs = (None, None);
        let mut high = None;

        // Build some
        for cards in pair_iter {
            let len = cards.len();

            macro_rules! carrd {
                ($i:expr) => {
                    to_array![cards.drain(..); $i].unwrap()
                };
            };

            match len {
                // Return immediately since Fives can't be beaten
                5 => return Rank::Fives(carrd!(5)),
                4 if quads.is_none() => quads = Some(carrd!(4)),
                3 if trips.is_none() => trips = Some(carrd!(3)),
                2 => {
                    if pairs.0.is_none() {
                        pairs.0 = Some(carrd!(2))
                    } else if pairs.1.is_none() {
                        pairs.1 = Some(carrd!(2))
                    }
                }
                1 if high.is_none() => high = Some(carrd!(1)),
                _ => (),
            }
        }

        // Get some
        match (quads, trips, pairs, high) {
            (Some(quads), _, _, _) => Rank::Quads(quads),
            (_, Some(trips), (Some(pair), _), _) => Rank::House(trips, pair),
            (_, Some(trips), _, _) => Rank::Trips(trips),
            (_, _, (Some(pair0), Some(pair1)), _) => Rank::TwoPair(pair0, pair1),
            (_, _, (Some(pair), _), _) => Rank::Pair(pair),
            (_, _, _, Some(high)) => Rank::High(high),
            _ => Err(RankErr::Explained(format!("TODO Error: {:#?}", cards))),
        }
    }

    /// Maybe returns one rank after checking in order:
    /// **[StraightFlush, Flush, Straight]**
    pub fn straight_flush_rank(cards: &[CardRef]) -> Option<Result<Rank, RankErr>> {
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
    pub fn pair_groups(cards: &[CardRef]) -> Vec<Vec<CardRef>> {
        let mut cards = cards.to_vec();
        cards.sort_by(|a, b| a.cmp_rank_first(**b));

        // Ace rule
        let rotate = cards.len() - cards.iter().filter(|c| c.rank == card::Rank::Ace).count();
        cards.rotate_right(rotate);

        // Value to be returned
        let mut pairs: Vec<Vec<CardRef>> = Vec::new();
        // Main Sequence Generator
        let mut iter = cards.iter().cloned().peekable(); // TODO Should not need peekable()
        let mut temp_vec: Vec<CardRef> = Vec::new();
        let mut prev_rank = iter.peek().unwrap().rank;

        for card in iter {
            if prev_rank == card.rank {
                temp_vec.push(card);
            } else {
                pairs.push(temp_vec);
                prev_rank = card.rank;
                temp_vec = vec![card];
            }
        }

        pairs.push(temp_vec);
        pairs
    }

    /// Returns cards grouped together by these rules:
    /// 1. Cards are sorted by it's suit first.
    /// 2. Cards are grouped together if their neighbor has the same suit.
    pub fn flush_groups(cards: &[CardRef]) -> Vec<Vec<CardRef>> {
        let mut cards = cards.to_vec();
        cards.sort_by(|a, b| a.cmp_suit_first(**b));

        // Value to be returned
        let mut flush_groupings: Vec<Vec<CardRef>> = Vec::new();
        // Main Sequence Generator
        let mut iter = cards.iter().cloned();
        let mut temp_vec;
        let mut prev_suit;

        // First card initiates things
        if let Some(first_card) = iter.next() {
            prev_suit = first_card.suit;
            temp_vec = vec![first_card];

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
                prev_suit = card.suit;
                temp_vec = vec![card];
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
    pub fn straight_groups(cards: &[CardRef]) -> Vec<Vec<CardRef>> {
        let mut cards = cards.to_vec();
        cards.sort_by(|a, b| a.cmp_rank_first(**b));

        // Value to be returned
        let mut straight_groupings = Vec::<Vec<CardRef>>::new();
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
                prev_rank = card.rank;
                temp_vec.push(card);
            // Drop temp_vec into straight_groupings and start a new one
            } else {
                straight_groupings.push(temp_vec);
                prev_rank = card.rank;
                temp_vec = vec![card];
            }
        }

        straight_groupings.push(temp_vec);

        // Ace rule
        match (cards.first(), cards.last(), straight_groupings.last()) {
            (Some(ace), Some(king), Some(broadway))
                if ace.rank == card::Rank::Ace && king.rank == card::Rank::King =>
            {
                let mut broadway = broadway.clone();
                broadway.push(ace.clone());
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
    fn straight_flush_cards(flush_grouping: &[Vec<CardRef>]) -> Option<[CardRef; 5]> {
        for group in flush_grouping.iter().rev().filter(|v| v.len() >= 5) {
            if let Some(cards) = Hand::extract_last_cards(&Hand::straight_groups(&group)) {
                return Some(cards);
            }
        }

        None
    }

    /// Iterate, in reverse, over groupings that has size 5 or over.
    /// Extract it's 5 most valuable cards (last cards).
    fn extract_last_cards(groupings: &[Vec<CardRef>]) -> Option<[CardRef; 5]> {
        if let Some(cards) = groupings.iter().rev().find(|v| v.len() >= 5) {
            let cards = to_array![cards[cards.len()-5..].iter().cloned(); 5];

            if cards.is_some() {
                cards
            } else {
                unreachable!();
            }
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
