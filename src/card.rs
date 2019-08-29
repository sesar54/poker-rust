
/* enum used as i32 with implicit discriminator so (Ace=0, ... , King=12, Joker=13),  */
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {

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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Suit {

    Clubs,
    Diamonds,
    Hearts,
    Spades,
    
}

impl Suit {
    pub const Size: usize = 4;
}


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {

    // (Ace=0, Two=1, ... , King=12, Joker>=13)
    pub value: Value,
    pub suit: Suit,

}

/* 
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {

        /* Compare by Value*/
        if (self.value as u8) > (other.value as u8) {
            return Ordering::Greater;
        
        } else if (self.value as u8) < (other.value as u8) {
            return Ordering::Less;

        } else {

            /* Compare by Suit */
            if (self.suit as u8) > (other.suit as u8) {
                return Ordering::Greater;

            } else if (self.suit as u8) < (other.suit as u8) {
                return Ordering::Less;

            } else {
                return Ordering::Equal

            }
        }
    }
} */
/* 
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
} */
/* 
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        if self.value == other.value && self.suit == other.suit {
            return true;
        } else {
            return false;
        }
    }
} */