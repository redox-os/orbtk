use serde::{Deserialize, Serialize};
use std::fmt;

/// A UTF-16 encoded, growable string.
///
/// # Examples
///
/// let mut string16 = String16::from("Übung");
/// string16.push('ä');
#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct String16 {
    utf16: Vec<u16>,
}

impl String16 {
    /// Creates a new empty `String16`.
    pub fn new() -> Self {
        String16 { utf16: vec![] }
    }

    /// Returns the length of this `String16`, in bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// let a = String16::from("Übung");
    ///
    /// assert_eq!(a.len(), 5);
    #[inline]
    pub fn len(&self) -> usize {
        self.utf16.len()
    }

    /// Returns a slice of [`u16`]s bytes that were attempted to convert to a `String`.
    pub fn as_bytes(&self) -> &[u16] {
        &self.utf16
    }

    /// Returns a mutable slice of [`u16`]s bytes that were attempted to convert to a `String`.
    pub fn as_bytes_mut(&mut self) -> &mut [u16] {
        &mut self.utf16
    }

    /// Inserts a string slice into this `String16` at a byte position.
    pub fn insert_str(&mut self, idx: usize, string: &str) {
        let mut counter = idx;
        for u in string.encode_utf16() {
            self.utf16.insert(counter, u);
            counter += 1;
        }
    }

    /// Appends a given char onto the end of this `String16`.
    pub fn push(&mut self, ch: char) {
        let mut buf = [0; 2];

        for part in ch.encode_utf16(&mut buf) {
            self.utf16.push(*part)
        }
    }

    /// Removes a [`char`] from this `String16` at a byte position and returns it.
    pub fn remove(&mut self, idx: usize) {
        self.utf16.remove(idx);
    }

    /// Returns `true` if this `String16` has a length of zero, and `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.utf16.is_empty()
    }

    /// Returns `true` if this `String16` ends with the given string slice, or `false` otherwise.
    pub fn ends_with(&self, pat: &str) -> bool {
        self.as_string().ends_with(pat)
    }

    /// Truncates this `String16`, removing all contents.
    pub fn clear(&mut self) {
        self.utf16.clear()
    }

    /// Returns a string part begins with the `start`and ends with the `end` index.
    pub fn get_string(&self, start: usize, end: usize) -> Option<String> {
        self.utf16.get(start..end).map(String::from_utf16_lossy)
    }

    /// Converts the `String16` value to a String.
    pub fn as_string(&self) -> String {
        String::from_utf16_lossy(&self.utf16)
    }
}

impl From<&str> for String16 {
    fn from(s: &str) -> Self {
        String16 {
            utf16: s.encode_utf16().collect(),
        }
    }
}

impl From<String> for String16 {
    fn from(string: String) -> Self {
        String16 {
            utf16: string.encode_utf16().collect(),
        }
    }
}

impl fmt::Debug for String16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "String16 {}", self.as_string())
    }
}

impl fmt::Display for String16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_string() {
        let string16 = String16::from(String::from("Übung"));
        assert_eq!(string16.len(), 5);

        let string16 = String16::from(String::from("World"));
        assert_eq!(string16.len(), 5);
    }

    #[test]
    fn from_str() {
        let string16 = String16::from("Übung");
        assert_eq!(string16.len(), 5);

        let string16 = String16::from("World");
        assert_eq!(string16.len(), 5);
    }

    #[test]
    fn push() {
        // Single-u16 encoded char
        let mut string16 = String16::from("Fo");
        string16.push('o');
        assert_eq!(string16, String16::from("Foo"));

        // Two-u16 encoded char
        let mut string16 = String16::from("Bar");
        string16.push('𝕊');
        assert_eq!(string16, String16::from("Bar𝕊"));
    }
}
