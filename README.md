# Turbo Ecosystem

[![License](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-blue.svg)](#license)
[![Rust](https://img.shields.io/badge/rust-stable_1.75.0%2B-orange.svg)](#rust-toolchain)

Turbo is a high-performance, open-source infrastructure and data processing platform for **Rust**, featuring high-fidelity official bindings for **Node.js** (TypeScript via `napi-rs`) and **Python** (via `PyO3`).

Engineered as a cohesive, low-latency ecosystem, Turbo adheres to the philosophy of *Zero Cost Abstractions*, efficient CPU/memory utilization (cache locality, SIMD, object pooling), and robust concurrency safety.

---

## ⚡ Architectural Principles

* **Zero-Cost Abstractions**: Pay only for what you use. Abstractions compile down to highly optimized machine code.
* **Ergonomics & Consistency**: Clean, predictable, and discoverable APIs. The user should feel like they are working with a single unified framework.
* **Non-Panicking APIs**: Public APIs never panic. Errors are strongly-typed, predictable, and managed using `Result`.
* **Memory Optimization**: Cache-friendly memory layouts, reuse of byte buffers, arenas, and object pools to minimize allocation pressure and avoid GC latency.
* **Unsafe Rust**: Allowed only when backed by benchmarks showing measurable gains, thoroughly documented with safety invariants, and covered by strict test suites.

---

## 📦 Workspace Layout

The ecosystem is organized as a single Cargo Workspace to ensure consistent versioning and configuration:

```
turbo/
├── Cargo.toml                # Workspace definition
├── rust-toolchain.toml       # Pinned Rust toolchain (MSRV)
├── README.md                 # Project overview and documentation
├── LICENSE                   # MIT / Apache 2.0 dual license
├── CHANGELOG.md              # Keeping track of changes
├── crates/                   # Rust core libraries
│   ├── turbo-core            # Shared types, custom allocators, macros, errors
│   ├── turbo-bytes           # Zero-copy binary buffers and cursor streams
│   └── ... (other components)
├── bindings/                 # Official bindings for Node.js and Python
│   ├── node/                 # TypeScript modules powered by napi-rs
│   └── python/               # CPython wheels powered by PyO3
├── benchmarks/               # Criterion benchmark suites
└── examples/                 # Executable application examples
```

---

## 🚀 Order of Implementation

1. **`turbo-core`** - Shared core types, errors, results, and allocation traits.
2. **`turbo-bytes`** - Efficient byte buffer views and cursor readers/writers.
3. **`turbo-string`** - Highly optimized UTF-8 text processing.
4. **`turbo-hash`** - SwissTable-based custom high-performance hashing.
5. **`turbo-collections`** - Arena-backed and stable dense arrays.
6. **`turbo-pool`** - Slabs, arenas, and object pools.
7. **`turbo-worker`** - Scoped and priority work-stealing thread pools.
8. **`turbo-csv`** - Ultra-fast CSV parser with SIMD acceleration.
9. **`turbo-json`** - JSON parser, DOM, serialization, and patching.
10. **`turbo-query`** - Query pipeline engine, projection, and aggregations.
11. **`turbo-data`** - DataFrames and Series.
12. **`turbo-http`** - High-speed HTTP parser.
13. **`turbo-cache`** - Thread-safe LRU, LFU, ARC cache implementations.
14. **`turbo-log`** - Async structured logging.
15. **`turbo-metrics`** - High-resolution telemetry (gauges, counters, histograms).
16. **`turbo-cli`** - Fast command-line parser.
17. **`turbo-config`** - Flexible TOML/YAML config loading.

---

## 🤝 Contributing

We welcome contributions from the community! To maintain the highest standards of code quality, performance, and API design, we ask that all contributors follow these guidelines.

### Development Workflow

1. **Fork & Branch**: Fork the repository and create your feature branch:
   ```bash
   git checkout -b feat/your-awesome-feature
   ```
2. **Implement Incrementally**: Keep pull requests focused. Avoid generating thousands of lines of changes at once.
3. **Write Tests**: Ensure your code is thoroughly covered by unit and integration tests.
4. **Benchmark**: If changing performance-sensitive code, add Criterion benchmarks comparing your changes against baseline implementations.
5. **Format & Lint**: Ensure your code compiles cleanly without any warnings:
   ```bash
   cargo fmt --all
   cargo clippy --workspace --all-targets -- -D warnings
   ```
6. **Commit**: Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for git messages:
   - `feat(core): ...`
   - `fix(bytes): ...`
   - `perf(hash): ...`
   - `docs(string): ...`

### Code Quality Checklist

* **No Warnings**: Code must compile with zero warnings (`-D warnings` is enforced in CI).
* **No Unused Code/TODOs**: Remove dead code and do not leave dangling `TODO` comments.
* **Documentation**: All public APIs must have description, safety invariants (if unsafe), performance notes, and a compilable code example.
* **Unsafe Code Guidelines**: If you write `unsafe`, you must:
  * Provide a benchmark justifying the performance gain.
  * Add a comment block explaining the safety invariants (`// SAFETY:`).
  * Write extensive property-based or integration tests verifying bounds.
* **MSRV Guard**: Keep code compatible with the ecosystem Minimum Supported Rust Version (`1.75.0`).

---

## ⚡ Performance Benchmarks

All core components are benchmarked using [Criterion.rs](https://github.com/bheisler/criterion.rs). Below is a telemetry summary of the performance compared to alternative designs or standard library structures, measured on the development target:

### 1. Custom Allocations & Core Operations (`turbo-core`)
* **Standard Heap Allocation**: `global_alloc_alloc_dealloc` takes **~8.7 ns**
* **Tracking Allocator Overhead**: `tracking_alloc_alloc_dealloc` takes **~19.0 ns** (providing lightweight heap usage counting with minimal overhead)
* **Formatted Error Construction**: `error_to_string_capacity_overflow` takes **~135.5 ns**

### 2. Byte Buffers & Zero-Copy Cursor Streams (`turbo-bytes`)
* **Buffer Recycling**: `pool_acquire_and_recycle_4k` checkout and release takes **~62.0 ns** (minimizing raw heap pressure under cyclic usage)
* **Binary Cursor Writers**: `cursor_write_u32_le_loop` takes **~0.27 ns per write operation** (ideal for low-latency network serializations)

### 3. High-Performance String Buffers (`turbo-string`)
* **Small String Optimization (SSO)**: `small_string_new_short` (inline stack string of <= 22 bytes) takes **~31.7 ns** without allocating on the heap
* **String Builder Formatting**: `string_builder_format` takes **~150.5 ns**
* **Substring Replacement**: `turbo_string_replace` takes **~155.6 ns**

### 4. Cache-Friendly Collections (`turbo-hash`)
* **HashMap Insertion**: `turbo_hash_map_insert` (inserting 100 entries) takes **~2.08 µs** (vs `std::collections::HashMap` taking **~3.80 µs** — **1.8x speedup**)
* **HashMap Lookup**: `turbo_hash_map_lookup` (100 query operations) takes **~195.2 ns** (vs `std::collections::HashMap` taking **~1.36 µs** — **7.0x speedup!**)

---

## 📜 License

Distributed under the terms of both the **MIT** and **Apache 2.0** licenses. See [LICENSE](file:///home/lauros/Workspace/projects/turbo-core/LICENSE) for details.
