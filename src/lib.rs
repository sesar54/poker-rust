// Modules
pub mod card;
pub mod deck;

pub mod holdem;

// Prelude
pub use card::{ Suit::*, Value::*, * };

#[macro_use]
pub mod macros;

pub struct Clump<T,F,E>
    where 
        F: Fn(&T) -> E,
        E: PartialEq,
{

    pub r#trait: E,
    pub test: F,
    pub elems: Vec<T>,

}

impl<T,F,E> Clump<T,F,E> 
    where 
        F: Fn(&T) -> E,
        E: PartialEq,
{

    fn new (r#trait: E, test: F) -> Clump<T,F,E> {
        Clump {
            r#trait: r#trait,
            test: test,
            elems: vec![],
        }

    }

    fn news (elem: T, test: F) -> Clump<T,F,E> {
        Clump {
            r#trait: test(&elem),
            test: test,
            elems: vec![elem],
        }
    }

    fn push(&mut self, element: T) {
        


    }

    fn teste(&self, element: &T) -> bool {

        test(element)

    }

}


/*
 * Clumps together adjacent elements into a vector of iterators, by comparing
 * their data to be equal to func.
 * 
 * Example:
 * ```rust
 * let foo = [ 1, 2, 2, 3, 2, 3, 3 ];
 * let bar = clump(&foo, )
 * 
 * assert_eq!(bar, [ [1], [2, 2], [3], [2], [3, 3] ]);
 * ```
 *
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


}*/
