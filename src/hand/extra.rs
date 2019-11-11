use crate::card::{Circular, face::*};
use crate::{card, suits, ranks};

use std::convert::TryFrom;

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

    let rank = Rank::try_from(rng.gen_range(0, 12)).unwrap();
    let suits = suits!(&mut rng);

    let mut pair = card!(rank, suits[0]; rank, suits[1]);
    pair.sort();
    pair
}

///
/// # Example
/// ```
/// # use ace_of_spades::hand::*;
/// # for _ in 0..1_000 {
/// let pairs = two_pairs_cards();
/// Rank::TwoPair(pairs.0, pairs.1)?;
/// # }
/// # Ok::<(), RankErr>(())
/// ```
pub fn two_pairs_cards() -> ([Card; 2], [Card; 2]) {
    loop {
        let cards = (pair_cards(), pair_cards());

        if cards.0[0].rank != cards.1[0].rank {
            return if cards.0[0] > cards.1[0] {
                (cards.1, cards.0)
            } else {
                cards
            };
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

    let rank = ranks!(&mut rng)[0];
    let suits = suits!(&mut rng);

    let mut trips = card!(
        rank, suits[0];
        rank, suits[1];
        rank, suits[2]
    );

    trips.sort();
    trips
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
    let rank = ranks!()[rng.gen_range(1, 10)];

    card!(
        rank,         suits!(&mut rng)[0];
        rank.step(1), suits!(&mut rng)[0];
        rank.step(2), suits!(&mut rng)[0];
        rank.step(3), suits!(&mut rng)[0];
        rank.step(4), suits!(&mut rng)[0]
    )
}

///
/// # Example
/// ```
/// # use ace_of_spades::hand::*;
/// # for _ in 0..1_000 {
/// let (trips, pair) = house_cards();
/// Rank::House(trips, pair)?;
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
    let rank = Rank::try_from(rng.gen_range(0, 12)).unwrap();

    card!(rank, Clubs; rank, Diamonds; rank, Hearts; rank, Spades)
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

    let rank = Rank::try_from(rng.gen_range(0, 12)).unwrap();
    let suits = suits!();

    let mut cards = card!(
        rank, *suits.choose(&mut rng).unwrap();
        rank, *suits.choose(&mut rng).unwrap();
        rank, *suits.choose(&mut rng).unwrap();
        rank, *suits.choose(&mut rng).unwrap();
        rank, *suits.choose(&mut rng).unwrap()
    );
    cards.sort();
    cards
}
