pub mod face;
mod fmt;
mod r#impl;

/// Basic Card struct.
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Eq, PartialOrd, Ord)]
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
/// | Ace   | 0  | A    |
/// | Two   | 1  | 2    |
/// | Three | 2  | 3    |
/// | Four  | 3  | 4    |
/// | Five  | 4  | 5    |
/// | Six   | 5  | 6    |
/// | Seven | 6  | 7    |
/// | Eight | 7  | 8    |
/// | Nine  | 8  | 9    |
/// | Ten   | 9  | 1    |
/// | Jack  | 10 | J    |
/// | Queen | 11 | Q    |
/// | King  | 12 | K    |
#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
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
