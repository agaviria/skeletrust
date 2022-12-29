//! Crate prelude

// Re-export Error from `crate::error`.
pub use crate::error::Error;

// Type alias for `Result`
pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple struct for newtype pattern.
// Used mostly for external type-to-type From/TryFrom conversions
//
// visit repo for more context on how this can be utilized:
//      https://github.com/jeremychone-channel/rust-base/blob/main/src/main.rs
//
// pub struct W<T>(pub T);

