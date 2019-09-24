
use crate::card::Card;

#[allow(dead_code)]
impl crate::holdem::Deck {

    fn pop(mut self) -> Option<Card> {
        self.card_deck.pop()
    }

}