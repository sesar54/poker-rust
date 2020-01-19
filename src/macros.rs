/// > I don't share your greed, the only card I need is...
/// >
/// > The Ace of Spades
#[macro_export]
macro_rules! card {
    ($rank:expr, $suit:expr) => {
        $crate::card::Card::new($rank, $suit)
    };
    ($rand:expr) => {
        card!(ranks!($rand)[0], suits!($rand)[0])
    };
}

#[macro_export]
macro_rules! cards {
    ($($rank:expr, $suit:expr); +) => {
        [
            $(
                card!($rank, $suit),
            )+
        ]
    }
}

#[macro_export]
macro_rules! suits {
    () => {{
        use $crate::card::Suit::*;
        [Clubs, Diamonds, Hearts, Spades]
    }};
    ($rand:expr) => {{
        let mut suits = suits!();
        suits.shuffle($rand);
        suits
    }};
}

#[macro_export]
macro_rules! ranks {
    () => {{
        use $crate::card::Rank::*;
        [
            Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
        ]
    }};
    ($rand:expr) => {{
        let mut ranks = ranks!();
        ranks.shuffle($rand);
        ranks
    }};
}

///
///
//TODO REDO ALL MACROS
#[macro_export]
macro_rules! deck {
    ($($rank:expr, $suit:expr); +) => {
        $crate::deck::Deck::new_custom(
            $(
                card!($rank, $suit)
            )+
        )
    };
}

/// TODO
#[macro_export]
macro_rules! hand {
    ($($card:expr), +) => {
        {
            let mut cards = Vec::new();
            $(
                cards.push($card);
            )*
            holdem::Hand::new(&cards)
        }
    };
    ($($rank:expr, $suit:expr); +) => {
        hand![
            $(
                card!($rank, $suit)
            )+
        ]
    };
}

/// Creates an array from yielding a iterator.
#[macro_export]
macro_rules! to_array {
    {$iter:expr; $size:expr} => {{
        use seq_macro::seq;
        let mut iter = $iter; // seq! does not like $iter, so rename is required

        // Closure returns Option<[T; $size]>
        (|| Some(
            seq!(_ in 0..$size {
                [
                    #(
                        iter.next()?,
                    )*
                ]
            }
        )))()
    }};
}
