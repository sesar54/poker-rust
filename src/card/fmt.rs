use super::{Card, Rank, Suit};
use std::fmt;

impl fmt::Display for Card {
    /// Formats `Card` into a human readable string.
    /// # Example
    /// ```
    /// # use ace_of_spades::{*, card::face::*};
    /// assert_eq!(format!("{}", card!(Ace, Spades)), "Ace of Spades");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.rank, self.suit)
    }
}

impl fmt::Debug for Card {
    /// Formats `Card` into 2 characters.
    /// First character depicts the cards `Rank`.
    /// Second character depicts the cards `Suit`.
    /// # Example
    /// ```
    /// # use ace_of_spades::{*, card::face::*};
    /// assert_eq!(format!("{:?}", card!(King, Diamonds)), "KD");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chars: [char; 2] = [self.rank.into(), self.suit.into()];
        write!(f, "{}{}", chars[0], chars[1])
    }
}

impl fmt::UpperHex for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}{:X}", self.rank, self.suit)
    }
}

impl fmt::UpperHex for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", *self as u8)
    }
}

impl fmt::UpperHex for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", *self as u8)
    }
}

impl fmt::LowerHex for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}{:x}", self.rank, self.suit)
    }
}

impl fmt::LowerHex for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", *self as u8)
    }
}

impl fmt::LowerHex for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", *self as u8)
    }
}
