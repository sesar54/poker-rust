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
