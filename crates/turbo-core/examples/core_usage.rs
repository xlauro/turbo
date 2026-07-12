use turbo_core::alloc::{GlobalAlloc, TrackingAlloc};
use turbo_core::{Error, Result, TurboAlloc, EstimateSize, ZeroCopy};
use turbo_core::{static_assert, turbo_ensure};
use core::alloc::Layout;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct DataPacket {
    id: u64,
    val: f64,
}

// Safety: DataPacket has repr(C) / stable layout, no references/pointers, and is Copy.
unsafe impl ZeroCopy for DataPacket {}

fn main() -> Result<()> {
    // 1. Static compile-time checks
    static_assert!(core::mem::size_of::<DataPacket>() == 16);
    println!("Static checks passed: DataPacket size is 16 bytes.");

    // 2. Tracking allocations
    let tracking_alloc = TrackingAlloc::new(GlobalAlloc);
    let layout = Layout::new::<DataPacket>();

    println!("Initial allocated bytes: {}", tracking_alloc.allocated_bytes());

    let ptr = tracking_alloc.alloc(layout)?;
    println!("Allocated 1 packet. Current allocated bytes: {}", tracking_alloc.allocated_bytes());

    unsafe {
        let packet_ref = &mut *(ptr.as_ptr() as *mut DataPacket);
        packet_ref.id = 1;
        packet_ref.val = 3.14159;

        println!("Packet contents: id={}, val={}", packet_ref.id, packet_ref.val);
        
        tracking_alloc.dealloc(core::ptr::NonNull::new(ptr.as_ptr() as *mut u8).unwrap(), layout);
    }
    println!("Deallocated. Current allocated bytes: {}", tracking_alloc.allocated_bytes());

    // 3. Size estimation
    let packet = DataPacket { id: 42, val: 2.71828 };
    println!("Packet stack size: {}", core::mem::size_of_val(&packet));
    println!("Packet heap size (via EstimateSize): {}", packet.heap_size());
    println!("Packet total size: {}", packet.total_size());

    // 4. Non-panicking API validation
    let validation_result = check_packet_id(packet.id);
    match validation_result {
        Ok(()) => println!("Packet ID check passed!"),
        Err(err) => println!("Check failed: {}", err),
    }

    Ok(())
}

fn check_packet_id(id: u64) -> Result<()> {
    // Return Error if packet ID is 0 (invalid)
    turbo_ensure!(id > 0, Error::InvalidData { reason: "Packet ID must be non-zero" });
    Ok(())
}
