//! Various helpful macros related to implementing `SerHex`.


/// helper macro for implementing the `into_hex_raw` function for
/// bytearray-style types.
#[macro_export]
macro_rules! into_hex_bytearray {
    ($src: ident, $dst: ident, $len: expr) => {
        {
            let src: &[u8] = $src.as_ref();
            debug_assert!(src.len() == $len);
            // add prefix if we are doing such things.
            if <C as $crate::HexConf>::withpfx() { $dst.write_all("0x".as_bytes())?; }
            // if 
            if <C as $crate::HexConf>::compact() {
                // find index and location of first non-zero byte.
                if let Some((idx,val)) = src.iter().enumerate().find(|&(_,v)| *v > 0u8) {
                    // if first non-zero byte is less than `0x10`, repr w/ one hex char.
                    if *val < 0x10 {
                        if <C as $crate::HexConf>::withcap() {
                            $dst.write_all(&[$crate::utils::fromvalcaps(*val)?])?;
                            $crate::utils::writehexcaps(&src[(idx + 1)..],$dst)
                        } else {
                            $dst.write_all(&[$crate::utils::fromval(*val)?])?;
                            $crate::utils::writehex(&src[(idx + 1)..],$dst)
                        }
                    } else {
                        if <C as $crate::HexConf>::withcap() {
                            $crate::utils::writehexcaps(&src[idx..],$dst)
                        } else {
                            $crate::utils::writehex(&src[idx..],$dst)
                        }
                    }
                // if no non-zero byte was found, just write in a zero.
                } else {
                    $dst.write_all(&[b'0'])?;
                    Ok(())
                }
            } else {
                if <C as $crate::HexConf>::withcap() {
                    $crate::utils::writehexcaps(src,$dst)
                } else {
                    $crate::utils::writehex(src,$dst)
                }
            }
        }
    }
}


/// helper macro for implementing the `into_hex_raw` function for
/// bytearray-style types.
#[macro_export]
macro_rules! from_hex_bytearray {
    ($src: ident, $len: expr) => {
        {
            let raw: &[u8] = $src.as_ref();
            let hex = if <C as $crate::HexConf>::withpfx() {
                let pfx = "0x".as_bytes();
                if raw.starts_with(pfx) { &raw[2..] } else { raw }
            } else {
                raw
            };
            let mut buf = [0u8;$len];
            if <C as $crate::HexConf>::compact() {
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
            Ok(buf)
        }
    }
}


/// macro for implementing `SerHex` for a type which implements
/// `From<[u8;n]>` and `AsRef<[u8]>`.
#[macro_export]
macro_rules! impl_serhex_bytearray {
    ($type: ty, $len: expr) => {
        impl<C> $crate::SerHex<C> for $type where C: $crate::HexConf {
            type Error = $crate::types::Error;
            fn into_hex_raw<D>(&self, mut dst: D) -> Result<(),Self::Error> where D: $crate::std::io::Write {
                into_hex_bytearray!(self,dst,$len)?;
                Ok(())
            }
            fn from_hex_raw<S>(src: S) -> Result<Self,Self::Error> where S: AsRef<[u8]> {
                let rslt: Result<[u8;$len],Self::Error> = from_hex_bytearray!(src,$len);
                match rslt {
                    Ok(buf) => Ok(buf.into()),
                    Err(e) => Err(e)
                }
            }
             
        }
    }
}


#[cfg(test)]
mod tests {
    use ::{SerHex,Strict,StrictPfx,StrictCap,StrictCapPfx,Compact,CompactPfx,CompactCap,CompactCapPfx};

    #[derive(Debug,PartialEq,Eq)]
    struct Foo([u8;4]);
    impl_newtype_bytearray!(Foo,4);
    impl_serhex_bytearray!(Foo,4);


    #[test]
    fn hex_strict_ok() {
        let f1 = Foo([0,1,2,3]);
        let hs = <Foo as SerHex<Strict>>::into_hex(&f1).unwrap();
        let f2 = <Foo as SerHex<Strict>>::from_hex(&hs).unwrap();
        assert_eq!(f1,f2);
    }

    #[test]
    #[should_panic]
    fn hex_strict_err() {
        let _ = <Foo as SerHex<Strict>>::from_hex("faaffaa").unwrap();
    }

    #[test]
    fn hex_compact() {
        let f1 = Foo([0,0,0x0a,0xff]);
        let hs = <Foo as SerHex<Compact>>::into_hex(&f1).unwrap();
        assert_eq!(&hs,"aff");
        let f2 = <Foo as SerHex<Compact>>::from_hex(&hs).unwrap();
        assert_eq!(f1,f2);
    }

    #[test]
    fn hex_variants() {
        let f = Foo([0x00,0x0f,0xff,0x11]);
        assert_eq!("0x000fff11",<Foo as SerHex<StrictPfx>>::into_hex(&f).unwrap());
        assert_eq!("000FFF11",<Foo as SerHex<StrictCap>>::into_hex(&f).unwrap());
        assert_eq!("0x000FFF11",<Foo as SerHex<StrictCapPfx>>::into_hex(&f).unwrap());
        assert_eq!("0xfff11",<Foo as SerHex<CompactPfx>>::into_hex(&f).unwrap());
        assert_eq!("FFF11",<Foo as SerHex<CompactCap>>::into_hex(&f).unwrap());
        assert_eq!("0xFFF11",<Foo as SerHex<CompactCapPfx>>::into_hex(&f).unwrap());

    }
}
