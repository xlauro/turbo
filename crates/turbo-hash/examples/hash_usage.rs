use turbo_hash::{HashMap, HashSet};

fn main() -> turbo_core::Result<()> {
    // 1. HashMap Basic operations
    println!("--- 1. HashMap Operations ---");
    let mut map = HashMap::new();
    map.insert("Apple", 3)?;
    map.insert("Banana", 5)?;
    map.insert("Orange", 2)?;

    println!("Banana count: {:?}", map.get(&"Banana"));
    println!("Grape count: {:?}", map.get(&"Grape"));

    // Mutable update
    if let Some(count) = map.get_mut(&"Orange") {
        *count += 10;
    }
    println!("Orange count after update: {:?}", map.get(&"Orange"));

    // 2. HashMap Entry API
    println!("\n--- 2. Entry API ---");
    // Insert if absent
    map.entry("Peach").or_insert(7);
    println!("Peach count: {:?}", map.get(&"Peach"));

    // Modify in place
    *map.entry("Apple").or_insert(0) += 20;
    println!("Apple count after entry update: {:?}", map.get(&"Apple"));

    // 3. HashSet Basic operations
    println!("\n--- 3. HashSet Operations ---");
    let mut set = HashSet::new();
    set.insert("Rust")?;
    set.insert("C++")?;
    set.insert("Zig")?;

    println!("Contains C++: {}", set.contains(&"C++"));
    println!("Contains Go: {}", set.contains(&"Go"));

    set.remove(&"Zig");
    println!("Set size after removing Zig: {}", set.len());

    Ok(())
}
