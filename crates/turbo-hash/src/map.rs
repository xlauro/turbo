use core::hash::{Hash, Hasher, BuildHasher};
use core::borrow::Borrow;
use turbo_core::Result;
use crate::hasher::BuildFxHasher;

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::alloc_crate::vec::Vec;

#[derive(Clone)]
struct TableEntry<K, V> {
    hash: u64,
    key: K,
    value: V,
    dis: usize,
}

/// A bucket in the Robin Hood Hash Map.
#[derive(Clone)]
pub struct Bucket<K, V> {
    entry: Option<TableEntry<K, V>>,
}

/// A high-performance, cache-friendly Hash Map utilizing Robin Hood hashing.
///
/// Implements backward-shift deletion to preserve short probe distances.
/// Generic over the hashing state, defaulting to [`BuildFxHasher`].
#[derive(Clone)]
pub struct HashMap<K, V, S = BuildFxHasher> {
    buckets: Vec<Bucket<K, V>>,
    len: usize,
    capacity: usize,
    build_hasher: S,
}

impl<K, V> HashMap<K, V, BuildFxHasher> {
    /// Creates a new, empty `HashMap` with the default [`BuildFxHasher`].
    ///
    /// Does not allocate capacity on the heap.
    #[inline]
    pub fn new() -> Self {
        Self::with_hasher(BuildFxHasher)
    }

    /// Creates a new `HashMap` with at least the specified capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, BuildFxHasher)
    }
}

impl<K, V, S: BuildHasher> HashMap<K, V, S> {
    /// Creates an empty `HashMap` using the given build hasher.
    #[inline]
    pub fn with_hasher(hash_builder: S) -> Self {
        Self {
            buckets: Vec::new(),
            len: 0,
            capacity: 0,
            build_hasher: hash_builder,
        }
    }

    /// Creates an empty `HashMap` with the given capacity and build hasher.
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        let capacity = capacity.checked_next_power_of_two().unwrap_or(8).max(8);
        let mut buckets = Vec::with_capacity(capacity);
        buckets.resize_with(capacity, || Bucket { entry: None });
        Self {
            buckets,
            len: 0,
            capacity,
            build_hasher: hash_builder,
        }
    }

    /// Returns the number of elements in the map.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the map contains no elements.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the capacity of the map.
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Clears the map, removing all key-value pairs.
    pub fn clear(&mut self) {
        self.buckets.clear();
        self.buckets.resize_with(self.capacity, || Bucket { entry: None });
        self.len = 0;
    }

    /// Hash helper function.
    #[inline]
    #[allow(clippy::manual_hash_one)]
    fn hash_key<Q: ?Sized + Hash>(&self, key: &Q) -> u64 {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
    }
}

impl<K: Eq + Hash, V, S: BuildHasher + Clone> HashMap<K, V, S> {
    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    /// If the map did have this key present, the value is updated, and the old value is returned.
    pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>> {
        if self.capacity == 0 {
            *self = Self::with_capacity_and_hasher(8, self.build_hasher.clone());
        }

        // Trigger resize if load factor exceeds 75%
        if self.len * 4 >= self.capacity * 3 {
            self.grow()?;
        }

        let hash = self.hash_key(&key);
        let mut entry = TableEntry {
            hash,
            key,
            value,
            dis: 0,
        };

        let mut idx = (hash as usize) & (self.capacity - 1);
        loop {
            match &mut self.buckets[idx].entry {
                None => {
                    self.buckets[idx].entry = Some(entry);
                    self.len += 1;
                    return Ok(None);
                }
                Some(existing) => {
                    if existing.hash == entry.hash && existing.key == entry.key {
                        let old_val = core::mem::replace(&mut existing.value, entry.value);
                        return Ok(Some(old_val));
                    }

                    // Robin Hood swap: if existing has a smaller distance, swap and continue probing
                    if existing.dis < entry.dis {
                        core::mem::swap(existing, &mut entry);
                    }

                    entry.dis += 1;
                    idx = (idx + 1) & (self.capacity - 1);
                }
            }
        }
    }

