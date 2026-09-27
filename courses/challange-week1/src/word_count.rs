//! Task 2: count the frequency of words in a text file with a `HashMap`,
//! normalising capitalisation and punctuation so that different surface
//! forms of the same word are combined.

use std::collections::HashMap;
use std::error::Error;
use std::fs;

/// Reduces a raw token to its canonical word form: lowercased, with any
/// leading/trailing punctuation stripped.
///
/// Returns `None` when the token contains no alphabetic character at all.
/// Inner apostrophes and hyphens are preserved, so `don't` stays one word.
#[must_use]
pub fn normalize(token: &str) -> Option<String> {
    let lower = token.to_lowercase();
    let start = lower.find(|c: char| c.is_alphabetic())?;
    let (last_byte, last_char) = lower
        .char_indices()
        .rev()
        .find(|&(_, c)| c.is_alphabetic())?;
    let end = last_byte + last_char.len_utf8();
    Some(lower[start..end].to_string())
}

/// Counts the normalised words of `text`, one `HashMap` entry per distinct word.
#[must_use]
pub fn word_frequencies(text: &str) -> HashMap<String, usize> {
    let mut frequencies: HashMap<String, usize> = HashMap::new();
    for token in text.split_whitespace() {
        if let Some(word) = normalize(token) {
            *frequencies.entry(word).or_default() += 1;
        }
    }
    frequencies
}

/// Reads the file at `path` and counts its words.
///
/// # Errors
///
/// Propagates file I/O errors.
pub fn word_frequencies_file(path: &str) -> Result<HashMap<String, usize>, Box<dyn Error>> {
    Ok(word_frequencies(&fs::read_to_string(path)?))
}
