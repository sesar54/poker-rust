#![allow (dead_code)]

/* Lets start with a hand and work downwards */ 
struct Hand {

    cards: Vec<Card>,
    winning_cards: Vec<Card>,
    score: Score,

}

impl Hand {
    fn new(cards: Vec<Card>) -> Hand {

        Hand {
            cards,
            score = Score::new(&cards),

        }
    }
}


struct Card {

    rank: Rank,
    suit: Suit,

}


struct Score {
    hand_type: HandType,
    type_rank: u8,
}

impl Score {
    fn new(cards: &Vec<Card>) -> Score {
        Score {

            

        }
    }
}


/* enum with implicit disciminator (Ace=0, ... , King=12, Joker=13),  */
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
    

}
