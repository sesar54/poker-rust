use super::{rank::Rank, Error, Hand};
use crate::card::{
    Card,
    Rank::{Ace, King},
};
use mimpl::mimpl;
use std::convert::{TryFrom, TryInto};
use std::error;
use std::fmt;
use std::ops::Range;
use std::sync::Arc;
use std::thread;

type Result<R> = std::result::Result<R, Box<Error>>;
type Outcome<R> = Option<Result<R>>;

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
        let cards: Arc<Vec<Card>> =
            Arc::new(cards.iter().chain(community.iter()).cloned().collect());
        let pair_thread;
        let sf_thread;
        {
            let cards_ref = Arc::clone(&cards);
            pair_thread = thread::spawn(move || crate::hand::Hand::pair_rank(&cards_ref));
        }
        {
            let cards_ref = Arc::clone(&cards);
            sf_thread = thread::spawn(move || crate::hand::Hand::straight_flush_rank(&cards_ref));
        }

        pair_thread.join().expect("Thread sf crashed");

        unimplemented!();
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
        let mut pair_cards = Hand::pair_pattern(cards.clone().into()).collect::<Vec<_>>();
        let largest_pair = pair_cards.pop();

        if let Some(largest_pair) = largest_pair {
            match largest_pair.len() {
                5.. => Rank::fives_try_from(&largest_pair),
                4 => Rank::quads_try_from(&largest_pair),
                // Check house before trips
                3 => {
                    if let Some(pair) = pair_cards.iter().filter(|cards| cards.len() == 2).last() {
                        Rank::house_try_from(&largest_pair, &pair)
                    } else {
                        Rank::trips_try_from(&largest_pair)
                    }
                }
                // Check Two Pair before Pair
                2 => {
                    if let Some(pair) = pair_cards.iter().filter(|cards| cards.len() == 2).last() {
                        Rank::two_pair_try_from(&largest_pair, &pair)
                    } else {
                        Rank::pair_try_from(&largest_pair)
                    }
                }
                1 => Ok(Rank::high_from(&largest_pair[0])),
                _ => unreachable!(),
            }
        } else {
            Err(Error::EmptyHand)
        }
        .map_err(Box::new)
    }

    /// Maybe returns one rank after checking in order:
    /// **[StraightFlush, Flush, Straight]**
    pub fn straight_flush_rank(cards: &[Card]) -> Outcome<Rank> {
        // Look where flush_grouping is used and draw some conclusions on what
        // is happening and in which order.
        let cards: Box<[Card]> = cards.clone().into();

        // This thread turns all cards into a flush pattern,
        // filters only patterns equal or greater than 5 in lenght,
        // copies the last pattern (if any) for safekeeping.
        //
        // Then turn all previous patterns and sees if there are any of straight
        // patterns inside with length 5 or more.
        // If so try to verify it as a rank and return whatever comes out of it.
        //
        // Else try to verify the last flush pattern
        // and return what comes of it.
        //
        // Then return an optional of a result.
        let flush_candidates = cards.clone();
        let flush_thread = thread::spawn(move || {
            let flush_cards = Hand::flush_pattern(flush_candidates)
                .filter(|cards| cards.len() >= 5)
                .collect::<Vec<_>>();

            let flush_opt = flush_cards.last().cloned();

            flush_cards
                .into_iter()
                .flat_map(Hand::straight_pattern)
                .filter(|cards| cards.len() >= 5)
                .last()
                .map_or_else(
                    || flush_opt.map(|flush| Rank::flush_try_from(&flush)),
                    |sf| Some(Rank::straight_flush_try_from(&sf)),
                )
        });

        // This thread maps  all cards to a straight pattern,
        // filters only patterns equal or greater than 5 in length,
        // takes the last of the patterns
        // and try to get it verified as a straight.
        let straight_candidates = cards.clone();
        let straigh_thread = thread::spawn(move || {
            Hand::straight_pattern(straight_candidates)
                .filter(|cards| cards.len() >= 5)
                .last()
                .map(|card| Rank::straight_try_from(&card))
        });

        // Merges the result of threads.
        // Flush thread might not return a None in which case straight might
        // return something.
        //
        // Then box the error (if any).
        flush_thread
            .join()
            .unwrap()
            .or_else(|| straigh_thread.join().unwrap())
            .map(|result| result.map_err(Box::new))
    }

    /// Returns cards grouped together by these rules:
    pub fn pair_pattern(mut cards: Box<[Card]>) -> impl Iterator<Item = Box<[Card]>> {
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
    pub fn straight_pattern(mut cards: Box<[Card]>) -> impl Iterator<Item = Box<[Card]>> {
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
            (Some(first), Some(last)) if first.rank == Ace && last.rank == King => {
                let mut broadway = cards[ranges.pop().unwrap()].to_vec();
                broadway.push(*first);
                Some(broadway.into_boxed_slice())
            }
            _ => None,
        };

        ranges
            .into_iter()
            .map(move |range| cards[range].into())
            .chain(broadway)
    }

    /// Returns cards grouped together by these rules:
    /// 1. Cards are sorted by it's suit first.
    /// 2. Cards are grouped together if their neighbor has the same suit.
    pub fn flush_pattern(mut cards: Box<[Card]>) -> impl Iterator<Item = Box<[Card]>> {
        cards.sort_by(|a, b| a.suit.cmp(&b.suit));

        let mut ranges = vec![];
        {
            let mut iter = cards.iter();
            let mut start = 0;
            while let Some(suit) = iter.by_ref().map(|c| c.suit).next() {
                let counter = iter.clone().take_while(|c| c.suit == suit).count();
                iter.by_ref().skip(counter);
                let end = start + counter;
                ranges.push(Range { start, end });
                start = end;
            }
        }

        ranges.into_iter().map(move |range| cards[range].into())
    }
}
