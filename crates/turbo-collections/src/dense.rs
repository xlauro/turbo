use crate::index::Index;

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::alloc_crate::vec::Vec;

#[derive(Clone, Debug)]
enum SlotData {
    Free { next_free: Option<u32> },
    Occupied { dense_idx: u32 },
}

#[derive(Clone, Debug)]
struct DenseSlot {
    generation: u32,
    data: SlotData,
}

/// A generational collection providing contiguous memory iteration with swap-and-pop deletion.
///
/// Keeps two arrays: a dense contiguous array of values and a mapping array of slots.
/// Iteration operates directly on the dense array, ensuring maximum CPU cache efficiency.
#[derive(Clone, Debug)]
pub struct DenseMap<T> {
    slots: Vec<DenseSlot>,
    dense: Vec<T>,
    reverse: Vec<u32>, // reverse[dense_idx] -> slot index
    free_list_head: Option<u32>,
}

impl<T> DenseMap<T> {
    /// Creates a new, empty `DenseMap`.
    ///
    /// Does not allocate heap memory.
    #[inline]
    pub const fn new() -> Self {
        Self {
            slots: Vec::new(),
            dense: Vec::new(),
            reverse: Vec::new(),
            free_list_head: None,
        }
    }

    /// Creates a new `DenseMap` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            slots: Vec::with_capacity(capacity),
            dense: Vec::with_capacity(capacity),
            reverse: Vec::with_capacity(capacity),
            free_list_head: None,
        }
    }

    /// Returns the number of occupied elements in the collection.
    #[inline]
    pub fn len(&self) -> usize {
        self.dense.len()
    }

    /// Returns `true` if the collection contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.dense.is_empty()
    }

    /// Inserts a value, returning its stable [`Index`].
    pub fn insert(&mut self, value: T) -> Index {
        let dense_idx = self.dense.len() as u32;
        self.dense.push(value);

        if let Some(idx) = self.free_list_head {
            let slot = &mut self.slots[idx as usize];
            let next_free = match slot.data {
                SlotData::Free { next_free } => next_free,
                SlotData::Occupied { .. } => unreachable!("Occupied slot in free list"),
            };
            self.free_list_head = next_free;
            slot.data = SlotData::Occupied { dense_idx };
            self.reverse.push(idx);
            Index::new(idx, slot.generation)
        } else {
            let idx = self.slots.len() as u32;
            let slot = DenseSlot {
                generation: 1,
                data: SlotData::Occupied { dense_idx },
            };
            self.slots.push(slot);
            self.reverse.push(idx);
            Index::new(idx, 1)
        }
    }

    /// Removes the element corresponding to the index, returning it if present.
    ///
    /// Contiguity is maintained using swap-and-pop. Stale index lookups are prevented.
    pub fn remove(&mut self, index: Index) -> Option<T> {
        let idx = index.index() as usize;
        if idx >= self.slots.len() {
            return None;
        }

        let slot = &self.slots[idx];
        if slot.generation != index.generation() {
            return None;
        }

        let dense_idx = match slot.data {
            SlotData::Occupied { dense_idx } => dense_idx,
            SlotData::Free { .. } => return None,
        };

        // Recycle slot first
        let slot_mut = &mut self.slots[idx];
        slot_mut.generation = slot_mut.generation.wrapping_add(1).max(1);
        slot_mut.data = SlotData::Free {
            next_free: self.free_list_head,
        };
        self.free_list_head = Some(index.index());

        let last_idx = (self.dense.len() - 1) as u32;
        if dense_idx == last_idx {
            self.reverse.pop();
            self.dense.pop()
        } else {
            // Swap-and-pop implementation
            self.dense.swap(dense_idx as usize, last_idx as usize);
            self.reverse.swap(dense_idx as usize, last_idx as usize);
            self.reverse.pop();

            // Update the slot pointer of the swapped element
            let swapped_slot_idx = self.reverse[dense_idx as usize];
            self.slots[swapped_slot_idx as usize].data = SlotData::Occupied { dense_idx };

            self.dense.pop()
        }
    }

    /// Retrieves a shared reference to the element at the given index.
    pub fn get(&self, index: Index) -> Option<&T> {
        let idx = index.index() as usize;
        if idx >= self.slots.len() {
            return None;
        }

        let slot = &self.slots[idx];
        if slot.generation != index.generation() {
            return None;
        }

        match slot.data {
            SlotData::Occupied { dense_idx } => Some(&self.dense[dense_idx as usize]),
            SlotData::Free { .. } => None,
        }
    }

    /// Retrieves a mutable reference to the element at the given index.
    pub fn get_mut(&mut self, index: Index) -> Option<&mut T> {
        let idx = index.index() as usize;
        if idx >= self.slots.len() {
            return None;
        }

        let slot = &self.slots[idx];
        if slot.generation != index.generation() {
            return None;
        }

        match slot.data {
            SlotData::Occupied { dense_idx } => Some(&mut self.dense[dense_idx as usize]),
            SlotData::Free { .. } => None,
        }
    }

    /// Clears the collection, resetting length but preserving capacity.
    pub fn clear(&mut self) {
        self.slots.clear();
        self.dense.clear();
        self.reverse.clear();
        self.free_list_head = None;
    }

    /// Returns a slice of the contiguous dense values.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.dense
    }

    /// Returns a mutable slice of the contiguous dense values.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.dense
    }

    /// Returns an iterator over the contiguous dense values.
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.dense.iter()
    }

    /// Returns a mutable iterator over the contiguous dense values.
    #[inline]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.dense.iter_mut()
    }
}

impl<T> Default for DenseMap<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
