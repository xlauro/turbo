use turbo_hash::{HashMap, HashSet};

#[test]
fn test_hash_map_basic() {
    let mut map = HashMap::new();
    assert_eq!(map.len(), 0);
    assert!(map.is_empty());

    // Insertion
    assert_eq!(map.insert("one", 1).unwrap(), None);
    assert_eq!(map.insert("two", 2).unwrap(), None);
    assert_eq!(map.len(), 2);

    // Retrieval
    assert_eq!(map.get(&"one"), Some(&1));
    assert_eq!(map.get(&"two"), Some(&2));
    assert_eq!(map.get(&"three"), None);

    // Update
    assert_eq!(map.insert("one", 10).unwrap(), Some(1));
    assert_eq!(map.get(&"one"), Some(&10));

    // Mutable retrieval
    if let Some(val) = map.get_mut(&"two") {
        *val = 20;
    }
    assert_eq!(map.get(&"two"), Some(&20));

    // Removal
    assert_eq!(map.remove(&"one"), Some(10));
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&"one"), None);
}

#[test]
fn test_hash_map_collisions_and_backshift() {
    let mut map = HashMap::with_capacity(8);
    // Keys that collide under a simple modulo or build hashes
    // We insert multiple values to trigger collisions
    for i in 0..10 {
        map.insert(i, i * 100).unwrap();
    }
    assert_eq!(map.len(), 10);

    // Retrieve all
    for i in 0..10 {
        assert_eq!(map.get(&i), Some(&(i * 100)));
    }

    // Delete elements to verify backshift deletion doesn't break probe paths
    assert_eq!(map.remove(&4), Some(400));
    assert_eq!(map.remove(&7), Some(700));

    // Re-verify remaining elements
    for &i in &[0, 1, 2, 3, 5, 6, 8, 9] {
        assert_eq!(map.get(&i), Some(&(i * 100)));
    }
}

#[test]
fn test_hash_map_entry_api() {
    let mut map = HashMap::new();

    // Vacant entry or_insert
    let val_ref = map.entry("key1").or_insert(100);
    assert_eq!(*val_ref, 100);
    assert_eq!(map.get(&"key1"), Some(&100));

    // Occupied entry or_insert (does not overwrite)
    let val_ref = map.entry("key1").or_insert(200);
    assert_eq!(*val_ref, 100);

    // or_insert_with
    let val_ref = map.entry("key2").or_insert_with(|| 300);
    assert_eq!(*val_ref, 300);
    assert_eq!(map.get(&"key2"), Some(&300));
}

#[test]
fn test_hash_set_basic() {
    let mut set = HashSet::new();
    assert_eq!(set.len(), 0);

    assert!(set.insert("apple").unwrap());
    assert!(set.insert("banana").unwrap());
    // Duplicate insert should return false
    assert!(!set.insert("apple").unwrap());

    assert!(set.contains(&"apple"));
    assert!(set.contains(&"banana"));
    assert!(!set.contains(&"cherry"));

    assert!(set.remove(&"apple"));
    assert!(!set.contains(&"apple"));
    assert_eq!(set.len(), 1);
}
