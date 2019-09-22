use std::fmt;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    // (Ace=0, Two=1, ... , King=12, Joker>=13)
    pub value: Value,
    pub suit: Suit,
}


impl fmt::UpperHex for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", *self as u8)
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

impl Card {

    fn into(&self) -> u8 {
        (self.value as u8) << 2 + (self.suit as u8)
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

/// First argument: Value
/// Second argument: Suit 
/// As the saying goes:
/// 
/// > I don't share your greed, the only card I need is...
/// 
/// > The Ace of Spades
/// 
/// It feels more natural to type in this order
#[macro_export]
macro_rules! card {
    ( $val:expr, $suit:expr ) => {{

        use $crate::card::{ Suit::*, Value::*, Card };

        let card = Card {
            value: $val,
            suit: $suit,
        };

        card

    }};
}

#[cfg(test)]
mod tests {

    use crate::card::{ Suit::*, Value::*, Card };

    #[test]
    fn check_build() {

        let card0 = Card {
            value: Eight,
            suit: Diamonds,
        };
        
        let card1 = card!(Ace, Spades);

        assert_ne!(card0, card1);

        println!("{}", card1);
        println!("{:?}", card1);
        
    }

}
