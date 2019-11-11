mod r#impl;
pub mod face;


/// Basic Card struct.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

/// Enum used by `card::Card`.
/// Uses it's implicit discriminator which ranges from 0 to 3.
/// This gives:
/// * Clubs the value of 0.
/// * Diamonds the value of 1.
/// * Hearts the value of 2.
/// * Spades the value of 3.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/// Enum used by `card::Card`.
/// Uses it's implicit discriminator which ranges from 0 to 13
/// This gives: 
/// * Wild (as the joker) the value of 0. 
/// * Ace the value of 1. 
/// * Ten the value of 10.
/// * King the value of 13.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Wild,
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

pub trait Circular<T> {
    fn step(self, t: T) -> Self;
}
