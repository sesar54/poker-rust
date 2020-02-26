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
