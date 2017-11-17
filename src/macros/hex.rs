//! this module contains various helpful macros for hexadecimal
//! serialization/deserialization.


/// implements `SerHex` for a type which implements `From<[u8;n]>`
/// and `AsRef<[u8]>`, and requires strict hexadecimal representations.
#[macro_export]
macro_rules! impl_hex_array_strict {
    ($outer: ty, $len: expr) => {
        impl $crate::SerHex for $outer {
            // use default error type for this crate.
            type Error = $crate::types::Error;
            // impl strict variant of `into_hex`.
            fn into_hex(&self) -> $crate::types::Result<String> {
                // allocate a vactor with exact expected size.
                let mut hex = vec![0u8;2 + ($len * 2)];
                // add prefix.
                hex[0] = b'0';
                hex[1] = b'x';
                $crate::utils::intohex(&mut hex[2..], self.as_ref())?;
                Ok(String::from_utf8(hex).expect("should always be valid UTF-8"))
            }

            // impl strict variant of `from_hex`.
            fn from_hex<T: AsRef<[u8]>>(src: T) -> $crate::types::Result<Self> {
                let raw = src.as_ref();
                let pfx = "0x".as_bytes();
                let hex = if raw.starts_with(pfx) { &raw[2..] } else { raw }; 
                let mut buf = [0u8;$len];
                $crate::utils::fromhex(&mut buf, hex)?;
                Ok(buf.into())
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
                let bytes = self.as_ref();
                if let Some((idx,val)) = bytes.iter().enumerate().find(|&(_,v)| *v > 0u8) {
                    // if leading value is less than 16, we represent it with a single character.
                    let rem = if *val <= 0xf { 1 } else { 0 };
                    // allocate a vector of the exact expected length.
                    let mut buf = vec![0u8;(bytes.len() - idx) * 2 + 2 - rem];
                    // add the `0x` prefix.
                    buf[0] = b'0'; buf[1] = b'x';
                    // if leading value was less than 16, use the underlying `fromval`
                    // function to generate a single hexadecimal character.
                    if rem > 0 { buf[2] = $crate::utils::fromval(bytes[idx])?; }
                    // if there are still values to convert, pass off remaining work
                    // to the standard `intohex` function (always true unless final
                    // value is going to be in range `0x0f..0x01`).
                    if idx < $len - rem {
                        $crate::utils::intohex(&mut buf[(2 + rem)..],&bytes[(idx + rem)..])?;
                    }
                    Ok(String::from_utf8(buf).expect("should always be valid UTF-8"))
                } else {
                    // if the iterator returned `None`, then all bytes are zeroes.
                    Ok("0x0".to_string())
                }
            }

            // impl compact variant of `from_hex`.
            fn from_hex<T: AsRef<[u8]>>(src: T) -> $crate::types::Result<Self> {
                let raw = src.as_ref();
                let pfx = "0x".as_bytes();
                let hex = if raw.starts_with(pfx) { &raw[2..] } else { raw };
                if hex.len() == 0 ||  hex.len() > $len * 2 {
                    return Err($crate::utils::HexError::BadSize(hex.len()).into());
                }
                let body = $len - (hex.len() / 2);
                let head = hex.len() % 2;
                let mut buf = [0u8;$len];
                let src = if head > 0 {
                    buf[body-head] = $crate::utils::intobyte(b'0',hex[0])?;
                    &hex[head..]
                } else {
                    hex
                };
                $crate::utils::fromhex(&mut buf[body..],src)?;
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
    impl_hex_array_strict!(Bar,4);

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
}

