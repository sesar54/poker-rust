#[macro_use]
pub mod macros;
mod r#impl;

pub struct Deck {
    inner_deck: Vec<crate::card::Card>,
}
