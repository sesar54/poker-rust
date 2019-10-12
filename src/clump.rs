
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

pub struct Clump<T,C>
    where
        C: Fn(&T, &T) -> bool,
{

    pub check: C,
    pub elems: Vec<Vec<T>>

}

#[allow(dead_code)]
impl<T,C> Clump<T,C>
    where
        C: Fn(&T, &T) -> bool,
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
    pub fn new (check: C) -> Clump<T,C> {
        Clump {
            check: check,
            elems: vec![],
        }
    }

    pub fn push(&mut self, elem: T) -> bool {

        let last_elem = {
            if let Some(vec) = self.elems.last() {
                vec.last()

            } else {
                self.elems.push(vec![elem]);
                return true;

            }

        };

        if let Some(last_elem) = last_elem {
            if (self.check)(&elem, last_elem) {
                self.push(elem);

            } else {
                return false;

            }

        } else {
            self.elems.push(vec![elem]);

        }

        return true;

    }

}

#[cfg(test)]
mod test {

    use crate::card::{Value::*, Suit::*, Card};

    #[test]
    fn test() {

        let cards = vec!(card!(Ace, Spades), card!(Ace, Spades), card!(Two, Spades), card!(Ace, Spades), card!(Five, Spades), card!(Ace, Spades), card!(Ace, Spades));

        let clump = clump!(|c: &Card| c.value, cards);

        println!("{:?}", clump.elems);


    }
}
