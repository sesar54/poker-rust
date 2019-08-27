mod Card;

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    
    let _hand = Card::Rank::High(Card::Card{value: 5, suit: 2,});
    let h = Card::Hand::new(vec!(Card::Card{value: 1, suit: 3 }));

}
