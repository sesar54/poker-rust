use super::*;

use std::char;
use std::cmp::Ordering::*;
use std::convert::TryFrom;
use std::fmt;

// -------------------------------------------------------------------------- //
// Impl Card                                                                  //
// -------------------------------------------------------------------------- //

impl Card {
    /// # Example
    /// ```
    /// //extern crate dead_mans_hand as poker;
    /// //use poker::*;
    /// use crate::*;
    /// use Value::*;
    /// use Suit::*;
    /// println!("{}", Card::new(Value::King, Suit::Hearts));
    /// ```
    pub fn new(value: Value, suit: Suit) -> Card {
        Card { value, suit }
    }

    pub fn cmp_suit_first(self, other: Self) -> std::cmp::Ordering {
        match self.suit.cmp(&other.suit) {
            Equal => self.value.cmp(&other.value),
            ord => ord,
        }
    }

    pub fn cmp_value_first(self, other: Self) -> std::cmp::Ordering {
        match self.value.cmp(&other.value) {
            Equal => self.suit.cmp(&other.suit),
            ord => ord,
        }
    }
}


impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.value, self.suit)
    }
}

impl From<u8> for Card {
    fn from(u: u8) -> Card {
        Card::new(Value::from(u), Suit::from(u))
    }
}

/// TODO
impl Into<u8> for Card {
    fn into(self) -> u8 {
        ((self.value as u8) << 4) + (self.suit as u8)
    }
}

impl TryFrom<[char; 2]> for Card {
    type Error = String;

    fn try_from(c: [char; 2]) -> Result<Self, Self::Error> {
        Ok(Card::new(Value::try_from(c[0])?, Suit::try_from(c[1])?))
    }
}

// -------------------------------------------------------------------------- //
// Impl Suit enums                                                            //
// -------------------------------------------------------------------------- //

impl From<u8> for Suit {
    fn from(u: u8) -> Suit {
        match u & 0x03 {
            0 => Suit::Clubs,
            1 => Suit::Diamonds,
            2 => Suit::Hearts,
            3 => Suit::Spades,
            _ => unreachable!(),
        }
    }
}

impl Into<u8> for Suit {
    fn into(self) -> u8 {
        match self {
            Suit::Clubs => 0,
            Suit::Diamonds => 1,
            Suit::Hearts => 2,
            Suit::Spades => 3,
        }
    }
}

impl TryFrom<char> for Suit {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c.to_ascii_uppercase() {
            'C' => Ok(Suit::Clubs),
            'D' => Ok(Suit::Diamonds),
            'H' => Ok(Suit::Hearts),
            'S' => Ok(Suit::Spades),
            _ => Err(format!("Card::Suit can't be converted from char '{}'.", c)),
        }
    }
}

impl Into<char> for Suit {
    fn into(self) -> char {
        match self {
            Suit::Clubs => 'C',
            Suit::Diamonds => 'D',
            Suit::Hearts => 'H',
            Suit::Spades => 'S',
        }
    }
}

// TODO
impl fmt::UpperHex for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", *self as u8)
    }
}

// -------------------------------------------------------------------------- //
// Impl Value enums                                                           //
// -------------------------------------------------------------------------- //

impl Value {

    /// # Examples
    /// ```rust
    /// use crate::*;
    /// use Value::*;
    ///
    /// let val = Wild;
    /// assert_eq(val.next(1), Wild);
    ///
    /// let val = Ace;
    /// assert_eq(val.next(1), Ace);
    /// ```
    pub fn next(self, u: i32) -> Self {
        if self == Value::Wild {
            self
        } else {
            Value::from((((((self as u8 >> 4) as i32 - 1 + u) % 4) + 1) << 4) as u8)
        }
    }
}

impl From<u8> for Value {
    fn from(u: u8) -> Value {
        match u >> 4 {
            0 => Value::Wild,
            1 => Value::Ace,
            2 => Value::Two,
            3 => Value::Three,
            4 => Value::Four,
            5 => Value::Five,
            6 => Value::Six,
            7 => Value::Seven,
            8 => Value::Eight,
            9 => Value::Nine,
            10 => Value::Ten,
            11 => Value::Jack,
            12 => Value::Queen,
            13 => Value::King,
            _ => unreachable!(),
        }
    }
}

impl Into<u8> for Value {
    fn into(self) -> u8 {
        (self as u8) << 4
    }
}

impl TryFrom<char> for Value {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use rand::Rng;

        let u: u8 = match c.to_ascii_uppercase() {
            'W' => 0,
            'A' => 1,
            '1' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            '.' => rand::thread_rng().gen_range(0, 13),
            c if c.is_digit(10) && c != '0' => c.to_digit(9).unwrap() as u8 - 1,
            c => return Err(format!("Card::Value can't be converted from char '{}'.", c)),
        };

        Ok(Value::from(u))
    }
}

impl Into<char> for Value {
    fn into(self) -> char {
        match self {
            Value::Wild => 'W',
            Value::Ace => 'A',
            Value::Ten => '1',
            Value::Jack => 'J',
            Value::Queen => 'Q',
            Value::King => 'K',
            v => char::from_u32(v as u32).expect("TODO"),
        }
    }
}

impl fmt::UpperHex for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", *self as u8)
    }
}
