use super::{Rank, RankErr, RankInner};
use crate::card::{Card, Rank::Ace, Rank::King};

use std::cmp::Ordering;
use std::fmt;

use std::rc::*;

type ResultRank = Result<Rank, RankErr>;

#[allow(non_snake_case)] // Do not remove
impl Rank {}

type CardRef = Rc<Card>;

impl Rank {
    /// Always Returns one high card.
    pub fn High(card: [CardRef; 1]) -> ResultRank {
        Ok(Rank(RankInner::High(card)))
    }

    /// Returns Pair, if both cards share the same rank
    /// and suit are ordered.
    pub fn Pair(cards: [CardRef; 2]) -> ResultRank {
        let rank = Rank(RankInner::Pair(cards));

        if cards[0].rank != cards[1].rank {
            Err(RankErr::Invalid(rank))
        } else if cards[0] > cards[1] {
            Err(RankErr::Unsorted(rank))
        } else {
            Ok(rank)
        }
    }

    /// Returns Two Pairs, if both pairs is sufficient pairs and
    /// pair.0 is the least significant pair.Ace
    pub fn TwoPair(pair0: [CardRef; 2], pair1: [CardRef; 2]) -> ResultRank {
        let rank = Rank(RankInner::TwoPair(pair0, pair1));

        if let Err(E) = Rank::Pair(pair0) {
            Err(RankErr::Explained(format!(
                "In Rank::TwoPair with rank: {:?} Pair 1 of 2 returned: {:?}",
                rank, E,
            )))
        } else if let Err(E) = Rank::Pair(pair1) {
            Err(RankErr::Explained(format!(
                "In Rank::TwoPair with rank: {:?} Pair 2 of 2 returned: {:?}",
                rank, E,
            )))
        } else if pair0[0].rank == pair1[0].rank {
            Err(RankErr::Explained(format!(
                "In Rank::TwoPair with rank: {:?} Pairs are actually Quads.",
                rank,
            )))
        } else if pair0 > pair1 {
            Err(RankErr::Unsorted(rank))
        } else {
            Ok(rank)
        }
    }

    ///
    pub fn Trips(cards: [CardRef; 3]) -> ResultRank {
        let rank = Rank(RankInner::Trips(cards));

        if cards[0].rank != cards[1].rank || cards[1].rank != cards[2].rank {
            Err(RankErr::Invalid(rank))
        } else if cards[0] > cards[1] || cards[1] > cards[2] {
            Err(RankErr::Unsorted(rank))
        } else {
            Ok(rank)
        }
    }

    ///
    pub fn Straight(cards: [CardRef; 5]) -> ResultRank {
        let rank = Rank(RankInner::Straight(cards));

        let ranks = if cards[3].rank == King {
            if cards[4].rank != Ace {
                return Err(RankErr::Explained(format!(
                    "In Rank::Straight with rank: {:?} King not followed by Ace",
                    rank
                )));
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
                return Err(RankErr::Invalid(rank));
            }
        }

        Ok(rank)
    }

    ///
    pub fn Flush(cards: [CardRef; 5]) -> ResultRank {
        let rank = Rank(RankInner::Flush(cards));

        // See if all suits match
        for card in &cards {
            if cards[0].suit != card.suit {
                return Err(RankErr::Invalid(rank));
            }
        }

        // See if cards are sorted
        for i in 0..=3 {
            if cards[i] > cards[i + 1] {
                return Err(RankErr::Unsorted(rank));
            }
        }

        Ok(rank)
    }

    pub fn House(trips: [CardRef; 3], pair: [CardRef; 2]) -> ResultRank {
        let rank = Rank(RankInner::House(trips, pair));

        // See if both Trips and Pair is ok and return rank
        // Else return an explained error
        if let Err(E) = Rank::Trips(trips) {
            Err(RankErr::Explained(format!(
                "In Rank::House with rank: {:?} Function Trips returned: {:?}",
                rank, E
            )))
        } else if let Err(E) = Rank::Pair(pair) {
            Err(RankErr::Explained(format!(
                "In Rank::House with rank: {:?} Function Pair returned: {:?}",
                rank, E
            )))
        } else {
            Ok(rank)
        }
    }

    pub fn Quads(cards: [CardRef; 4]) -> ResultRank {
        let rank = Rank(RankInner::Quads(cards));
        // See if all ranks match
        for card in &cards {
            if cards[0].rank != card.rank {
                return Err(RankErr::Invalid(rank));
            }
        }

        // See if cards are sorted
        for i in 0..=2 {
            if cards[i] > cards[i + 1] {
                return Err(RankErr::Unsorted(rank));
            }
        }

        Ok(rank)
    }

    pub fn StraightFlush(sf: [CardRef; 5]) -> ResultRank {
        let rank = Rank(RankInner::StraightFlush(sf));

        if let Err(E) = Rank::Straight(sf) {
            Err(RankErr::Explained(format!(
                "In Rank::StraightFlush with rank: {:?} Function Rank::Straight returned: {:?}",
                rank, E
            )))
        } else if let Err(E) = {
            // Ace is always sorted last for Flush()
            if sf[4].rank == Ace {
                Rank::Flush([sf[4], sf[0], sf[1], sf[2], sf[3]])
            } else {
                Rank::Flush(sf)
            }
        } {
            Err(RankErr::Explained(format!(
                "In Rank::StraightFlush with rank: {:?}. Function Rank::Flush returned: {:?}",
                rank, E
            )))
        } else {
            Ok(rank)
        }
    }

    pub fn Fives(cards: [CardRef; 5]) -> ResultRank {
        let rank = Rank(RankInner::Fives(cards));

        // See if all values match
        for card in &cards {
            if cards[0].rank != card.rank {
                return Err(RankErr::Invalid(rank));
            }
        }

        // See if cards are sorted
        for i in 0..=2 {
            if cards[i] > cards[i + 1] {
                return Err(RankErr::Unsorted(rank));
            }
        }

        Ok(rank)
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
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
        let default = || self.cmp(other);

        let ord: Option<Ordering> = match (self.0, other.0) {
            (RankInner::High(this), RankInner::High(other)) => {
                if this[0].rank == other[0].rank {
                    Some(default())
                } else if this[0].rank == Ace {
                    Some(Ordering::Greater)
                } else if other[0].rank == Ace {
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
