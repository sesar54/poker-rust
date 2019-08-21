
/* enum with implicit disciminator (Ace=0, ... , King=12, Joker=13),  */
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

pub struct Card {

    // (Ace=0, Two=1, ... , King=12, Joker>=13)
    value: u8,

    /* (Clubs=0, Diamonds=1, Hearts=2, Spades=3) */
    suit: u8,

}

impl Card {

    const VALUE_SIZE: usize = 14;
    const SUIT_SIZE: usize = 4;

}

/* implicit discriminator, higher score is better (duh...) */
pub enum Rank {

    High            (u8, Card),
    Pair            (u8, Card, Card),
    TwoPair         (u8, (Card, Card), (Card, Card)),
    Trips           (u8, Card, Card, Card),
    Straight        (u8, Card, Card, Card, Card, Card),
    Flush           (u8, Card, Card, Card, Card, Card),
    House           (u8, (Card, Card, Card), (Card, Card)),
    Quads           (u8, Card, Card, Card, Card),
    StraightFlush   (u8, Card, Card, Card, Card, Card),
    FivePair        (u8, Card, Card, Card, Card, Card),

}

pub struct Hand {

    cards: Vec<Card>,
    rank: Rank,

}


impl Hand {

    pub fn new(cards: Vec<Card>) -> Hand {

        fn ranking(cards: Vec<Card>) -> (Rank) {

            let mut group_card = vec![vec![]];

            /* Sort cards by their value. Grouping together cards with the same value */
            {
                let mut old_card = cards[0];

                for card in cards {

                    if old_card.value == card.value {

                        group_card.last_mut().unwrap().push(card);

                    } else {

                        group_card.push(vec![card]);
                        old_card = card;

                    }

                }

            }

            /* Check if 5 values are grouped one after another */
            {
                let i = group_card.len() - 5;
                
                while i > 0 {

                    

                    i-=1
                }

            }

            println!("Things {:?}", group_card);


        }


        return Hand {

            cards,
            rank: ranking(cards),

        }
        
    }
}
