use crate::deck::Deck;
use crate::player::Player;

pub struct Table {
    deck: Deck,
    players: Vec<Player>,
}

impl Table {
    pub fn deal(&mut self, size: usize) {
        for player in self.players.as_mut_slice() {
            player.take(self.deck.deal(size).unwrap()).ok();
        }
    }
}

impl Default for Table {
    fn default() -> Self {
        Table {
            deck: Deck::new_shuffled(),
            players: Vec::new(),
        }
    }
}
