use crate::Card;
mod r#impl;

#[derive(Debug)]
pub struct Deck {
    inner_deck: Vec<Card>,
}
