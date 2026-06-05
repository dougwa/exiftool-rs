//! Endian-aware byte cursor over an in-memory buffer.
//!
//! ExifTool reads everything out of a byte buffer with an explicit byte order
//! (`II` little-endian / `MM` big-endian for TIFF). This mirrors that: a slice
//! plus a byte order, with bounds-checked typed reads at arbitrary offsets.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ByteOrder {
    Little,
    Big,
}

impl ByteOrder {
    pub fn u16(self, b: [u8; 2]) -> u16 {
        match self {
            ByteOrder::Little => u16::from_le_bytes(b),
            ByteOrder::Big => u16::from_be_bytes(b),
        }
    }
    pub fn u32(self, b: [u8; 4]) -> u32 {
        match self {
            ByteOrder::Little => u32::from_le_bytes(b),
            ByteOrder::Big => u32::from_be_bytes(b),
        }
    }
    pub fn u64(self, b: [u8; 8]) -> u64 {
        match self {
            ByteOrder::Little => u64::from_le_bytes(b),
            ByteOrder::Big => u64::from_be_bytes(b),
        }
    }
}

/// A read-only view over a buffer with a fixed byte order.
#[derive(Clone, Copy)]
pub struct Reader<'a> {
    pub data: &'a [u8],
    pub order: ByteOrder,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8], order: ByteOrder) -> Self {
        Reader { data, order }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn bytes(&self, off: usize, len: usize) -> Option<&'a [u8]> {
        self.data.get(off..off.checked_add(len)?)
    }

    pub fn u8(&self, off: usize) -> Option<u8> {
        self.data.get(off).copied()
    }

    pub fn u16(&self, off: usize) -> Option<u16> {
        let b = self.bytes(off, 2)?;
        Some(self.order.u16([b[0], b[1]]))
    }

    pub fn u32(&self, off: usize) -> Option<u32> {
        let b = self.bytes(off, 4)?;
        Some(self.order.u32([b[0], b[1], b[2], b[3]]))
    }

    pub fn u64(&self, off: usize) -> Option<u64> {
        let b = self.bytes(off, 8)?;
        Some(self.order.u64([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
    }

    pub fn i16(&self, off: usize) -> Option<i16> {
        self.u16(off).map(|v| v as i16)
    }
    pub fn i32(&self, off: usize) -> Option<i32> {
        self.u32(off).map(|v| v as i32)
    }
    pub fn i64(&self, off: usize) -> Option<i64> {
        self.u64(off).map(|v| v as i64)
    }
}

/// Read a big-endian u16 from a slice start (used by container parsers like JPEG/PNG
/// that are always big-endian regardless of any embedded TIFF byte order).
pub fn be_u16(b: &[u8]) -> Option<u16> {
    Some(u16::from_be_bytes([*b.first()?, *b.get(1)?]))
}
pub fn be_u32(b: &[u8]) -> Option<u32> {
    Some(u32::from_be_bytes([*b.first()?, *b.get(1)?, *b.get(2)?, *b.get(3)?]))
}
