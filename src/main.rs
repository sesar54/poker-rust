#![allow (dead_code)]

/* Lets start with a hand and work downwards */ 
struct Hand {

    cards: Vec<Card>,
    r#type: Hands,
    score: u8,

}

impl Hand {
    fn new(cards: Vec<Card>) -> Hand {

        Hand {
            cards,

            let (type, score) = Score::new(cards),
        }
    }
}

#[derive(Debug)]
struct Card {

    // (Ace=0, Two=1, ... , King=12, Joker>=13)
    rank: u8,

    /* (Clubs=0, Diamonds=1, Hearts=2, Spades=3) */
    suit: u8,

}

impl Card {
    const RANK_SIZE: usize = 14;
    const SUIT_SIZE: usize = 4;
}


pub struct Score {

    r#type: Hands,
    score: u8,

}


impl Score {

    fn new(cards: Vec<&Card>) -> Score {
        
        {

            let mut old_card = &cards[0];
            let mut group_card = vec![vec![]];

            for card in cards {

                
                if old_card.rank == card.rank {

                    group_card.last_mut().unwrap().push(card);

                } else {

                    group_card.push(vec![card]);
                    old_card = card;

                }

            }

            println!("Things {:?}", group_card);

        }

        let hand = Hands::High(1);

        return Score{ r#type: hand,  score: 2 };

        /* 
        match card {

            5 => {pair_score = 5; return },


            /* Check for pair, two pair, full house */
            // 2 => ,

            /* High card,  */
            1 | 4 => 
                    1 | 4 => 
            1 | 4 => 

        } 
        */


        /*
        let straight: Score = {

        };

        let flush: Score = {

        };


        Score {

            grade: ,
            r#type: ,
            winning_hand: ,

        }

        */

    }

}

/*
/* enum with implicit disciminator (Ace=0, ... , King=12, Joker=13),  */
enum Rank {
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

} */

/* 
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
} */

/* 
enum Suit {

    Clubs,
    Diamonds,
    Hearts,
    Spades,
    
} */


/* implicit discriminator, higher score is better (duh...) */
enum Hands {

    High            (Card),
    Pair            (Card, Card),
    TwoPair         ((Card, Card), (Card, Card)),
    Trips           (Card, Card, Card),
    Straight        (Card, Card, Card, Card, Card),
    Flush           (Card, Card, Card, Card, Card),
    House           ((Card, Card, Card), (Card, Card)),
    Quads           (Card, Card, Card, Card),
    StraightFlush   (Card, Card, Card, Card, Card),
    FivePair        (Card, Card, Card, Card, Card),

}




fn main() {
    
    let hand = Hands::High(Card{rank: 5, suit: 2,});
    let h = Hand::new(vec!(Card{ rank: 1, suit: 3 }));

}
