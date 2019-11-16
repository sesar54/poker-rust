#[macro_use]
extern crate aces_high as poker;

#[cfg(test)]
mod deck {
    use poker::{card::face::*, deck::Deck};

    #[test]
    #[ignore]
    fn test() {
        let cards = card!(Ace, Spades; King, Hearts);
        let deck = Deck::new_custom(&cards);
        println!("{:?}", deck);
    }
}
