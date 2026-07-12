#![no_std]

//! # Turbo String
//!
//! `turbo-string` provides memory-efficient string storage (SSO), zero-copy
//! string slicing, formatting builders, and optimized string manipulation helpers.

#[cfg(any(feature = "std", feature = "alloc"))]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

pub mod builder;
pub mod ops;
pub mod small;
pub mod view;

// Re-exports
pub use builder::StringBuilder;
pub use ops::{join, normalize, replace, trim};
pub use small::SmallString;
pub use view::StringView;
