extern crate dead_mans_hand as lib;

#[cfg(test)]
mod deck {

    use lib::*;
    use Suit::*;
    use Value::*;

    #[test]
    fn test() {
        let cards = card!(Ace, Spades; King, Hearts);

        let deck = Deck::new_custom(&cards);

        println!("{:?}", deck);
    }
}
