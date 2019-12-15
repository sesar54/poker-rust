use crate::card::Card;
use crate::hand::{Hand, rank::Error};

pub struct Player {
    pub pot: u32,
    pub tot_bet: u32,

    hand: Option<Hand>,
}

pub struct Action(ActionInner);

enum ActionInner {
    Fold(Vec<Card>),
    /// Call is multifunctional. It works as both check, call and raise.
    ///
    /// It will automatically match any bid if 0 is provided and
    /// automatically raise value if not 0.
    ///
    /// Mind that the table might ignore small raises.
    Call(u32),
}

impl Action {
    pub fn folding(player: &mut Player) -> Action {
        Action(ActionInner::Fold(player.discard()))
    }

    pub fn calling(player: &mut Player, mut bet: u32) -> Action {
        bet = std::cmp::min(bet, player.pot);
        player.pot -= bet;
        Action(ActionInner::Call(bet))
    }
}

impl Player {
    pub fn take(&mut self, cards: Vec<Card>) -> Result<(), Error> {
        match &mut self.hand {
            None => {
                self.hand = Some(Hand::new(cards)?);
                Ok(())
            }
            _ => panic!(), // TODO
        }
    }

    pub fn discard(&mut self) -> Vec<Card> {
        match self.hand.take() {
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

    pub fn hand_is_none(&self) -> bool {
        self.hand.is_none()
    }
}

trait Inter {
    fn decide(&self) -> Action;
}
