//! The `serde-hex` crate contains various utilities for Serialization/Deserialization
//! of hexadecimal values using [`serde`](https://crates.io/crates/serde).
#![warn(missing_docs)]

extern crate serde;

#[macro_use]
pub mod macros;
pub mod types;
pub mod utils;

use serde::{Serializer,Deserializer,Deserialize};
use std::error;

pub use types::Error;

/// Trait specifying custom serialization and deserialization logic from a
/// hexadecimal string to some arbitrary type.  This trait can be used to apply
/// custom parsing when using serde's `#[derive(Serialize,Deserialize)]`
/// flag.  Just add `#[serde(with = "SerHex")]` above any fields which implement
/// this trait.  Simplistic default implimentations for the the `serialize` and
/// `deserialize` methods are provided based on `into_hex` and `from_hex` respectively.
pub trait SerHex: Sized {
    /// Any error type which implements the `Error` trait can seamlessly
    /// interop with `serde` serializde/deserialize functionality.
    type Error: error::Error;

    /// Attempt to convert `self` into a hexadecimal string representation.
    fn into_hex(&self) -> Result<String,Self::Error>;

    /// Attempt to convert a slice of hexadecimal bytes into an instance of `Self`.
    fn from_hex<T: AsRef<[u8]>>(src: T) -> Result<Self,Self::Error>;

    /// Attempt to serialize `self` into a hexadecimal string representation.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok,S::Error> where S: Serializer {
        use serde::ser::Error;
        let hex = self.into_hex().map_err(S::Error::custom)?;
        serializer.serialize_str(&hex)
    }

    /// Attempt to deserialize a hexadecimal string into an instance of `Self`.
    fn deserialize<'de, D>(deserializer: D) -> Result<Self,D::Error> where D: Deserializer<'de> {
        use serde::de::Error;
        let data: &str = Deserialize::deserialize(deserializer)?;
        Self::from_hex(data).map_err(D::Error::custom)
    }
}

/// A macro for providing strict implementations of `SerHex` 
/// for non-wrapped byte-arrays (`[u8;n]`).  Can only be used 
/// inside of this crate, so we do not export this macro.
macro_rules! impl_hex_array_raw {
    ( $( $len: expr),+ ) => {
        $( impl_hex_array_strict!([u8;$len],$len); )+
    }
}


// implement `SerHex` for `u8` arrays of lengths from 1 to 32.  we don't currently implement this
// trait for arrays of length zero, since it is unclear what hexadecimal representation that would
// even take, and it would break the impl used by these macros.
impl_hex_array_raw!(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32);

// implement `SerHex` for common exponents of two.
impl_hex_array_raw!(64,128,256,512,1024,2048);

/*
/// A macro for providing `SerHex` implementations for unsigned integers.
// TODO: macro was added as an afterthought and currently just forwards
// work to the implementation of the appropriate sized byte-array.
// Should circle back and do this right.
macro_rules! impl_hex_uint {
    ($name: ty, $bytes: expr) => {

    }
}
*/

