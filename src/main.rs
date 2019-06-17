#![allow(dead_code)]

extern crate num;
#[macro_use]
extern crate num_derive;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;


#[derive(FromPrimitive)]
pub enum Order {
    A = 0,
    O2 = 1,
    O3 = 2,
    O4 = 3,
    O5 = 4,
    O6 = 5,
    O7 = 6,
    O8 = 7,
    O9 = 8,
    O10 = 9,
    J = 10,
    Q = 11,
    K = 12,
    Joker,

}

impl Order {

    pub fn as_str(&self) -> Vec<&str> {
        match self {
            
            &Order::A =>    vec!("ace",     "aces"),
            &Order::O2 =>   vec!("two",     "twos", "deuce", "deuces"),
            &Order::O3 =>   vec!("three",   "threes"),
            &Order::O4 =>   vec!("four",    "fours"),
            &Order::O5 =>   vec!("five",    "fives"),
            &Order::O6 =>   vec!("six",     "sixes"),
            &Order::O7 =>   vec!("seven",   "sevens"),
            &Order::O8 =>   vec!("eight",   "eights"),
            &Order::O9 =>   vec!("nine",    "nines"),
            &Order::O10 =>  vec!("ten",     "tens",     "dime",     "dimes"),
            &Order::J =>    vec!("jack",    "jacks",    "john",     "john"),
            &Order::Q =>    vec!("queen",   "queens",   "dame",     "dames",    "lady", "ladies"),
            &Order::K =>    vec!("king",    "kings",    "knight",   "knights"),
            &Order::Joker =>vec!("joker",   "jokers",   "wildcard", "wildcards"),

        }
    }

    pub fn to_order(card: &u8) -> &Order {

        if card >= &(4 * 13) {

            return &Order::Joker;

        } else {
            match FromPrimitive::from_u8(card % 13) {

                Some(Order::A) =>    &Order::A,
                Some(Order::O2) =>   &Order::O2,
                Some(Order::O3) =>   &Order::O3,
                Some(Order::O4) =>   &Order::O4,
                Some(Order::O5) =>   &Order::O5,
                Some(Order::O6) =>   &Order::O6,
                Some(Order::O7) =>   &Order::O7,
                Some(Order::O8) =>   &Order::O8,
                Some(Order::O9) =>   &Order::O9,
                Some(Order::O10) =>  &Order::O10,
                Some(Order::J) =>    &Order::J,
                Some(Order::Q) =>    &Order::Q,
                Some(Order::K) =>    &Order::K,

                /* Unreachable */
                Some(Order::Joker) => &Order::Joker,
                None => panic!(),

            }
        }
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
        match self {

            Suit::C => vec!("club",    "clubs"),
            Suit::D => vec!("diamond", "diamonds"),
            Suit::H => vec!("heart",   "hearts"),
            Suit::S => vec!("spade",   "spades"),

        }
    }

    
    pub fn to_suit(card: &u8) -> &Suit {

        match FromPrimitive::from_u8(card % 4) {

            Some(Suit::C) => &Suit::C,
            Some(Suit::D) => &Suit::D,
            Some(Suit::H) => &Suit::H,
            Some(Suit::S) => &Suit::S,

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
fn poker_hand(hand: &[usize]) {

    /* */
    for card in hand {

        

    }


}

fn main() {
    
    let v = vec![ 1, 3, 7];

    poker_hand(&v);

}
