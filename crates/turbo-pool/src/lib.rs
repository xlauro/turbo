#![no_std]

//! # Turbo Pool
//!
//! `turbo-pool` provides dynamically expanding homogenous slab allocators (`Slab`)
//! and thread-safe object pools (`ObjectPool`) to reuse allocations and minimize GC/allocation overhead.

#[cfg(any(feature = "std", feature = "alloc"))]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

pub mod object;
pub mod slab;

// Re-exports
pub use object::{DefaultRecycler, NoOpRecycler, ObjectPool, Pooled, Recycler};
pub use slab::Slab;
