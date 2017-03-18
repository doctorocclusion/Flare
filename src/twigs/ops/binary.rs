use std::ops::*;
use {Expressible, BoxedExpressible};

#[derive(Clone, Copy, Debug)]
pub enum BinaryOp {
    ADD,
    SUB,
    MUL,
    DIV,
    REM,
    BOR,
    OR,
    BAND,
    AND,
    BXOR,
    SHL,
    SHR,
}

pub struct BinaryOpTwig {
    op: BinaryOp,
    a: BoxedExpressible,
    b: BoxedExpressible,
}

impl Expressible for BinaryOpTwig {

}

impl BinaryOpTwig {
    pub fn get_left(&self) -> &Expressible {
        &*self.a
    }

    pub fn get_right(&self) -> &Expressible {
        &*self.b
    }

    pub fn get_op(&self) -> BinaryOp {
        self.op
    }
}

macro_rules! impl_binary_op {
    ($stdtrait:ident, $op:ident, #[$fdoc:meta] $stdfunc:ident) => {
        impl $stdtrait<BoxedExpressible> for BoxedExpressible {
            type Output = BinaryOpTwig;

            fn $stdfunc(self, other: BoxedExpressible) -> BinaryOpTwig {
                use BinaryOp::*;
                BinaryOpTwig {
                    op: $op,
                    a: self,
                    b: other,
                }
            }
        }

        #[$fdoc]
        #[inline]
        pub fn $stdfunc<A: Expressible + 'static, B: Expressible + 'static>(a: A, b: B) -> BinaryOpTwig {
            a.to_boxed().$stdfunc(b.to_boxed())
        }
    };
}

macro_rules! binop_for {
    ($name:path, $stdtrait:ident, $stdfunc:ident) => {
        impl<O: Expressible + 'static> $stdtrait<O> for $name {
            type Output = BinaryOpTwig;

            #[inline]
            fn $stdfunc(self, other: O) -> BinaryOpTwig {
                $stdfunc(self, other)
            }
        }
    }
}

impl_binary_op!(Add, ADD,
    /// Join two expression twigs with a + operation
    add);
impl_binary_op!(Sub, SUB,
    /// Join two expression twigs with a - operation
    sub);
impl_binary_op!(Mul, MUL,
    /// Join two expression twigs with a * operation
    mul);
impl_binary_op!(Div, DIV,
    /// Join two expression twigs with a / operation
    div);
impl_binary_op!(Rem, REM,
    /// Join two expression twigs with a % operation
    rem);
impl_binary_op!(BitOr, BOR,
    /// Join two expression twigs with an % operation
    bitor);
impl_binary_op!(BitAnd, BAND,
    /// Join two expression twigs with an & operation
    bitand);
impl_binary_op!(BitXor, BXOR,
    /// Join two expression twigs with an ^  operation
    bitxor);
impl_binary_op!(Shl, SHL,
    /// Join two expression twigs with a << operation
    shl);
impl_binary_op!(Shr, SHR,
    /// Join two expression twigs with a >> operation
    shr);