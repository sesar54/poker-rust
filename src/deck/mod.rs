mod r#impl;

#[derive(Debug)]
pub struct Deck {
    inner_deck: Vec<crate::card::Card>,
}
