use turbo_string::{SmallString, StringView, StringBuilder, join, replace, trim, normalize};
use core::fmt::Write;

fn main() -> turbo_core::Result<()> {
    // 1. SmallString (SSO) Showcase
    println!("--- 1. SmallString (SSO) ---");
    let mut s1 = SmallString::from_str("hello")?;
    println!("String 1: '{}' (Inline state)", s1);
    
    s1.push_str(" world! This now extends to spill onto the heap.")?;
    println!("String 1 after push: '{}'", s1);
    println!("String 1 len: {}", s1.len());

    // 2. Zero-Copy StringView
    println!("\n--- 2. Zero-Copy StringView ---");
    let view = StringView::from_str("Rust is highly performant.")?;
    let sliced = view.slice(0..4)?;
    println!("Full view: '{}'", view);
    println!("Sliced view (0..4): '{}'", sliced);

    // 3. StringBuilder with Write macro
    println!("\n--- 3. StringBuilder formatting ---");
    let mut builder = StringBuilder::new();
    let framework = "Turbo";
    write!(builder, "Number: {}, Framework: {}", 100, framework)?;
    let built_str = builder.into_string()?;
    println!("Formatted String: '{}'", built_str);

    // 4. String Operations
    println!("\n--- 4. String Operations ---");
    let list = &["one", "two", "three"];
    let joined = join(list, " - ")?;
    println!("Joined: '{}'", joined);

    let replaced = replace("abababa", "b", "x")?;
    println!("Replaced: '{}'", replaced);

    let text = "   many    spaces   and\ttabs   ";
    let normalized = normalize(text)?;
    println!("Normalized: '{}'", normalized);

    let padded = "  padded text  ";
    println!("Trimmed: '{}'", trim(padded));

    Ok(())
}
