
/**
 * A vector but all elements share one quirk and th
 * 
 * It acts like a read only vector with a strict push rule.
 * 
 * # Example:
 * ```rust
 * 
 * ```
 */

pub struct Clump<T,F,Q>
    where 
        F: Fn(&T) -> Q,
        Q: PartialEq,
{

    pub quirk: Q,
    pub check: F,
    pub elems: Vec<T>

}

#[allow(dead_code)]
impl<T,F,Q> Clump<T,F,Q> 
    where 
        F: Fn(&T) -> Q,
        Q: PartialEq,
{

    /**
     * Construct a new, empty `Clump<T,F,Q>`.
     * 
     * 
     * 
     * # Example
     * ```
     * let clump = Clump(1, |x| x % 4);
     * clump.push(5);
     * assert!(clump.elems.is_empty());
     * ```
     */
    pub fn new (quirk: Q, check: F) -> Clump<T,F,Q> {
        Clump {
            quirk: quirk,
            check: check,
            elems: vec![],
        }

    }

    pub fn push(&mut self, elem: T) -> bool {
        
        if self.quirk == (self.check)(&elem) {
            self.elems.push(elem);
            return true;

        } else {
            return false;
        }

    }

}


/*
pub fn clump<T,F,E> (slice: &[T], func: F) -> Vec::<&[T]>
    where
        F: Fn(&T) -> E,
        E: PartialEq,

{

    let foo = [ 1, 2, 2, 3, 2, 3, 3 ];
    let bar = clump(&foo, |x| x);

    assert_eq!(bar, [ {1}, {2, 2}, {3}, {2}, {3, 3} ]);



    let trail = slice.iter();

    if let trail_test = func(trail.next()) {

    };

    let last_test = func(trail);

    for &item in slice {

        let test = func(item);

    }


}
*/
