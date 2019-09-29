use crate::*;
use crate::deck::Deck;

impl Deck {
    fn pop(mut self) -> Option<Card> {
        self.card_deck.pop()
    }
}