use crate::card::Card;
use crate::hand::{RankErr, Hand};

pub struct Player {

    pub name: String,

    pub pot: u32,
    pub tot_bet: u32,

    hand: Option<Hand>,

}

pub enum Betting {
    Fold(Vec<Card>),
    /// Call is multifunctional. It works as both check, call and raise.
    ///
    /// It will automatically match any bid if 0 is provided and
    /// automatically raise value if not 0.
    ///
    /// Mind that the table might not accept any small raise.
    Call(u32),
}

impl Player {

    pub fn folding(&mut self) -> Betting {
        Betting::Fold(self.discard())
    }

    pub fn calling(&mut self, mut bet: u32) -> Betting {

        if bet > self.pot {
            bet = self.pot;
        }

        self.pot -= bet;

        Betting::Call(bet)

    }

    pub fn take(&mut self, cards: Vec<Card>) -> Result<(), RankErr> {
        match &mut self.hand {
            Some(hand) => hand.take(cards),
            None => {
                self.hand = Some(Hand::new(cards)?);
                Ok(())
            },
        }
    }

    pub fn discard(&mut self) -> Vec<Card> {

        let hand = self.hand.take();

        match hand {

            Some(hand) => hand.discard(),
            None => Vec::new(),

        }

    }

    pub fn hand_len(&self) -> usize {
        match &self.hand {
            Some(hand) => hand.len(),
            None => 0,
        }
    }

    pub fn hand_is_empty(&self) -> bool {
        self.hand.is_none()
    }

}
