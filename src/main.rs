use crate::holdem;

#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {

    let rank = Rank::High(Card{value: Ace, suit: Clubs});

    let cards = vec!(Card{value: Three, suit: Hearts});

    let h = Hand::new(&cards);

    h.cards;

    //h.cards.push(Card{value:3,suit:6});

}
