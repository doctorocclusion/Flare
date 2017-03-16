#![cfg_attr(feature = "nightly", feature(rustc_private))]

extern crate aster;

#[cfg(feature = "nightly")]
extern crate syntax;

#[cfg(not(feature = "nightly"))]
extern crate syntex_syntax as syntax;

mod twigs;
pub mod ty;

pub use ty::{Ty, BoxedTy};
pub use twigs::bind::*;
// pub use twigs::lit::{literal};
pub use twigs::{ToExpressible, BoxedExpressible, Expressible, Attachable};
pub use twigs::ops::*;
pub use twigs::lets::*;

type PExpr = syntax::ptr::P<syntax::ast::Expr>;
type PStmt = syntax::ptr::P<syntax::ast::Stmt>;