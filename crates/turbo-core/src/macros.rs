/// Asserts that a boolean expression is true at compile time.
///
/// # Examples
/// ```
/// turbo_core::static_assert!(core::mem::size_of::<usize>() >= 4);
/// ```
#[macro_export]
macro_rules! static_assert {
    ($cond:expr) => {
        const _: () = {
            if !$cond {
                panic!("Static assertion failed");
            }
        };
    };
}

/// Helper macro to return early with an error.
#[macro_export]
macro_rules! turbo_bail {
    ($err:expr) => {
        return core::result::Result::Err(core::convert::Into::into($err));
    };
}

/// Helper macro to assert a condition or return early with an error.
#[macro_export]
macro_rules! turbo_ensure {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return core::result::Result::Err(core::convert::Into::into($err));
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::alloc_crate::string::ToString;
    use crate::error::Error;

    fn test_bail_func(value: bool) -> Result<(), Error> {
        if value {
            turbo_bail!(Error::Custom("bailed"));
        }
        Ok(())
    }

    fn test_ensure_func(value: bool) -> Result<(), Error> {
        turbo_ensure!(value, Error::Custom("not true"));
        Ok(())
    }

    #[test]
    fn test_macros() {
        // static_assert is checked at compile-time.
        static_assert!(1 + 1 == 2);

        assert!(test_bail_func(false).is_ok());
        let err = test_bail_func(true).unwrap_err();
        assert_eq!(err.to_string(), "Custom error: bailed");

        assert!(test_ensure_func(true).is_ok());
        let err = test_ensure_func(false).unwrap_err();
        assert_eq!(err.to_string(), "Custom error: not true");
    }
}
