use turbo_collections::{Arena, DenseMap};

fn main() -> turbo_core::Result<()> {
    // 1. Generational Arena (SlotMap)
    println!("--- 1. Generational Arena ---");
    let mut arena = Arena::new();
    let index_a = arena.insert("Entity A");
    let index_b = arena.insert("Entity B");

    println!("Index A: {:?}", index_a);
    println!("Index B: {:?}", index_b);
    println!("Retrieving Index A: {:?}", arena.get(index_a));

    // Remove Entity A (recycles the slot)
    println!("\nRemoving Entity A...");
    arena.remove(index_a);
    println!("Retrieving Index A (stale): {:?}", arena.get(index_a));

    // Re-insert: slot is reused with incremented generation
    let index_c = arena.insert("Entity C");
    println!("Index C (reused slot 0): {:?}", index_c);
    println!("Retrieving Index C: {:?}", arena.get(index_c));
    println!("Retrieving Index A (stale check): {:?}", arena.get(index_a));

    // 2. DenseMap (Contiguous iteration)
    println!("\n--- 2. DenseMap (Swap-and-Pop) ---");
    let mut map = DenseMap::new();
    let index_1 = map.insert("Value 1");
    let index_2 = map.insert("Value 2");
    let index_3 = map.insert("Value 3");

    println!("Dense slice before remove: {:?}", map.as_slice());

    // Remove Value 2 (index_2)
    // Contiguity preserved by swapping Value 3 with Value 2, then popping
    println!("\nRemoving Value 2...");
    map.remove(index_2);
    println!("Dense slice after remove: {:?}", map.as_slice());

    // Iteration directly on the dense array
    println!("\nIterating over DenseMap contiguously:");
    for val in map.iter() {
        println!(" - {}", val);
    }

    // Verify lookup for Value 3 still works at its new swapped index
    println!("\nRetrieving Value 3: {:?}", map.get(index_3));
    println!("Retrieving Value 1: {:?}", map.get(index_1));

    Ok(())
}
