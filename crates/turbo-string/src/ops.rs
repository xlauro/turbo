use crate::builder::StringBuilder;
use crate::small::SmallString;
use turbo_core::Result;

/// Joins a slice of string slices with a separator, returning a [`SmallString`].
pub fn join(slices: &[&str], sep: &str) -> Result<SmallString> {
    if slices.is_empty() {
        return Ok(SmallString::new());
    }

    let total_len = slices.iter().map(|s| s.len()).sum::<usize>() + sep.len() * (slices.len() - 1);

    let mut builder = StringBuilder::with_capacity(total_len)?;
    for (i, slice) in slices.iter().enumerate() {
        if i > 0 {
            builder.push_str(sep)?;
        }
        builder.push_str(slice)?;
    }
    builder.into_string()
}

/// Trims whitespace from both ends of a string slice.
#[inline]
pub fn trim(s: &str) -> &str {
    s.trim()
}

/// Replaces all occurrences of a substring with another substring, returning a [`SmallString`].
pub fn replace(s: &str, from: &str, to: &str) -> Result<SmallString> {
    if s.is_empty() || from.is_empty() {
        return SmallString::from_str(s);
    }

    let mut result = StringBuilder::new();
    let mut last_end = 0;

    // Use standard match_indices on &str
    for (start, part) in s.match_indices(from) {
        result.push_str(&s[last_end..start])?;
        result.push_str(to)?;
        last_end = start + part.len();
    }

    result.push_str(&s[last_end..])?;
    result.into_string()
}

/// Normalizes a string slice by trimming and collapsing multiple spaces into a single space.
pub fn normalize(s: &str) -> Result<SmallString> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Ok(SmallString::new());
    }

    let mut result = StringBuilder::with_capacity(trimmed.len())?;
    let mut in_whitespace = false;

    for ch in trimmed.chars() {
        if ch.is_whitespace() {
            if !in_whitespace {
                result.push(' ')?;
                in_whitespace = true;
            }
        } else {
            result.push(ch)?;
            in_whitespace = false;
        }
    }
    result.into_string()
}
