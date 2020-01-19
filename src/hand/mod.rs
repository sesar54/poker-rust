use crate::card::Card;

mod r#impl;
pub mod rank;

pub mod extra;
pub use extra::*;

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
    cards: Vec<Card>,
    rank: rank::Rank,
    kickers: Vec<Card>,
}
