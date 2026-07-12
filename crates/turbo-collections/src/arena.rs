use crate::index::Index;

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::alloc_crate::vec::Vec;

#[derive(Clone, Debug)]
enum SlotData<T> {
    Free { next_free: Option<u32> },
    Occupied(T),
}

#[derive(Clone, Debug)]
struct Slot<T> {
    generation: u32,
    data: SlotData<T>,
}

/// A generational arena (`SlotMap`) providing safe, stable pointer-free indexing.
///
/// Under the hood, memory is allocated contiguously and recycled using an internal free list.
/// Indices carry a generation number, preventing access to stale data.
#[derive(Clone, Debug)]
pub struct Arena<T> {
    slots: Vec<Slot<T>>,
    free_list_head: Option<u32>,
    len: usize,
}

impl<T> Arena<T> {
    /// Creates a new, empty `Arena`.
    ///
    /// Does not allocate heap memory.
    #[inline]
    pub const fn new() -> Self {
        Self {
            slots: Vec::new(),
            free_list_head: None,
            len: 0,
        }
    }

    /// Creates a new `Arena` with at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            slots: Vec::with_capacity(capacity),
            free_list_head: None,
            len: 0,
        }
    }

    /// Returns the number of active occupied elements inside the arena.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the arena contains no active elements.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the capacity (number of slots allocated) of the arena.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.slots.capacity()
    }

    /// Inserts an element into the arena, returning its stable [`Index`].
    pub fn insert(&mut self, value: T) -> Index {
        if let Some(idx) = self.free_list_head {
            let slot = &mut self.slots[idx as usize];
            let next_free = match slot.data {
                SlotData::Free { next_free } => next_free,
                SlotData::Occupied(..) => unreachable!("Occupied slot in free list"),
            };
            self.free_list_head = next_free;
            slot.data = SlotData::Occupied(value);
            self.len += 1;
            Index::new(idx, slot.generation)
        } else {
            let idx = self.slots.len() as u32;
            let slot = Slot {
                generation: 1,
                data: SlotData::Occupied(value),
            };
            self.slots.push(slot);
            self.len += 1;
            Index::new(idx, 1)
        }
    }

    /// Removes the element corresponding to the index, returning it if present.
    ///
    /// The slot is recycled, and its generation is incremented to invalidate stale indices.
    pub fn remove(&mut self, index: Index) -> Option<T> {
        let idx = index.index() as usize;
        if idx >= self.slots.len() {
            return None;
        }

        let slot = &mut self.slots[idx];
        if slot.generation != index.generation() {
            return None;
        }

        match core::mem::replace(
            &mut slot.data,
            SlotData::Free {
                next_free: self.free_list_head,
            },
        ) {
            SlotData::Occupied(value) => {
                // Increment generation to invalidate all existing indices pointing to this slot
                slot.generation = slot.generation.wrapping_add(1).max(1);
                self.free_list_head = Some(index.index());
                self.len -= 1;
                Some(value)
            }
            SlotData::Free { next_free } => {
                // Restore slot state if it was already free (should be unreachable due to generation check)
                slot.data = SlotData::Free { next_free };
                None
            }
        }
    }

    /// Retrieves a shared reference to the element at the given index.
    ///
    /// Returns `None` if the element was removed or if the generation does not match.
    pub fn get(&self, index: Index) -> Option<&T> {
        let idx = index.index() as usize;
        if idx >= self.slots.len() {
            return None;
        }

        let slot = &self.slots[idx];
        if slot.generation != index.generation() {
            return None;
        }

        match &slot.data {
            SlotData::Occupied(val) => Some(val),
            SlotData::Free { .. } => None,
        }
    }

    /// Retrieves a mutable reference to the element at the given index.
    ///
    /// Returns `None` if the element was removed or if the generation does not match.
    pub fn get_mut(&mut self, index: Index) -> Option<&mut T> {
        let idx = index.index() as usize;
        if idx >= self.slots.len() {
            return None;
        }

        let slot = &mut self.slots[idx];
        if slot.generation != index.generation() {
            return None;
        }

        match &mut slot.data {
            SlotData::Occupied(val) => Some(val),
            SlotData::Free { .. } => None,
        }
    }

    /// Clears the arena, removing all elements and resetting the length.
    ///
    /// Retains the allocated capacity, but resets all slot generations.
    pub fn clear(&mut self) {
        self.slots.clear();
        self.free_list_head = None;
        self.len = 0;
    }

    /// Returns an iterator yielding stable index and shared reference pairs.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            slots: self.slots.iter().enumerate(),
        }
    }
}

impl<T> Default for Arena<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// An iterator over the elements of an [`Arena`].
pub struct Iter<'a, T> {
    slots: core::iter::Enumerate<core::slice::Iter<'a, Slot<T>>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (Index, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        for (idx, slot) in &mut self.slots {
            if let SlotData::Occupied(val) = &slot.data {
                return Some((Index::new(idx as u32, slot.generation), val));
            }
        }
        None
    }
}
