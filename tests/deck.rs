#[macro_use]
extern crate aces_high as poker;

#[cfg(test)]
mod deck {
    use poker::{card::face::*, deck::Deck};

    #[test]
    fn test() {
        let cards = cards!(Ace, Spades; King, Hearts);
        let deck = Deck::new_custom(&cards);
        println!("{:?}", Card {__inner: 0x31});
        println!("CARD:  {:X}", cards!(Ace, Spades)[0]);
        println!("{:?}", deck);
        println!("{:X}", cards[1]);
    }
}
