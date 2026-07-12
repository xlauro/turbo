use turbo_pool::{Slab, ObjectPool, Recycler};

// A custom recycler for Vec<String> that preserves memory allocation capacity.
struct StringVecRecycler;

impl Recycler<Vec<String>> for StringVecRecycler {
    fn recycle(&self, item: &mut Vec<String>) {
        item.clear(); // clears elements but keeps capacity
    }
}

fn main() -> turbo_core::Result<()> {
    // 1. Slab Allocator Showcase
    println!("--- 1. Slab Allocator ---");
    let mut slab = Slab::new();
    let id_a = slab.insert("Task Alpha");
    let id_b = slab.insert("Task Beta");

    println!("Slab len: {}, capacity: {}", slab.len(), slab.capacity());
    println!("Slot id_a: {}, value: {:?}", id_a, slab.get(id_a));
    println!("Slot id_b: {}, value: {:?}", id_b, slab.get(id_b));

    // Remove Task Alpha (recycles slot 0)
    slab.remove(id_a);
    println!("Slot id_a after remove: {:?}", slab.get(id_a));

    // Insert Task Gamma: slot 0 should be reused
    let id_c = slab.insert("Task Gamma");
    println!("Slot id_c (reused slot {}): {:?}", id_c, slab.get(id_c));

    // 2. Object Pool Showcase
    println!("\n--- 2. Object Pool ---");
    let pool = ObjectPool::new(|| Vec::with_capacity(32), StringVecRecycler);
    println!("Initial pool cache size: {}", pool.len());

    {
        // Checkout v1
        let mut v1 = pool.checkout();
        v1.push("First transaction".to_string());
        v1.push("Second transaction".to_string());
        println!("Checked out v1: len={}, capacity={}", v1.len(), v1.capacity());
        // v1 returned automatically here on drop
    }

    println!("Pool cache size after return: {}", pool.len());

    // Checkout again - we should reuse the same vector allocation (cleared but with capacity)
    let v2 = pool.checkout();
    println!("Checked out v2 (reused): len={}, capacity={}", v2.len(), v2.capacity());

    Ok(())
}
