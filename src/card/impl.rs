use super::face::*;
use crate::num_traits::FromPrimitive;
use mimpl::mimpl;
use std::char;
use std::convert::*;
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

// -------------------------------------------------------------------------- //
// Impl Rank and Suit enums                                                   //
// -------------------------------------------------------------------------- //

impl Suit {
    pub fn count() -> usize {
        4
    }

    pub fn next(&mut self) -> Self {
        self.step_by(1)
    }

    pub fn last(&mut self) -> Self {
        self.step_by(-1)
    }

    pub fn step_by(&mut self, n: isize) -> Self {
        *self = Self::from_isize(isize::rem_euclid(
            *self as isize + n,
            Self::count().try_into().unwrap(),
        ))
        .unwrap();
        *self
    }
}

impl Rank {
    pub fn count() -> usize {
        13
    }

    pub fn next(&mut self) -> Self {
        self.step_by(1)
    }

    pub fn last(&mut self) -> Self {
        self.step_by(-1)
    }

    pub fn step_by(&mut self, n: isize) -> Self {
        *self = Self::from_isize(isize::rem_euclid(
            *self as isize + n,
            Self::count().try_into().unwrap(),
        ))
        .unwrap();
        *self
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
