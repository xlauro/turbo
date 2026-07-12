use core::alloc::Layout;
use turbo_core::alloc::{GlobalAlloc, TrackingAlloc};
use turbo_core::{turbo_bail, turbo_ensure};
use turbo_core::{Error, EstimateSize, Result, TurboAlloc, ZeroCopy};

#[test]
fn test_integration_errors_and_macros() {
    fn try_check(val: i32) -> Result<()> {
        turbo_ensure!(
            val > 0,
            Error::OutOfBounds {
                index: val as usize,
                len: 100
            }
        );
        if val == 42 {
            turbo_bail!(Error::Custom("special code 42"));
        }
        Ok(())
    }

    assert!(try_check(10).is_ok());

    let err1 = try_check(-5).unwrap_err();
    match err1 {
        Error::OutOfBounds { index, len } => {
            assert_eq!(index, usize::MAX - 4); // underflow cast representation
            assert_eq!(len, 100);
        }
        _ => panic!("Expected OutOfBounds error"),
    }

    let err2 = try_check(42).unwrap_err();
    assert_eq!(err2.to_string(), "Custom error: special code 42");
}

#[test]
fn test_integration_alloc_and_tracking() {
    let allocator = TrackingAlloc::new(GlobalAlloc);
    let layout = Layout::from_size_align(1024, 8).unwrap();

    assert_eq!(allocator.allocated_bytes(), 0);

    let block = allocator.alloc(layout).unwrap();
    assert_eq!(block.len(), 1024);
    assert_eq!(allocator.allocated_bytes(), 1024);

    unsafe {
        allocator.dealloc(
            core::ptr::NonNull::new(block.as_ptr() as *mut u8).unwrap(),
            layout,
        );
    }
    assert_eq!(allocator.allocated_bytes(), 0);
}

#[test]
fn test_integration_zero_copy_and_estimate_size() {
    #[derive(Copy, Clone, Debug)]
    #[repr(C)]
    struct MyPoint {
        x: i32,
        y: i32,
    }

    // Safety: MyPoint has repr(C) / stable layout, no pointers, and is Copy.
    // For the sake of the test, let's implement it.
    unsafe impl ZeroCopy for MyPoint {}

    let point = MyPoint { x: 10, y: 20 };
    assert_eq!(point.x, 10);
    assert_eq!(point.y, 20);
    assert_eq!(point.heap_size(), 0);
    assert_eq!(point.total_size(), std::mem::size_of::<MyPoint>());
}
