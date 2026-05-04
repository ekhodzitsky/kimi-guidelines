//! Guidelines demonstration: types proving invariants, contracts in comments,
//! property-based testing, and mechanized verification.
//!
//! # Example
//! ```
//! use rust_demo::units::*;
//! use rust_demo::sorted_vec::SortedVec;
//! use rust_demo::algebra::{Semigroup, Monoid};
//!
//! // Types prove invariants: cannot add meters to seconds
//! let d = Quantity::<Meters>::meters(100.0);
//! let t = Quantity::<Seconds>::seconds(10.0);
//! let v = velocity(d, t);
//! assert!((v.value() - 10.0).abs() < f64::EPSILON);
//!
//! // SortedVec maintains invariant automatically
//! let mut sv = SortedVec::new();
//! sv.insert(3);
//! sv.insert(1);
//! assert_eq!(sv.as_slice(), &[1, 3]);
//!
//! // Monoid laws verified by property tests
//! let a = "hello".to_string();
//! let b = "world".to_string();
//! assert_eq!(a.combine(&b), "helloworld");
//! ```

pub mod algebra;
pub mod units;
pub mod sorted_vec;

#[cfg(kani)]
pub mod kani_proofs;
