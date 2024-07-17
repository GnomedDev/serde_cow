use alloc::{borrow::Cow, vec::Vec};

use crate::CowBytes;

pub(crate) struct CowBytesVisitor;

impl<'de> serde::de::Visitor<'de> for CowBytesVisitor {
    type Value = CowBytes<'de>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("bytes")
    }

    fn visit_borrowed_bytes<E>(self, val: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CowBytes(Cow::Borrowed(val)))
    }

    fn visit_bytes<E>(self, val: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_byte_buf(val.into())
    }

    fn visit_byte_buf<E>(self, val: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CowBytes(Cow::Owned(val)))
    }
}
