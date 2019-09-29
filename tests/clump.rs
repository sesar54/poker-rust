use lib::*;

#[test]
fn clump_new() {

    let mut clump = Clump::new(1, |x| x % 4);
    clump.push(5);
    assert!(clump.elems.is_empty());

}