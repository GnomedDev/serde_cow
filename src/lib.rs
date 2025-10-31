//! A small library with a [`Cow`] wrappers that implements [`serde::Deserialize`] in the ways that should be expected.
//!
//! By default, [`Cow`] is always deserialized to the [`Cow::Owned`] variant due to a lack of specialisation, but this library
//! provides [`CowStr`] and [`CowBytes`] which deserialize to [`Cow::Borrowed`] if possible, borrowing from the original data.
//!
//! # Example
//!
//! ```
//! use std::borrow::Cow;
//!
//! use serde_cow::CowStr;
//!
//! let source = r#""Hello World""#;
//!
//! let normal: Cow<str> = serde_json::from_str(source).unwrap();
//! assert!(matches!(normal, Cow::Owned(_))); // Wasteful!
//!
//! let efficent: CowStr = serde_json::from_str(source).unwrap();
//! assert!(matches!(efficent.0, Cow::Borrowed(_))); // Zero copy!
//! ```
//!
//! ## Minimum Supported Rust Version
//!
//! This is currently 1.56 and is considered a breaking change to increase.

#![no_std]
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use alloc::borrow::Cow;
use core::fmt::Formatter;

use serde::Deserializer;

mod bytes;
mod str;

extern crate alloc;

/// A wrapper around [`Cow<str>`] to implement [`serde::Deserialize`] in the expected way.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CowStr<'de>(pub Cow<'de, str>);

impl core::fmt::Debug for CowStr<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.0, f)
    }
}

impl<'a> core::fmt::Display for CowStr<'a> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

impl serde::Serialize for CowStr<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for CowStr<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_string(str::CowStrVisitor)
    }
}

impl<'de, T> From<T> for CowStr<'de> where Cow<'de, str>: From<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

/// A wrapper around `Cow<[u8]>` to implement [`serde::Deserialize`] in the expected way.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CowBytes<'de>(pub Cow<'de, [u8]>);

impl core::fmt::Debug for CowBytes<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.0, f)
    }
}

impl serde::Serialize for CowBytes<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for CowBytes<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_bytes(bytes::CowBytesVisitor)
    }
}

impl<'de, T> From<T> for CowBytes<'de> where Cow<'de, [u8]>: From<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
