//! The `serde-hex` crate contains various utilities for Serialization/Deserialization
//! of hexadecimal values using [`serde`](https://crates.io/crates/serde).
#![warn(missing_docs)]

extern crate array_init;
extern crate smallvec;
extern crate serde;

#[macro_use]
pub mod macros;
pub mod types;
pub mod utils;

use smallvec::SmallVec;
use serde::{Serializer,Deserializer,Deserialize};
use std::{io,error};

pub use types::Error;

/// Trait specifying custom serialization and deserialization logic from a
/// hexadecimal string to some arbitrary type.  This trait can be used to apply
/// custom parsing when using serde's `#[derive(Serialize,Deserialize)]`
/// flag.  Just add `#[serde(with = "SerHex")]` above any fields which implement
/// this trait.  Simplistic default implimentations for the the `serialize` and
/// `deserialize` methods are provided based on `into_hex_raw` and `from_hex_raw` respectively.
pub trait SerHex<C>: Sized where C: HexConf {
    /// Any error type which implements the `Error` trait can seamlessly
    /// interop with `serde` serializde/deserialize functionality.
    type Error: error::Error;

    /// Attept to convert `self` to hexadecimal, writing the resultant bytes to some buffer.
    fn into_hex_raw<D>(&self, dst: D) -> Result<(),Self::Error> where D: io::Write;
    
    /// Attempt to parse some buffer of hexadecimal bytes into an instance of `Self`.
    fn from_hex_raw<S>(src: S) -> Result<Self,Self::Error> where S: AsRef<[u8]>;

    /// Attempt to convert `self` into a hexadecimal string representation.
    fn into_hex(&self) -> Result<String,Self::Error> {
        let mut dst: Vec<u8> = Vec::with_capacity(32);
        self.into_hex_raw(&mut dst)?;
        Ok(String::from_utf8(dst).expect("invalid UTF-8 bytes in parsing"))
    }

    /// Attempt to convert a slice of hexadecimal bytes into an instance of `Self`.
    fn from_hex<S>(src: S) -> Result<Self,Self::Error> where S: AsRef<[u8]> {
        Self::from_hex_raw(src)
    }
    
    /// Attempt to serialize `self` into a hexadecimal string representation.
    /// *NOTE*: The default implementation attempts to avoid heap-allocation with a
    /// [`SmallVec`](https://docs.rs/smallvec/) of size `[u8;64]`. This default will
    /// prevent heap-alloc for non-prefixed serializations of `[u8;32]` or smaller.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok,S::Error> where S: Serializer {
        use serde::ser::Error;
        let mut dst = SmallVec::<[u8;64]>::new();
        self.into_hex_raw(&mut dst).map_err(S::Error::custom)?;
        serializer.serialize_bytes(dst.as_ref())
    }

    /// Attempt to deserialize a hexadecimal string into an instance of `Self`.
    fn deserialize<'de, D>(deserializer: D) -> Result<Self,D::Error> where D: Deserializer<'de> {
        use serde::de::Error;
        let buff: &[u8] = Deserialize::deserialize(deserializer)?; 
        let rslt = Self::from_hex_raw(buff).map_err(D::Error::custom)?;
        Ok(rslt)
    }
}


// implement strict variants of `SerHex` for arrays of `T` with
// lengths of 1 through 64 (where `T` implements the strict variants
// of `SerHex` as well).
impl_serhex_strict_array!(
     1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,16,
    17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,
    33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,
    49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64
);


/// Trait for supplying configuration to `SerHex`.
/// This trait takes no `self` parameters, as it is
/// intended to be applied unit structs.  All default
/// implementation are set to `false`.
pub trait HexConf {
    /// function indicating whether to use compact 
    /// (as apposed to strict) representation.
    #[inline]
    fn compact() -> bool { false }
    /// function indicating whether to prefixing (`0x`).
    #[inline]
    fn withpfx() -> bool { false }
    /// function indicating whether to use capital letters (`A-F`).
    #[inline]
    fn withcap() -> bool { false }
}

// Strict Variants: Strict,StrictPfx,StrictCap,StrictCapPfx
// Compact Variants: Compact,CompactPfx,CompactCap,CompactCapPfx

/// Config indicating a strict representation
/// with no capiltaization and no prefixing.
pub struct Strict;
impl HexConf for Strict { }

/// Config indicating a strict representation
/// with prefixing but no capitalization.
pub struct StrictPfx;
impl HexConf for StrictPfx {
    fn withpfx() -> bool { true }
}

/// Config indicating a strict representation
/// with capitalization but no prefixing.
pub struct StrictCap;
impl HexConf for StrictCap {
    fn withcap() -> bool { true }
}

/// Config indicating a strict representation
/// with capitalization and prefixing.
pub struct StrictCapPfx;
impl HexConf for StrictCapPfx {
    fn withpfx() -> bool { true }
    fn withcap() -> bool { true }
}

/// Config indicating compact representation
/// with no capitalization and no prefixing.
pub struct Compact;
impl HexConf for Compact {
    fn compact() -> bool { true }
}

/// Config indicating compact representation
/// with prefixing but no capitalization.
pub struct CompactPfx;
impl HexConf for CompactPfx {
    fn compact() -> bool { true }
    fn withpfx() -> bool { true }
}

/// Config indicating compact representation
/// with capitalization but no prefixing.
pub struct CompactCap;
impl HexConf for CompactCap {
    fn compact() -> bool { true }
    fn withcap() -> bool { true }
}

/// Config indicating compact representation
/// with capitalization and prefixing.
pub struct CompactCapPfx;
impl HexConf for CompactCapPfx {
    fn compact() -> bool { true }
    fn withcap() -> bool { true }
    fn withpfx() -> bool { true }
}



