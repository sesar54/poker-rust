extern crate dead_mans_hand as lib; 

#[cfg(test)]
mod deck {

    use lib::*;

    #[test]
    fn test() {

        let cards = cards!(Ace, Spades; King, Hearts);

        let deck = Deck::new_custom(&cards);

        println!("{:?}", deck);

    }

}