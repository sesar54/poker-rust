#[cfg(test)]
mod hand {

    extern crate ace_of_spades as poker;
    use poker::prelude::*;

    use std::thread;

    extern crate rand;
    use rand::Rng;

    #[test]
    #[ignore]
    fn iterate_over_hands() {
        let mut threads = vec![];

        for _ in 0..1 {
            threads.push(thread::spawn(move || {
                for _ in 0..1_000_000 {
                    Hand::new(
                        Deck::new_shuffled()
                            .deal(rand::thread_rng().gen_range(1, 52))
                            .unwrap(),
                    );
                }
            }));
        }

        for thread in threads {
            thread.join().expect("Thread crashed");
        }
    }

    #[test]
    fn straight_card() {
        hand::extra::straight_cards();
    }
}
