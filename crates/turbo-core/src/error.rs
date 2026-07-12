/// Core error type representing all failure modes in the Turbo ecosystem.
///
/// This enum is non-exhaustive to allow adding new variants in a backwards-compatible manner.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Occurs when an operation exceeds a pre-allocated capacity or buffer limit.
    #[error("Capacity overflow: limit is {limit}, requested {requested}")]
    CapacityOverflow {
        /// The maximum allowed capacity.
        limit: usize,
        /// The requested capacity that caused the overflow.
        requested: usize,
    },

    /// Index out of bounds error.
    #[error("Out of bounds: index {index} is out of bounds for length {len}")]
    OutOfBounds {
        /// The accessed index.
        index: usize,
        /// The total length of the sequence.
        len: usize,
    },

    /// Emitted when data validation or parsing fails.
    #[error("Invalid data: {reason}")]
    InvalidData {
        /// Description of why the data is invalid.
        reason: &'static str,
    },

    /// Occurs when a memory allocator fails to allocate the requested layout.
    #[error("Allocation failed")]
    AllocError,

    /// Emitted when worker thread-pools, object pools, or other limited resources are depleted.
    #[error("Resource exhausted: {resource}")]
    ResourceExhausted {
        /// The name of the exhausted resource.
        resource: &'static str,
    },

    /// Emitted when a background task or execution is explicitly cancelled.
    #[error("Operation cancelled")]
    Cancellation,

    /// Wrapper around standard library I/O errors.
    #[cfg(feature = "std")]
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// A fallback variant for generic or custom errors.
    #[error("Custom error: {0}")]
    Custom(&'static str),

    /// Emitted when string formatting fails.
    #[error("Format error")]
    Format,
}

#[cfg(feature = "std")]
impl From<Error> for std::io::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::Io(e) => e,
            other => std::io::Error::new(std::io::ErrorKind::Other, other),
        }
    }
}

impl From<core::fmt::Error> for Error {
    #[inline]
    fn from(_: core::fmt::Error) -> Self {
        Error::Format
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alloc_crate::string::ToString;

    #[test]
    fn test_error_formatting() {
        let err = Error::CapacityOverflow {
            limit: 1024,
            requested: 2048,
        };
        assert_eq!(
            err.to_string(),
            "Capacity overflow: limit is 1024, requested 2048"
        );

        let err = Error::OutOfBounds { index: 10, len: 5 };
        assert_eq!(
            err.to_string(),
            "Out of bounds: index 10 is out of bounds for length 5"
        );

        let err = Error::InvalidData {
            reason: "bad encoding",
        };
        assert_eq!(err.to_string(), "Invalid data: bad encoding");

        let err = Error::AllocError;
        assert_eq!(err.to_string(), "Allocation failed");

        let err = Error::ResourceExhausted {
            resource: "threads",
        };
        assert_eq!(err.to_string(), "Resource exhausted: threads");

        let err = Error::Cancellation;
        assert_eq!(err.to_string(), "Operation cancelled");

        let err = Error::Custom("some custom error");
        assert_eq!(err.to_string(), "Custom error: some custom error");

        let err = Error::Format;
        assert_eq!(err.to_string(), "Format error");
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_io_error_conversion() {
        let std_io = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: Error = Error::Io(std_io);
        let back_to_io: std::io::Error = err.into();
        assert_eq!(back_to_io.kind(), std::io::ErrorKind::NotFound);

        let err = Error::AllocError;
        let io_err: std::io::Error = err.into();
        assert_eq!(io_err.kind(), std::io::ErrorKind::Other);
        assert_eq!(io_err.to_string(), "Allocation failed");
    }
}
