extern crate dead_mans_hand as poker;

#[cfg(test)]
mod hand {

    use poker::*;
    use poker::holdem::Hand;

    #[test]
    fn test() {

        // let cards = cards!(Ace, Spades; King, Spades; Queen, Diamonds; Jack, Clubs; Ten, Clubs; Nine, Spades; Eight, Spades; Seven, Spades);

        let mut card_as_numb = Vec::<u8>::new();

        fn overflow_clock(vec: &mut Vec<u8>, i: usize) {

            if vec.len() == i {
                vec.push(0);

            } else {
                vec[i] += 1;

                if vec[i] > 52 {
                    vec[i] = 0;

                    if vec.len() -1 == i {
                        vec.push(0)
                    } else {
                        vec[i+1] +=1;
                        overflow_clock(vec, i+1);
                    }

                }


            }
        }

        while card_as_numb.len() <= 5 {
            overflow_clock(&mut card_as_numb, 0);

            let mut cards: Vec<Card> = Vec::new();

            for i in card_as_numb.iter() {
                cards.push(Card::from(*i));
            }

            let hand = Hand::new(&cards);
            println!("{:?}", hand);


        }

    }
}
