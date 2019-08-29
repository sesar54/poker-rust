
fn main() {

    let val = 0;

    let not_closure = || -> u8 {

        val

    }();

    println!("{}", not_closure);

}