use std::fmt;

use crate::holdem::{Rank, RankInner};

impl fmt::Display for Rank {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        use RankInner::*;

        match self.0 {

            High(..) =>             write!(f, "High card"),
            Pair(..) =>             write!(f, "Pair"),
            TwoPair(..) =>          write!(f, "Two pairs"),
            Trips(..) =>            write!(f, "Three of a kind"),
            Straight(..) =>         write!(f, "Straight"),
            Flush(..) =>            write!(f, "Flush"),
            House(..) =>            write!(f, "Full house"),
            Quads(..) =>            write!(f, "Four of a kind"),
            StraightFlush(card, ..) => match card.value {
                _ =>                write!(f, "Straight flush"),
                Ace =>              write!(f, "Royal flush"),
            }
            Fives(..) =>         write!(f, "Five of a kind"),

        }
    }
}

macro_rules! ok_rank {
    ($rank:expr) => {
        return Ok(Rank($rank))
    };
}

pub mod RankBuilder {

    use crate::card::{Card, Value::{Ace, King}};
    use crate::holdem::{Rank, RankInner};

    type ResRank = Result<Rank, &'static str>;

    pub fn High(card: Card) -> ResRank {

        ok_rank!(RankInner::High(card))

    }

    pub fn Pair(pair: (Card, Card)) -> ResRank {

        if pair.0.value != pair.1.value {
            Err("Not pair")

        } else if pair.0 < pair.1 {
            Err("Not Sorted")

        } else {
            ok_rank!(RankInner::Pair(pair.0, pair.1))

        }
    }

    pub fn TwoPair(pair0: (Card, Card), pair1: (Card, Card))
        -> ResRank {

        Pair(pair0)?;
        Pair(pair1)?;

        if pair0 < pair1 {
            Err("Not Sorted")

        } else {
            ok_rank!(RankInner::TwoPair(pair0, pair1))

        }


    }

    pub fn Trips(trips: (Card, Card, Card)) -> ResRank {

        if trips.0.value != trips.1.value || trips.1.value != trips.2.value {
            Err("Not Trips")

        } else if trips.0 < trips.1 || trips.1 < trips.2 {
            Err("Not Sorted")

        } else {
            ok_rank!(RankInner::Trips(trips.0, trips.1, trips.2))

        }

    }

    pub fn Straight
        (straight: (Card, Card, Card, Card, Card))
        -> ResRank {

        let values = if straight.0.value == Ace {

            if straight.1.value != King {
                return Err("Ace not followed by King");
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

        // See if cards[0] is greater than every other item by i ammount
        // Also check if cards are in order.
        // Ace is not included in this range. See above
        for i in 0..values.len() {
            if values[0] != values[i] - i as u8 {
                return Err("Not Straight");
            }
        }

        ok_rank!(RankInner::Straight(
            straight.0,
            straight.1,
            straight.2,
            straight.3,
            straight.4
        ))

    }

    /**
     * Takes five cards in order
     */
    pub fn Flush
        (flush: (Card, Card, Card, Card, Card))
        -> ResRank {

        let cards = [
            flush.0,
            flush.1,
            flush.2,
            flush.3,
            flush.4,
        ];

        // See if all suits match
        for card in &cards {
            if flush.0.suit != card.suit {
                return Err("Not Flush");
            }
        }

        // See if cards are sorted
        for i in 0..=3 {
            if cards[i] <= cards[i + 1] {
                return Err("Not Sorted");
            }

        }

        ok_rank!(RankInner::Flush(flush.0, flush.1, flush.2, flush.3, flush.4))

    }

    pub fn House
        (trips: (Card, Card, Card), pair: (Card, Card))
        -> ResRank {

        // See if both
        Trips(trips)?;
        Pair(pair)?;

        ok_rank!(RankInner::House(trips,pair))

    }

    pub fn Quads(quads: (Card, Card, Card, Card)) -> ResRank {

        let cards = [
            quads.0,
            quads.1,
            quads.2,
            quads.3,
        ];

        // See if all values match
        for card in &cards {
            if quads.0.value != card.value {
                return Err("Not Quads");
            }
        }

        // See if cards are sorted
        for i in 0..=2 {
            if cards[i] < cards[i + 1] {
                return Err("Not Sorted");
            }
        }

        ok_rank!(RankInner::Quads(quads.0, quads.1, quads.2, quads.3))

    }

    pub fn StraightFlush
        (SF: (Card, Card, Card, Card, Card))
        -> ResRank {

        Straight(SF)?;

        // Ace is always last in order for Flush()
        if SF.0.value == Ace {
            Flush((SF.1, SF.2, SF.3, SF.4, SF.0))?;

        } else {
            Flush(SF)?;

        }

        ok_rank!(RankInner::StraightFlush(
            SF.0,
            SF.1,
            SF.2,
            SF.3,
            SF.4
        ))

    }

    pub fn Fives(fives: (Card, Card, Card, Card, Card)) -> ResRank {

        let cards = [
            fives.0,
            fives.1,
            fives.2,
            fives.3,
            fives.4,
        ];

        // See if all values match
        for card in &cards {
            if fives.0.value != card.value {
                return Err("Not Quads");
            }
        }

        // See if cards are sorted
        for i in 0..=2 {
            if cards[i] < cards[i + 1] {
                return Err("Not Sorted");
            }
        }

        ok_rank!(RankInner::Fives(fives.0, fives.1, fives.2, fives.3, fives.4))

    }

}

#[cfg(test)]
mod tests {
/*
    use crate::*;
    use crate::holdem::RankBuilder::*;

    #[test]
    fn display() {
        println!("{}", High(card!()));
        println!("{}", Pair(card!(),card!()))

    }
*/
}
