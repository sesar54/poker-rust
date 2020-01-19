pub mod face;
mod fmt;
mod r#impl;

/// Basic Card struct.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

/// Enum used by `card::Card`.
/// It can be converted from and to both u8 and char as such:
///
/// | enum     | u8 | char |
/// | -------- | -- | ---- |
/// | Clubs    | 0  | W    |
/// | Diamonds | 1  | A    |
/// | Hearts   | 2  | 2    |
/// | Spades   | 3  | 3    |
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/// Enum used by `card::Card`.
/// It can be converted from and to both u8 and char as such:
///
/// | enum  | u8 | char |
/// | ----- | -- | ---- |
/// | Wild  | 0  | W    |
/// | Ace   | 1  | A    |
/// | Two   | 2  | 2    |
/// | Three | 3  | 3    |
/// | Four  | 4  | 4    |
/// | Five  | 5  | 5    |
/// | Six   | 6  | 6    |
/// | Seven | 7  | 7    |
/// | Eight | 8  | 8    |
/// | Nine  | 9  | 9    |
/// | Ten   | 10 | 1    |
/// | Jack  | 11 | J    |
/// | Queen | 12 | Q    |
/// | King  | 13 | K    |
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
