pub mod lit;
pub mod bind;
pub mod ops;
pub mod lets;

use syntax::ast;

pub struct CodePaver {

}

pub trait Attachable {
    type After;
}

impl Attachable for ast::Stmt {
    type After = ();

    // TODO
}

pub trait Expressible {
}

pub trait ToExpressible {
    fn to_boxed(self) -> BoxedExpressible;
}

impl<T> ToExpressible for T where T: Expressible + 'static {
    fn to_boxed(self) -> BoxedExpressible {
        Box::new(self) as BoxedExpressible
    }
}

impl Expressible for ast::Expr {
    // TODO
}

pub type BoxedExpressible = Box<Expressible>;

impl ToExpressible for BoxedExpressible {
    fn to_boxed(self) -> BoxedExpressible {
        self
    }
}