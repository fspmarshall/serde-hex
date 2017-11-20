//! This module contains various helpful macros for hexadecimal
//! serialization/deserialization.



/// macro for converting a type which implements `AsRef<[u8]>`
/// into a strict hexadecimal representation.  Takes the variable
/// to be converted, and the lenght of the variable in bytes
/// as its argumgnets, and returns a `Result<String>`. 
#[macro_export]
macro_rules! into_hex_strict {
    ($src: ident, $len: expr) => {
        {
            // get a handle to `$src` as a `u8` slice.
            let src: &[u8] = $src.as_ref();
            // allocate a vector with exact expected size.
            let mut hex = vec![0u8;2 + ($len * 2)];
            // add prefix.
            hex[0] = b'0';
            hex[1] = b'x';
            match $crate::utils::intohex(&mut hex[2..],src) {
                Ok(()) => {
                    Ok(String::from_utf8(hex).expect("should always be valid UTF-8"))   
                },
                Err(e) => Err(e)
            }
        }
    }
}

/// Macro for strictly parsing a buffer of hexadecimal bytes
/// (anything which implements `AsRef<[u8]>`) into a
/// byte-array.  Takes the variable representing the buffer,
/// and the size of the array (in bytes) as arguments,
/// and returns a `Result<[u8;n]>`.
#[macro_export]
macro_rules! from_hex_strict {
    ($src: ident, $len: expr) => {
        {
            let raw: &[u8] = $src.as_ref();
            let pfx = "0x".as_bytes();
            let hex = if raw.starts_with(pfx) { &raw[2..] } else { raw }; 
            let mut buf = [0u8;$len];
            match $crate::utils::fromhex(&mut buf, hex) {
                Ok(()) => Ok(buf),
                Err(e) => Err(e)
            }
        }
    }
}


/// Implements `SerHex` for a type which implements `From<[u8;n]>`
/// and `AsRef<[u8]>`, and requires strict hexadecimal representations.
#[macro_export]
macro_rules! impl_hex_array_strict {
    ($outer: ty, $len: expr) => {
        impl $crate::SerHex for $outer {
            // use default error type for this crate.
            type Error = $crate::types::Error;
            // impl strict variant of `into_hex`.
            fn into_hex(&self) -> $crate::types::Result<String> {
                into_hex_strict!(self,$len)
            }

            // impl strict variant of `from_hex`.
            fn from_hex<T: AsRef<[u8]>>(src: T) -> $crate::types::Result<Self> {
                let buf = from_hex_strict!(src,$len)?;
                Ok(buf.into())
            }
        }
    }
}



/// Macro for attempting to parse a type which implements `AsRef<[u8]>` into a
/// buffer of hexadecimal bytes in compact representation.  Takes the variable
/// representing the source type, and the length of the source (in bytes) as
/// arguments.  Returns a `Result<Vec<u8>>` which should be trivially castable
/// into a string with `String::from_utf8` if the process succeeded.
#[macro_export]
macro_rules! into_hex_compact {
    ($src: ident, $len: expr) => {
        {
            let bytes: &[u8] = $src.as_ref();
            // get the index and value of the first nonzero byte.
            if let Some((idx,val)) = bytes.iter().enumerate().find(|&(_,v)| *v > 0u8) {
                // if leading value is less than 16, we represent it with a single character.
                let rem = if *val <= 0xf { 1 } else { 0 };
                // allocate a vector of the exact expected length.
                let mut buf = vec![0u8;(bytes.len() - idx) * 2 + 2 - rem];
                // add the `0x` prefix.
                buf[0] = b'0'; buf[1] = b'x';
                // attempt to parse the body of the bytes into hexadecimal.
                // NOTE: this works even if the body length is zero (i.e.; resulting hex will
                // be a single character), because ranges of the form `[n..]` where `n` is the
                // length of the array or slice just return an empty slice.
                match $crate::utils::intohex(&mut buf[(2 + rem)..],&bytes[(idx + rem)..]) {
                    Ok(()) => {
                        // check if a leading character needs to be added.
                        if rem > 0 {
                            match $crate::utils::fromval(bytes[idx]) {
                                Ok(val) => {
                                    buf[2] = val;
                                    Ok(buf)
                                },
                                Err(e) => Err(e)
                            }

                        } else {
                            Ok(buf)
                        }
                    },
                    Err(e) => Err(e)
                }
            } else {
                // if the iterator returned `None`, then all bytes are zeroes.
                Ok(vec![b'0',b'x',b'0'])
            }

        }

    }
}


