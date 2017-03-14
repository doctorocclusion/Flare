#![cfg_attr(feature = "nightly", feature(rustc_private))]

extern crate aster;

#[cfg(feature = "nightly")]
extern crate syntax;

#[cfg(not(feature = "nightly"))]
extern crate syntex_syntax as syntax;

mod twigs;
mod ty;

type PExpr = syntax::ptr::P<syntax::ast::Expr>;
type PStmt = syntax::ptr::P<syntax::ast::Stmt>;

pub use twigs::lit::literal;
pub use twigs::{Expressible, Attachable};