#![allow(non_camel_case_types)]

use super::Ty;

macro_rules! primitive {
    ($n:ident) => {
        pub struct $n;

        impl Ty for $n {

        }
    }
}

primitive!(u8);
primitive!(i8);
primitive!(u16);
primitive!(i16);
primitive!(u32);
primitive!(i32);
primitive!(u64);
primitive!(i64);
primitive!(usize);
primitive!(isize);