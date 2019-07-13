#![allow(dead_code)]

extern crate num;
extern crate num_derive;
extern crate enum_map;

use enum_map::EnumMap;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub enum Score {

    High        = 0,
    Pair        = 1,
    Pairx2      = 2,
    Trips       = 3,
    Straight    = 4,
    Flush       = 5,
    House       = 6,
    Quads       = 7,
    SF          = 8,
    Five        = 9,

}

impl Score {

    pub fn as_str(&self, hand: &[u8]) -> Vec<&str> {

        use Score::*;

        match self {

            High        => vec!("High Card", ""),
            Pair        => vec!("", ""),
            Pairx2      => vec!("", ""),
            Trips       => vec!("", ""),
            Straight    => vec!("", ""),
            Flush       => vec!("", ""),
            House       => vec!("", ""),
            Quads       => vec!("", ""),
            SF          => vec!("", ""),

        }
    }

    pub fn nickname(&self, hand: &[u8]) -> &str {

        match self {






            Quads {





            }



        }


    }

}

pub enum Collapse {
    Order = 13, /* Skipping wildcard */
    Suit = 4,
}

impl Collapse {

    pub fn to_order(&self, hand: &[u8]) -> Vec<u8> {

        let mut new_hand = Vec::new();
        new_hand.clone_from_slice(hand);

        for card in &mut new_hand {

            *card %= *self as u8;

        }

        return new_hand;

    }


}


#[derive(FromPrimitive)]
pub enum Order {
    OA = 0, /* Ace */
    O2 = 1,
    O3 = 2,
    O4 = 3,
    O5 = 4,
    O6 = 5,
    O7 = 6,
    O8 = 7,
    O9 = 8,
    O1 = 9, /* Ten */
    OJ = 10,
    OQ = 11,
    OK = 12,
    WJ, /* Wildcard / Joker */

}

impl Order {

    pub fn as_str(&self) -> Vec<&str> {

        use Order::*;

        match self {
            
            &OA => vec!("ace",      "aces"),
            &O2 => vec!("two",      "twos",     "deuce",    "deuces"),
            &O3 => vec!("three",    "threes"),
            &O4 => vec!("four",     "fours"),
            &O5 => vec!("five",     "fives"),
            &O6 => vec!("six",      "sixes"),
            &O7 => vec!("seven",    "sevens"),
            &O8 => vec!("eight",    "eights"),
            &O9 => vec!("nine",     "nines"),
            &O1 => vec!("ten",      "tens",     "dime",     "dimes"),
            &OJ => vec!("jack",     "jacks",    "john",     "john"),
            &OQ => vec!("queen",    "queens",   "dame",     "dames",    "lady", "ladies"),
            &OK => vec!("king",     "kings",    "knight",   "knights"),
            &WJ => vec!("joker",    "jokers",   "wildcard", "wildcards"),

        }
    }

    pub fn to_order(card: &u8) -> &Order {
        
        use Order::*;

        if card >= &(4 * 13) {

            return &WJ;

        } else {
            match FromPrimitive::from_u8(card % 13) {

                Some(OA) => &OA,
                Some(O2) => &O2,
                Some(O3) => &O3,
                Some(O4) => &O4,
                Some(O5) => &O5,
                Some(O6) => &O6,
                Some(O7) => &O7,
                Some(O8) => &O8,
                Some(O9) => &O9,
                Some(O1) => &O1,
                Some(OJ) => &OJ,
                Some(OQ) => &OQ,
                Some(OK) => &OK,

                /* Unreachable */
                Some(WJ) => &WJ,
                None => panic!(),

            }
        }
    }

    pub fn to_hand(hand: &Vec<Order>) -> Vec<u8> {

        let mut new_hand = Vec::new();

        


    }

}


#[derive(FromPrimitive)]
pub enum Suit {

    C = 0,
    D = 1,
    H = 2,
    S = 3,
    
}

impl Suit {

    pub fn as_str(&self) -> Vec<&str> {

        use Suit::*;

        match self {

            C => vec!("club",    "clubs"),
            D => vec!("diamond", "diamonds"),
            H => vec!("heart",   "hearts"),
            S => vec!("spade",   "spades"),

        }
    }

    
    pub fn to_suit(card: &u8) -> &Suit {
        
        use Suit::*;

        match FromPrimitive::from_u8(card % 4) {

            Some(C) => &C,
            Some(D) => &D,
            Some(H) => &H,
            Some(S) => &S,

            /* Unreachable since (unsigned % 4) can only be equal to one enum */
            None => panic!(),

        }

    }

}

pub fn single(names: Vec<&str>) -> Vec<&str> {

    let mut i = 0;
    let mut v = Vec::new();

    for name in names {

        if i % 2 == 0 {
            v.push(name);
        }

        i+=1;

    }

    return v;


}

pub fn plural(names: Vec<&str>) -> Vec<&str> {

    let mut i = 0;
    let mut v = Vec::new();

    for name in names {

        if i % 2 == 1 {
            v.push(name);
        }

        i+=1;

    }

    return v;

}

/**
 * This function takes an vector of cards
 */
fn poker_hand(hand: &[u8]) {

    /* */
    {

    for card in hand {

        Order::to_order(card);



    }

    }
}

fn main() {
    
    let v = vec![ 1, 3, 7];

    poker_hand(&v);

}
