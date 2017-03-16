use std::ops::*;

use {Expressible, BoxedExpressible};

macro_rules! impl_binary_op {
    ($stdtrait:ident, #[$fdoc:meta] $stdfunc:ident, #[$doc:meta] $st:ident) => {
        #[$doc]
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

        impl<O: Expressible + 'static> $stdtrait<O> for BoxedExpressible {
            type Output = $st;

            fn $stdfunc(self, other: O) -> $st {
                $st {
                    a: self,
                    b: other.to_boxed(),
                }
            }
        }

        #[$fdoc]
        pub fn $stdfunc<A: Expressible + 'static, B: Expressible + 'static>(a: A, b: B) -> $st {
            a.to_boxed().$stdfunc(b)
        }
    };
}

impl_binary_op!(Add, 
    /// Join two expression twigs with a + operation
    add,
    /// A twig composed of two other expression twigs attached together by a + operation
    AddTwig);
impl_binary_op!(Sub, 
    /// Join two expression twigs with a - operation
    sub, 
    /// A twig composed of two other expression twigs attached together by a - operation
    SubTwig);
impl_binary_op!(Mul, 
    /// Join two expression twigs with a * operation
    mul,
    /// A twig composed of two other expression twigs attached together by a * operation 
    MulTwig);
impl_binary_op!(Div, 
    /// Join two expression twigs with a / operation
    div, 
    /// A twig composed of two other expression twigs attached together by a / operation 
    DivTwig);
impl_binary_op!(Rem, 
    /// Join two expression twigs with a % operation
    rem, 
    /// A twig composed of two other expression twigs attached together by a % operation 
    RemTwig);
impl_binary_op!(BitOr, 
    /// Join two expression twigs with an % operation
    bitor, 
    /// A twig composed of two other expression twigs attached together by an | operation 
    BitOrTwig);
impl_binary_op!(BitAnd, 
    /// Join two expression twigs with an & operation
    bitand, 
    /// A twig composed of two other expression twigs attached together by an & operation 
    BitAndTwig);
impl_binary_op!(BitXor, 
    /// Join two expression twigs with an ^  operation
    bitxor, 
    /// A twig composed of two other expression twigs attached together by an ^ operation 
    BitXorTwig);
impl_binary_op!(Shl, 
    /// Join two expression twigs with a << operation
    shl, 
    /// A twig composed of two other expression twigs attached together by a << operation 
    ShlTwig);
impl_binary_op!(Shr, 
    /// Join two expression twigs with a >> operation
    shr, 
    /// A twig composed of two other expression twigs attached together by a >> operation 
    ShrTwig);

macro_rules! impl_unary_op {
    ($stdtrait:ident, #[$fdoc:meta] $stdfunc:ident, #[$doc:meta] $st:ident) => {
        #[$doc]
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

        #[$fdoc]
        pub fn $stdfunc<A: Expressible + 'static>(a: A) -> $st {
            a.to_boxed().$stdfunc()
        }
    };
}

impl_unary_op!(Not, 
    /// Apply the ! operation to an expression twig
    not, 
    /// The ! operation applied to an expression twig
    NotTwig);
impl_unary_op!(Neg, 
    /// Apply the - operation to an expression twig
    neg, 
    /// The - operation applied to an expression twig
    NegTwig);

/// A twig composed of a mutable or immutable borrow of another twig
pub struct RefTwig {
    mutable: bool,
    of: BoxedExpressible,
}