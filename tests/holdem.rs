extern crate dead_mans_hand as poker;
extern crate rand;

#[cfg(test)]
mod hand {

    use poker::*;
    use std::thread;
    use rand::Rng;

    #[test]
    fn test() {

        let mut threads = vec!();

        for _ in 0..=3 {

            threads.push(thread::spawn(move || {
                for _ in 0..1000000 {
                    Hand::new(Deck::new().deal(rand::thread_rng().gen_range(1, 52)).unwrap());
                }
            }));

        }

        for thread in threads {
            thread.join().expect("Thread crashed");
        }
        
    }
}
