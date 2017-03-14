use aster::AstBuilder;

pub trait LiteralValue {
    fn ast_expr(&self, builder: &AstBuilder) -> ::PExpr;
}

macro_rules! impl_literal {
    ($a:ident, $b:ident) => {
        impl_literal!($a, ex, s, ex.$b(*s));
    };

    ($a:ident, $b:ident, $s:ident, $f:expr) => {
        impl LiteralValue for $a {
            fn ast_expr(&self, builder: &AstBuilder) -> ::PExpr {
                let $s = self;
                let $b = builder.expr();
                $f
            }
        }
    }
}

impl_literal!(u8, u8);
impl_literal!(i8, i8);
impl_literal!(u16, u16);
impl_literal!(i16, i16);
impl_literal!(u32, u32);
impl_literal!(i32, i32);
impl_literal!(u64, u64);
impl_literal!(i64, i64);
impl_literal!(usize, usize);
impl_literal!(isize, isize);
impl_literal!(String, ex, s, ex.str(s.as_str()));

pub fn literal<L: LiteralValue>(lit: L) -> LiteralTwig<L> {
    LiteralTwig {
        val: lit
    }
}

impl<V: LiteralValue> ::twigs::ToExpressible<LiteralTwig<V>> for V {
    fn to_expressible(self) -> LiteralTwig<V> {
        literal(self)
    }
}

pub struct LiteralTwig<V: LiteralValue> {
    val: V
}

impl<V: LiteralValue> ::twigs::Expressible for LiteralTwig<V> {
    fn ast_expr(&self, builder: &AstBuilder) -> ::PExpr {
        self.val.ast_expr(builder)
    }
}