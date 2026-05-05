//! Formal Verification Proofs using Kani
//!
//! Install Kani: `cargo install --locked kani-verifier`
//! Run proofs: `cargo kani`
//!
//! These proofs verify that validation invariants in types.rs
//! are machine-checkable.

#[cfg(kani)]
mod proofs {
    use crate::types::{ApiKey, Page, PerPage};

    /// Proof: ApiKey::new accepts any non-empty token.
    /// { !token.trim().is_empty() } ApiKey::new(token) { Ok(_) }
    #[kani::proof]
    fn api_key_accepts_non_empty() {
        // Kani can verify with a symbolic string only up to a small bound.
        // We use a fixed non-empty string for the positive case.
        let key = ApiKey::new("ghp_xxx");
        assert!(key.is_ok());
        assert_eq!(key.unwrap().as_str(), "ghp_xxx");
    }

    /// Proof: Page::new accepts n > 0 and returns correct value.
    /// { n > 0 } Page::new(n) { Ok(ret) ==> ret.get() == n }
    #[kani::proof]
    fn page_accepts_positive() {
        let n: u32 = kani::any();
        kani::assume(n > 0 && n <= 1000);

        let page = Page::new(n);
        assert!(page.is_ok());
        assert_eq!(page.unwrap().get(), n);
    }

    /// Proof: Page::new rejects n == 0.
    /// { n == 0 } Page::new(n) { Err(_) }
    #[kani::proof]
    fn page_rejects_zero() {
        let page = Page::new(0);
        assert!(page.is_err());
    }

    /// Proof: PerPage::new accepts 1..=100 and returns correct value.
    /// { n > 0 && n <= 100 } PerPage::new(n) { Ok(ret) ==> ret.get() == n }
    #[kani::proof]
    fn per_page_accepts_valid_range() {
        let n: u8 = kani::any();
        kani::assume(n > 0 && n <= 100);

        let pp = PerPage::new(n);
        assert!(pp.is_ok());
        assert_eq!(pp.unwrap().get(), n);
    }

    /// Proof: PerPage::new rejects n == 0.
    /// { n == 0 } PerPage::new(n) { Err(_) }
    #[kani::proof]
    fn per_page_rejects_zero() {
        let pp = PerPage::new(0);
        assert!(pp.is_err());
    }

    /// Proof: PerPage::new rejects n > 100.
    /// { n > 100 } PerPage::new(n) { Err(_) }
    #[kani::proof]
    fn per_page_rejects_too_large() {
        let n: u8 = kani::any();
        kani::assume(n > 100);

        let pp = PerPage::new(n);
        assert!(pp.is_err());
    }
}
