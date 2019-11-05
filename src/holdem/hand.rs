use std::fmt;

use crate::*;
use holdem::*;

impl Hand {
    /// Creating a new hand will cause all given cards to be automatically
    /// evaluated into a rank
    pub fn new(cards: Vec<Card>) -> Hand {
        let rank = Hand::ranking(&cards);

        match rank {
            Ok(rank) => {
                return Hand {
                    cards: cards,
                    rank: rank,
                }
            }
            Err(e) => panic!("{:#?}", e),
        }
    }

    /// Takes a sorted slice of any number of cards and return the best card rank
    /// hand.
    ///
    /// If given a slice of length 0, there wont be a rank to return so None is
    /// returned. This is considered very erroneous.
    ///
    /// If given a unsorted slice, the function will not work properly

    fn ranking(cards: &[Card]) -> Result<Rank, RankErr> {
        if cards.is_empty() {
            //panic!("No cards were given");
            return Err(RankErr::Explained(String::from("No cards were given")));
        }

        let pair = Hand::pair_rank(cards)?;

        let straight_flush = Hand::straight_flush_rank(cards);

        /* Compare and return a rank */
        if let Some(straight_flush) = straight_flush {
            Ok(std::cmp::max(pair, straight_flush))
        } else {
            Ok(pair)
        }
    }

    fn pair_rank(cards: &[Card]) -> Result<Rank, RankErr> {
        let pair = Hand::pair_groups(cards).pop().unwrap();
        match pair.len() {
            len @ 5..0xFF => {
                let pair = &pair[len - 5..];
                Rank::Fives([pair[0], pair[1], pair[2], pair[3], pair[4]])
            }
            4 => Rank::Quads([pair[0], pair[1], pair[2], pair[3]]),
            3 => Rank::Trips([pair[0], pair[1], pair[2]]),
            2 => Rank::Pair([pair[0], pair[1]]),
            1 => Rank::High(pair[0]),
            _ => unreachable!(),
        }
    }

    /// TODO Proof read
    /// Returns after checking in order:
    /// Some    StraightFlush,
    ///         Flush,
    ///         Straight.
    /// None
    fn straight_flush_rank(cards: &[Card]) -> Option<Rank> {
        // Copy, sort and const.
        let mut cards = cards.to_vec();
        cards.sort();
        let cards = cards;

        // Iterate over all flushes for all flushes that has 5 or more cards,
        // Then run those cards through the straight functions and
        // return if straight was found.
        //
        // Function straight_cards will always return the most valuable 5 cards,
        // or None. We are not returning None now as we still can return flush
        // and test for more straights.
        let flush_group = Hand::flush_groups(cards.as_slice());
        let mut flush_iter = flush_group.iter().rev().filter(|v| v.len() >= 5);

        while let Some(flush) = flush_iter.next() {
            let straight = Hand::straight_cards(&Hand::straight_groups(flush.as_slice()));

            if let Some(..) = straight {
                return straight;
            }
        }

        // If no straight was found it means that either there were no flush
        // groupings over the size of 5 or simply no straight was found for
        // in any flush range.
        //
        // Now iterate again over flush_group and return a flush or don't.
        let mut flush_iter = flush_group.iter().filter(|v| v.len() >= 5);

        if let Some(flush) = flush_iter.next() {
            let flush = [flush[0], flush[1], flush[2], flush[3], flush[4]];
            return Some(Rank::Flush(flush).unwrap());
        }

        // Now just see if there exist a straight in any of the cards
        // A straight ranks lower than a flush so they are checked
        // here after flush.
        //
        // Now return whatever straight_cards is returning.
        // It may it be Some or None.
        Hand::straight_cards(&Hand::straight_groups(cards.as_slice()))
    }

    /// Returns cards grouped together by these rules:
    /// 1. Cards are sorted by it's value first.
    /// 2. Cards are grouped together if their neighbor has the same value.
    fn pair_groups(cards: &[Card]) -> Vec<Vec<Card>> {
        let mut cards = cards.to_vec();
        cards.sort();

        // Value to be returned
        let mut pairs: Vec<Vec<Card>> = Vec::new();
        // Main Sequence Generator
        let mut iter = cards.iter().cloned().peekable();
        let mut temp_vec: Vec<Card> = Vec::new();
        let mut prev_value = iter.peek().unwrap().value;

        while let Some(card) = iter.next() {
            if prev_value == card.value {
                temp_vec.push(card);
            } else {
                pairs.push(temp_vec);
                temp_vec = vec![card];
                prev_value = card.value;
            }
        }

        pairs.push(temp_vec);
        pairs
    }

