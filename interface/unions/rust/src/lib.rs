//! org.wasmcloud.example.union_demo

mod union_demo;
pub use union_demo::*;

// If you wish, you can write some helper functions for the union
// This will probably be code generated in the future
impl AnyValue {
    /// Returns Some(value) if AnyValue is a u8, otherwise returns None
    pub fn as_u8(&self) -> Option<u8> {
        match self {
            AnyValue::ValU8(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns Some(value) if AnyValue is a u16, otherwise returns None
    pub fn as_u16(&self) -> Option<u16> {
        match self {
            AnyValue::ValU16(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns Some(value) if AnyValue is a u32, otherwise returns None
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            AnyValue::ValU32(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns Some(value) if AnyValue is a u64, otherwise returns None
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            AnyValue::ValU64(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns Some(value) if AnyValue is an f64, otherwise returns None
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            AnyValue::ValF64(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns Some(value) if AnyValue is an string, otherwise returns None
    pub fn as_str(&self) -> Option<&String> {
        match self {
            AnyValue::ValStr(v) => Some(v),
            _ => None,
        }
    }

    /// Returns Some(value) if AnyValue is a byte array, otherwise returns None
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            AnyValue::ValBin(v) => Some(v),
            _ => None,
        }
    }
}
