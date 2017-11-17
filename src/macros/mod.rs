//! Collection of useful macros for implementing hexadecimal conversion
//! on common patterns.  Also includes macros for defining useful triats
//! for byte-array newtypes (e.g.; `Foo([u8;n])`), since this is one of
//! the most common situations where hexadeciaml serialization/deserialization
//! is useful.

#[macro_use]
pub mod misc;

#[macro_use]
pub mod hex;

