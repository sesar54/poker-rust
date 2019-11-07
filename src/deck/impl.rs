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

    /// Draw some cards and return them as a Vector
    pub fn deal(&mut self, size: usize) -> Option<Vec<Card>> {
        if self.inner_deck.len() >= size {
            let mut cards = Vec::<Card>::new();

            for _ in 0..size {
                cards.push(self.inner_deck.pop().unwrap());
            }

            Some(cards)
        } else {
            None
        }
    }

    /// Deletes top card from deck
    pub fn discard(&mut self) {
        self.inner_deck.pop();
    }

    /// Shuffle the deck
    pub fn shuffle(&mut self) {
        self.inner_deck.shuffle(&mut rand::thread_rng());
    }

    /// Returns the length of deck
    pub fn len(&self) -> usize {
        self.inner_deck.len()
    }

    /// Constructs a new, full `Deck` of 52 sorted unique cards.
    ///
    /// # Example
    /// ```
    /// # #![allow(unused_mut)]
    /// let mut deck = Deck::new_sorted();
    /// ```
    pub fn new_sorted() -> Deck {
        let mut deck = vec![];

        // TODO Iterate sorted
        for i in 0..=52 {
            deck.push(Card::from(i))
        }

        deck.sort();

        Deck { inner_deck: deck }
    }

    /// Constructs a new, full `Deck` of 52 unsorted unique cards.
    ///
    /// # Example
    /// ```
    /// # #![allow(unused_mut)]
    /// let mut deck = Deck::new_shuffled();
    /// println!("{}", deck);
    /// ```
    pub fn new_shuffled() -> Deck {
        let mut deck = Deck::new_sorted();
        deck.shuffle();
        deck
    }

    /// Constructs a new, custom `Deck`, by copying over from slice.
    ///
    /// # Example
    /// ```
    /// # #![allow(unused_mut)]
    /// let mut deck = Deck::new_custom(&card!(Ace,Spades; King, Hearts));
    /// assert_eq!(deck.draw(), card!(Ace, Spades));
    /// ```
    pub fn new_custom(cards: &[Card]) -> Deck {
        Deck {
            inner_deck: cards.to_vec(),
        }
    }

    /// Returns `true` if the deck contains no cards.
    pub fn is_empty(&self) -> bool {
        self.inner_deck.is_empty()
    }

    // Returns `true` if the deck is sorted.
    //pub fn is_sorted(&self) -> bool {
    //    self.inner_deck.is_sorted()
    //} // Unstable for now.
}
