#[macro_use]
extern crate dead_mans_hand as poker;

extern crate rand;

use rand::seq::SliceRandom;

#[cfg(test)]
mod deck {
    use poker::*;
    use poker::card;
    use prelude::*;
    use Suit::*;
    use Value::*;
    use macros;
    use macros::*;

    #[test]
    #[ignore]
    fn test() {
        let mute = values!();
        //let cards = card!(Ace, Spades; King, Hearts);
        //let deck = Deck::new_custom(&cards);
        //println!("{:?}", deck);
    }
}
