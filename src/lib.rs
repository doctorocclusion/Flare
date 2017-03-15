#![cfg_attr(feature = "nightly", feature(rustc_private))]

extern crate aster;

#[cfg(feature = "nightly")]
extern crate syntax;

#[cfg(not(feature = "nightly"))]
extern crate syntex_syntax as syntax;

#[macro_use]
extern crate lazy_static;

mod twigs;
mod ty;

pub use ty::{Ty, BoxedTy};
pub use twigs::bind::*;
pub use twigs::lit::{literal, LiteralTwig};
pub use twigs::{ToExpressible, ToExpressibleBox, Expressible, ToAttachable, Attachable};
pub use twigs::ops::*;

type Exprable = Box<Expressible>;

impl ToExpressibleBox for Exprable {
    fn to_expressible_box(self) -> Exprable {
        self
    }
}

type Attchable<R> = Box<Attachable<R>>;

type PExpr = syntax::ptr::P<syntax::ast::Expr>;
type PStmt = syntax::ptr::P<syntax::ast::Stmt>;