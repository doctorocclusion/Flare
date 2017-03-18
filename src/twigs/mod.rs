pub mod lit;
pub mod bind;
pub mod ops;
pub mod stmt;
pub mod loops;

use syntax::ast;

pub trait Attachable {
    type After;
}

impl Attachable for ast::Stmt {
    type After = ();

    // TODO
}

pub trait Expressible {
    fn to_boxed(self) -> BoxedExpressible where Self: Sized + 'static {
        Box::new(self) as BoxedExpressible
    }
}

impl Expressible for ast::Expr {
    // TODO
}

pub type BoxedExpressible = Box<Expressible>;
