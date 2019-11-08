extern crate rand;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::*;

pub fn high_card() -> [Card; 1] {
    [card!()]
}

pub fn pair_cards() -> [Card; 2] {
    let mut rng = rand::thread_rng();

    let value = Value::from(rng.gen_range(0, 12));
    let mut suits = [0, 1, 2, 3];
    suits.shuffle(&mut rng);

    card!(value, Suit::from(suits[0]); value, Suit::from(suits[1]))
}

pub fn two_pairs_cards() -> ([Card; 2], [Card; 2]) {
    loop {
        let cards = (pair_cards(), pair_cards());

        if cards.0[0].value != cards.1[0].value {
            return cards;
        }
    }
}

pub fn trips_cards() -> [Card; 3] {
    let mut rng = rand::thread_rng();

    let value = Value::from(rng.gen_range(0, 12));
    let mut suits = [0, 1, 2, 3];
    suits.shuffle(&mut rng);

    card!(
        value, Suit::from(suits[0]);
        value, Suit::from(suits[1]);
        value, Suit::from(suits[2])
    )
}

pub fn house_cards() -> ([Card; 3], [Card; 2]) {
    (trips_cards(), pair_cards())
}

pub fn quad_cards() -> [Card; 4] {
    let mut rng = rand::thread_rng();

    let value = Value::from(rng.gen_range(0, 12));
    let mut suits = [0, 1, 2, 3];
    suits.shuffle(&mut rng);

    card!(
        value, Suit::from(suits[0]);
        value, Suit::from(suits[1]);
        value, Suit::from(suits[2]);
        value, Suit::from(suits[3])
    )
}

pub fn five_cards() -> [Card; 5] {
    let mut rng = rand::thread_rng();

    let value = Value::from(rng.gen_range(0, 12));
    let mut suits = [0, 1, 2, 3];
    suits.shuffle(&mut rng);

    card!(
        value, Suit::from(*suits.choose(&mut rng).unwrap());
        value, Suit::from(*suits.choose(&mut rng).unwrap());
        value, Suit::from(*suits.choose(&mut rng).unwrap());
        value, Suit::from(*suits.choose(&mut rng).unwrap());
        value, Suit::from(*suits.choose(&mut rng).unwrap())
    )
}
