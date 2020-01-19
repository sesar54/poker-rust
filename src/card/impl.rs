use super::face::*;

use std::char;
use std::cmp::Ordering::*;
use std::convert::TryFrom;

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
        Card {
            __inner: rank as u8 + suit as u8,
        }
    }

    pub fn get_rank(self) -> Rank {
        Rank::try_from(self.__inner).unwrap()
    }

    pub fn get_suit(self) -> Suit {
        Suit::from(self.__inner)
    }

    pub fn cmp_suit_first(self, other: Self) -> std::cmp::Ordering {
        match self.get_suit().cmp(&other.get_suit()) {
            Equal => self.get_rank().cmp(&other.get_rank()),
            ord => ord,
        }
    }

    pub fn cmp_rank_first(self, other: Self) -> std::cmp::Ordering {
        match self.get_rank().cmp(&other.get_rank()) {
            Equal => self.get_suit().cmp(&other.get_suit()),
            ord => ord,
        }
    }

    pub const __RANK_FIELD: u8 = 0x0F;
    pub const __SUIT_FIELD: u8 = 0x30;
}

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
        self.__inner
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
        Ok(Card::new(Rank::try_from(u)?, Suit::from(u)))
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
        [self.get_rank().into(), self.get_suit().into()]
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

// -------------------------------------------------------------------------- //
// Impl Suit enums                                                            //
// -------------------------------------------------------------------------- //

impl super::Circular<isize> for Suit {
    /// Cycles over the elements of `Suit`, starting at `self`.
    /// Returns the n'th neighbor.
    /// # Examples
    /// ```rust
    /// use aces_high::card::{Circular, Rank::*};
    ///
    /// let val = Wild;
    /// assert_eq!(val.step(1), Wild);
    /// for i in 0..101 {
    ///     assert_eq!(val.step(i), Wild);
    /// }
    ///
    /// let val = Ace;
    /// assert_eq!(val.step(0), Ace);
    /// assert_eq!(val.step(1), Two);
    /// assert_eq!(val.step(9), Ten);
    /// assert_eq!(val.step(13), Ace);
    /// assert_eq!(val.step(-13), Ace);
    /// ```
    fn step(self, n: isize) -> Self {
        Suit::from(((self as isize + n) % 4) as u8)
    }
}

impl From<u8> for Suit {
    /// Converts `u8` into `Suit` by looking at 0011000.
    fn from(u: u8) -> Suit {
        match u & 0x30 {
            0x00 => Clubs,
            0x10 => Diamonds,
            0x20 => Hearts,
            0x30 => Spades,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<char> for Suit {
    type Error = String;

    /// Tries to convert `ASCII char` to `Suit`,
    /// by mapping each enum to a upper character.
    /// Can take lower characters.
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c.to_ascii_uppercase() {
            'C' => Ok(Clubs),
            'D' => Ok(Diamonds),
            'H' => Ok(Hearts),
            'S' => Ok(Spades),
            _ => Err(format!("Card::Suit can't be converted from char '{}'.", c)),
        }
    }
}

impl Into<char> for Suit {
    /// Converts `self` to `char` by mapping.
    fn into(self) -> char {
        match self {
            Clubs => 'C',
            Diamonds => 'D',
            Hearts => 'H',
            Spades => 'S',
        }
    }
}

// -------------------------------------------------------------------------- //
// Impl Value enums                                                           //
// -------------------------------------------------------------------------- //

impl super::Circular<isize> for Rank {
    /// Cycles over the elements of `Rank`, starting at `self`.
    /// Returns the n'th neighbor.
    /// # Examples
    /// ```rust
    /// use aces_high::card::{Circular, Rank::*};
    ///
    /// let val = Wild;
    /// assert_eq!(val.step(1), Wild);
    /// for i in 0..101 {
    ///     assert_eq!(val.step(i), Wild);
    /// }
    ///
    /// let val = Ace;
    /// assert_eq!(val.step(0), Ace);
    /// assert_eq!(val.step(1), Two);
    /// assert_eq!(val.step(9), Ten);
    /// assert_eq!(val.step(13), Ace);
    /// assert_eq!(val.step(-13), Ace);
    /// ```
    fn step(self, i: isize) -> Self {
        if self == Wild {
            self
        } else {
            Rank::try_from((((self as isize + i - 1) % 13) + 1) as u8).unwrap()
        }
    }
}

impl TryFrom<u8> for Rank {
    type Error = String;

    /// Tries to convert `u8` to `Rank` by mapping 0 to 13 to a Rank.
    /// All other numbers will result in an error.
    fn try_from(u: u8) -> Result<Self, Self::Error> {
        let rank = match u & Card::__RANK_FIELD {
            0x0 => Wild,
            0x1 => Ace,
            0x2 => Two,
            0x3 => Three,
            0x4 => Four,
            0x5 => Five,
            0x6 => Six,
            0x7 => Seven,
            0x8 => Eight,
            0x9 => Nine,
            0xA => Ten,
            0xB => Jack,
            0xC => Queen,
            0xD => King,
            u => return Err(format!("card::Rank can't be converted from u8: {}.", u)),
        };

        Ok(rank)
    }
}

impl Into<char> for Rank {
    /// Converts `self` to `char` by mapping.
    fn into(self) -> char {
        match self {
            Wild => 'W',
            Ace => 'A',
            Ten => '1',
            Jack => 'J',
            Queen => 'Q',
            King => 'K',
            v => char::from_u32(v as u32).unwrap(),
        }
    }
}

impl TryFrom<char> for Rank {
    type Error = String;

    /// Tries to convert `ASCII char` to `Rank`,
    /// by mapping each enum to a upper character.
    /// Can take lower characters.
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use rand::Rng;

        let u: u8 = match c.to_ascii_uppercase() {
            'W' => 0x0,
            'A' => 0x1,
            '1' => 0xA,
            'J' => 0xB,
            'Q' => 0xC,
            'K' => 0xD,
            '.' => rand::thread_rng().gen_range(0x0, 0xD),
            c if c.is_digit(0xA) && c != '0' => c.to_digit(0x9).unwrap() as u8,
            c => return Err(format!("card::Rank can't be converted from char '{}'.", c)),
        };

        Ok(Rank::try_from(u).unwrap())
    }
}
