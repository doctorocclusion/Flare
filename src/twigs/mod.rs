pub mod lit;

use aster::AstBuilder;

pub struct CodePaver {

}

pub trait Attachable<T> {
    fn attach(&self, pave: &mut CodePaver) -> T;
}

pub trait ToAttachable<T: Attachable<R>, R> {
    fn to_attachable(self) -> T;
}

pub trait Expressible {
    fn ast_expr(&self, build: &AstBuilder) -> ::PExpr;
}

pub trait ToExpressible<T: Expressible> {
    fn to_expressible(self) -> T;
}