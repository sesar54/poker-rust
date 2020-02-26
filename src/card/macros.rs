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

/// > TEST
#[macro_export]
macro_rules! cards {
    ($rank0:expr, $suit0:expr $(;$rank1:expr, $suit1:expr)+ $(;)*) => {
        [
            card!($rank0, $suit0),
            $(
                card!($rank1, $suit1),
            )+
        ]
    };
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
