use core::slice;
use std::fmt;
use std::ops::{Bound, RangeBounds};

use nucleo_matcher::Utf32Str;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Utf32String {
    /// A string represented as ASCII encoded bytes.
    /// Correctness invariant: must only contain valid ASCII (<=127)
    Ascii(Box<str>),
    /// A string represented as an array of unicode codepoints (basically UTF-32).
    Unicode(Box<[char]>),
}
impl Utf32String {
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Utf32String::Unicode(codepoints) => codepoints.len(),
            Utf32String::Ascii(ascii_bytes) => ascii_bytes.len(),
        }
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            Utf32String::Unicode(codepoints) => codepoints.is_empty(),
            Utf32String::Ascii(ascii_bytes) => ascii_bytes.is_empty(),
        }
    }

    /// Same as `slice` but accepts a u32 range for convenicene sine
    /// those are the indices returned by the matcher
    #[inline]
    pub fn slice(&self, range: impl RangeBounds<u32>) -> Utf32Str {
        let start = match range.start_bound() {
            Bound::Included(&start) => start as usize,
            Bound::Excluded(&start) => start as usize + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end as usize,
            Bound::Excluded(&end) => end as usize + 1,
            Bound::Unbounded => self.len(),
        };
        match self {
            Utf32String::Ascii(bytes) => Utf32Str::Ascii(&bytes.as_bytes()[start..end]),
            Utf32String::Unicode(codepoints) => Utf32Str::Unicode(&codepoints[start..end]),
        }
    }

    pub fn is_ascii(&self) -> bool {
        matches!(self, Utf32String::Ascii(_))
    }

    pub fn get(&self, idx: u32) -> char {
        match self {
            Utf32String::Ascii(bytes) => bytes.as_bytes()[idx as usize] as char,
            Utf32String::Unicode(codepoints) => codepoints[idx as usize],
        }
    }
    pub fn last(&self) -> char {
        match self {
            Utf32String::Ascii(bytes) => bytes.as_bytes()[bytes.len() - 1] as char,
            Utf32String::Unicode(codepoints) => codepoints[codepoints.len() - 1],
        }
    }
    pub fn chars(&self) -> Chars<'_> {
        match self {
            Utf32String::Ascii(bytes) => Chars::Ascii(bytes.as_bytes().iter()),
            Utf32String::Unicode(codepoints) => Chars::Unicode(codepoints.iter()),
        }
    }
}

impl From<&str> for Utf32String {
    fn from(value: &str) -> Self {
        if value.is_ascii() {
            Self::Ascii(value.to_owned().into_boxed_str())
        } else {
            Self::Unicode(value.chars().collect())
        }
    }
}

impl From<Box<str>> for Utf32String {
    fn from(value: Box<str>) -> Self {
        if value.is_ascii() {
            Self::Ascii(value)
        } else {
            Self::Unicode(value.chars().collect())
        }
    }
}
impl From<String> for Utf32String {
    fn from(value: String) -> Self {
        value.into_boxed_str().into()
    }
}

pub enum Chars<'a> {
    Ascii(slice::Iter<'a, u8>),
    Unicode(slice::Iter<'a, char>),
}
impl<'a> Iterator for Chars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Chars::Ascii(iter) => iter.next().map(|&c| c as char),
            Chars::Unicode(iter) => iter.next().copied(),
        }
    }
}

impl fmt::Debug for Utf32String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"")?;
        for c in self.chars() {
            for c in c.escape_debug() {
                write!(f, "{c}")?
            }
        }
        write!(f, "\"")
    }
}

impl fmt::Display for Utf32String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"")?;
        for c in self.chars() {
            write!(f, "{c}")?
        }
        write!(f, "\"")
    }
}
