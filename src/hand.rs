#![feature(drain_filter)]

use super::card::Card;
#[derive(PartialEq, Eq, PartialOrd, Ord)]

/**
 * A Rank consist of a number of cards in a specific configuration. They are
 * sorted by the lowest value first and greatest value last (actually in what
 * order they are written).
 */
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


/**
 * A hand consist of all cards "in hand or private cards" and 
 * "on table or public cards". But the important thing is to value these cards.
 * 
 * If we value our cards, chances are that some are worthless but they are
 * part of our hand. Therefore the cards are slotted into enum struct "Rank".
 * Only the highest ranking cards are saved in it.
 */
pub struct Hand<'a> {

    pub cards: &'a [Card],
    pub rank: Rank<'a>,

}

impl Hand<'_> {

    /**
     * Creating a new hand will cause all given cards to be automatically
     * evaluated into a rank
     */
    pub fn new(cards: &[Card]) -> Hand {

        let rank = Hand::ranking(cards);

        match rank {

            Some(rank) => return Hand { cards: cards, rank: rank },
            None => panic!(),

        }

    }

    /* TODO Rank ACE card */
    /**
     * Takes a sorted slice of any number of cards and return the best card rank 
     * hand.
     * 
     * If given a slice of length 0, there wont be a rank to return so None is 
     * returned. This is considered very erroneous.
     * 
     * If given a unsorted slice, the function cant guarantee the best result  
     */
    fn ranking(cards: &[Card]) -> Option<Rank> {

        /*
         * No hand supports zero card and cards must be sorted before ranking
         */

        if cards.is_empty() { //|| !cards.is_sorted() {
            return None;
        }

        /* Returns in order: Five of a kind, Quads, Full house, Two Pair
         * Pair, and lastly always a High card.      
         *                                                            
         * This code gives none if slice has length 0. This is considered very
         *  erroneous 
         */                                                          
        let pair = || -> Option<Rank> {

            /* Putting all cards into a struct. Based on how many of what
             * we can decide what type of cards we return.
             */
            struct Pairing {
    
                Fives: Vec<Rank>,
                Quads: Vec<Rank>,
                Trips: Vec<Rank>,
                Pair: Vec<Rank>,
                High: Vec<Rank>,

            };
            
            let mut pair = Pairing {
                Fives: Vec::new(),
                Quads: Vec::new(),
                Trips: Vec::new(),
                Pair: Vec::new(),
                High: Vec::new(),
            };


            /* Holds a sorted 2d of cards sorted by their value */
            let grouped_cards = || -> Vec<Vec<&Card>> {
                
                let mut old_card = cards[0];

                let mut grouped_cards: Vec<Vec<&Card>> = Vec::new();

                for card in cards {

                    if old_card.value == card.value {
                        grouped_cards.last_mut().unwrap().push(&card);

                    } else {
                        grouped_cards.push(vec![&card]);
                        old_card = *card;

                    }

                }

                return grouped_cards;

            }();


            /* Notice that these enum structures are simple, taking in just 
             * cards in a linear fashion.
             */
            use Rank::*;
            for c in grouped_cards {
                match c.len() {
                    5 =>
                    pair.Fives.push(FivePair(*c[0],*c[1],*c[2],*c[3],*c[4])),
                    4 => 
                    pair.Quads.push(Quads(*c[0],*c[1],*c[2],*c[3])),
                    3 => 
                    pair.Trips.push(Trips(*c[0],*c[1],*c[2])),
                    2 =>
                    pair.Pair.push(Pair(*c[0],*c[1])),
                    1 =>
                    pair.High.push(High(*c[0])),
                    0 => panic!(),
                    //TODO
                    _ => (),

                }
            }

            /* Simply return five or four pairs as they are scored highest */

            if let Some(_five_pair) = pair.Fives.pop() {
                return Some(_five_pair);
            
            } else if let Some(_quads) = pair.Quads.pop() {
                return Some(_quads);

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
                
                let mut iter_pair = pair.Pair.into_iter().rev();

                let _trips = pair.Trips.last();
                let _pair0 = iter_pair.next();

                /* House or Trips */
                if let Some(Rank::Trips(t0,t1,t2)) = _trips {
                    if let Some(Pair(p0,p1)) = _pair0 {
                        return Some(House((*t0,*t1,*t2),(p0,p1)));

                    } else {
                        return Some(Trips(*t0,*t1,*t2));

                    }

                /* TwoPair or pair */
                } else if let Some(Pair(p00,p01)) = _pair0 {
                    if let Some(Pair(p10,p11)) = iter_pair.next() {
                        return Some(TwoPair((p00,p01),(p10,p11)));

                    } else {
                        return Some(Pair(p00,p01));

                    }

                /* High Card */
                } else if let Some(_high) = pair.High.pop() {
                    return Some(_high);

                /* All failed somehow. The given vector must be empty */
                } else {
                    return None;

                }

            }
        
        }();

        /* Return early else unwrap pair. If pair is None straight_flush will 
         * also be None, therefore return an early None.
         */
        match pair {
            
            Some(Rank::FivePair(..)) | None => return pair,
            _ => (),

        }
        
        let pair = pair.unwrap();
        

        /* Returns in order: either Straight, Flush, Straight Flush, 
         * or None.
         *
         * It's expected that one hand might not fit into any Rank stated, 
         * unlike fn pair(). 
         */
        let straight_flush = || -> Option<Rank> {

            /* First check for straight cards */
            let mut straight_cards: Vec<Vec<&Card>> = || -> Vec<Vec<&Card>> {

                let straight_cards: Vec<Vec<&Card>> = Vec::new();
                
                /* Lets see what happens if we don't initialize this */
                let mut last_val: u8; 

                /* Find coherent cards and group them together */
                for card in cards {

                    let val = card.value as u8;

                    if let Some(last_vec) = straight_cards.last() {
                        if val == last_val || val == last_val + 1 {
                            last_vec.push(card);

                        /* 
                        * We could check if the last vector isn't large enough
                        * for a straight card here but we would still also need 
                        * to check the last vector after this loop. The dry way 
                        * is to filter all later. 
                        
                        } else if last_vec.len() < 5 {
                            last_vec.clear();
                        */

                        /* Start a new series of cards */
                        } else {
                            straight_cards.push(vec![card]);

                        }

                    /* Create initial vector */
                    } else {
                        straight_cards.push(vec![card]);
                    
                    }

                    last_val = val;

                }

                /* Filtering insufficient number of cards for all groupings */
                straight_cards.drain_filter(|f| f.len() < 5);


                /* Sort by last card in each sub vector */
                straight_cards.sort_by_key(|f| f.last().unwrap());

                return straight_cards;

            }();


            let mut flush_cards: Vec<&Vec<&Card>> = Vec::with_capacity(super::card::Suit::Size);
            
            for card in cards {
                flush_cards[card.suit as usize].push(&card);
            }

            /* Filtering insufficient number of cards for all groupings */
            flush_cards.drain_filter(|f| f.len() < 5);


            if straight_cards.is_empty() {
                if flush_cards.is_empty() {
                    return None;

                } else {
                    
                    for (_, cards) in flush_cards {
                        if cards.len() >= 5 {
                            


                        } 

                    }

                    return None;

                }


            }

            return None;

        }();


        /* Compare and return a rank */
        if let Some(straight_flush) = straight_flush {
            return Some(std::cmp::max(pair, straight_flush));

        } else {
            return Some(pair);

        }


    }

    pub fn update(&self, cards: Vec<Card>) {

    }

}
