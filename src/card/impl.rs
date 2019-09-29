use crate::{Card, Suit, Value};
use std::fmt;

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

