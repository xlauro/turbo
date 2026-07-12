/// A stable, generational index pointing to an element in a generational collection.
///
/// Combines a raw vector index and a generation number to prevent stale references
/// from accessing recycled slots (use-after-free safety).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Index {
    index: u32,
    generation: u32,
}

impl Index {
    /// Creates a new `Index` with the given index and generation.
    #[inline]
    pub const fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    /// Returns the raw slot index in the backing array.
    #[inline]
    pub const fn index(&self) -> u32 {
        self.index
    }

    /// Returns the generation count of the slot.
    #[inline]
    pub const fn generation(&self) -> u32 {
        self.generation
    }
}
