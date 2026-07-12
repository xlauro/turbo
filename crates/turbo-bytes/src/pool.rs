use crate::alloc_crate::vec::Vec;
use crate::buffer::ByteBuffer;
use parking_lot::Mutex;
use std::sync::Arc;
use turbo_core::alloc::GlobalAlloc;
use turbo_core::Result;

const BUCKET_SIZES: [usize; 3] = [4096, 16384, 65536];
const MAX_POOL_DEPTH: usize = 32; // Limit buffers cached per bucket to prevent memory bloat

/// A thread-safe buffer pool to cache and recycle [`ByteBuffer`] allocations.
///
/// Recycling buffers prevents frequent heap allocations and deallocations, significantly
/// reducing overhead in high-throughput workloads.
pub struct BufferPool {
    buckets: [Mutex<Vec<ByteBuffer<GlobalAlloc>>>; 3],
}

impl BufferPool {
    /// Creates a new `BufferPool` with empty buckets.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            buckets: [
                Mutex::new(Vec::new()),
                Mutex::new(Vec::new()),
                Mutex::new(Vec::new()),
            ],
        })
    }

    /// Acquires a byte buffer with at least the requested capacity.
    ///
    /// If a compatible buffer is available in the pool, it is returned.
    /// Otherwise, a new buffer is allocated.
    pub fn acquire(self: &Arc<Self>, capacity: usize) -> Result<PooledBuffer> {
        let bucket_idx = if capacity <= BUCKET_SIZES[0] {
            Some(0)
        } else if capacity <= BUCKET_SIZES[1] {
            Some(1)
        } else if capacity <= BUCKET_SIZES[2] {
            Some(2)
        } else {
            None // Too large for pooling, will bypass cache
        };

        if let Some(idx) = bucket_idx {
            let mut bucket = self.buckets[idx].lock();
            if let Some(mut buf) = bucket.pop() {
                buf.clear();
                return Ok(PooledBuffer {
                    buffer: Some(buf),
                    pool: Arc::clone(self),
                    bucket_index: Some(idx),
                });
            }
            // Allocate fresh buffer of bucket size
            let buf = ByteBuffer::with_capacity(BUCKET_SIZES[idx], GlobalAlloc)?;
            Ok(PooledBuffer {
                buffer: Some(buf),
                pool: Arc::clone(self),
                bucket_index: Some(idx),
            })
        } else {
            // Bypass pooling and allocate exactly requested capacity
            let buf = ByteBuffer::with_capacity(capacity, GlobalAlloc)?;
            Ok(PooledBuffer {
                buffer: Some(buf),
                pool: Arc::clone(self),
                bucket_index: None,
            })
        }
    }

    /// Returns a buffer to the specified bucket inside the pool.
    fn return_buffer(&self, idx: usize, buf: ByteBuffer<GlobalAlloc>) {
        let mut bucket = self.buckets[idx].lock();
        if bucket.len() < MAX_POOL_DEPTH {
            bucket.push(buf);
        }
        // If pool bucket is full, buf is dropped and deallocated automatically.
    }
}

/// A RAII guard wrapping a recycled [`ByteBuffer`].
///
/// When dropped, `PooledBuffer` automatically clears the buffer contents and returns it
/// back to the parent [`BufferPool`].
pub struct PooledBuffer {
    buffer: Option<ByteBuffer<GlobalAlloc>>,
    pool: Arc<BufferPool>,
    bucket_index: Option<usize>,
}

impl PooledBuffer {
    /// Consumes the wrapper and returns the inner buffer without recycling it.
    pub fn into_inner(mut self) -> ByteBuffer<GlobalAlloc> {
        self.buffer.take().unwrap()
    }
}

impl core::ops::Deref for PooledBuffer {
    type Target = ByteBuffer<GlobalAlloc>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.buffer.as_ref().unwrap()
    }
}

impl core::ops::DerefMut for PooledBuffer {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.buffer.as_mut().unwrap()
    }
}

impl Drop for PooledBuffer {
    fn drop(&mut self) {
        if let Some(mut buf) = self.buffer.take() {
            if let Some(idx) = self.bucket_index {
                buf.clear();
                self.pool.return_buffer(idx, buf);
            }
        }
    }
}
