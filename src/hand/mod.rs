use crate::Card;

mod r#impl;
mod rank;

/**
 * A hand consist of all cards "in hand or private cards" and
 * "on table or public cards". But the important thing is to value these cards.
 *
 * If we value our cards, chances are that some are worthless but they are
 * part of our hand. Therefore the cards are slotted into enum struct "Rank".
 * Only the highest ranking cards are saved in it.
 */
#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub rank: Rank,
}

/**
 * A Rank consist of a number of cards in a specific configuration. They are
 * sorted by the lowest value first and greatest value last (actually in what
 * order they are written).
 */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rank(RankInner);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum RankInner {
    High([Card; 1]),
    Pair([Card; 2]),
    TwoPair([Card; 2], [Card; 2]),
    Trips([Card; 3]),
    Straight([Card; 5]),
    Flush([Card; 5]),
    House([Card; 3], [Card; 2]),
    Quads([Card; 4]),
    StraightFlush([Card; 5]),
    Fives([Card; 5]),
}

#[derive(Debug)]
pub enum RankErr {
    Invalid(Rank),
    Unsorted(Rank),
    Explained(String),
}
