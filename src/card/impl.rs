use crate::{Card, Suit, Value};
use std::fmt;
use std::cmp::Ordering::*;


impl Card {

    pub fn new(value: Value, suit: Suit) -> Card {
        Card {
            value: value,
            suit: suit,
        }
    }

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

impl From<u8> for Card {
    fn from(u: u8) -> Card {
        Card::new(Value::from(u), Suit::from(u))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Impl Value and Suit enums                                                  //
////////////////////////////////////////////////////////////////////////////////

impl From<u8> for Suit {
    fn from(u: u8) -> Suit {
        match u % 4 {

            0 => Suit::Clubs,
            1 => Suit::Diamonds,
            2 => Suit::Hearts,
            3 => Suit::Spades,
            _ => unreachable!()

        }
    }
}

impl From<u8> for Value {
    fn from(u: u8) -> Value {
        if u > 52 {
            Value::Joker
        } else {
            match u % 13 {
                0 =>    Value::Ace,
                1 =>    Value::Two,
                2 =>    Value::Three,
                3 =>    Value::Four,
                4 =>    Value::Five,
                5 =>    Value::Six,
                6 =>    Value::Seven,
                7 =>    Value::Eight,
                8 =>    Value::Nine,
                9 =>    Value::Ten,
                10 =>   Value::Jack,
                11 =>   Value::Queen,
                12 =>   Value::King,
                _ => unreachable!()
            }
        }

    }
}
