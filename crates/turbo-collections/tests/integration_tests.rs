use turbo_collections::{Arena, DenseMap, Index};

#[test]
fn test_arena_generational_checks() {
    let mut arena = Arena::new();
    assert!(arena.is_empty());

    let idx1 = arena.insert("Value A");
    let idx2 = arena.insert("Value B");
    assert_eq!(arena.len(), 2);

    assert_eq!(arena.get(idx1), Some(&"Value A"));
    assert_eq!(arena.get(idx2), Some(&"Value B"));

    // Remove first item
    assert_eq!(arena.remove(idx1), Some("Value A"));
    assert_eq!(arena.len(), 1);

    // Stale index lookup should fail
    assert_eq!(arena.get(idx1), None);

    // Re-insert: slot should be recycled
    let idx3 = arena.insert("Value C");
    // Raw index should be same (0), but generation should be different (2)
    assert_eq!(idx3.index(), idx1.index());
    assert_ne!(idx3.generation(), idx1.generation());

    // Stale lookup on idx1 still fails, but idx3 succeeds
    assert_eq!(arena.get(idx1), None);
    assert_eq!(arena.get(idx3), Some(&"Value C"));
}

#[test]
fn test_arena_iteration() {
    let mut arena = Arena::new();
    let idx1 = arena.insert(10);
    let idx2 = arena.insert(20);
    arena.remove(idx1);
    let idx3 = arena.insert(30);

    let items: Vec<(Index, i32)> = arena.iter().map(|(idx, &val)| (idx, val)).collect();
    assert_eq!(items.len(), 2);
    // idx2 and idx3 remain
    assert!(items.contains(&(idx2, 20)));
    assert!(items.contains(&(idx3, 30)));
}

#[test]
fn test_dense_map_contiguity_and_swap_pop() {
    let mut map = DenseMap::new();
    let idx1 = map.insert("A");
    let idx2 = map.insert("B");
    let idx3 = map.insert("C");

    assert_eq!(map.len(), 3);
    // Values should be stored contiguously in insertion order
    assert_eq!(map.as_slice(), &["A", "B", "C"]);
    assert_eq!(map.get(idx1), Some(&"A"));

    // Remove middle element "B" (index 1)
    // Swap-and-pop: "C" (last element) swaps into "B"'s slot, and then we pop
    assert_eq!(map.remove(idx2), Some("B"));
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &["A", "C"]);

    // Verify lookup for "C" (idx3) still works, but "B" (idx2) fails
    assert_eq!(map.get(idx3), Some(&"C"));
    assert_eq!(map.get(idx2), None);

    // Mutate in place
    if let Some(val) = map.get_mut(idx3) {
        *val = "New C";
    }
    assert_eq!(map.get(idx3), Some(&"New C"));
    assert_eq!(map.as_slice(), &["A", "New C"]);
}
