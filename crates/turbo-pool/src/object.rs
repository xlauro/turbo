use core::ops::{Deref, DerefMut};

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::alloc_crate::vec::Vec;

/// A recycler trait defining hooks to reset object state before reuse.
pub trait Recycler<T> {
    /// Resets the given item to its clean state, preserving pre-allocated capacity where possible.
    fn recycle(&self, item: &mut T);
}

/// A no-op recycler that leaves checked-in items unmodified.
///
/// Ideal for reusing raw capacity containers where elements are overwritten on checkout.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoOpRecycler;

impl<T> Recycler<T> for NoOpRecycler {
    #[inline]
    fn recycle(&self, _item: &mut T) {}
}

/// A recycler that resets elements to their default values.
#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultRecycler;

impl<T: Default> Recycler<T> for DefaultRecycler {
    #[inline]
    fn recycle(&self, item: &mut T) {
        *item = T::default();
    }
}

#[cfg(not(feature = "std"))]
struct SpinLock<T> {
    lock: core::sync::atomic::AtomicBool,
    data: core::cell::UnsafeCell<T>,
}

#[cfg(not(feature = "std"))]
unsafe impl<T: Send> Sync for SpinLock<T> {}
#[cfg(not(feature = "std"))]
unsafe impl<T: Send> Send for SpinLock<T> {}

#[cfg(not(feature = "std"))]
impl<T> SpinLock<T> {
    fn new(data: T) -> Self {
        Self {
            lock: core::sync::atomic::AtomicBool::new(false),
            data: core::cell::UnsafeCell::new(data),
        }
    }

    fn lock(&self) -> SpinLockGuard<'_, T> {
        while self.lock.swap(true, core::sync::atomic::Ordering::Acquire) {
            core::hint::spin_loop();
        }
        SpinLockGuard { parent: self }
    }
}

#[cfg(not(feature = "std"))]
struct SpinLockGuard<'a, T> {
    parent: &'a SpinLock<T>,
}

#[cfg(not(feature = "std"))]
impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.parent.data.get() }
    }
}

#[cfg(not(feature = "std"))]
impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.parent.data.get() }
    }
}

#[cfg(not(feature = "std"))]
impl<'a, T> Drop for SpinLockGuard<'a, T> {
    #[inline]
    fn drop(&mut self) {
        self.parent
            .lock
            .store(false, core::sync::atomic::Ordering::Release);
    }
}

/// A thread-safe concurrent object pool.
///
/// Houses recycled objects of type `T` to avoid allocation pressure.
/// Checking out an item returns a [`Pooled`] smart pointer that resets and checks
/// the item back into the pool automatically upon drop.
pub struct ObjectPool<T, R = NoOpRecycler> {
    #[cfg(feature = "std")]
    pool: parking_lot::Mutex<Vec<T>>,
    #[cfg(not(feature = "std"))]
    pool: SpinLock<Vec<T>>,
    recycler: R,
    factory: fn() -> T,
}

impl<T, R> ObjectPool<T, R> {
    /// Creates a new `ObjectPool` using the given factory constructor and recycler hook.
    pub fn new(factory: fn() -> T, recycler: R) -> Self {
        Self {
            #[cfg(feature = "std")]
            pool: parking_lot::Mutex::new(Vec::new()),
            #[cfg(not(feature = "std"))]
            pool: SpinLock::new(Vec::new()),
            recycler,
            factory,
        }
    }
}

impl<T, R: Recycler<T>> ObjectPool<T, R> {
    /// Checks out an object from the pool.
    ///
    /// If a recycled object is available, it is returned. Otherwise, a new object
    /// is constructed using the pool's factory function.
    pub fn checkout(&self) -> Pooled<'_, T, R> {
        let item = {
            #[cfg(feature = "std")]
            let mut guard = self.pool.lock();
            #[cfg(not(feature = "std"))]
            let mut guard = self.pool.lock();
            guard.pop()
        };

        let val = item.unwrap_or_else(self.factory);
        Pooled {
            pool: self,
            item: Some(val),
        }
    }

    /// Pushes a recycled item back onto the pool stack.
    #[inline]
    fn return_item(&self, item: T) {
        #[cfg(feature = "std")]
        let mut guard = self.pool.lock();
        #[cfg(not(feature = "std"))]
        let mut guard = self.pool.lock();
        guard.push(item);
    }

    /// Returns the number of pooled elements currently cached.
    pub fn len(&self) -> usize {
        #[cfg(feature = "std")]
        let guard = self.pool.lock();
        #[cfg(not(feature = "std"))]
        let guard = self.pool.lock();
        guard.len()
    }

    /// Returns `true` if there are no cached objects in the pool.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A smart pointer representing an object checked out from the [`ObjectPool`].
///
/// Implements [`Deref`] and [`DerefMut`] to expose the underlying item.
/// Releasing this pointer (via drop) automatically recycles the item and returns
/// it to the parent pool.
pub struct Pooled<'a, T, R: Recycler<T>> {
    pool: &'a ObjectPool<T, R>,
    item: Option<T>,
}

impl<'a, T, R: Recycler<T>> Deref for Pooled<'a, T, R> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.item.as_ref().unwrap()
    }
}

impl<'a, T, R: Recycler<T>> DerefMut for Pooled<'a, T, R> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.item.as_mut().unwrap()
    }
}

impl<'a, T, R: Recycler<T>> Drop for Pooled<'a, T, R> {
    fn drop(&mut self) {
        if let Some(mut item) = self.item.take() {
            self.pool.recycler.recycle(&mut item);
            self.pool.return_item(item);
        }
    }
}
