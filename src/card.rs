
/* enum with implicit discriminator (Ace=0, ... , King=12, Joker=13),  */
enum Value {

    Ace = 0,
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
    Joker,

}

enum Suit {

    Clubs,
    Diamonds,
    Hearts,
    Spades,
    
}

#[derive(Copy, Clone)]
pub struct Card {

    // (Ace=0, Two=1, ... , King=12, Joker>=13)
    pub value: u8,

    /* (Clubs=0, Diamonds=1, Hearts=2, Spades=3) */
    pub suit: u8,

}

impl Card {

    const VALUE_SIZE: usize = 14;
    const SUIT_SIZE: usize = 4;

}
