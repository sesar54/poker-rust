use std::fmt;

use crate::card::{
    Card,
    Value::{Ace, King},
};
use crate::holdem::{Rank, RankErr, RankErr::*, RankInner};

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
            RankInner::StraightFlush(card, ..) => match card.value {
                Ace => write!(f, "Royal flush"),
                _ => write!(f, "Straight flush"),
            },
            RankInner::Fives(..) => write!(f, "Five of a kind"),
        }
    }
}

type ResultRank = Result<Rank, RankErr>;

#[allow(non_snake_case)]
impl Rank {
    /// Always Returns one high card.
    pub fn High(card: Card) -> ResultRank {
        Ok(Rank(RankInner::High(card)))
    }

    /// Returns Pair, if both cards share the same value
    /// and suit are ordered.
    pub fn Pair(cards: [Card; 2]) -> ResultRank {
        let rank = Rank(RankInner::Pair(cards[0], cards[1]));

        if cards[0].value != cards[1].value {
            Err(Invalid(rank))
        } else if cards[0] > cards[1] {
            Err(Unsorted(rank))
        } else {
            Ok(rank)
        }
    }

    /// Returns Two Pairs, if both pairs is sufficient pairs and
    /// pair.0 is the least significant pair.Ace
    pub fn TwoPair(pair0: [Card; 2], pair1: [Card; 2]) -> ResultRank {
        let rank = Rank(RankInner::TwoPair(
            (pair0[0], pair0[1]),
            (pair1[0], pair1[1]),
        ));

        if let Err(E) = Rank::Pair(pair0) {
            Err(Explained(format!(
                "In Rank::TwoPair with rank: {:#?}\nPair 1 of 2 returned: {:#?}",
                rank, E,
            )))
        } else if let Err(E) = Rank::Pair(pair1) {
            Err(Explained(format!(
                "In Rank::TwoPair with rank: {:#?}\nPair 2 of 2 returned: {:#?}",
                rank, E,
            )))
        } else if pair0 > pair1 {
            Err(Unsorted(rank))
        } else {
            Ok(rank)
        }
    }

    /// Returns
    pub fn Trips(cards: [Card; 3]) -> ResultRank {
        let rank = Rank(RankInner::Trips(cards[0], cards[1], cards[2]));

        if cards[0].value != cards[1].value || cards[1].value != cards[2].value {
            Err(Invalid(rank))
        } else if cards[0] > cards[1] || cards[1] > cards[2] {
            Err(Unsorted(rank))
        } else {
            Ok(rank)
        }
    }

    pub fn Straight(cards: [Card; 5]) -> ResultRank {
        let rank = Rank(RankInner::Straight(
            cards[0], cards[1], cards[2], cards[3], cards[4],
        ));

        let values: &[Card] = if cards[3].value == King {
            if cards[4].value != Ace {
                return Err(Explained(format!(
                    "In Rank::Straight with rank: {:#?}\nKing not followed by Ace",
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
        for i in 0..values.len() {
            if values[0].value as u8 + i as u8 != values[i].value as u8 {
                return Err(Invalid(rank));
            }
        }

        Ok(rank)
    }

    ///
    pub fn Flush(cards: [Card; 5]) -> ResultRank {
        let rank = Rank(RankInner::Flush(
            cards[0], cards[1], cards[2], cards[3], cards[4],
        ));

        // See if all suits match
        for card in &cards {
            if cards[0].suit != card.suit {
                return Err(Invalid(rank));
            }
        }

        // See if cards are sorted
        for i in 0..=3 {
            if cards[i] > cards[i + 1] {
                return Err(Unsorted(rank));
            }
        }

        Ok(rank)
    }

    pub fn House(trips: [Card; 3], pair: [Card; 2]) -> ResultRank {
        let rank = Rank(RankInner::House(
            (trips[0], trips[1], trips[2]),
            (pair[0], pair[1]),
        ));

        // See if both Trips and Pair is ok and return rank
        // Else return an explained error
        if let Err(E) = Rank::Trips(trips) {
            Err(Explained(format!(
                "In Rank::House with rank: {:#?}\nFunction Trips returned: {:#?}",
                rank, E
            )))
        } else if let Err(E) = Rank::Pair(pair) {
            Err(Explained(format!(
                "In Rank::House with rank: {:#?}\nFunction Pair returned: {:#?}",
                rank, E
            )))
        } else {
            Ok(rank)
        }
    }

    pub fn Quads(cards: [Card; 4]) -> ResultRank {
        let rank = Rank(RankInner::Quads(cards[0], cards[1], cards[2], cards[3]));
        // See if all values match
        for card in &cards {
            if cards[0].value != card.value {
                return Err(Invalid(rank));
            }
        }

        // See if cards are sorted
        for i in 0..=2 {
            if cards[i] > cards[i + 1] {
                return Err(Unsorted(rank));
            }
        }

        Ok(rank)
    }

    pub fn StraightFlush(sf: [Card; 5]) -> ResultRank {
        let rank = Rank(RankInner::StraightFlush(sf[0], sf[1], sf[2], sf[3], sf[4]));

        if let Err(E) = Rank::Straight(sf) {
            Err(Explained(format!(
                "In Rank::StraightFlush with rank: {:#?}\nFunction Rank::Straight returned: {:#?}",
                rank, E
            )))
        } else if let Err(E) = {
            // Ace is always sorted last for Flush()
            if sf[4].value == Ace {
                Rank::Flush([sf[4], sf[0], sf[1], sf[2], sf[3]])
            } else {
                Rank::Flush(sf)
            }
        } {
            Err(Explained(format!(
                "In Rank::StraightFlush with rank: {:#?}\nFunction Rank::Flush returned: {:#?}",
                rank, E
            )))
        } else {
            Ok(rank)
        }
    }

    pub fn Fives(cards: [Card; 5]) -> ResultRank {
        let rank = Rank(RankInner::Fives(
            cards[0], cards[1], cards[2], cards[3], cards[4],
        ));

        // See if all values match
        for card in &cards {
            if cards[0].value != card.value {
                return Err(Invalid(rank));
            }
        }

        // See if cards are sorted
        for i in 0..=2 {
            if cards[i] > cards[i + 1] {
                return Err(Unsorted(rank));
            }
        }

        Ok(rank)
    }
}
