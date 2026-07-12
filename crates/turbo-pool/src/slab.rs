#[cfg(any(feature = "std", feature = "alloc"))]
use crate::alloc_crate::vec::Vec;

/// An entry in the [`Slab`].
#[derive(Clone, Debug)]
pub enum Entry<T> {
    /// A vacant slot in the slab, storing a link to the next free slot.
    Free {
        /// Index of the next free slot.
        next: Option<usize>,
    },
    /// An occupied slot storing the active value.
    Occupied(T),
}

/// A pre-allocated homogeneous slab allocator for low-latency element management.
///
/// Under the hood, memory capacity grows dynamically by doubling when filled.
/// Slots are recycled using an internal linked free-list.
#[derive(Clone, Debug)]
pub struct Slab<T> {
    entries: Vec<Entry<T>>,
    free_head: Option<usize>,
    len: usize,
}

impl<T> Slab<T> {
    /// Creates a new, empty `Slab`.
    ///
    /// Does not allocate heap memory.
    #[inline]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
            free_head: None,
            len: 0,
        }
    }

    /// Creates a new `Slab` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            free_head: None,
            len: 0,
        }
    }

    /// Returns the number of occupied slots in the slab.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the slab contains no occupied slots.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the capacity of the slab backing store.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.entries.capacity()
    }

    /// Inserts a value into the slab, returning its assigned slot index.
    pub fn insert(&mut self, value: T) -> usize {
        if let Some(idx) = self.free_head {
            let slot = &mut self.entries[idx];
            let next_free = match slot {
                Entry::Free { next } => *next,
                Entry::Occupied(..) => unreachable!("Occupied slot in free list"),
            };
            self.free_head = next_free;
            *slot = Entry::Occupied(value);
            self.len += 1;
            idx
        } else {
            let idx = self.entries.len();
            self.entries.push(Entry::Occupied(value));
            self.len += 1;
            idx
        }
    }

    /// Removes the value at the given index, returning it if present.
    ///
    /// The slot is recycled, making it available for subsequent insertions.
    pub fn remove(&mut self, key: usize) -> Option<T> {
        if key >= self.entries.len() {
            return None;
        }

        match &self.entries[key] {
            Entry::Occupied(..) => {
                let old_entry = core::mem::replace(
                    &mut self.entries[key],
                    Entry::Free {
                        next: self.free_head,
                    },
                );
                self.free_head = Some(key);
                self.len -= 1;
                match old_entry {
                    Entry::Occupied(value) => Some(value),
                    Entry::Free { .. } => unreachable!(),
                }
            }
            Entry::Free { .. } => None,
        }
    }

    /// Retrieves a shared reference to the value at the given slot index.
    pub fn get(&self, key: usize) -> Option<&T> {
        if key >= self.entries.len() {
            return None;
        }

        match &self.entries[key] {
            Entry::Occupied(value) => Some(value),
            Entry::Free { .. } => None,
        }
    }

    /// Retrieves a mutable reference to the value at the given slot index.
    pub fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        if key >= self.entries.len() {
            return None;
        }

        match &mut self.entries[key] {
            Entry::Occupied(value) => Some(value),
            Entry::Free { .. } => None,
        }
    }

    /// Clears the slab, removing all occupied values and resetting capacity.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.free_head = None;
        self.len = 0;
    }

    /// Returns an iterator yielding slot index and shared value references.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            iter: self.entries.iter().enumerate(),
        }
    }
}

impl<T> Default for Slab<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// An iterator over the occupied elements of a [`Slab`].
pub struct Iter<'a, T> {
    iter: core::iter::Enumerate<core::slice::Iter<'a, Entry<T>>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        for (idx, entry) in &mut self.iter {
            if let Entry::Occupied(val) = entry {
                return Some((idx, val));
            }
        }
        None
    }
}