    /// Grows the table capacity, doubling it, and re-hashes all elements.
    fn grow(&mut self) -> Result<()> {
        let new_capacity = self.capacity.checked_mul(2).unwrap_or(self.capacity);
        let mut new_map = Self::with_capacity_and_hasher(new_capacity, self.build_hasher.clone());

        for bucket in self.buckets.drain(..) {
            if let Some(entry) = bucket.entry {
                new_map.insert(entry.key, entry.value)?;
            }
        }

        *self = new_map;
        Ok(())
    }

    /// Retrieves a reference to the value corresponding to the key.
    pub fn get<Q: ?Sized + Hash + Eq>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        if self.capacity == 0 {
            return None;
        }

        let hash = self.hash_key(key);
        let mut idx = (hash as usize) & (self.capacity - 1);
        let mut dis = 0;

        loop {
            match &self.buckets[idx].entry {
                None => return None,
                Some(entry) => {
                    // If we've searched further than the entry's distance, it cannot exist
                    if dis > entry.dis {
                        return None;
                    }
                    if entry.hash == hash && entry.key.borrow() == key {
                        return Some(&entry.value);
                    }
                    dis += 1;
                    idx = (idx + 1) & (self.capacity - 1);
                }
            }
        }
    }

    /// Retrieves a mutable reference to the value corresponding to the key.
    pub fn get_mut<Q: ?Sized + Hash + Eq>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
    {
        if self.capacity == 0 {
            return None;
        }

        let hash = self.hash_key(key);
        let mut idx = (hash as usize) & (self.capacity - 1);
        let mut dis = 0;

        let found_idx = loop {
            match &self.buckets[idx].entry {
                None => return None,
                Some(entry) => {
                    if dis > entry.dis {
                        return None;
                    }
                    if entry.hash == hash && entry.key.borrow() == key {
                        break idx;
                    }
                    dis += 1;
                    idx = (idx + 1) & (self.capacity - 1);
                }
            }
        };

        Some(&mut self.buckets[found_idx].entry.as_mut().unwrap().value)
    }

    /// Removes a key from the map, returning the value if the key was previously in the map.
    ///
    /// Preserves ideal slot clustering using backward-shift deletion.
    pub fn remove<Q: ?Sized + Hash + Eq>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
    {
        if self.capacity == 0 {
            return None;
        }

        let hash = self.hash_key(key);
        let mut idx = (hash as usize) & (self.capacity - 1);
        let mut dis = 0;

        let found_idx = loop {
            match &self.buckets[idx].entry {
                None => return None,
                Some(entry) => {
                    if dis > entry.dis {
                        return None;
                    }
                    if entry.hash == hash && entry.key.borrow() == key {
                        break idx;
                    }
                    dis += 1;
                    idx = (idx + 1) & (self.capacity - 1);
                }
            }
        };

        // Remove the entry
        let removed_entry = self.buckets[found_idx].entry.take().unwrap();
        self.len -= 1;

        // Backward-shift deletion to fill the gap
        let mut curr = found_idx;
        loop {
            let next = (curr + 1) & (self.capacity - 1);
            match &mut self.buckets[next].entry {
                None => break,
                Some(entry) => {
                    if entry.dis == 0 {
                        break; // Already in its ideal position
                    }
                    // Shift entry backward
                    entry.dis -= 1;
                    self.buckets[curr].entry = self.buckets[next].entry.take();
                    curr = next;
                }
            }
        }

        Some(removed_entry.value)
    }

    /// Returns an iterator over the map's key-value pairs.
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            iter: self.buckets.iter(),
        }
    }

    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V, S> {
        if self.capacity == 0 {
            // Allocate initial capacity
            *self = Self::with_capacity_and_hasher(8, self.build_hasher.clone());
        }

        let hash = self.hash_key(&key);
        let mut idx = (hash as usize) & (self.capacity - 1);
        let mut dis = 0;

        loop {
            match &self.buckets[idx].entry {
                None => {
                    return Entry::Vacant(VacantEntry {
                        map: self,
                        key,
                        hash,
                        idx,
                        dis,
                    });
                }
                Some(entry) => {
                    if dis > entry.dis {
                        return Entry::Vacant(VacantEntry {
                            map: self,
                            key,
                            hash,
                            idx,
                            dis,
                        });
                    }
                    if entry.hash == hash && entry.key == key {
                        return Entry::Occupied(OccupiedEntry {
                            map: self,
                            idx,
                        });
                    }
                    dis += 1;
                    idx = (idx + 1) & (self.capacity - 1);
                }
            }
        }
    }
}

