//! various helper functions.
use std::borrow::Borrow;
use std::io;
use types::{Result,Error};


/// convert a byte from a hex string to its numeric value.
/// use the `tobyte` function to convert a pair of hex characters
/// to their actual byte representation.
#[inline]
pub fn intoval(c: u8) -> Result<u8> {
    // ------------------------------------------------------
    // NOTE: the below logic is a nearly exact copy of an
    // equivalent function in the `hex` crate.
    // ------------------------------------------------------
    //   repository - https://github.com/KokaKiwi/rust-hex
    //   crates.io  - https://crates.io/crates/hex
    // ------------------------------------------------------
    // copyright at time of shameless theft:
    //   Copyright (c) 2013-2014 The Rust Project Developers.
    //   Copyright (c) 2015-2016 The rust-hex Developers
    // ------------------------------------------------------
    // licensing at time of shameless theft:
    //   MIT/APACHE (at your option)
    // ------------------------------------------------------
    match c {
        b'A'...b'F' => Ok(c - b'A' + 10),
            b'a'...b'f' => Ok(c - b'a' + 10),
            b'0'...b'9' => Ok(c - b'0'),
            _ => {
                Err(Error::BadChar(c as char))
            }
    }
}


/// convert a byte value in range `0x00` to `0x0f` to its
/// corresponding lowercase hexadecimal character.
#[inline]
pub fn fromval(v: u8) -> Result<u8> {
    match v {
        0xa...0xf => Ok(v - 0xa + b'a'),
        0x0...0x9 => Ok(v + b'0'),
        _ => Err(Error::BadByte(v))
    }
}


/// convert a byte value in range `0x00` to `0x0f` to its
/// corresponding uppercase hexadecimal character.
#[inline]
pub fn fromvalcaps(v: u8) -> Result<u8> {
    match v {
        0xA...0xF => Ok(v - 0xa + b'A'),
        0x0...0x9 => Ok(v + b'0'),
        _ => Err(Error::BadByte(v))
    }
}


/// attemt to convert a pair of bytes from a hexadecimal string into their
/// underlying byte representation.
#[inline]
pub fn intobyte(a: u8, b: u8) -> Result<u8> {
    Ok(intoval(a)? << 4 | intoval(b)?)
}


/// attempt to convert a byte value into a pair of hexadecimal values.
#[inline]
pub fn frombyte(val: u8) -> Result<(u8,u8)> {
    Ok((fromval(val >> 4)?,fromval(val & 0x0f)?))
}


/// attempt to convert a byte value into a pair of uppercase hexadecimal values.
#[inline]
pub fn frombytecaps(val: u8) -> Result<(u8,u8)> {
    Ok((fromvalcaps(val >> 4)?,fromvalcaps(val & 0x0f)?))
}


/// Helper function which takes a mutable slice of expected byte-length and
/// attempts to parse an immutable slice of bytes as hexadecimal characters.
/// Returns an error if `src` is not exactly twice the size of `buf`, or if
/// any non-hexadecimal characters are found.
pub fn fromhex(buf: &mut [u8], src: &[u8]) -> Result<()> {
    if src.len() != buf.len() * 2 {
        return Err(Error::BadSize(src.len()));
    }
    for (idx,pair) in src.chunks(2).enumerate() {
        buf[idx] = intobyte(pair[0],pair[1])?;
    }
    Ok(())

}


/// Helper function which takes a mutable slice of expected hex-length and
/// attempts to convert an immutable slice of bytes into hexadecimal characters.
/// Returns an error if `buf` is not exactly twice the size of `src`.
pub fn intohex(buf: &mut [u8], src: &[u8]) -> Result<()> {
    if buf.len() != src.len() * 2 {
        return Err(Error::BadSize(buf.len()));
    }
    for (i,byte) in src.iter().enumerate() {
        let (a,b) = frombyte(*byte)?;
        let idx = i * 2;
        buf[idx] = a;
        buf[idx + 1] = b;
    }
    Ok(())
}

/// Helper function which takes a mutable slice of expected hex-length and
/// attempts to convert an immutable slice of bytes into capital hexadecimal characters.
/// Returns an error if `buf` is not exactly twice the size of `src`.
pub fn intohexcaps(buf: &mut [u8], src: &[u8]) -> Result<()> {
    if buf.len() != src.len() * 2 {
        return Err(Error::BadSize(buf.len()));
    }
    for (i,byte) in src.iter().enumerate() {
        let (a,b) = frombytecaps(*byte)?;
        let idx = i * 2;
        buf[idx] = a;
        buf[idx + 1] = b;
    }
    Ok(())
}


/// Helper function which attempts to convert an immutable set of bytes into 
/// hexadecimal characters and write them to some destination.
pub fn writehex<S,B,D>(src: S, mut dst: D) -> Result<()> where S: IntoIterator<Item=B>, B: Borrow<u8>, D: io::Write { 
    for byte in src.into_iter() {
        let (a,b) = frombyte(*byte.borrow())?;
        dst.write_all(&[a,b])?;
    }
    Ok(())
}


/// Helper function which attempts to convert an immutable set of bytes into 
/// capital hexadecimal characters and write them to some destination.
pub fn writehexcaps<S,B,D>(src: S, mut dst: D) -> Result<()> where S: IntoIterator<Item=B>, B: Borrow<u8>, D: io::Write { 
    for byte in src.into_iter() {
        let (a,b) = frombytecaps(*byte.borrow())?;
        dst.write_all(&[a,b])?;
    }
    Ok(())
}



#[cfg(test)]
mod tests {
    #[test]
    fn hex_bytes() {
        use utils::{frombyte,intobyte};
        for i in 0..255u8 {
            let h = frombyte(i).unwrap();
            let b = intobyte(h.0,h.1).unwrap();
            assert_eq!(i,b);
        }
        let hex = ["ff","aa","f0","a0","0f","0a","00","99","90","09"];
        for s in hex.iter() {
            let s: &[u8] = s.as_ref();
            let v = intobyte(s[0],s[1]).unwrap();
            let (a,b) = frombyte(v).unwrap();
            assert_eq!(s,&[a,b]);
        }
    }

    #[test]
    fn hex_strings() {
        use utils::{fromhex,intohex};
        let hv = ["ff","aa","f0f0","a0a0","1234","5678","0000","0123456789abfdef"];
        for hs in hv.iter() {
            let src: &[u8] = hs.as_ref();
            let mut buff = vec![0u8;src.len() / 2];
            let mut rslt = vec![0u8;buff.len() * 2];
            fromhex(&mut buff, src).unwrap();
            intohex(&mut rslt, &buff).unwrap();
            assert_eq!(src,AsRef::<[u8]>::as_ref(&rslt));
        }
    }
}
