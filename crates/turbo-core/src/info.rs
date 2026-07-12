/// Returns the semver version of the compiled `turbo-core` library.
///
/// This retrieves the package version defined in `Cargo.toml` at compilation time.
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Returns the Minimum Supported Rust Version (MSRV) for the Turbo ecosystem.
///
/// We guarantee compatibility with this compiler version and any newer stable compiler.
pub const fn msrv() -> &'static str {
    "1.75.0"
}

/// Returns a slice listing all compile-time features enabled in the active build.
///
/// Useful for debugging, system diagnostics, or printing library capabilities.
pub fn active_features() -> &'static [&'static str] {
    &[
        #[cfg(feature = "std")]
        "std",
        #[cfg(feature = "alloc")]
        "alloc",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info() {
        assert_eq!(version(), env!("CARGO_PKG_VERSION"));
        assert_eq!(msrv(), "1.75.0");
        let active = active_features();
        #[cfg(feature = "std")]
        assert!(active.contains(&"std"));
        #[cfg(feature = "alloc")]
        assert!(active.contains(&"alloc"));
    }
}