impl<K, V, S> Default for HashMap<K, V, S>
where
    S: BuildHasher + Default,
{
    #[inline]
    fn default() -> Self {
        Self::with_hasher(S::default())
    }
}

/// An iterator over the entries of a `HashMap`.
pub struct Iter<'a, K, V> {
    iter: core::slice::Iter<'a, Bucket<K, V>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        for bucket in &mut self.iter {
            if let Some(entry) = &bucket.entry {
                return Some((&entry.key, &entry.value));
            }
        }
        None
    }
}

/// A view into a single entry in a map, which may either be vacant or occupied.
pub enum Entry<'a, K: Eq + Hash, V, S: BuildHasher + Clone> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, K, V, S>),
    /// A vacant entry.
    Vacant(VacantEntry<'a, K, V, S>),
}

impl<'a, K: Eq + Hash, V, S: BuildHasher + Clone> Entry<'a, K, V, S> {
    /// Ensures a value is in the entry by inserting the default if empty.
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => entry.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the function if empty.
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => entry.insert(default()),
        }
    }
}

/// A view into an occupied entry in a `HashMap`.
pub struct OccupiedEntry<'a, K: Eq + Hash, V, S: BuildHasher + Clone> {
    map: &'a mut HashMap<K, V, S>,
    idx: usize,
}

impl<'a, K: Eq + Hash, V, S: BuildHasher + Clone> OccupiedEntry<'a, K, V, S> {
    /// Gets a mutable reference to the value in the entry.
    #[inline]
    pub fn into_mut(self) -> &'a mut V {
        &mut self.map.buckets[self.idx].entry.as_mut().unwrap().value
    }
}

/// A view into a vacant entry in a `HashMap`.
pub struct VacantEntry<'a, K: Eq + Hash, V, S: BuildHasher + Clone> {
    map: &'a mut HashMap<K, V, S>,
    key: K,
    hash: u64,
    idx: usize,
    dis: usize,
}

impl<'a, K: Eq + Hash, V, S: BuildHasher + Clone> VacantEntry<'a, K, V, S> {
    /// Inserts the value into the vacant entry.
    pub fn insert(self, value: V) -> &'a mut V {
        // Trigger resize if load factor exceeds 75%
        if self.map.len * 4 >= self.map.capacity * 3 {
            // If we grow, indices change, so we delegate to general insert
            let hash = self.hash;
            let key = self.key;
            let _ = self.map.insert(key, value);
            // Look up again to get a mutable reference (split borrow out of loop)
            let mut idx = (hash as usize) & (self.map.capacity - 1);
            let found_idx = loop {
                let entry = self.map.buckets[idx].entry.as_ref().unwrap();
                if entry.hash == hash {
                    break idx;
                }
                idx = (idx + 1) & (self.map.capacity - 1);
            };
            return &mut self.map.buckets[found_idx].entry.as_mut().unwrap().value;
        }

        let mut entry = TableEntry {
            hash: self.hash,
            key: self.key,
            value,
            dis: self.dis,
        };

        let mut idx = self.idx;
        let mut target_idx = idx;
        let mut first = true;

        loop {
            match &mut self.map.buckets[idx].entry {
                None => {
                    self.map.buckets[idx].entry = Some(entry);
                    self.map.len += 1;
                    if first {
                        target_idx = idx;
                    }
                    break;
                }
                Some(existing) => {
                    if existing.dis < entry.dis {
                        if first {
                            target_idx = idx;
                            first = false;
                        }
                        core::mem::swap(existing, &mut entry);
                    }
                    entry.dis += 1;
                    idx = (idx + 1) & (self.map.capacity - 1);
                }
            }
        }

        &mut self.map.buckets[target_idx].entry.as_mut().unwrap().value
    }
}