/// Macro for parsing a compact buffer of hexadecimal bytes
/// (anything which implements `AsRef<[u8]>`) into a
/// byte-array.  Takes the variable representing the buffer,
/// and the size of the array (in bytes) as arguments,
/// and returns a `Result<[u8;n]>`.
#[macro_export]
macro_rules! from_hex_compact {
    ($src: ident, $len: expr) => {
        {
            let raw = $src.as_ref();
            let pfx = "0x".as_bytes();
            let hex = if raw.starts_with(pfx) { &raw[2..] } else { raw };
            if hex.len() == 0 ||  hex.len() > $len * 2 {
                return Err($crate::types::Error::BadSize(hex.len()).into());
            }
            let body = $len - (hex.len() / 2);
            let head = hex.len() % 2;
            let mut buf = [0u8;$len];
            // --------------------------------------------------------------------------
            // NOTE: the mess below this comment would not be necessary if this code were
            // inside a normal function body and we could use the `?` operator without
            // causing weird behavior.  this is a cautionary tale about sleep deprivation
            // and reckless macro-related enthusiasm.  ye be warned.
            // --------------------------------------------------------------------------
            // attempt to parse the body of the hexadecimal buffer.
            // this implementation works even if body does not exist, a slicing
            // on `[n..]` where n is the length of the array or slice actually
            // returns an empty slice.
            match $crate::utils::fromhex(&mut buf[body..],&hex[head..]) {
                Ok(()) => {
                    // check if leading character exists.
                    if head > 0 {
                        // attempt to parse leading character.
                        match $crate::utils::intobyte(b'0',hex[0]) {
                            Ok(val) => {
                                buf[body-head] = val;
                                Ok(buf)
                            },
                            Err(e) => Err(e)
                        }
                    } else {
                        Ok(buf)
                    }
                },
                Err(e) => Err(e)
            }
        }
    }
}



/// implements `SerHex` for a type which implements `From<[u8;n]>`
/// and allows for compact hexadecimal representations.
#[macro_export]
macro_rules! impl_hex_array_compact {
    ($outer: ty, $len: expr) => {
        impl $crate::SerHex for $outer {
            // use default error type for this crate.
            type Error = $crate::types::Error;
            
            // impl compact variant of `into_hex`.
            fn into_hex(&self) -> $crate::types::Result<String> {
                let buf: Vec<u8> = into_hex_compact!(self,$len)?;
                Ok(String::from_utf8(buf).expect("should always be valid UTF-8"))
            }

            // impl compact variant of `from_hex`.
            fn from_hex<T: AsRef<[u8]>>(src: T) -> $crate::types::Result<Self> {
                let buf = from_hex_compact!(src,$len)?;
                Ok(buf.into())
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use ::SerHex;

    #[derive(Debug,PartialEq,Eq)]
    struct Foo([u8;4]);
    impl_newtype!(Foo,[u8;4]);
    impl_hex_array_strict!(Foo,4);

    #[derive(Debug,PartialEq,Eq)]
    struct Bar([u8;4]);
    impl_newtype!(Bar,[u8;4]);
    impl_hex_array_compact!(Bar,4);

    #[test]
    fn hex_strict_ok() {
        let f1 = Foo([0,1,2,3]);
        let hs = f1.into_hex().unwrap();
        let f2 = Foo::from_hex(&hs).unwrap();
        assert_eq!(f1,f2);
    }

    #[test]
    #[should_panic]
    fn hex_strict_err() {
        let _ = Foo::from_hex("0xfaaffaa").unwrap();
    }

    #[test]
    fn hex_compact_ok() {
        let b1 = Bar([16,32,64,128]);
        let hs = b1.into_hex().unwrap();
        let b2 = Bar::from_hex(&hs).unwrap();
        assert_eq!(b1,b2);
    }

    #[test]
    #[should_panic]
    fn hex_compact_err() {
        let _ = Bar::from_hex("0x").unwrap();
    }

    #[test]
    fn hex_compact_alloc() {
        for i in 0..255 {
            let b = Bar([0,0,0,i]);
            let h = b.into_hex().unwrap();
            assert!(h.len() <= 4);
            assert_eq!(h.len(),h.capacity());
        }
    }
}

