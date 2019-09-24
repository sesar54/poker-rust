
#[cfg(test)]
mod test {

    #[test]
    fn check_hand() {

        let hand0 = hand!(card!(Ace, Spades), card!(King, Hearts));

        println!("{}", hand0);

    }

}