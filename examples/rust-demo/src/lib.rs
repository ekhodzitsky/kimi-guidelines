//! Theorem: Mathematical Programming in Rust
//!
//! This crate demonstrates the principles from kimi-dotfiles:
//! - Types as axioms (Newtype, Phantom, Typestate)
//! - Functions as lemmas (Hoare logic, contracts)
//! - Algebraic structures (Semigroup, Monoid)
//! - Property-based testing (universal quantification)

pub mod algebra;
pub mod units;
pub mod sorted_vec;

#[cfg(kani)]
pub mod kani_proofs;
