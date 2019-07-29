#![allow (dead_code)]

extern crate enum_map;
#[macro_use]

use enum_map::EnumMap;
use std::collections::HashMap;

/* Lets start with a hand and work downwards */ 
struct Hand {

    cards: Vec<Card>,
    score: Score,

}

impl Hand {
    fn new(cards: Vec<Card>) -> Hand {
        Hand {
            cards,
            score: Score::new(&cards),
        }
    }
}


struct Card {

    rank: u8,
    suit: Suit,

}


pub struct Score {

    grade: u8,
    r#type: HandType,
    winning_hand: Vec<Card>,

}

impl Score {

    fn new(cards: &Vec<Card>) -> Score {

        /* Return tupple of Suit { High, Pair, TwoPair, Trips, House, Quads, Five } */
        let pair: Score = {

            //let mut rank_counter = EnumMap::new();
            let mut rank_counter = HashMap::new();

            for card in cards {
                
            }

            Score::new(cards: &Vec<Card>)


        };

        let straight: Score = {

        };

        let flush: Score = {

        };


        Score {

            grade: ,
            r#type: ,
            winning_hand: ,

        }

    }

}


/* enum with implicit disciminator (Ace=0, ... , King=12, Joker=13),  */
/* 
enum Rank {
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
    Joker,

}
 */

mod Rank {
    const Ace   : u8 = 0;
    const Two   : u8 = 1;
    const Three : u8 = 2;
    const Four  : u8 = 3;
    const Five  : u8 = 4;
    const Six   : u8 = 5;   
    const Seven : u8 = 6;
    const Eight : u8 = 7;
    const Nine  : u8 = 8;
    const Ten   : u8 = 9;   
    const Jack  : u8 = 10;
    const Queen : u8 = 11;
    const King  : u8 = 12;
    const Joker : u8 = 13;
}


enum Suit {

    Clubs,
    Diamons,
    Hearts,
    Spades,
    
}


/* implicit discriminator, higher score is better (duh...) */
enum HandType {

    High,
    Pair,
    TwoPair,
    Trips,
    Straight,
    Flush,
    House,
    Quads,
    StraighFlush,
    Five,

}




fn main() {
    
    let h = Hand::new(vec!(Card{ rank: Rank::Ace, suit: Suit::Clubs }));

}
