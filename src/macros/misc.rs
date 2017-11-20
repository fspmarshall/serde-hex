//! This module contains various helpful macros which are not
//! strictly part of Hexadecimal serialization/deserialization.


/// implements useful traits for the 'newtype' pattern.
/// this macro is automatically implemented by `impl_newtype_bytearray`,
/// so prefer that macro if `inner` is a byte-array (`[u8;n]`).
#[macro_export]
macro_rules! impl_newtype {

    ($outer: ident, $inner: ty) => {

        // dereference to inner value.
        impl ::std::ops::Deref for $outer {
            type Target = $inner;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        // convert from the inner value to the outer value.
        impl ::std::convert::From<$inner> for $outer {
            fn from(inner: $inner) -> Self {
                $outer(inner)
            }
        }

    }
}



/// implements useful traits for byte-array newtypes
/// (e.g.; `Foo([u8;n])`).  Includes all implementations from
/// the `impl_newtype` macro.
#[macro_export]
macro_rules! impl_newtype_bytearray {
    
    ($outer: ident, $len: expr) => {
        impl_newtype!($outer,[u8;$len]);

        // get reference as byte-slice.
        impl AsRef<[u8]> for $outer {
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }

        // implement the `LowerHex` trait to allow generation
        // of lowercase hexadecimal representations.
        impl ::std::fmt::LowerHex for $outer {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                for byte in self.as_ref().iter() {
                    write!(f,"{:02x}",byte)?;
                }
                Ok(())
            }
        }

        // implement the `UpperHex` trait to allow generation
        // of uppercase hexadecimal representations.
        impl ::std::fmt::UpperHex for $outer {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                for byte in self.as_ref().iter() {
                    write!(f,"{:02X}",byte)?;
                }
                Ok(())
            }
        }
    }
}


/// implements useful traits for byte-array newtypes
/// (e.g.; `Foo([u8;n])`) if grater than 32 elements in length.
/// Includes all implementations from the `impl_newtype` macro,
/// as well as s number of useful traits which cannot be
/// derived via `#[derive(...)]`.
#[macro_export]
macro_rules! impl_newtype_bytearray_ext {
    ($outer: ident, $len:expr) => {
        // implement everything from the nomral bytearray macro.
        impl_newtype_bytearray!($outer,$len);

        // manually implemented `Clone` trait for easy copying.
        impl Clone for $outer {
            fn clone(&self) -> Self {
                let mut buf = [0u8;$len];
                for (idx,itm) in self.as_ref().iter().enumerate() {
                    buf[idx] = *itm;
                }
                buf.into()
            }
        }

        // manuall implement `Default` trait for getting empty instances.
        impl Default for $outer {
            fn default() -> Self {
                $outer([0u8;$len])
            }
        }

        // manually implemented `Debug` trait for printouts.
        impl ::std::fmt::Debug for $outer {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}({:?})",stringify!($ident),self.as_ref())
            }
        }

        // manually implement `PartialEq` for comparison operations.
        impl ::std::cmp::PartialEq for $outer {
            fn eq(&self, other: &$outer) -> bool {
                self.as_ref() == other.as_ref()
            }
        }

        // manually flag type as `Eq` for full equivalence relations.
        impl ::std::cmp::Eq for $outer { }
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn implementation() {
        struct Bar([u8;36]);
        impl_newtype_bytearray_ext!(Bar,36);
    }
}

