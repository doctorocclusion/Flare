use std::ops::*;
use {Expressible, BoxedExpressible};

#[derive(Clone, Copy, Debug)]
pub enum UnaryOp {
    NOT,
    NEG,
}

pub struct UnaryOpTwig {
    on: BoxedExpressible,
    op: UnaryOp,
}

impl UnaryOpTwig {
    pub fn get_val(&self) -> &Expressible {
        &*self.on
    }

    pub fn get_op(&self) -> UnaryOp {
        self.op
    }
}

impl Expressible for UnaryOpTwig {
}

macro_rules! impl_unary_op {
    ($stdtrait:ident, $op:ident, #[$fdoc:meta] $stdfunc:ident) => {
        impl $stdtrait for BoxedExpressible {
            type Output = UnaryOpTwig;

            fn $stdfunc(self) -> UnaryOpTwig {
                use UnaryOp::*;
                UnaryOpTwig {
                    op: $op,
                    on: self,
                }
            }
        }

        #[$fdoc]
        pub fn $stdfunc<A: Expressible + 'static>(a: A) -> UnaryOpTwig {
            a.to_boxed().$stdfunc()
        }
    };
}

macro_rules! unop_for {
    ($name:path, $stdtrait:ident, $stdfunc:ident) => {
        impl $stdtrait for $name {
            type Output = UnaryOpTwig;

            #[inline]
            fn $stdfunc(self) -> UnaryOpTwig {
                ::$stdfunc(self)
            }
        }
    }
}

impl_unary_op!(Not, NOT,
    /// Apply the ! operation to an expression twig
    not);
impl_unary_op!(Neg, NEG,
    /// Apply the - operation to an expression twig
    neg);