#![no_std]

//! # Turbo Core
//!
//! `turbo-core` contains the baseline abstractions, traits, custom allocator interfaces,
//! and macro utilities shared across the entire Turbo ecosystem.
//!
//! ## Design & Philosophy
//!
//! * **Zero-Overhead Abstractions**: Provide lightweight traits (like `ZeroCopy`, `EstimateSize`, `TurboAlloc`) that do not incur runtime performance penalties.
//! * **no_std Support**: Designed to be compiled without the standard library by default, ensuring compatibility with bare-metal, embedded, and WASM runtimes.
//! * **Non-Panicking APIs**: Promotes safety by avoiding `panic!`, `unwrap()`, or `expect()` in public paths, instead utilizing strongly-typed errors.

#[cfg(any(feature = "std", feature = "alloc"))]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

pub mod alloc;
pub mod error;
pub mod info;
pub mod macros;
pub mod result;
pub mod traits;

// Re-exports
pub use alloc::TurboAlloc;
pub use error::Error;
pub use result::Result;
pub use traits::{EstimateSize, ZeroCopy};
