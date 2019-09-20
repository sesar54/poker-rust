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

impl Suit {
    pub const SIZE: usize = 4;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    // (Ace=0, Two=1, ... , King=12, Joker>=13)
    pub value: Value,
    pub suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.value, self.suit)
    }

}

#[macro_export]
/** First argument: Value, Second argument: Suit */
macro_rules! Card {
    ( $val:expr, $suit:expr ) => {{
        use crate::card::Value::*;
        use crate::card::Suit::*;

        let card = super::Card {
            value: $val,
            suit: $suit,
        };
        card
    }};
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_build() {
        let card0 = Card {
            value: Value::Eight,
            suit: Suit::Diamonds,
        };

        let card1 = card!(Eight, Diamonds);

        assert_eq!(card0, card1);
    
        println!("Test");
        
    }

    #[test]
    fn check_order() {
        assert!(Value::Ace < Value::Eight);
    }

}
