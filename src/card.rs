#![allow(dead_code)]

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/** enum used as i32 with implicit discriminator so (Ace=0, ... , King=12, Joker=13),  */
pub enum Value {
    Ace = 0,
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub const SIZE: usize = 4;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    // (Ace=0, Two=1, ... , King=12, Joker>=13)
    pub value: Value,
    pub suit: Suit,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_order() {

        assert!(Value::Ace < Value::Eight);
    }


}