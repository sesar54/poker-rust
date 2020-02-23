use super::{
    rank::{mediator, Rank},
    EmptyHandError, Hand,
};
use crate::card::{self, Card};
use std::convert::TryFrom;
use std::error;
use std::fmt;
extern crate log;
use std::convert::TryInto;
use std::ops::Range;
use std::thread;

pub type Result<R> = std::result::Result<R, Box<dyn error::Error>>;

impl Hand {
    /// Creating a new hand will cause all given cards to be automatically
    /// evaluated into a rank
    pub fn new(cards: Vec<Card>) -> Result<Hand> {
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
        self.cards
    }

    /// Takes a slice of community cards and return the best card rank
    ///
    /// If given a slice of length 0, return immediately with an error.
    pub fn ranking(cards: &[Card], community: &[Card]) -> Result<(Rank, Vec<Card>)> {
        let cards: Vec<Card> = cards.iter().chain(community.iter()).cloned().collect();

        if cards.is_empty() {
            Err(EmptyHandError {}.into())
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
            let kickers = cards
                .drain_filter(|card0| rank.to_boxed_slice().iter().any(|card1| card1 == card0))
                .collect();

            Ok((rank, kickers))
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn update(&mut self, community: &[Card]) -> Result<()> {
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
    pub fn pair_rank(cards: &[Card]) -> Result<Rank> {
        // Define some
        let mut quads = None;
        let mut trips = None;
        let mut pairs = (None, None);
        let mut high = None;

        // Build some
        for cards in Hand::pair_pattern(cards.clone().into()).iter() {
            let cards: &[Card] = &cards;
            let len = cards.len();

            match len {
                // Return immediately since Fives can't be beaten
                5 => {
                    return Rank::try_from(mediator::Fives(cards.try_into().unwrap()))
                        .map_err(|e| e.into())
                }
                4 if quads.is_none() => quads = Some(mediator::Quads(cards.try_into().unwrap())),
                3 if trips.is_none() => trips = Some(mediator::Trips(cards.try_into().unwrap())),
                2 => {
                    if pairs.0.is_none() {
                        pairs.0 = Some(mediator::Pair(cards.try_into().unwrap()))
                    } else if pairs.1.is_none() {
                        pairs.1 = Some(mediator::Pair(cards.try_into().unwrap()))
                    }
                }
                1 if high.is_none() => high = Some(mediator::High(cards[1])),
                _ => (),
            }
        }

        // Get some
        match (quads, trips, pairs, high) {
            (Some(quads), _, _, _) => Rank::try_from(quads),
            (_, Some(trips), (Some(pair), _), _) => Rank::try_from(mediator::House { trips, pair }),
            (_, Some(trips), _, _) => Rank::try_from(trips),
            (_, _, (Some(pair0), Some(pair1)), _) => {
                Rank::try_from(mediator::TwoPair(pair0, pair1))
            }
            (_, _, (Some(pair), _), _) => Rank::try_from(pair),
            (_, _, _, Some(high)) => Ok(Rank::from(high)),
            _ => unimplemented!(),
            //_ => Err(Error::Explained(format!("TODO Error: {:#?}", cards))),
        }
        .map_err(|e| e.into())
    }

    /// Maybe returns one rank after checking in order:
    /// **[StraightFlush, Flush, Straight]**
    pub fn straight_flush_rank(cards: &[Card]) -> Option<Result<Rank>> {
        // Look where flush_grouping is used and draw some conclusions on what
        // is happening and in which order.
        let cards: Box<[Card]> = cards.clone().into();

        let flush_candidates = cards.clone();
        let flush_thread = thread::spawn(move || {
            let flush_cards = Hand::flush_pattern(flush_candidates);
            let flush_iter = flush_cards.iter().rev().filter(|cards| cards.len() >= 5);

            if let Some(straight_flush) = flush_iter
                .cloned()
                .flat_map(|cards| Hand::straight_pattern(cards).iter())
                .find_map(|&cards| if cards.len() >= 5 { Some(cards) } else { None })
            {
                Some(Rank::try_from(mediator::Straight::try_from(
                    straight_flush,
                )?))
            //} else if let Some(flush) = flush_iter.next() {
            //    Some(flush)
            } else {
                None
            }
        });

        let straight_candidates = cards.clone();

        let straigh_thread = thread::spawn(move || {
            let straight_pattern = Hand::straight_pattern(straight_candidates);
        });

        flush_thread.join();
        straigh_thread.join();
        None

        /*
        let result = if let Some(straight_flush) = Hand::straight_flush_cards(&flush_grouping) {
            match Rank::StraightFlush(straight_flush) {
                    Err(e) => Err(SomeError::Explained(format!("Function straight_cards() with grouping from flush_groups() generated a false positive. Error: {:?}", e))),
                    sf => sf
                }
        } else if let Some(flush) = Hand::extract_last_cards(&flush_grouping) {
            match Rank::Flush(flush) {
                    Err(e) => Err(SomeError::Explained(format!("Function flush_cards() with grouping from flush_groups() generated a false positive. Error: {:?}", e))),
                    flush => flush
                }
        } else if let Some(straight) =
            Hand::extract_last_cards(&Hand::straight_groups(cards.as_slice()))
        {
            match Rank::Straight(straight) {
                    Err(e) => Err(SomeError::Explained(format!("Function straight_cards() with grouping from straight_groups() generated a false positive. Error: {:?}", e))),
                    straight => straight
                }
        } else {
            return None;
        };

        Some(result)*/
    }

    /// Returns cards grouped together by these rules:
    pub fn pair_pattern(mut cards: Box<[Card]>) -> Box<[Box<[Card]>]> {
        cards.sort();

        let mut ranges = vec![];
        {
            let mut start = 0;
            let mut iter = cards.iter();

            while let Some(rank) = iter.by_ref().map(|card| card.rank).next() {
                let counter = iter.clone().filter(|card| card.rank == rank).count();
                iter.by_ref().skip(counter);
                let end = start + counter;
                ranges.push(Range { start, end });
                start = end;
            }
        }

        ranges
            .into_iter()
            .map(move |range| cards[range].to_vec().into_boxed_slice())
            .collect()
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
    pub fn straight_pattern(mut cards: Box<[Card]>) -> Box<[Box<[Card]>]> {
        cards.sort_by(|a, b| a.rank.cmp(&b.rank));

        let mut ranges = vec![];
        {
            let mut iter = cards.iter();

            let mut start = 0;
            while let Some(mut rank) = iter.by_ref().map(|card| card.rank).next() {
                let counter = iter
                    .clone()
                    .filter(|card| card.rank == rank || card.rank == rank.next())
                    .count();
                iter.by_ref().skip(counter);
                let end = start + counter;
                ranges.push(Range { start, end });
                start = end;
            }
        }

        let broadway = match (cards.first(), cards.last()) {
            (Some(first), Some(last))
                if first.rank == card::Rank::Ace && last.rank == card::Rank::King =>
            {
                let mut broadway = cards[ranges.pop().unwrap()].to_vec();
                broadway.push(*first);
                Some(broadway.into_boxed_slice())
            }
            _ => None,
        };

        ranges
            .into_iter()
            .map(|range| cards[range].into())
            .chain(broadway)
            .collect()
    }

    /// Returns cards grouped together by these rules:
    /// 1. Cards are sorted by it's suit first.
    /// 2. Cards are grouped together if their neighbor has the same suit.
    pub fn flush_pattern(mut cards: Box<[Card]>) -> Box<[Box<[Card]>]> {
        cards.sort_by(|a, b| a.suit.cmp(&b.suit));

        let mut ranges = vec![];
        {
            let mut iter = cards.iter();
            let mut start = 0;
            while let Some(suit) = iter.by_ref().map(|card| card.suit).next() {
                let counter = iter.clone().take_while(|card| card.suit == suit).count();
                iter.by_ref().skip(counter);
                let end = start + counter;
                ranges.push(Range { start, end });
                start = end;
            }
        }

        ranges
            .into_iter()
            .map(|range| cards[range].into())
            .collect()
    }
}

mimpl!(Display; EmptyHandError, |this: &EmptyHandError, f: &mut fmt::Formatter|
    write!(f, "{:?}", this)
);

impl error::Error for EmptyHandError {}
