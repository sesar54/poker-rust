/// First argument: Value
/// Second argument: Suit 
/// As the saying goes:
/// 
/// > I don't share your greed, the only card I need is...
/// 
/// > The Ace of Spades
#[macro_export]
macro_rules! card {

    ( $val:expr, $suit:expr ) => {

        use $crate::{ Suit::*, Value::*, Card };

        let card = Card {
            value: $val,
            suit: $suit,
        };

        card

    };
    
}

#[macro_export]
macro_rules! hand {

    ( $( $card:expr ),* ) => {
        {
            use crate::Card;
            let mut cards = Vec::new();
            $(
                cards.push($card);
            )*
            super::Hand::new(&cards)
        }
    };
    
}

#[macro_export]
macro_rules! clump {
    ( $elem:expr, *, $func:expr) => {
        let clump = Clump::new($func($elem), $func);

        $(
            clump.push($elem);
        )*

    };
}

#[cfg(test)]
mod test {

    #[test]
    fn clump() {
        clump!(2, 4, |g| g);
    }

}