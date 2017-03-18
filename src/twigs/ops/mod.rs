#[macro_use]
mod unary;
#[macro_use]
mod binary;

pub use self::unary::*;
pub use self::binary::*;

use std::ops::*;
use Expressible;

macro_rules! ops_for {
    ($name:path) => (
        binop_for!($name, Add, add);
        binop_for!($name, Sub, sub);
        binop_for!($name, Mul, mul);
        binop_for!($name, Div, div);
        binop_for!($name, Rem, rem);
        binop_for!($name, BitOr, bitor);
        binop_for!($name, BitAnd, bitand);
        binop_for!($name, BitXor, bitxor);
        binop_for!($name, Shl, shl);
        binop_for!($name, Shr, shr);
        unop_for!($name, Not, not);
        unop_for!($name, Neg, neg);
    )
}

ops_for!(::BinaryOpTwig);
ops_for!(::UnaryOpTwig);
ops_for!(::VariableTwig);
ops_for!(::ExistingVarTwig);
ops_for!(::CopiedTwig);
ops_for!(::ArgumentTwig);