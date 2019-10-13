/// First argument: Value
/// Second argument: Suit
/// As the saying goes:
///
/// > I don't share your greed, the only card I need is...
///
/// > The Ace of Spades

pub use crate::card::{Card, Value::*, Suit::*};

#[macro_export]
macro_rules! card {

    () => {

        crate::card::Card {
            value: Ace,
            suit: Spades,
        }

    };

    ( $val:expr, $suit:expr ) => {

        crate::card::Card {
            value: $val,
            suit: $suit,
        };

    };

}

#[macro_export]
macro_rules! hand {

    ( $( $card:expr ),* ) => {
        {
            let mut cards = Vec::new();
            $(
                cards.push($card);
            )*
            holdem::Hand::new(&cards)
        }
    };

}
