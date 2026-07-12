use core::fmt::Write;
use turbo_string::{join, normalize, replace, trim, SmallString, StringBuilder, StringView};

#[test]
fn test_small_string_sso() {
    // 1. Inline creation (<= 22 bytes)
    let s1 = SmallString::from_str("hello").unwrap();
    assert!(matches!(s1, SmallString::Inline { .. }));
    assert_eq!(s1.len(), 5);
    assert_eq!(s1.as_str(), "hello");

    // 2. Heap conversion (> 22 bytes)
    let s2 =
        SmallString::from_str("this is a very long string that will spill onto the heap").unwrap();
    assert!(matches!(s2, SmallString::Heap(..)));
    assert_eq!(s2.len(), 56);

    // 3. Dynamic transition (Inline -> Heap)
    let mut s3 = SmallString::from_str("inline").unwrap();
    assert!(matches!(s3, SmallString::Inline { .. }));
    s3.push_str(" spill over to heap allocation!").unwrap();
    assert!(matches!(s3, SmallString::Heap(..)));
    assert_eq!(s3.as_str(), "inline spill over to heap allocation!");

    // 4. Comparison and hashing
    let s4 = SmallString::from_str("hello").unwrap();
    assert_eq!(s1, s4);
    assert_eq!(s1, "hello");
}

#[test]
fn test_string_view_zero_copy() {
    let raw = "Rust is a systems programming language";
    let view = StringView::from_str(raw).unwrap();
    assert_eq!(view.len(), raw.len());
    assert_eq!(view.as_str(), raw);

    // Safe zero-copy slicing
    let sub = view.slice(10..38).unwrap();
    assert_eq!(sub.as_str(), "systems programming language");
    assert_eq!(sub.offset(), 10);

    // Slice range on char boundaries check (fails if middle of character)
    let emoji_str = "hello 🦀 world";
    let emoji_view = StringView::from_str(emoji_str).unwrap();
    // 🦀 starts at index 6 and takes 4 bytes. Slicing at index 7 should fail.
    assert!(emoji_view.slice(6..8).is_err());
    assert!(emoji_view.slice(6..10).is_ok());
}

#[test]
fn test_string_builder() {
    let mut builder = StringBuilder::new();
    builder.push_str("Turbo").unwrap();
    builder.push('-').unwrap();
    builder.push_str("String").unwrap();

    let s = builder.into_string().unwrap();
    assert_eq!(s.as_str(), "Turbo-String");

    // Test Write trait
    let mut format_builder = StringBuilder::new();
    let name = "Rust";
    write!(format_builder, "Value: {}, Name: {}", 42, name).unwrap();
    assert_eq!(format_builder.as_str(), "Value: 42, Name: Rust");
}

#[test]
fn test_string_ops() {
    // Trim
    assert_eq!(trim("  hello  "), "hello");

    // Join
    let joined = join(&["Rust", "C++", "Go"], " | ").unwrap();
    assert_eq!(joined.as_str(), "Rust | C++ | Go");

    // Replace
    let replaced = replace("banana", "a", "o").unwrap();
    assert_eq!(replaced.as_str(), "bonono");

    // Normalize
    let normalized = normalize("  hello   world  \t new  ").unwrap();
    assert_eq!(normalized.as_str(), "hello world new");
}
