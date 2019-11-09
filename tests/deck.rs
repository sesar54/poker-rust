#[macro_use]
extern crate dead_mans_hand as poker;

#[cfg(test)]
mod deck {
    use poker::prelude::*;
    use Suit::*;
    use Value::*;

    #[test]
    #[ignore]
    fn test() {
        let cards = card!(Ace, Spades; King, Hearts);
        let deck = Deck::new_custom(&cards);
        println!("{:?}", deck);
    }
}
