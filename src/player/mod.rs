use crate::card::Card;
use crate::hand::Hand;

pub struct Player {

    pub name: String,

    pub pot: u32,
    pub bet: u32,

    hand: Hand,

}

impl Player {

    pub fn take(&mut self, cards: Vec<Card>) {
        self.hand.take(cards);
    }

}
