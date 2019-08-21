mod Card;

fn main() {
    
    let hand = Card::Rank::High(1, Card::Card{value: 5, suit: 2,});
    let h = Card::Hand::new(vec!(Card::Card{value: 1, suit: 3 }));

}
