//! Theorem: Monoid Structure for String Concatenation
//!
//! Invariant: String concatenation is associative with "" as identity.

use std::marker::PhantomData;

/// Axiom: ∀a,b,c. combine(a, combine(b, c)) == combine(combine(a, b), c)
pub trait Semigroup: Clone + PartialEq {
    fn combine(&self, other: &Self) -> Self;
}

/// Axiom:
/// - Associativity (inherited from Semigroup)
/// - Identity: ∃e. ∀a. combine(e, a) == a && combine(a, e) == a
pub trait Monoid: Semigroup {
    fn identity() -> Self;
}

/// { true }
/// impl Semigroup for String
/// { combine(a, b) == a + b (concatenation) }
impl Semigroup for String {
    fn combine(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.push_str(other);
        result
    }
}

/// { true }
/// impl Monoid for String
/// { identity() == "" }
impl Monoid for String {
    fn identity() -> Self {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn associativity(a in "[a-z]*", b in "[a-z]*", c in "[a-z]*") {
            let left = a.combine(&b.combine(&c));
            let right = a.combine(&b).combine(&c);
            assert_eq!(left, right);
        }

        #[test]
        fn left_identity(a in "[a-z]*") {
            let e = String::identity();
            assert_eq!(e.combine(&a), a);
        }

        #[test]
        fn right_identity(a in "[a-z]*") {
            let e = String::identity();
            assert_eq!(a.combine(&e), a);
        }
    }
}
