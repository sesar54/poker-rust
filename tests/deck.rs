#[macro_use]
extern crate ace_of_spades as poker;

#[cfg(test)]
mod deck {
    use poker::{card::face::*, deck::Deck};

    #[test]
    fn test() {
        let cards = card!(Ace, Spades; King, Hearts);
        let deck = Deck::new_custom(&cards);
        println!("{:?}", deck);
        println!("{:X}", cards[1]);
    }
}
