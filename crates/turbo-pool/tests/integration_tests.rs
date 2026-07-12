use turbo_pool::{DefaultRecycler, ObjectPool, Recycler, Slab};

#[test]
fn test_slab_basic() {
    let mut slab = Slab::new();
    assert!(slab.is_empty());

    let k1 = slab.insert("apple");
    let k2 = slab.insert("banana");
    assert_eq!(slab.len(), 2);

    assert_eq!(slab.get(k1), Some(&"apple"));
    assert_eq!(slab.get(k2), Some(&"banana"));

    assert_eq!(slab.remove(k1), Some("apple"));
    assert_eq!(slab.len(), 1);
    assert_eq!(slab.get(k1), None);

    // Reuse slot
    let k3 = slab.insert("cherry");
    assert_eq!(k3, k1); // slot should be recycled
    assert_eq!(slab.get(k3), Some(&"cherry"));
}

struct VecRecycler;
impl Recycler<Vec<i32>> for VecRecycler {
    fn recycle(&self, item: &mut Vec<i32>) {
        item.clear(); // preserves capacity!
    }
}

#[test]
fn test_object_pool_basic() {
    let pool = ObjectPool::new(|| Vec::with_capacity(16), VecRecycler);
    assert_eq!(pool.len(), 0);

    {
        let mut v1 = pool.checkout();
        assert_eq!(v1.len(), 0);
        assert_eq!(v1.capacity(), 16);
        v1.push(10);
        v1.push(20);
        assert_eq!(v1.len(), 2);
        // v1 returned on drop
    }

    assert_eq!(pool.len(), 1);

    // Next checkout gets the same vector (already recycled/cleared)
    let v2 = pool.checkout();
    assert_eq!(v2.len(), 0);
    assert_eq!(v2.capacity(), 16);
}

#[cfg(feature = "std")]
#[test]
fn test_object_pool_concurrency() {
    use std::sync::Arc;
    use std::thread;

    let pool = Arc::new(ObjectPool::new(
        || String::with_capacity(32),
        DefaultRecycler,
    ));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let pool_clone = Arc::clone(&pool);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                let mut s = pool_clone.checkout();
                s.push_str("thread_test");
                assert_eq!(*s, "thread_test");
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    // Since threads release their checked-out strings, they should be in the pool
    assert!(pool.len() > 0);
}
