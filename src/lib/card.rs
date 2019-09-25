#[macro_use]
mod r#impl;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
/** enum used as i32 with implicit discriminator so (Ace=0, ... , King=12, Joker=13),  */
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






