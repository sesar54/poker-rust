
use crate::Card;

#[allow(dead_code)]
impl crate::Deck {

    fn pop(mut self) -> Option<Card> {
        self.card_deck.pop()
    }

}
