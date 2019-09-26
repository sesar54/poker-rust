pub mod holdem;

mod r#impl;

#[macro_use]
pub mod macros;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

pub fn clump<T,F,E> (slice: &[T], func: F) -> Vec::<&[T]>
    where
        F: Fn(&T) -> E,
        E: PartialEq,

{

    let trail = slice.iter();

    if let trail_test = func(trail.next()) {

    };

    let last_test = func(trail);

    for &item in slice {

        let test = func(item);

    }


}

pub struct Deck {
    pub card_deck: Vec<Card>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}


/** enum used as i32 with implicit discriminator so (Ace=0, ... , King=12, Joker=13),  */
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Joker,
}
