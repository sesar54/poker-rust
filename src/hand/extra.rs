use crate::card::{Card, Value};
use crate::{card, values, suits};

extern crate rand;
use rand::seq::SliceRandom;
use rand::Rng;

pub fn high_card() -> [Card; 1] {
    [card!(&mut rand::thread_rng())]
}

pub fn pair_cards() -> [Card; 2] {
    let mut rng = rand::thread_rng();

    let value = Value::from(rng.gen_range(0, 12));
    let suits = suits!(&mut rng);

    card!(value, suits[0]; value, suits[1])
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

    let value = values!(&mut rng)[0];
    let suits = suits!(&mut rng);

    card!(
        value, suits[0];
        value, suits[1];
        value, suits[2]
    )
}

pub fn straight_cards() -> [Card; 5] {
    let mut rng = rand::thread_rng();

    let value = values!()[rng.gen_range(0, 9)];

    card!(
        value,         suits!(&mut rng)[0];
        value.next(1), suits!(&mut rng)[0];
        value.next(2), suits!(&mut rng)[0];
        value.next(3), suits!(&mut rng)[0];
        value.next(4), suits!(&mut rng)[0]
    )

}

pub fn house_cards() -> ([Card; 3], [Card; 2]) {
    (trips_cards(), pair_cards())
}

pub fn quad_cards() -> [Card; 4] {
    let mut rng = rand::thread_rng();

    let value = Value::from(rng.gen_range(0, 12));
    let suits = suits!(&mut rng);

    card!(
        value, suits[0];
        value, suits[1];
        value, suits[2];
        value, suits[3]
    )
}

pub fn five_cards() -> [Card; 5] {
    let mut rng = rand::thread_rng();

    let value = Value::from(rng.gen_range(0, 12));
    let suits = suits!();

    card!(
        value, *suits.choose(&mut rng).unwrap();
        value, *suits.choose(&mut rng).unwrap();
        value, *suits.choose(&mut rng).unwrap();
        value, *suits.choose(&mut rng).unwrap();
        value, *suits.choose(&mut rng).unwrap()
    )
}
