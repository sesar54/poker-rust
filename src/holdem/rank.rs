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
    pub fn Pair(pair: (Card, Card)) -> ResultRank {
        let rank = Rank(RankInner::Pair(pair.0, pair.1));

        if pair.0.value != pair.1.value {
            Err(Invalid(rank))
        } else if pair.0 < pair.1 {
            Err(Unsorted(rank))
        } else {
            Ok(rank)
        }
    }

    /// Returns Two Pairs, if both pairs is sufficient pairs and
    /// pair.0 is the least significant pair.Ace
    pub fn TwoPair(pair0: (Card, Card), pair1: (Card, Card)) -> ResultRank {
        let rank = Rank(RankInner::TwoPair(pair0, pair1));

        if let Err(E) = Rank::Pair(pair0) {
            Err(Explaned(
                rank,
                format!("Pair 1 of 2 returned: {:?}", E),
            ))
        } else if let Err(E) = Rank::Pair(pair1) {
            Err(Explaned(
                rank,
                format!("Pair 2 of 2 returned: {:?}", E),
            ))
        } else if pair0 > pair1 {
            Err(Unsorted(rank))
        } else {
            Ok(rank)
        }
    }

    /// Returns
    pub fn Trips(trips: (Card, Card, Card)) -> ResultRank {
        let rank = Rank(RankInner::Trips(trips.0, trips.1, trips.2));

        if trips.0.value != trips.1.value || trips.1.value != trips.2.value {
            Err(Invalid(rank))
        } else if trips.0 < trips.1 || trips.1 < trips.2 {
            Err(Unsorted(rank))
        } else {
            Ok(rank)
        }
    }

    pub fn Straight(straight: (Card, Card, Card, Card, Card)) -> ResultRank {
        let rank = Rank(RankInner::Straight(
            straight.0, straight.1, straight.2, straight.3, straight.4,
        ));

        let values = if straight.0.value == Ace {
            if straight.1.value != King {
                return Err(Explaned(rank, "Ace not followed by King".into()));
            }

            vec![
                straight.1.value as u8,
                straight.2.value as u8,
                straight.3.value as u8,
                straight.4.value as u8,
            ]
        } else {
            vec![
                straight.0.value as u8,
                straight.1.value as u8,
                straight.2.value as u8,
                straight.3.value as u8,
                straight.4.value as u8,
            ]
        };

        // See if cards[0] is greater than every other item by i amount
        // Also check if cards are in order.
        // Ace is not included in this range. See above
        for i in 0..values.len() {
            if values[0] != values[i] - i as u8 {
                return Err(Invalid(rank));
            }
        }

        Ok(rank)
    }

    ///
    pub fn Flush(flush: (Card, Card, Card, Card, Card)) -> ResultRank {
        let rank = Rank(RankInner::Flush(
            flush.0, flush.1, flush.2, flush.3, flush.4,
        ));
        let cards = [flush.0, flush.1, flush.2, flush.3, flush.4];

        // See if all suits match
        for card in &cards {
            if flush.0.suit != card.suit {
                return Err(Invalid(rank));
            }
        }

        // See if cards are sorted
        for i in 0..=3 {
            if cards[i] <= cards[i + 1] {
                return Err(Unsorted(rank));
            }
        }

        Ok(rank)
    }

    pub fn House(trips: (Card, Card, Card), pair: (Card, Card)) -> ResultRank {
        let rank = Rank(RankInner::House(trips, pair));

        // See if both Trips and Pair is ok and return rank
        // Else return an explained error
        if let Err(E) = Rank::Trips(trips) {
            Err(Explaned(rank, format!("Trips returned: {:?}", E)))
        } else if let Err(E) = Rank::Pair(pair) {
            Err(Explaned(rank, format!("Pair returned: {:?}", E)))
        } else {
            Ok(rank)
        }
    }

    pub fn Quads(quads: (Card, Card, Card, Card)) -> ResultRank {
        let rank = Rank(RankInner::Quads(quads.0, quads.1, quads.2, quads.3));
        let cards = [quads.0, quads.1, quads.2, quads.3];

        // See if all values match
        for card in &cards {
            if quads.0.value != card.value {
                return Err(Invalid(rank));
            }
        }

        // See if cards are sorted
        for i in 0..=2 {
            if cards[i] < cards[i + 1] {
                return Err(Unsorted(rank));
            }
        }

        Ok(rank)
    }

    pub fn StraightFlush(sf: (Card, Card, Card, Card, Card)) -> ResultRank {
        let rank = Rank(RankInner::StraightFlush(sf.0, sf.1, sf.2, sf.3, sf.4));

        if let Err(E) = Rank::Straight(sf) {
            Err(Explaned(rank, format!("Straight function returned: {:?}", E)))
        } else if let Err(E) = {

            // Ace is always last in order for Flush()
            if sf.0.value == Ace {
                Rank::Flush((sf.1, sf.2, sf.3, sf.4, sf.0))
            } else {
                Rank::Flush(sf)
            }

        } {
            Err(Explaned(rank, format!("Flush function returned: {:?}", E)))
        } else {
            Ok(rank)
        }
    }

    pub fn Fives(fives: (Card, Card, Card, Card, Card)) -> ResultRank {
        let rank = Rank(RankInner::Fives(
            fives.0, fives.1, fives.2, fives.3, fives.4,
        ));
        let cards = [fives.0, fives.1, fives.2, fives.3, fives.4];

        // See if all values match
        for card in &cards {
            if fives.0.value != card.value {
                return Err(Invalid(rank));
            }
        }

        // See if cards are sorted
        for i in 0..=2 {
            if cards[i] < cards[i + 1] {
                return Err(Unsorted(rank));
            }
        }

        Ok(rank)
    }
}
