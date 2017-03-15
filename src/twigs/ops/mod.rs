use std::ops::*;

use Exprable;
use Expressible;
use ToExpressibleBox;

macro_rules! impl_binary_op {
    ($stdtrait:ident, $stdfunc:ident, $st:ident) => {
        pub struct $st {
            a: Exprable,
            b: Exprable
        }

        impl Expressible for $st {
        }

        impl $stdtrait<ToExpressibleBox> for ToExpressibleBox {
            type Output = Box<$st>;

            fn $stdfunc(self, other: Exprable) -> Box<$st> {
                Box::new($st {
                    a: self,
                    b: other,
                })
            }
        }

    };
}

impl_binary_op!(Add, add, AddTwig);
impl_binary_op!(Sub, sub, SubTwig);
impl_binary_op!(Mul, mul, MulTwig);
impl_binary_op!(Div, div, DivTwig);
impl_binary_op!(Rem, rem, RemTwig);
impl_binary_op!(BitOr, bitor, BitOrTwig);
impl_binary_op!(BitAnd, bitand, BitAndTwig);
impl_binary_op!(BitXor, bitxor, BitXorTwig);
impl_binary_op!(Shl, shl, ShlTwig);
impl_binary_op!(Shr, shr, ShrTwig);