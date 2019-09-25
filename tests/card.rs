#[cfg(test)]
mod test {
    
    use lib::{Value::*, Suit::*, *};
    
    #[test]
    fn check_build() {

        let card0 = Card {
            value: Eight,
            suit: Diamonds,
        };
        
        let card1 = card!(Ace, Spades);

        assert_ne!(card0, card1);

        let i: u8 = card1.into();

        println!("{:#X}", i);

    }

}