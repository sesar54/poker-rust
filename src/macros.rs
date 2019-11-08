/// > I don't share your greed, the only card I need is...
/// >
/// > The Ace of Spades
#[macro_export]
macro_rules! card {
    //($val:expr, $suit:expr) TODO -- REPLACE BELOW
    () => {{
        extern crate rand;

        let rng = rand::thread_rng();

        card!(value!(&mut rng, 1) , suit!(&mut rng, 1))

    }};
    ($val:expr, $suit:expr) => {Card {value: $val, suit: $suit}};
    ($($val:expr, $suit:expr);*) => {[$(card!($val,$suit),)*]};
}

//TODO REDO ALL MACROS
#[macro_export]
macro_rules! suit {
    () => {{
        use Suit::*;
        [Clubs, Diamonds, Hearts, Spades]
    }};
    ($rand:expr) => {{
        let mut suits = suit!();
        suits.shuffle($rand);
        suits
    }};
     ($rand:expr, 1) => {{
        *suit!().choose($rand).unwrap()
    }};
    ($rand:expr, $num:expr) => {{
        let suits = suit!($rand);
        
        let mut suits_collect = vec![];
        for i in 0..$num {
            suits_collect[i] = suits[i];
        }

        suits_collect
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
    ($rand:expr) => {{
        let values = value!();
        values.shuffle($rand);
        values
    }};
    ($rand:expr, 1) => {{
        *value!().choose($rand).unwrap()
    }};
    ($rand:expr, $num:expr) => {{
        let values = value!($rand);
        
        let mut values_collect = Vec::<Suit>::new();
        for _ in 0..$num {
            values_collect.push(*values.choose($rand).unwrap())
        }

        values_collect
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
