extern crate rand;
use rand::seq::SliceRandom;

use crate::deck::Deck;
use crate::*;

#[allow(dead_code)]
impl Deck {
    /// Draw the top card from the deck, or return None if empty
    pub fn draw(&mut self) -> Option<Card> {
        self.inner_deck.pop()
    }

    /// Shuffle the deck
    pub fn shuffle(&mut self) {
        self.inner_deck.shuffle(&mut rand::thread_rng());
    }

    /// Constructs a new, full `Deck` of 52 sorted unique cards.
    /// 
    /// # Example
    /// ```
    /// # #![allow(unused_mut)]
    /// let mut deck = Deck::new_sorted();
    /// ```
    pub fn new_sorted() -> Deck {
        use {Value::*, Suit::*};

        let values = [
            Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, 
            Jack, Queen, King,
        ];
        let suits = [Clubs, Diamonds, Hearts, Spades];

        let mut deck = vec![];

        for suit in &suits {
            for value in &values {
                deck.push(card!(*value, *suit));
            }
        }

        Deck { inner_deck: deck }
    }

    /// Constructs a new, full `Deck` of 52 unsorted unique cards.
    /// /// 
    /// # Example
    /// ```
    /// # #![allow(unused_mut)]
    /// let mut deck = Deck::new_unsorted();
    /// ```
    pub fn new_unsorted() -> Deck {
        let mut deck = Deck::new_sorted();
        deck.shuffle();
        deck
    }

    /// Constructs a new, custom `Deck`.
    /// /// 
    /// # Example
    /// ```
    /// # #![allow(unused_mut)]
    /// let mut deck = Deck::new_custom(&card!(Ace,Spades; King, Hearts));
    /// ```
    pub fn new_custom(cards: &[Card]) -> Deck {
        Deck { inner_deck: cards.to_vec() }
    }

}

#[cfg(test)]
mod test {

    #[test]
    fn test() {

        use crate::*;

        let cards = cards!(Ace, Spades; King, Hearts);

        let deck = Deck::new_custom(&cards);

        println!("{:?}", deck);

    }

}