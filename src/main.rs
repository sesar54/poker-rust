#![allow(dead_code)]

pub enum Order {
    Ace = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Jack = 10,
    Queen = 11,
    King = 12,
    Joker = 13,

    Size = 14,

}

impl Order {
    pub fn as_str(&self) -> &str {
        match self {
            
            &Order::Ace => "Ace",
            &Order::Two => "",
            &Order::Three => "",
            &Order:: => "",
            &Order:: => "",
            &Order:: => "",
            &Order:: => "",
            &Order:: => "",
            &Order:: => "",
            &Order:: => "",
            &Order:: => "",
            &Order:: => "",
            &Order:: => "",
            &Order:: => "",
            &Order:: => "",


&Order:: => "",
        }
    }
}

let order_names = HashMap<i8,Vec<String>>::new();

pub enum Suits {

    C = 0,
    D = 1,
    H = 2,
    S = 3,

    Size = 4,

}

impl Suits {
    pub fn as_str(&self) -> &str {
        match self {

            &Suits::C => "Clubs",
            &Suits::D => "Diamonds",
            &Suits::H => "Hearts",
            &Suits::S => "Spades",

        }
    }
}

pub struct card {

    serial: i8,

}

impl card {

    pub fn short_name(& self) -> String {

        let card_order = self.serial / 4;

        suits::Diamonds.to_String();

    }
    

}


/**
 * This function takes an vector of cards
 */
fn poker_hand(hand: &[usize]) {

    fn color(card: &i8) ->  {

    }

    /* */
    for card in hand {



    }


}

fn main() {
    
    let v = vec![ 1, 3, 7];

    poker_hand(&v);

}
