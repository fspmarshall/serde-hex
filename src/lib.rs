//! The `serde-hex` crate contains various utilities for Serialization/Deserialization
//! of hexadecimal values using [`serde`](https://crates.io/crates/serde).
#![warn(missing_docs)]

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
pub struct StrictCaps;
impl HexConf for StrictCaps {
    fn withcap() -> bool { true }
}

/// Config indicating a strict representation
/// with capitalization and prefixing.
pub struct StrictCapsPfx;
impl HexConf for StrictCapsPfx {
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
pub struct CompactCaps;
impl HexConf for CompactCaps {
    fn compact() -> bool { true }
    fn withcap() -> bool { true }
}

/// Config indicating compact representation
/// with capitalization and prefixing.
pub struct CompactCapsPfx;
impl HexConf for CompactCapsPfx {
    fn compact() -> bool { true }
    fn withcap() -> bool { true }
    fn withpfx() -> bool { true }
}


/// macro for implementing `SerHex` for a type which implements
/// `From<[u8;n]>` and `AsRef<[u8]>`.
macro_rules! impl_core_bytearray {
    ($type: ty, $len: expr) => {
        impl<C> $crate::SerHex<C> for $type where C: HexConf {
            type Error = $crate::types::Error;

            fn into_hex_raw<D>(&self, mut dst: D) -> Result<(),Self::Error> where D: io::Write {
                let src: &[u8] = self.as_ref();
                debug_assert!(src.len() == $len);
                // add prefix if we are doing such things.
                if <C as HexConf>::withpfx() { dst.write_all("0x".as_bytes())?; }
                // if 
                if <C as HexConf>::compact() {
                    // find index and location of first non-zero byte.
                    if let Some((idx,val)) = src.iter().enumerate().find(|&(_,v)| *v > 0u8) {
                        // if first non-zero byte is less than `0x10`, repr w/ one hex char.
                        if *val < 0x10 {
                            if <C as HexConf>::withcap() {
                                dst.write_all(&[$crate::utils::fromvalcaps(*val)?])?;
                                $crate::utils::writehexcaps(&src[(idx + 1)..],dst)
                            } else {
                                dst.write_all(&[$crate::utils::fromval(*val)?])?;
                                $crate::utils::writehex(&src[(idx + 1)..],dst)
                            }
                        } else {
                            if <C as HexConf>::withcap() {
                                $crate::utils::writehexcaps(&src[idx..],dst)
                            } else {
                                $crate::utils::writehex(&src[idx..],dst)
                            }
                        }
                    // if no non-zero byte was found, just write in a zero.
                    } else {
                        dst.write_all(&[b'0'])?;
                        Ok(())
                    }
                } else {
                    if <C as HexConf>::withcap() {
                        $crate::utils::writehexcaps(src,dst)
                    } else {
                        $crate::utils::writehex(src,dst)
                    }
                }
            }

            fn from_hex_raw<S>(src: S) -> Result<Self,Self::Error> where S: AsRef<[u8]> {
                let raw: &[u8] = src.as_ref();
                let hex = if <C as HexConf>::withpfx() {
                    let pfx = "0x".as_bytes();
                    if raw.starts_with(pfx) { &raw[2..] } else { raw }
                } else {
                    raw
                };
                let mut buf = [0u8;$len];
                if <C as HexConf>::compact() {
                    if hex.len() == 0 ||  hex.len() > $len * 2 {
                        return Err($crate::types::Error::BadSize(hex.len()));
                    }
                    let body = $len - (hex.len() / 2);
                    let head = hex.len() % 2;
                    if head > 0 {
                        buf[body-head] = $crate::utils::intobyte(b'0',hex[0])?;
                    }
                    $crate::utils::fromhex(&mut buf[body..],&hex[head..])?;
                } else {
                    $crate::utils::fromhex(&mut buf[..], hex)?;
                }
                Ok(buf.into())
            }
        }
    }
}



