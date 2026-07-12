#![no_std]

//! # Turbo Collections
//!
//! `turbo-collections` provides generational arenas (`Arena`) and dense, cache-friendly
//! generational maps (`DenseMap`) to build stable, index-based graphs and pointer-free data structures.

#[cfg(any(feature = "std", feature = "alloc"))]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

pub mod arena;
pub mod dense;
pub mod index;

// Re-exports
pub use arena::Arena;
pub use dense::DenseMap;
pub use index::Index;
