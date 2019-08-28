

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

#[derive(Copy, Clone)]
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

    High            (Card),
    Pair            (Card,  Card),
    TwoPair         ((Card, Card),(Card,  Card)),
    Trips           (Card,  Card,  Card),
    Straight        (Card,  Card,  Card,  Card,  Card),
    Flush           (Card,  Card,  Card,  Card,  Card),
    House          ((Card,  Card,  Card),(Card,  Card)),
    Quads           (Card,  Card,  Card,  Card),
    StraightFlush   (Card,  Card,  Card,  Card,  Card),
    FivePair        (Card,  Card,  Card,  Card,  Card),

}

pub struct Hand {

    cards: Vec<Card>,
    rank: Rank,

}


impl Hand {

    pub fn new(cards: Vec<Card>) -> Result<Hand,&'static str> {

        fn ranking(cards: &Vec<Card>) -> Option<Rank> {

            fn grouping (cards: &Vec<Card>) -> Vec<Vec<&Card>> {

                let mut grouped_cards: Vec<Vec<&Card>> = Vec::new();
                let mut old_card = cards[0];

                for card in *cards {

                    if old_card.value == card.value {

                        grouped_cards.last_mut().unwrap().push(&card);

                    } else {

                        grouped_cards.push(vec![&card]);
                        old_card = card;

                    }

                }

                return grouped_cards;

            }

            /* Holds a sorted 2d of cards sorted by their value. Suit is not
             * cared for.
             */
            let grouped_cards = grouping(cards);



            /* Return either High, Pair, Two Pair, Trips, House, Quads or 
             * Five Pair (Or None if no card was provided)
             */
            fn paired (grouped_cards: Vec<Vec<&Card>>) -> Option<Rank> {
                /* Function want to return as early as possible, thats why its 
                 * matching in a special order
                 */

                use std::collections::HashMap;
                let mut map_cards: HashMap::<usize, Vec<Vec<&Card>>> = HashMap::new();

                for c in grouped_cards {

                    /* Length is the key */
                    let mut len = c.len();


                    /* Stop mapping early if matching */
                    /* All pairs greater than 5 cards will be truncated to 5 */
                    if len > 5 {
                        len = 5;
                    }

                    match len {

                        5 => return Some(Rank::FivePair(*c[0], *c[1], *c[2], *c[3], *c[4])),
                        4 => return Some(Rank::Quads(*c[0], *c[1], *c[2], *c[3])),
                        _ => {}

                    }

                    /* Put cards in mapped group, creates one if missing */
                    if let Some(group) = map_cards.get_mut(&len) {
                        group.push(c);

                    } else {
                        map_cards.insert(len, vec![c]);

                    }

                }

                
                /* Check for house */
                let trips_group = map_cards.get(&3);
                let pair_group = map_cards.get(&2);t
                
                if let Some(trips_group) = trips_group {
                    if let Some(pair_group) = pair_group {
                        return trips_group pair_group

                    } else {
                        return trips_group

                    }

                } else if let Some(pair_group) = pair_group {
                    if pair_group.len() >= 2 {
                        return Rank::TwoPair
                    
                    } else {
                        return Rank::Pair

                    }

                } else {
                    return Rank::High

                }

            }

            /* Return a vector of groups of cards where each group of cards 
             * has a value lower than the next. The function does not return
             * a vector smaller than 5.
             */
            
            fn straight (grouped_cards: Vec<Vec<&Card>>) -> Option<Vec<Vec<&Card>>> {

                let straight_cards: Vec<Vec<&Card>> = Vec::new();

                let mut last_val: u8 = 0;

                for card_group in grouped_cards {

                    let val = card_group[0].value;

                    if val == last_val + 1 {

                        straight_cards.push(card_group);

                    } else {

                        straight_cards.clear();

                    }

                    last_val = val;

                }

                if straight_cards.len() < 5 {
                    return None;
                } else {
                    return Some(straight_cards);
                }

            }



            return None;

        }

        let rank = ranking(&cards);

        match rank {

            Some(rank) => return Ok(Hand { cards: cards, rank: rank }),
            None => return Err("None"),

        }

    }
}