    /// Returns cards grouped together by these rules:
    /// 1. Cards are sorted by it's value first.
    /// 2. Cards are grouped together if their neighbor has the same value
    ///     OR value + 1
    /// 3. If the absolute last card is a King
    ///     and the absolute first card is an Ace,
    ///     make a copy of the last group and append it with the Ace card.
    /// TODO update 3
    /// This function does not attempt to return a straight!
    fn straight_groups(cards: &[Card]) -> Vec<Vec<Card>> {
        let mut cards = cards.to_vec();
        cards.sort();

        // Value to be returned
        let mut straight_groupings = Vec::<Vec<Card>>::new();
        // Main Sequence Generator
        let mut iter = cards.iter().cloned();
        let mut temp_vec;
        let mut prev_value;

        // First card initiates things
        if let Some(first_card) = iter.next() {
            prev_value = first_card.value;
            temp_vec = vec![first_card];
        } else {
            return straight_groupings;
        }

        while let Some(card) = iter.next() {
            if prev_value == card.value || card.value as u8 == prev_value as u8 + 1 {
                temp_vec.push(card);
            // Drop temp_vec into straight_groupings and start a new one
            } else {
                straight_groupings.push(temp_vec);
                temp_vec = vec![card];
            }

            prev_value = card.value;
        }

        straight_groupings.push(temp_vec);

        // Ace rule (Not proven broadway).
        // Copy the last straight grouping and append Ace as the last argument,
        // if both an Ace and a King is found in the slice cards.
        //
        // Cards are sorted numerically so they appear as the first and
        // last card.
        if let Some(ace_maybe) = cards.first() {
            if ace_maybe.value == Ace {
                if let Some(king_perhaps) = cards.last() {
                    if king_perhaps.value == King {
                        if let Some(broadway) = straight_groupings.last() {
                            let mut broadway = broadway.clone();

                            broadway.push(*ace_maybe);
                            straight_groupings.push(broadway);
                        }
                    }
                }
            }
        }

        straight_groupings
    }

    /// Returns cards grouped together by these rules:
    /// 1. Cards are sorted by it's suit first.
    /// 2. Cards are grouped together if their neighbor has the same suit.
    fn flush_groups(cards: &[Card]) -> Vec<Vec<Card>> {
        let mut cards = cards.to_vec();
        cards.sort_by_key(|c| c.suit);

        // Value to be returned
        let mut flushes: Vec<Vec<Card>> = Vec::new();
        // Main Sequence Generator
        let mut iter = cards.iter().cloned().peekable();
        let mut temp_vec: Vec<Card> = Vec::new();
        let mut prev_suit = iter.peek().unwrap().suit;

        while let Some(card) = iter.next() {
            if prev_suit == card.suit {
                temp_vec.push(card);
            } else {
                flushes.push(temp_vec);
                temp_vec = vec![card];
                prev_suit = card.suit;
            }
        }

        flushes.push(temp_vec);
        flushes
    }

    /// Returns either 5 cards in a straight or None.
    fn straight_cards(cards: &Vec<Vec<Card>>) -> Option<Rank> {
        let mut straight_iter = cards.iter().rev().filter(|v| v.len() >= 5);
        /* Has to be initialized */
        let mut straights = [card!(); 5];
        while let Some(cards) = straight_iter.next() {
            let mut card_iter = cards.iter().rev();
            let mut space_iter = 4;
            let mut cards_left = cards.len();
            let mut prev_value;

            if let Some(first_card) = card_iter.next() {
                cards_left -= 1;

                card_iter.len();

                prev_value = first_card.value;
                straights[space_iter] = *first_card;
                space_iter -= 1;
            } else {
                eprintln!("[hand/straight_cards]: First series of card are empty");
                continue;
            }

            while let Some(card) = card_iter.next() {
                cards_left -= 1;

                if card.value < prev_value {
                    prev_value = card.value;
                    straights[space_iter] = *card;

                    // Return before overflow if straights was successfully
                    // constructed.
                    if space_iter == 0 {
                        return Some(Rank::Straight(straights).expect("TEWST"));
                    } else {
                        space_iter -= 1;
                    }
                // Break if there is not enough cards to construct with.
                } else if cards_left <= space_iter {
                    break;
                }
            }
        }

        None
    }

    //pub fn update(&self, cards: Vec<Card>) {}
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.cards)
    }
}
