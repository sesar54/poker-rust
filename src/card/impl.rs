use super::face::*;

use mimpl::mimpl;

use std::char;
use std::convert::TryFrom;

use crate::num_traits::FromPrimitive;
// -------------------------------------------------------------------------- //
// Impl Card                                                                  //
// -------------------------------------------------------------------------- //

impl Card {
    /// Constructs a new `Card`.
    ///
    /// # Example
    /// ```rust
    /// # use aces_high::card::face::*;
    /// let card = Card::new(Rank::King, Suit::Hearts);
    /// ```
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }
}
/* TODO
impl Into<u8> for Card {
    /// Converts `Card` into `u8`.
    /// `Rank` gets inscribed into the first 4 bits.
    /// `Suit` gets inscribed into the last 4 bits.
    /// # Example
    /// ```
    /// # use aces_high::card::face::*;
    /// let u: u8 = Card::new(Ace, Spades).into();
    /// assert_eq!(u, 0x13);
    /// ```
    fn into(self) -> u8 {
        ((self.rank as u8) << 4) + (self.suit as u8)
    }
}

impl TryFrom<u8> for Card {
    type Error = String;

    /// Tries to create `Card` from `u8`.
    /// `Rank` is taken from the first 4 bits.
    /// `Suit` is taken from the last 4 bits.
    ///
    /// # Example
    /// ```
    /// # use aces_high::card::face::*;
    /// use std::convert::TryFrom;
    /// assert_eq!(Card::try_from(0x13).unwrap(), Card::new(Ace, Spades));
    /// ```
    fn try_from(u: u8) -> Result<Self, Self::Error> {
        Ok(Card::new(Rank::try_from(u >> 4)?, Suit::from(u)))
    }
}

impl Into<[char; 2]> for Card {
    /// Converts `Card` into 2 characters.
    /// First character depicts the cards `Rank`.
    /// Second character depicts the cards `Suit`.
    /// # Example
    /// ```
    /// # use aces_high::{*, card::face::*};
    /// let chars: [char; 2] = card!(Queen, Hearts).into();
    /// assert_eq!(chars, ['Q', 'H']);
    /// ```
    fn into(self) -> [char; 2] {
        [self.rank.into(), self.suit.into()]
    }
}

impl TryFrom<[char; 2]> for Card {
    type Error = String;

    /// Tries to convert `[char; 2]` into `Card`
    /// First character depicts the cards `Rank`.
    /// Second character depicts the cards `Suit`.
    /// # Example
    /// ```
    /// # use aces_high::{*, card::face::*};
    /// use std::convert::TryFrom;
    /// assert_eq!(Card::try_from(['2','C']).unwrap(), card!(Two, Clubs));
    fn try_from(c: [char; 2]) -> Result<Self, Self::Error> {
        println!("{:?}", c);
        println!("{:?} {:?}", Rank::try_from(c[0]), Suit::try_from(c[1]));
        Ok(Card::new(Rank::try_from(c[0])?, Suit::try_from(c[1])?))
    }
}
*/
// -------------------------------------------------------------------------- //
// Impl Rank and Suit enums                                                   //
// -------------------------------------------------------------------------- //

impl Iterator for Suit {
    type Item = Suit;

    fn next(&mut self) -> Option<Self::Item> {
        *self = Self::from_u32(*self as u32 + 1 % 4).unwrap();
        Some(*self)
    }
}

impl Iterator for Rank {
    type Item = Rank;

    fn next(&mut self) -> Option<Self::Item> {
        Self::from_u32(*self as u32 + 1 % 13)
    }
}

mimpl!(Default; Suit, || Spades);
mimpl!(Default; Rank, || Ace);

mimpl!(From; Suit, char, |suit: Suit| 
    match suit {
        Clubs => 'C',
        Diamonds => 'D',
        Hearts => 'H',
        Spades => 'S',
    }
);

// -------------------------------------------------------------------------- //
// Impl Value enums                                                           //
// -------------------------------------------------------------------------- //

impl TryFrom<u8> for Rank {
    type Error = String;

    /// Tries to convert `u8` to `Rank` by mapping 0 to 13 to a Rank.
    /// All other numbers will result in an error.
    fn try_from(u: u8) -> Result<Self, Self::Error> {
        let rank = match u {
            0 => Ace,
            1 => Two,
            2 => Three,
            3 => Four,
            4 => Five,
            5 => Six,
            6 => Seven,
            7 => Eight,
            8 => Nine,
            9 => Ten,
            10 => Jack,
            11 => Queen,
            12 => King,
            u => return Err(format!("card::Rank can't be converted from u8 '{}'.", u)),
        };

        Ok(rank)
    }
}

mimpl!(From; Rank, char, |rank: Rank| 
    match rank {
        Ace => 'A',
        Ten => '1',
        Jack => 'J',
        Queen => 'Q',
        King => 'K',
        v => char::from_u32(v as u32).unwrap(),
    }
);

mimpl!(TryFrom; char, Rank, String, |c: char| {
        use rand::Rng;

        let u: u8 = match c.to_ascii_uppercase() {
            'A' => 1,
            '1' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            '.' => rand::thread_rng().gen_range(0, 13),
            c if c.is_digit(10) && c != '0' => c.to_digit(9).unwrap() as u8,
            c => return Err(format!("card::Rank can't be converted from char '{}'.", c)),
        };

        Ok(Rank::try_from(u).unwrap())
    }
);
