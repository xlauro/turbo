use crate::error::Error;

/// Core Result type alias for the Turbo ecosystem.
pub type Result<T, E = Error> = core::result::Result<T, E>;
