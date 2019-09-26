pub mod holdem;

mod r#impl;

#[macro_use]
pub mod macros;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

pub fn clump<T,F,E> (slice: &mut Vec<T>, value: T, func: F) -> bool
    where 
        F: Fn(&T) -> E, 
        E: PartialEq, 
{

    if let Some(first_value) = slice.first() {
        if func(first_value) == func(&value) {
            
            slice.push(value);
            return true;

        }

    } else {
        slice.push(value);
        return true;

    }

    return false;

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
