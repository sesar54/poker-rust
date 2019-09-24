/// First argument: Value
/// Second argument: Suit 
/// As the saying goes:
/// 
/// > I don't share your greed, the only card I need is...
/// 
/// > The Ace of Spades
/// 
/// It feels more natural to type in this order

#[macro_export]
macro_rules! card {

    ( $val:expr, $suit:expr ) => {{

        use $crate::card::{ Suit::*, Value::*, Card };

        let card = Card {
            value: $val,
            suit: $suit,
        };

        card

    }};
    
}


#[macro_export]
macro_rules! hand {

    ( $( $card:expr ),* ) => {
        {
            use crate::card;
            let mut cards = Vec::new();
            $(
                cards.push($card);
            )*
            super::Hand::new(&cards)
        }
    };
    
}