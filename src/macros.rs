/// First argument: Value
/// Second argument: Suit
/// As the saying goes:
///
/// > I don't share your greed, the only card I need is...
///
/// > The Ace of Spades

#[macro_export]
macro_rules! card {
    () => {{
        extern crate rand;
        use rand::seq::*;

        use {Suit::*, Value::*};
        let mut rng = rand::thread_rng();

        let values = [
            Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
        ];
        let suits = [Clubs, Diamonds, Hearts, Spades];

        Card {
            value: *values.iter().choose(&mut rng).unwrap(),
            suit: *suits.iter().choose(&mut rng).unwrap(),
        }
    }};

    ( $val:expr, $suit:expr ) => {
        Card {
            value: $val,
            suit: $suit,
        };
    };
}

#[macro_export]
macro_rules! cards {
    ( $($val:expr, $suit:expr);* ) => {

        [
            $(
                card!($val,$suit),
            )*
        ]

    }
}

/// TODO
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
