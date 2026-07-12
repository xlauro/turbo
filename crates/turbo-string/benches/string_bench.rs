use core::fmt::Write;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use turbo_string::{replace, SmallString, StringBuilder};

fn bench_small_string_creation(c: &mut Criterion) {
    c.bench_function("std_string_new_short", |b| {
        b.iter(|| {
            let s = String::from(black_box("hello"));
            black_box(s);
        })
    });

    c.bench_function("small_string_new_short", |b| {
        b.iter(|| {
            let s = SmallString::from_str(black_box("hello")).unwrap();
            black_box(s);
        })
    });
}

fn bench_string_builder_vs_format(c: &mut Criterion) {
    c.bench_function("std_format_macro", |b| {
        b.iter(|| {
            let s = format!("Value: {}, Name: {}", black_box(42), black_box("Rust"));
            black_box(s);
        })
    });

    c.bench_function("string_builder_format", |b| {
        b.iter(|| {
            let mut builder = StringBuilder::new();
            let _ = write!(
                builder,
                "Value: {}, Name: {}",
                black_box(42),
                black_box("Rust")
            );
            let s = builder.into_string().unwrap();
            black_box(s);
        })
    });
}

fn bench_string_replace(c: &mut Criterion) {
    c.bench_function("std_string_replace", |b| {
        b.iter(|| {
            let s = black_box("banana").replace(black_box("a"), black_box("o"));
            black_box(s);
        })
    });

    c.bench_function("turbo_string_replace", |b| {
        b.iter(|| {
            let s = replace(black_box("banana"), black_box("a"), black_box("o")).unwrap();
            black_box(s);
        })
    });
}

criterion_group!(
    benches,
    bench_small_string_creation,
    bench_string_builder_vs_format,
    bench_string_replace
);
criterion_main!(benches);
