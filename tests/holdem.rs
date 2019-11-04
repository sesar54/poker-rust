extern crate dead_mans_hand as poker;

#[cfg(test)]
mod hand {

    use poker::*;

    #[test]
    fn test() {

        for _ in 0..10000 {

            Hand::new(Deck::new().deal(7).unwrap());

        }
    }
}
