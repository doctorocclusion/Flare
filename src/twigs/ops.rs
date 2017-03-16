use std::ops::*;

use {Expressible, ToExpressible, BoxedExpressible};

macro_rules! impl_binary_op {
    ($stdtrait:ident, $stdfunc:ident, $st:ident) => {
        pub struct $st {
            a: BoxedExpressible,
            b: BoxedExpressible,
        }

        impl $st {
            pub fn get_left(&self) -> &Expressible {
                &*self.a
            }

            pub fn get_right(&self) -> &Expressible {
                &*self.b
            }
        }

        impl Expressible for $st {
        }

        impl<O: ToExpressible> $stdtrait<O> for BoxedExpressible {
            type Output = $st;

            fn $stdfunc(self, other: O) -> $st {
                $st {
                    a: self,
                    b: other.to_boxed(),
                }
            }
        }

        pub fn $stdfunc<A: ToExpressible, B: ToExpressible>(a: A, b: B) -> $st {
            a.to_boxed().$stdfunc(b)
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

macro_rules! impl_unary_op {
    ($stdtrait:ident, $stdfunc:ident, $st:ident) => {
        pub struct $st {
            on: BoxedExpressible,
        }

        impl $st {
            pub fn get_val(&self) -> &Expressible {
                &*self.on
            }
        }

        impl Expressible for $st {
        }

        impl $stdtrait for BoxedExpressible {
            type Output = $st;

            fn $stdfunc(self) -> $st {
                $st {
                    on: self,
                }
            }
        }

        pub fn $stdfunc<A: ToExpressible>(a: A) -> $st {
            a.to_boxed().$stdfunc()
        }
    };
}

impl_unary_op!(Not, not, NotTwig);
impl_unary_op!(Neg, neg, NegTwig);