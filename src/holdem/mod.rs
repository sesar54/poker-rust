use crate::Card;

mod hand;
mod rank;

/**
 * A hand consist of all cards "in hand or private cards" and
 * "on table or public cards". But the important thing is to value these cards.
 *
 * If we value our cards, chances are that some are worthless but they are
 * part of our hand. Therefore the cards are slotted into enum struct "Rank".
 * Only the highest ranking cards are saved in it.
 */
pub struct Hand {
    pub cards: Vec<Card>,
    pub rank: Rank,
}

/**
 * A Rank consist of a number of cards in a specific configuration. They are
 * sorted by the lowest value first and greatest value last (actually in what
 * order they are written).
 */
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Rank(RankInner);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum RankInner {
    High(Card),
    Pair(Card, Card),
    TwoPair((Card, Card), (Card, Card)),
    Trips(Card, Card, Card),
    Straight(Card, Card, Card, Card, Card),
    Flush(Card, Card, Card, Card, Card),
    House((Card, Card, Card), (Card, Card)),
    Quads(Card, Card, Card, Card),
    StraightFlush(Card, Card, Card, Card, Card),
    Royal(Card, Card, Card, Card, Card),
    FivePair(Card, Card, Card, Card, Card),
}
