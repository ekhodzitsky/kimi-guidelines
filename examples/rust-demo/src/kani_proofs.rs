//! Formal Verification Proofs using Kani
//!
//! Install Kani: `cargo install --locked kani-verifier`
//! Run proofs: `cargo kani`
//!
//! These proofs verify that SortedVec maintains its invariant
//! for all possible inputs within bounded ranges.

#[cfg(kani)]
mod proofs {
    use crate::sorted_vec::SortedVec;

    /// Proof: Inserting any single element into an empty SortedVec
    /// maintains the sorted invariant.
    #[kani::proof]
    fn insert_single_maintains_sorted() {
        let mut sv = SortedVec::new();
        let item: i32 = kani::any();
        kani::assume(item >= 0 && item <= 100);

        sv.insert(item);

        let slice = sv.as_slice();
        assert!(slice.len() == 1);
        assert!(slice[0] == item);
    }

    /// Proof: Inserting two elements into an empty SortedVec
    /// maintains the sorted invariant.
    #[kani::proof]
    fn insert_two_maintains_sorted() {
        let mut sv = SortedVec::new();
        let a: i32 = kani::any();
        let b: i32 = kani::any();
        kani::assume(a >= 0 && a <= 100);
        kani::assume(b >= 0 && b <= 100);

        sv.insert(a);
        sv.insert(b);

        let slice = sv.as_slice();
        assert!(slice.len() == 2);
        assert!(slice[0] <= slice[1]);
    }

    /// Proof: from_vec produces a sorted output for any 3-element input.
    #[kani::proof]
    fn from_vec_three_elements_maintains_sorted() {
        let a: i32 = kani::any();
        let b: i32 = kani::any();
        let c: i32 = kani::any();
        kani::assume(a >= 0 && a <= 100);
        kani::assume(b >= 0 && b <= 100);
        kani::assume(c >= 0 && c <= 100);

        let sv = SortedVec::from_vec(vec![a, b, c]);

        let slice = sv.as_slice();
        assert!(slice.len() == 3);
        assert!(slice[0] <= slice[1]);
        assert!(slice[1] <= slice[2]);
    }
}
