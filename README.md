# serde-hex
Rust crate for easy serialization/deserialization of hexadecimal values.

## Overview

The core of this crate is the `SerHex` trait, which can be used with
[serde-derive](https://crates.io/crates/serde_derive) to allow for easy
derivation of the Serialize and Deserialize traits.  If some type
`Bar` implements `SerHex`, then any type which contains it may
rely on its `SerHex` implementation, like so:

```rust

#[derive(Serialize,Deserialize)]
struct Foo {
    #[serde(with = "SerHex")]
    bar: Bar
}

```

This crate includes a set of macros making it easy to define custom `SerHex`
implementations for strict or compact hexadecimal representations (those which
disallow or allow truncation of leading zeroes).


## Note

Check out the widely used [`hex`](https://crates.io/crates/hex) crate if you are just 
looking for hexadecimal conversion traits, and do not care about serde interop and/or 
compact representations.



