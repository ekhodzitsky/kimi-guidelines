//! Theorem: `SortedVec<T>` — A Vector Maintaining Order Invariant
//!
//! Invariant: for all i: `0 <= i < len-1` implies `data[i] <= data[i+1]`

/// Axiom: `SortedVec<T>` maintains ascending order after every operation
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SortedVec<T: Ord + Clone> {
    data: Vec<T>,
}

impl<T: Ord + Clone> SortedVec<T> {
    /// { true }
    /// fn new() -> SortedVec<T>
    /// { ret.data.is_empty() }
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// { true }
    /// fn from_vec(v: Vec<T>) -> SortedVec<T>
    /// { ret.data is sorted permutation of v }
    pub fn from_vec(mut v: Vec<T>) -> Self {
        v.sort();
        Self { data: v }
    }

    /// { true }
    /// fn insert(&mut self, item: T)
    /// { post: self.data is sorted && contains item }
    pub fn insert(&mut self, item: T) {
        let idx = self.data.binary_search(&item).unwrap_or_else(|e| e);
        self.data.insert(idx, item);
        // Invariant preserved by binary_search insertion point
    }

    /// { true }
    /// fn contains(&self, item: &T) -> bool
    /// { ret ==> item in self.data }
    pub fn contains(&self, item: &T) -> bool {
        self.data.binary_search(item).is_ok()
    }

    /// { true }
    /// fn len(&self) -> usize
    /// { ret == self.data.len() }
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// { true }
    /// fn is_empty(&self) -> bool
    /// { ret == (self.data.len() == 0) }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// { true }
    /// fn as_slice(&self) -> &[T]
    /// { ret == &self.data }
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }
}

impl<T: Ord + Clone> Default for SortedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn insert_maintains_order() {
        let mut sv = SortedVec::new();
        sv.insert(3);
        sv.insert(1);
        sv.insert(4);
        sv.insert(1);
        sv.insert(5);
        assert_eq!(sv.as_slice(), &[1, 1, 3, 4, 5]);
    }

    proptest! {
        #[test]
        fn from_vec_preserves_elements(v in prop::collection::vec(0i32..100, 0..50)) {
            let mut original = v.clone();
            original.sort();
            let sv = SortedVec::from_vec(v);
            assert_eq!(sv.as_slice(), original.as_slice());
        }

        #[test]
        fn insert_then_sorted(initial in prop::collection::vec(0i32..100, 0..20), item in 0i32..100) {
            let mut sv = SortedVec::from_vec(initial);
            sv.insert(item);
            let slice = sv.as_slice();
            for i in 1..slice.len() {
                assert!(slice[i-1] <= slice[i], "invariant violated at index {}", i);
            }
        }

        #[test]
        fn len_increases_after_insert(
            initial in prop::collection::vec(0i32..100, 0..20),
            item in 0i32..100
        ) {
            let mut sv = SortedVec::from_vec(initial);
            let old_len = sv.len();
            sv.insert(item);
            assert_eq!(sv.len(), old_len + 1);
        }

        #[test]
        fn contains_after_insert(
            initial in prop::collection::vec(0i32..100, 0..20),
            item in 0i32..100
        ) {
            let mut sv = SortedVec::from_vec(initial);
            sv.insert(item);
            assert!(sv.contains(&item));
        }
    }
}
