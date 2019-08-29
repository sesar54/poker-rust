mod hand;
mod card;

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    use hand::*;
    use card::Card;
    use card::*;
    use Suit::*;
    use Value::*;

    let rank = Rank::High(Card{value: Ace, suit: Clubs});
    let h = Hand::new(vec!(Card{value: Three, suit: Hearts}));

    h.cards;

    //h.cards.push(Card{value:3,suit:6});

}
