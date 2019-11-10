use crate::card::{Card, Value};
use crate::{card, values, suits};

extern crate rand;
use rand::seq::SliceRandom;
use rand::Rng;

///
/// # Example
/// ```
/// # use ace_of_spades::hand::*;
/// # for _ in 0..1_000 {
/// Rank::High(high_card())?;
/// # }
/// # Ok::<(), RankErr>(())
/// ```
pub fn high_card() -> [Card; 1] {
    [card!(&mut rand::thread_rng())]
}

///
/// # Example
/// ```
/// # use ace_of_spades::hand::*;
/// # for _ in 0..1_000 {
/// Rank::Pair(pair_cards())?;
/// # }
/// # Ok::<(), RankErr>(())
/// ```
pub fn pair_cards() -> [Card; 2] {
    let mut rng = rand::thread_rng();

    let value = Value::from(rng.gen_range(0, 12));
    let suits = suits!(&mut rng);

    card!(value, suits[0]; value, suits[1])
}

///
/// # Example
/// ```
/// # use ace_of_spades::hand::*;
/// # for _ in 0..1_000 {
/// Rank::TwoPair(two_pairs_cards())?;
/// # }
/// # Ok::<(), RankErr>(())
/// ```
pub fn two_pairs_cards() -> ([Card; 2], [Card; 2]) {
    loop {
        let cards = (pair_cards(), pair_cards());

        if cards.0[0].value != cards.1[0].value {
            return cards;
        }
    }
}

///
/// # Example
/// ```
/// # use ace_of_spades::hand::*;
/// # for _ in 0..1_000 {
/// Rank::Trips(trips_cards())?;
/// # }
/// # Ok::<(), RankErr>(())
/// ```
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

///
/// # Example
/// ```
/// # use ace_of_spades::hand::*;
/// # for _ in 0..1_000 {
/// Rank::Straight(straight_cards())?;
/// # }
/// # Ok::<(), RankErr>(())
/// ```
pub fn straight_cards() -> [Card; 5] {
    let mut rng = rand::thread_rng();

    /* Generate one value in range: [Ace - Ten] */
    let value = values!()[rng.gen_range(1, 10)];

    card!(
        value,         suits!(&mut rng)[0];
        value.step(1), suits!(&mut rng)[0];
        value.step(2), suits!(&mut rng)[0];
        value.step(3), suits!(&mut rng)[0];
        value.step(4), suits!(&mut rng)[0]
    )


}

///
/// # Example
/// ```
/// # use ace_of_spades::hand::*;
/// # for _ in 0..1_000 {
/// Rank::House(house_cards())?;
/// # }
/// # Ok::<(), RankErr>(())
/// ```
pub fn house_cards() -> ([Card; 3], [Card; 2]) {
    (trips_cards(), pair_cards())
}

///
/// # Example
/// ```
/// # use ace_of_spades::hand::*;
/// # for _ in 0..1_000 {
/// Rank::Quads(quad_cards())?;
/// # }
/// # Ok::<(), RankErr>(())
/// ```
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

///
/// # Example
/// ```
/// # use ace_of_spades::hand::*;
/// # for _ in 0..1_000 {
/// Rank::Fives(five_cards())?;
/// # }
/// # Ok::<(), RankErr>(())
/// ```
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
