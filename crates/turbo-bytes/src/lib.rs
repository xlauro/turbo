#![no_std]

//! # Turbo Bytes
//!
//! `turbo-bytes` provides high-performance memory buffers, zero-copy slice views,
//! cursor streams, and safe binary reading/writing primitives.

#[cfg(any(feature = "std", feature = "alloc"))]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

pub mod buffer;
pub mod cursor;
pub mod pool;
pub mod reader;
pub mod view;
pub mod writer;

// Re-exports
pub use buffer::ByteBuffer;
pub use cursor::Cursor;
pub use pool::{BufferPool, PooledBuffer};
pub use reader::ByteReader;
pub use view::ByteView;
pub use writer::ByteWriter;
