use super::{r#type, Error, Rank, RankInner};
use crate::card::CardRef;
use crate::card::{Rank::Ace, Rank::King};

use std::cmp::Ordering;
use std::fmt;
use std::ops::Index;

type ResultRank = Result<Rank, Error>;

macro_rules! explained {
    ($string:expr, $($value:expr),*) => {
        Err(Error::Explained(format!($string, $( $value, )*)))
    };
}

macro_rules! unsorted {
    ($value:expr) => {
        Err(Error::Unsorted($value))
    };
}

macro_rules! invalid {
    ($value:expr) => {
        Err(Error::Invalid($value))
    };
}

#[allow(non_snake_case)] // Do not remove
impl Rank {
    /// Constructors
    /// Always Returns one high card.
    pub fn High(cards: r#type::High) -> ResultRank {
        Ok(Rank(RankInner::High(cards)))
    }

    /// Returns Pair, if both cards share the same rank
    /// and suit are ordered.
    pub fn Pair(cards: r#type::Pair) -> ResultRank {
        let rank = |cards| Rank(RankInner::Pair(cards));

        if cards[0].rank != cards[1].rank {
            invalid!(rank(cards))
        } else if cards[0] > cards[1] {
            unsorted!(rank(cards))
        } else {
            Ok(rank(cards))
        }
    }

    /// Returns Two Pairs, if both pairs is sufficient pairs and
    /// pair.0 is the least significant pair.Ace
    pub fn TwoPair(pair0: r#type::Pair, pair1: r#type::Pair) -> ResultRank {
        // Value to be returned.
        // Create TwoPair out of Two Pairs inner array. Called once.
        let rank = |pair0: Rank, pair1: Rank| {
            if let (RankInner::Pair(pair0), RankInner::Pair(pair1)) = (pair0.0, pair1.0) {
                Rank(RankInner::TwoPair(pair0, pair1))
            } else {
                unreachable!()
            }
        };

        // Error format to be used
        macro_rules! err {
            () => {
                "In Rank::TwoPair: [ pair0: {:?}, pair1: {:?} ]"
            };
        }

        match (Rank::Pair(pair0), Rank::Pair(pair1)) {
            (Err(E), pair1) => explained!(err!(), E, pair1),
            (pair0, Err(E)) => explained!(err!(), pair0, E),
            (Ok(pair0), Ok(pair1)) => {
                if pair0 > pair1 {
                    unsorted!(rank(pair0, pair1))
                } else {
                    Ok(rank(pair0, pair1))
                }
            }
        }
    }

    ///
    pub fn Trips(cards: r#type::Trips) -> ResultRank {
        let rank = |cards| Rank(RankInner::Trips(cards));

        if cards[0].rank != cards[1].rank || cards[1].rank != cards[2].rank {
            invalid!(rank(cards))
        } else if cards[0] > cards[1] || cards[1] > cards[2] {
            unsorted!(rank(cards))
        } else {
            Ok(rank(cards))
        }
    }

    ///
    pub fn Straight(cards: r#type::Straight) -> ResultRank {
        let rank = |cards| Rank(RankInner::Straight(cards));

        let ranks = if cards[3].rank == King {
            if cards[4].rank != Ace {
                return explained!(
                    "In Rank::Straight with rank: {:?} King not followed by Ace",
                    rank(cards)
                );
            }

            &cards[0..3]
        } else {
            &cards[0..4]
        };

        // See if cards[0] is greater than every other item by i amount
        // Also check if cards are in order.
        // Ace is not included in this range. See above
        for i in 0..ranks.len() {
            if ranks[0].rank as u8 + i as u8 != ranks[i].rank as u8 {
                return Err(Error::Invalid(rank(cards)));
            }
        }

        Ok(rank(cards))
    }

    ///
    pub fn Flush(cards: r#type::Flush) -> ResultRank {
        let rank = |cards| Rank(RankInner::Flush(cards));

        // See if all suits match
        for card in &cards {
            if cards[0].suit != card.suit {
                return Err(Error::Invalid(rank(cards)));
            }
        }

        // See if cards are sorted
        for i in 0..=3 {
            if cards[i] > cards[i + 1] {
                return Err(Error::Unsorted(rank(cards)));
            }
        }

        Ok(rank(cards))
    }

    pub fn House(trips: r#type::Trips, pair: r#type::Pair) -> ResultRank {
        // Value to be returned
        let rank = |trips: Rank, pair: Rank| {
            if let (RankInner::Trips(trips), RankInner::Pair(pair)) = (trips.0, pair.0) {
                Rank(RankInner::House(trips, pair))
            } else {
                unreachable!()
            }
        };

        // Error format to used returned
        macro_rules! err {
            () => {
                "In Rank::House: [trips: {:?}, pair: {:?}]."
            };
        }

        // See if both Trips and Pair is ok and return rank
        // Else return an explained error
        match (Rank::Trips(trips), Rank::Pair(pair)) {
            (Err(E), pair) => explained!(err!(), E, pair),
            (trips, Err(E)) => explained!(err!(), trips, E),
            (Ok(trips), Ok(pair)) => Ok(rank(trips, pair)),
        }
    }

    pub fn Quads(cards: r#type::Quads) -> ResultRank {
        let rank = |cards| Rank(RankInner::Quads(cards));
        // See if all ranks match
        for card in &cards {
            if cards[0].rank != card.rank {
                return invalid!(rank(cards));
            }
        }

        // See if cards are sorted
        for i in 0..=2 {
            if cards[i] > cards[i + 1] {
                return unsorted!(rank(cards));
            }
        }

        Ok(rank(cards))
    }

    pub fn StraightFlush(cards: r#type::StraightFlush) -> ResultRank {
        let rank = |cards| Rank(RankInner::StraightFlush(cards));

        let mut cards = match Rank::Straight(cards) {
            Ok(Straight) => Straight.drop_Straight(),
            Err(E) => {
                return explained!(
                    "In Rank::StraightFlush: Function Rank::Straight returned: {:?}",
                    E
                )
            }
        };

        let flush_check = |cards| match Rank::Flush(cards) {
            Ok(Flush) => Ok(Flush.drop_Flush()),
            Err(E) => {
                return explained!(
                    "In Rank::StraightFlush: Function Rank::Flush returned: {:?}",
                    E
                )
            }
        };

        let cards = if cards[4].rank == Ace {
            cards.rotate_right(1);
            let mut cards = flush_check(cards)?;
            cards.rotate_left(1);
            cards
        } else {
            flush_check(cards)?
        };

        Ok(rank(cards))
    }

    pub fn Fives(cards: r#type::Fives) -> ResultRank {
        let rank = |cards| Rank(RankInner::Fives(cards));

        // See if all values match
        for card in &cards {
            if cards[0].rank != card.rank {
                return invalid!(rank(cards));
            }
        }

        // See if cards are sorted
        for i in 0..=2 {
            if cards[i] > cards[i + 1] {
                return unsorted!(rank(cards));
            }
        }

        Ok(rank(cards))
    }
}

#[allow(non_snake_case)] // Do not remove
impl Rank {
    /// Deconstructors
    pub fn drop_High(self) -> [CardRef; 1] {
        match self.0 {
            RankInner::High(cards) => cards,
            _ => unreachable!(),
        }
    }

    pub fn drop_Pair(self) -> [CardRef; 2] {
        match self.0 {
            RankInner::Pair(cards) => cards,
            _ => unreachable!(),
        }
    }
    pub fn drop_TwoPair(self) -> ([CardRef; 2], [CardRef; 2]) {
        match self.0 {
            RankInner::TwoPair(cards0, cards1) => (cards0, cards1),
            _ => unreachable!(),
        }
    }

    pub fn drop_Trips(self) -> [CardRef; 3] {
        match self.0 {
            RankInner::Trips(cards) => cards,
            _ => unreachable!(),
        }
    }

    pub fn drop_Straight(self) -> [CardRef; 5] {
        match self.0 {
            RankInner::Straight(cards) => cards,
            _ => unreachable!(),
        }
    }
    pub fn drop_Flush(self) -> [CardRef; 5] {
        match self.0 {
            RankInner::Flush(cards) => cards,
            _ => unreachable!(),
        }
    }
    pub fn drop_House(self) -> ([CardRef; 3], [CardRef; 2]) {
        match self.0 {
            RankInner::House(trips, pair) => (trips, pair),
            _ => unreachable!(),
        }
    }

    pub fn drop_Quads(self) -> [CardRef; 4] {
        match self.0 {
            RankInner::Quads(cards) => cards,
            _ => unreachable!(),
        }
    }
    pub fn drop_StraightFlush(self) -> [CardRef; 5] {
        match self.0 {
            RankInner::StraightFlush(cards) => cards,
            _ => unreachable!(),
        }
    }
    pub fn drop_Fives(self) -> [CardRef; 5] {
        match self.0 {
            RankInner::Fives(cards) => cards,
            _ => unreachable!(),
        }
    }
}

#[allow(clippy::len_without_is_empty)]
impl Rank {
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self)
    }

    /// Will return number of cards in Rank. This is constant.
    /// * High            => 1
    /// * Pair            => 2
    /// * TwoPair         => 4
    /// * Trips           => 3
    /// * Straight        => 5
    /// * Flush           => 5
    /// * House           => 5
    /// * Quads           => 4
    /// * StraightFlush   => 5
    /// * Fives           => 5
    pub fn len(&self) -> usize {
        use RankInner::*;

        match &self.0 {
            High(..) => 1,
            Pair(..) => 2,
            TwoPair(..) => 4,
            Trips(..) => 3,
            Straight(..) => 5,
            Flush(..) => 5,
            House(..) => 5,
            Quads(..) => 4,
            StraightFlush(..) => 5,
            Fives(..) => 5,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RankInner::*;

        match &self.0 {
            High(..) => write!(f, "High card"),
            Pair(..) => write!(f, "Pair"),
            TwoPair(..) => write!(f, "Two pairs"),
            Trips(..) => write!(f, "Three of a kind"),
            Straight(..) => write!(f, "Straight"),
            Flush(..) => write!(f, "Flush"),
            House(..) => write!(f, "Full house"),
            Quads(..) => write!(f, "Four of a kind"),
            StraightFlush(cards) => match cards[4].rank {
                Ace => write!(f, "Royal flush"),
                _ => write!(f, "Straight flush"),
            },
            Fives(..) => write!(f, "Five of a kind"),
        }
    }
}

