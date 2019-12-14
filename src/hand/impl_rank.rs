use super::{Rank, RankErr, RankInner};
use crate::card::{Card, Rank::Ace, Rank::King};

use std::cmp::Ordering;
use std::fmt;
use std::ops::Index;

use std::rc::*;

type ResultRank = Result<Rank, RankErr>;

impl Index<usize> for Rank {
    type Output = CardRef;

    fn index(&self, index: usize) -> &Self::Output {
        use RankInner::*;

        &match self.0 {
            High(arr) => arr[index],
            Pair(arr) => arr[index],
            TwoPair(arr0, arr1) => {
                if index < arr0.len() {
                    arr0[index]
                } else {
                    arr1[index]
                }
            }
            Trips(arr) => arr[index],
            Straight(arr) => arr[index],
            Flush(arr) => arr[index],
            House(arr0, arr1) => {
                if index < arr0.len() {
                    arr0[index]
                } else {
                    arr1[index]
                }
            }
            Quads(arr) => arr[index],
            StraightFlush(arr) => arr[index],
            Fives(arr) => arr[index],
        }
    }
}

impl IntoIterator for Rank {
    type Item = CardRef;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        use RankInner::*;

        match self.0 {
            High(arr) => arr.into_iter(),
            unimplemented!()

        }


    }


}

type CardRef = Rc<Card>;

macro_rules! explained {
    ($string:expr, $($value:expr),*) => {
        Err(RankErr::Explained(format!($string, $( $value, )*)))
    };
}

macro_rules! unsorted {
    ($value:expr) => {
        Err(RankErr::Unsorted($value))
    };
}

macro_rules! invalid {
    ($value:expr) => {
        Err(RankErr::Invalid($value))
    };
}

#[allow(non_snake_case)] // Do not remove
impl Rank {
    /// Constructors
    /// Always Returns one high card.
    pub fn High(card: [CardRef; 1]) -> ResultRank {
        Ok(Rank(RankInner::High(card)))
    }

    /// Returns Pair, if both cards share the same rank
    /// and suit are ordered.
    pub fn Pair(cards: [CardRef; 2]) -> ResultRank {
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
    pub fn TwoPair(pair0: [CardRef; 2], pair1: [CardRef; 2]) -> ResultRank {
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
    pub fn Trips(cards: [CardRef; 3]) -> ResultRank {
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
    pub fn Straight(cards: [CardRef; 5]) -> ResultRank {
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
                return Err(RankErr::Invalid(rank(cards)));
            }
        }

        Ok(rank(cards))
    }

    ///
    pub fn Flush(cards: [CardRef; 5]) -> ResultRank {
        let rank = |cards| Rank(RankInner::Flush(cards));

        // See if all suits match
        for card in &cards {
            if cards[0].suit != card.suit {
                return Err(RankErr::Invalid(rank(cards)));
            }
        }

        // See if cards are sorted
        for i in 0..=3 {
            if cards[i] > cards[i + 1] {
                return Err(RankErr::Unsorted(rank(cards)));
            }
        }

        Ok(rank(cards))
    }

    pub fn House(trips: [CardRef; 3], pair: [CardRef; 2]) -> ResultRank {
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

    pub fn Quads(cards: [CardRef; 4]) -> ResultRank {
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

    pub fn StraightFlush(cards: [CardRef; 5]) -> ResultRank {
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

    pub fn Fives(cards: [CardRef; 5]) -> ResultRank {
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

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            RankInner::High(..) => write!(f, "High card"),
            RankInner::Pair(..) => write!(f, "Pair"),
            RankInner::TwoPair(..) => write!(f, "Two pairs"),
            RankInner::Trips(..) => write!(f, "Three of a kind"),
            RankInner::Straight(..) => write!(f, "Straight"),
            RankInner::Flush(..) => write!(f, "Flush"),
            RankInner::House(..) => write!(f, "Full house"),
            RankInner::Quads(..) => write!(f, "Four of a kind"),
            RankInner::StraightFlush(cards) => match cards[4].rank {
                Ace => write!(f, "Royal flush"),
                _ => write!(f, "Straight flush"),
            },
            RankInner::Fives(..) => write!(f, "Five of a kind"),
        }
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        if let (RankInner::High(high0), RankInner::High(high1)) = (&self.0, &other.0) {
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
