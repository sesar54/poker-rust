mod hand;
mod card;

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    use hand::*;
    use card::Card;

    let rank = Rank::High(Card{value: 4, suit: 3});
    let h = Hand::new(vec!(Card{value: 1, suit: 3 }));

    //h.cards.push(Card{value:3,suit:6});

}
