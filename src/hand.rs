use super::card::Card;
use std::cmp::Ordering;

#[derive(Eq)]
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

impl Rank {

    fn ord (&self) -> u8 {

        match self {
            High            => 0,
            Pair            => 1,
            TwoPair         => 2,
            Trips           => 3,
            Straight        => 4,
            Flush           => 5,
            House           => 6,
            Quads           => 7,
            StraightFlush   => 8,
            /* TODO */
        }

    }

}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {

        let self_score = Rank::ord(self);
        let other_score = Rank::ord(other);

        /* Easy hand comparision */
        if self_score > other_score {
            return Ordering::Greater;

        } else if self_score < other_score {
            return Ordering::Less;

        } else {
            return Ordering::Equal;
        }

    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Rank {
    fn eq(&self, other: &Self) -> bool {
        if self.ord() == other.ord() {
            return false;

        } else {
            return false;

        }
    }
}

/* A hand consist of all cards "in hand or private cards" and 
 * "on table or public cards". But the important thing is to value these cards.
 * 
 * If we value our cards, chances are that some are worthless but they are
 * part of our hand. Therefore the cards are slotted into enum struct "Rank".
 * Only the highest ranking cards are saved in it.
 */
pub struct Hand {

    cards: Vec<Card>,
    rank: Rank,

}

impl Hand {


    /* Creating a new hand will cause all given cards to be automatically
    * evaluated. 
    * 
    * 
    */
    pub fn new(cards: Vec<Card>) -> Hand {

        fn ranking(cards: &Vec<Card>) -> Option<Rank> {

            let mut grouped_cards: Vec<Vec<&Card>> = Vec::new();

            {

                let mut old_card = cards[0];

                for card in cards {

                    if old_card.value == card.value {

                        grouped_cards.last_mut().unwrap().push(&card);

                    } else {

                        grouped_cards.push(vec![&card]);
                        old_card = *card;

                    }

                }

            }

            /* Holds a sorted 2d of cards sorted by their value. Suit is not
             * cared for.
             */
            


            /* Returns in order: Five of a kind, Quads, Full house, Two Pair
             * Pair, and lastly always a High card.      
             *                                                            
             * This code panics if given a vector of len 0                
             */                                                          
            fn pair (grouped_cards: Vec<Vec<&Card>>) -> Rank {

                /* Putting all cards into a singleton struct. Based on whats in
                 * this struct and how many allows us to better decide what to 
                 * return.
                 */

                struct Pairing {
        
                    FivePair: Vec<Rank>,
                    Quads: Vec<Rank>,
                    Trips: Vec<Rank>,
                    Pair: Vec<Rank>,
                    High: Vec<Rank>,

                };
                
                let mut pair_cards = Pairing {
                    FivePair: Vec::new(),
                    Quads: Vec::new(),
                    Trips: Vec::new(),
                    Pair: Vec::new(),
                    High: Vec::new(),
                };

                /* Notice that these are  */
                use Rank::*;
                for c in grouped_cards {
                    match c.len() {
                        
                        5 =>
                        pair_cards.FivePair.push(FivePair(*c[0],*c[1],*c[2],*c[3],*c[4])),
                        4 => 
                        pair_cards.Quads.push(Quads(*c[0],*c[1],*c[2],*c[3])),
                        3 => 
                        pair_cards.Trips.push(Trips(*c[0],*c[1],*c[2])),
                        2 =>
                        pair_cards.Pair.push(Pair(*c[0],*c[1])),
                        1 =>
                        pair_cards.High.push(High(*c[0])),
                        0 => panic!(),
                        //TODO
                        _ => (),

                    }
                }

                /* Simply return five or four pairs as they are scored highest */

                if let Some(_five_pair) = pair_cards.FivePair.pop() {
                    return _five_pair;
                
                } else if let Some(_quads) = pair_cards.Quads.pop() {
                    return _quads;

                /* In order; check if components in pair_cards is enough to 
                 * build:
                 *  House (else return Trips).
                 *  Two Pair (else return Pair).
                 * 
                 * If all fails return high card.
                 * 
                 * To build higher order enum we have to first break down the
                 * smaller enums. [ t0, t1, p0, ... p11 ] are card references
                 * originally from lesser enums
                 */
                } else {
                    
                    let mut iter_pair = pair_cards.Pair.into_iter().rev();

                    let _trips = pair_cards.Trips.last();
                    let _pair0 = iter_pair.next();

                    /* House or Trips */
                    if let Some(Rank::Trips(t0,t1,t2)) = _trips {
                        if let Some(Pair(p0,p1)) = _pair0 {
                            return House((*t0,*t1,*t2),(p0,p1));

                        } else {
                            return Trips(*t0,*t1,*t2);

                        }

                    /* TwoPair or pair */
                    } else if let Some(Pair(p00,p01)) = _pair0 {
                        if let Some(Pair(p10,p11)) = iter_pair.next() {
                            return TwoPair((p00,p01),(p10,p11));

                        } else {
                            return Pair(p00,p01);

                        }

                    /* High Card */
                    } else if let Some(_high) = pair_cards.High.pop() {
                        return _high;

                    /* All failed somehow. The given vector must be empty */
                    } else {
                        panic!("Can't work without cards!")

                    }

                }
            
            }

            /* Returns in order: either Straight, Flush, Straight Flush, 
             * or None.
             *
             * It's expected that one hand might not fit into any Rank stated, 
             * unlike fn pair(). 
             */
            fn straight_flush (grouped_cards: Vec<Vec<&Card>>) -> Option<Rank> {

                let mut straight_cards: Vec<Vec<&Card>> = Vec::new();

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


                return None;

            }






            return None;

        }

        let rank = ranking(&cards);

        match rank {

            Some(rank) => return Hand { cards: cards, rank: rank },
            None => panic!(),

        }

    }
}
