use crate::card::Card;

mod impl_hand;
mod impl_rank;

pub mod extra;
pub use extra::*;

use std::rc::*;

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
    cards: Vec<Rc<Card>>,
    rank: Rank,
    kickers: Vec<Rc<Card>>,
}


/**
 * A Rank consist of a number of cards in a specific configuration. They are
 * sorted by the lowest value first and greatest value last (actually in what
 * order they are written).
 */
#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Rank(RankInner);

#[derive(Debug)]
pub enum RankErr {
    Explained(String),
    Invalid(Rank),
    Unsorted(Rank),
}

type CardRef = Rc<Card>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum RankInner {
    High([CardRef; 1]),
    Pair([CardRef; 2]),
    TwoPair([CardRef; 2], [CardRef; 2]),
    Trips([CardRef; 3]),
    Straight([CardRef; 5]),
    Flush([CardRef; 5]),
    House([CardRef; 3], [CardRef; 2]),
    Quads([CardRef; 4]),
    StraightFlush([CardRef; 5]),
    Fives([CardRef; 5]),
}
