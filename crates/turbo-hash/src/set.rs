use crate::hasher::BuildFxHasher;
use crate::map::HashMap;
use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};
use turbo_core::Result;

/// A high-performance, cache-friendly Hash Set.
///
/// Implemented as a wrapper around [`HashMap`].
/// Generic over the hashing state, defaulting to [`BuildFxHasher`].
#[derive(Clone)]
pub struct HashSet<T, S = BuildFxHasher> {
    map: HashMap<T, (), S>,
}

impl<T> HashSet<T, BuildFxHasher> {
    /// Creates a new, empty `HashSet` with the default [`BuildFxHasher`].
    ///
    /// Does not allocate capacity on the heap.
    #[inline]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Creates a new `HashSet` with at least the specified capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
        }
    }
}

impl<T, S: BuildHasher> HashSet<T, S> {
    /// Creates an empty `HashSet` using the given build hasher.
    #[inline]
    pub fn with_hasher(hash_builder: S) -> Self {
        Self {
            map: HashMap::with_hasher(hash_builder),
        }
    }

    /// Creates an empty `HashSet` with the given capacity and build hasher.
    #[inline]
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self {
            map: HashMap::with_capacity_and_hasher(capacity, hash_builder),
        }
    }

    /// Returns the number of elements in the set.
    #[inline]
    pub const fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if the set contains no elements.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns the capacity of the set.
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.map.capacity()
    }

    /// Clears the set, removing all values.
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }
}

impl<T: Eq + Hash, S: BuildHasher + Clone> HashSet<T, S> {
    /// Adds a value to the set.
    ///
    /// If the set did not have this value present, `true` is returned.
    /// If the set did have this value present, `false` is returned.
    #[inline]
    pub fn insert(&mut self, value: T) -> Result<bool> {
        self.map.insert(value, ()).map(|opt| opt.is_none())
    }

    /// Removes a value from the set. Returns `true` if the value was present in the set.
    #[inline]
    pub fn remove<Q: ?Sized + Hash + Eq>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q>,
    {
        self.map.remove(value).is_some()
    }

    /// Returns `true` if the set contains a value.
    #[inline]
    pub fn contains<Q: ?Sized + Hash + Eq>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
    {
        self.map.get(value).is_some()
    }

    /// Returns an iterator over the set's values.
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            iter: self.map.iter(),
        }
    }
}

impl<T, S> Default for HashSet<T, S>
where
    S: BuildHasher + Default,
{
    #[inline]
    fn default() -> Self {
        Self::with_hasher(S::default())
    }
}

/// An iterator over the elements of a `HashSet`.
pub struct Iter<'a, T> {
    iter: crate::map::Iter<'a, T, ()>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }
}
