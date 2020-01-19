use crate::card::{face::*, Card};
use crate::r#trait::Circular;
use crate::{ranks, suits};

use std::convert::TryFrom;
use std::convert::TryInto;

extern crate rand;
use rand::seq::SliceRandom;
use rand::Rng;

/// Creates a random, valid high card.
/// # Example
/// ```
/// # use aces_high::{prelude::*, hand::extra::*};
/// # for _ in 0..1_000 {
/// Rank::High(high_card())?;
/// # }
/// # Ok::<(), Error>(())
/// ```
pub fn high_card() -> [Card; 1] {
    [card!(&mut rand::thread_rng())]
}

/// Creates a random, valid pair of cards.
/// # Example
/// ```
/// # use aces_high::{prelude::*, hand::extra::*};
/// # for _ in 0..1_000 {
/// Rank::Pair(pair_cards())?;
/// # }
/// # Ok::<(), Error>(())
/// ```
pub fn pair_cards() -> [Card; 2] {
    let mut rng = rand::thread_rng();

    let rank = Rank::try_from(rng.gen_range(1, 13)).unwrap();
    let suits = suits!(&mut rng);

    // Suit's must differ so to not make the same card.
    let mut pair = cards!(rank, suits[0]; rank, suits[1]);
    pair.sort();
    to_array![pair.iter().map(|c| *c); 2].unwrap()
}

/// Creates a random, valid 2 pairs of cards.
/// # Example
/// ```
/// # use aces_high::{prelude::*, hand::extra::*};
/// # for _ in 0..1_000 {
/// let pairs = two_pairs_cards();
/// Rank::TwoPair(pairs.0, pairs.1)?;
/// # }
/// # Ok::<(), Error>(())
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

/// Creates a random, valid trips of cards.
/// # Example
/// ```
/// # use aces_high::{prelude::*, hand::extra::*};
/// # for _ in 0..1_000 {
/// Rank::Trips(trips_cards())?;
/// # }
/// # Ok::<(), Error>(())
/// ```
pub fn trips_cards() -> [Card; 3] {
    let mut rng = rand::thread_rng();

    let rank = ranks!(&mut rng)[0];
    let suits = suits!(&mut rng);

    // Suit's must differ so to not make the same card.
    let mut trips = cards!(
        rank, suits[0];
        rank, suits[1];
        rank, suits[2]
    );

    trips.sort();
    to_array![trips.iter().map(|c| *c); 3].unwrap()
}

/// Creates a random, valid straight.
/// # Example
/// ```
/// # use aces_high::{prelude::*, hand::extra::*};
/// # for _ in 0..1_000 {
/// Rank::Straight(straight_cards())?;
/// # }
/// # Ok::<(), Error>(())
/// ```
pub fn straight_cards() -> [Card; 5] {
    let mut rng = rand::thread_rng();

    // Generate one value in range: [Ace - Ten]
    let rank = ranks!()[rng.gen_range(1, 10)];
    let suits = suits!(&mut rng);

    let mut rng = || rng.gen_range(0, 3);

    // Make sure that 2 cards don't share a suit,
    // as it prevents creating a straight flush
    let cards = cards!(
        rank,         suits[0];
        rank.step(1), suits[1];
        rank.step(2), suits[rng()];
        rank.step(3), suits[rng()];
        rank.step(4), suits[rng()]
    );
    to_array![cards.iter().map(|c| *c); 5].unwrap()
}

/// Creates a random, valid flush.
/// # Example
/// ```
/// # use aces_high::{prelude::*, hand::extra::*};
/// # for _ in 0..1_000 {
/// Rank::Flush(flush_cards())?;
/// # }
/// # Ok::<(), Error>(())
/// ```
pub fn flush_cards() -> [Card; 5] {
    let mut rng = rand::thread_rng();

    let ranks: [Rank; 5] = loop {
        let mut ranks: [Rank; 5] = ranks!(&mut rng)[..5].try_into().unwrap();
        ranks.sort();

        if ranks[0].step(4) != ranks[4] {
            break ranks;
        }
    };

    let suit = suits!(&mut rng)[0];

    let cards = cards!(
        ranks[0], suit;
        ranks[1], suit;
        ranks[2], suit;
        ranks[3], suit;
        ranks[4], suit
    );
    to_array![cards.iter().map(|c| *c); 5].unwrap()
}

/// Creates a random, valid house of cards.
/// # Example
/// ```
/// # use aces_high::{prelude::*, hand::extra::*};
/// # for _ in 0..1_000 {
/// let (trips, pair) = house_cards();
/// Rank::House(trips, pair)?;
/// # }
/// # Ok::<(), Error>(())
/// ```
pub fn house_cards() -> ([Card; 3], [Card; 2]) {
    (trips_cards(), pair_cards())
}

/// Creates a random, valid quad.
/// # Example
/// ```
/// # use aces_high::{prelude::*, hand::extra::*};
/// # use aces_high::hand::extra::*;
/// # for _ in 0..1_000 {
/// Rank::Quads(quad_cards())?;
/// # }
/// # Ok::<(), Error>(())
/// ```
pub fn quad_cards() -> [Card; 4] {
    let mut rng = rand::thread_rng();
    let rank = Rank::try_from(rng.gen_range(0, 12)).unwrap();
    let suits = suits!();

    // Suit's must differ so to not make the same card.
    let cards = cards!(
        rank, suits[0];
        rank, suits[1];
        rank, suits[2];
        rank, suits[3]
    );
    to_array![cards.iter().map(|c| *c); 4].unwrap()
}

/// Creates a random, valid five cards pair.
/// # Example
/// ```
/// # use aces_high::{prelude::*, hand::extra::*};
/// # for _ in 0..1_000 {
/// Rank::Fives(five_cards())?;
/// # }
/// # Ok::<(), Error>(())
/// ```
pub fn five_cards() -> [Card; 5] {
    let mut rng = rand::thread_rng();

    let rank = Rank::try_from(rng.gen_range(0, 12)).unwrap();
    let suits = suits!();

    let mut rng = || rng.gen_range(0, 3);

    let mut cards = cards!(
        rank, suits[rng()];
        rank, suits[rng()];
        rank, suits[rng()];
        rank, suits[rng()];
        rank, suits[rng()]
    );
    cards.sort();
    to_array![cards.iter().map(|c| *c); 5].unwrap()
}
