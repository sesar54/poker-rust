
/// > I don't share your greed, the only card I need is...
/// >
/// > The Ace of Spades
#[macro_export]
macro_rules! card {
    () => {{
        extern crate rand;
        use rand::Rng;

        Card::from(rand::thread_rng().gen_range(0, 51))
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
    ( $($val:expr, $suit:expr);* ) => [$(card!($val,$suit),)*]
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
