#![no_std]

//! # Turbo Hash
//!
//! `turbo-hash` provides high-performance collections (`HashMap`, `HashSet`)
//! utilizing Robin Hood hashing and fast custom non-cryptographic hashers (`FxHasher`).

#[cfg(any(feature = "std", feature = "alloc"))]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

pub mod hasher;
pub mod map;
pub mod set;

// Re-exports
pub use hasher::{BuildFxHasher, FxHasher};
pub use map::{Entry, HashMap, OccupiedEntry, VacantEntry};
pub use set::HashSet;
