use core::alloc::Layout;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use turbo_core::alloc::{GlobalAlloc, TrackingAlloc};
use turbo_core::error::Error;
use turbo_core::TurboAlloc;

fn bench_allocator(c: &mut Criterion) {
    let global_alloc = GlobalAlloc;
    let tracking_alloc = TrackingAlloc::new(GlobalAlloc);
    let layout = Layout::from_size_align(1024, 8).unwrap();

    c.bench_function("global_alloc_alloc_dealloc", |b| {
        b.iter(|| {
            let ptr = global_alloc.alloc(black_box(layout)).unwrap();
            unsafe {
                global_alloc.dealloc(
                    core::ptr::NonNull::new(ptr.as_ptr() as *mut u8).unwrap(),
                    layout,
                );
            }
        })
    });

    c.bench_function("tracking_alloc_alloc_dealloc", |b| {
        b.iter(|| {
            let ptr = tracking_alloc.alloc(black_box(layout)).unwrap();
            unsafe {
                tracking_alloc.dealloc(
                    core::ptr::NonNull::new(ptr.as_ptr() as *mut u8).unwrap(),
                    layout,
                );
            }
        })
    });
}

fn bench_error_formatting(c: &mut Criterion) {
    c.bench_function("error_to_string_capacity_overflow", |b| {
        b.iter(|| {
            let err = Error::CapacityOverflow {
                limit: black_box(1024),
                requested: black_box(2048),
            };
            let _s = black_box(err.to_string());
        })
    });
}

criterion_group!(benches, bench_allocator, bench_error_formatting);
criterion_main!(benches);
