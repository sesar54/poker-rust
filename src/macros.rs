/// > I don't share your greed, the only card I need is...
/// >
/// > The Ace of Spades
#[macro_export]
macro_rules! card {
    ($rand:expr) => {card!(value!($rand), suit!($rand))};)
    ($val:expr, $suit:expr) => {Card {value: $val, suit: $suit}};
    ($($val:expr, $suit:expr);*) => {[$(card!($val,$suit),)*]};
}

//TODO REDO ALL MACROS
#[macro_export]
macro_rules! suit {
    () => {{use Suit::*; [Clubs, Diamonds, Hearts, Spades]}};
    ($rand:expr) => {suit!()[$rand]};
    ($rand:expr, $range:expr) => {{
        let mut suits = suit!();
        suits.shuffle($rand);
        suits[..$range]
    }};
}

#[macro_export]
macro_rules! value {
    () => {{
        use Value::*;
        [
            Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
        ]
    }};
    ($rand:expr) => {value!()[$rand]};
    ($rand:expr, $range:expr) => {{
        let mut values = value!();
        values.shuffle($ramd);
        values[..$range]
    }};

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