// TODO
impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        use RankInner::*;

        if let (High(high0), High(high1)) = (&self.0, &other.0) {
            if high0[0].rank == high1[0].rank {
                self.cmp(other)
            } else if high0[0].rank == Ace {
                Ordering::Greater
            } else if high1[0].rank == Ace {
                Ordering::Less
            } else {
                self.cmp(other)
            }
        } else {
            self.cmp(other)
        }
    }
}

impl Index<usize> for Rank {
    type Output = CardRef;

    fn index(&self, index: usize) -> &Self::Output {
        use RankInner::*;

        match &self.0 {
            High(arr) => &arr[index],
            Pair(arr) => &arr[index],
            TwoPair(arr0, arr1) => {
                if index < arr0.len() {
                    &arr0[index]
                } else {
                    &arr1[index]
                }
            }
            Trips(arr) => &arr[index],
            Straight(arr) => &arr[index],
            Flush(arr) => &arr[index],
            House(arr0, arr1) => {
                if index < arr0.len() {
                    &arr0[index]
                } else {
                    &arr1[index]
                }
            }
            Quads(arr) => &arr[index],
            StraightFlush(arr) => &arr[index],
            Fives(arr) => &arr[index],
        }
    }
}

pub struct Iter<'a> {
    iter: usize,
    len: usize,
    rank: &'a Rank,
}

impl<'a> Iter<'a> {
    fn new(rank: &'a Rank) -> Iter<'a> {
        Iter {
            iter: 0,
            len: rank.len(),
            rank,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a CardRef;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.iter < self.len {
            Some(&self.rank[self.iter])
        } else {
            None
        };

        self.iter += 1;
        result
    }
}
