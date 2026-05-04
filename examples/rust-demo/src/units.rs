//! Theorem: Physical Dimensions via Phantom Types
//!
//! Invariant: A Quantity<T> carries a dimension T. Operations on mismatched dimensions are compile-time errors.

use std::marker::PhantomData;

pub struct Meters;
pub struct Seconds;
pub struct MetersPerSecond;

/// Axiom: Quantity<T> represents a physical quantity with dimension T
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Quantity<T>(f64, PhantomData<T>);

impl Quantity<Meters> {
    /// { true }
    /// fn meters(v: f64) -> Quantity<Meters>
    /// { ret.0 == v }
    pub fn meters(v: f64) -> Self {
        Self(v, PhantomData)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Quantity<Seconds> {
    /// { true }
    /// fn seconds(v: f64) -> Quantity<Seconds>
    /// { ret.0 == v }
    pub fn seconds(v: f64) -> Self {
        Self(v, PhantomData)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Quantity<MetersPerSecond> {
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// { time.0 != 0.0 }
/// fn velocity(dist: Quantity<Meters>, time: Quantity<Seconds>) -> Quantity<MetersPerSecond>
/// { ret.0 == dist.0 / time.0 }
pub fn velocity(
    dist: Quantity<Meters>,
    time: Quantity<Seconds>,
) -> Quantity<MetersPerSecond> {
    debug_assert!(time.0 != 0.0, "time must be non-zero");
    Quantity(dist.0 / time.0, PhantomData)
}

/// { true }
/// fn add_distances(a: Quantity<Meters>, b: Quantity<Meters>) -> Quantity<Meters>
/// { ret.0 == a.0 + b.0 }
pub fn add_distances(
    a: Quantity<Meters>,
    b: Quantity<Meters>,
) -> Quantity<Meters> {
    Quantity(a.0 + b.0, PhantomData)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn velocity_computes_correctly() {
        let d = Quantity::<Meters>::meters(100.0);
        let t = Quantity::<Seconds>::seconds(10.0);
        let v = velocity(d, t);
        assert!((v.value() - 10.0).abs() < f64::EPSILON);
    }

    proptest! {
        #[test]
        fn add_distances_commutative(a in 0.0f64..1000.0, b in 0.0f64..1000.0) {
            let x = Quantity::<Meters>::meters(a);
            let y = Quantity::<Meters>::meters(b);
            assert_eq!(
                add_distances(x, y).value(),
                add_distances(y, x).value()
            );
        }
    }
}
