use crate::{Card, Suit, Value};
use std::fmt;

impl Card {

    pub fn cmp_suit_first(&self, other: &Self) -> std::cmp::Ordering {
        match self.suit.cmp(&other.suit) {
            Equal => self.value.cmp(&other.value),
            ord @ _ => ord,
        }
    }

    pub fn cmp_value_first(&self, other: &Self) -> std::cmp::Ordering {
        match self.value.cmp(&other.value) {
            Equal => self.suit.cmp(&other.suit),
            ord @ _ => ord,
        }
    }

}

/// ```
/// let card = card!(Ace, Spades);
/// println!("{}", card)
/// ```
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.value, self.suit)
    }
}

impl Into<u8> for Card {
    fn into(self) -> u8 {
        ((self.value as u8) << 4) + (self.suit as u8)
    }
}


impl Suit {
    pub const SIZE: usize = 4;
}

impl fmt::UpperHex for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", *self as u8)
    }
}


impl fmt::UpperHex for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", *self as u8)
    }
}


/*
impl From<u8> for Card {
    fn from(u8) -> Card {

    }
}*/

