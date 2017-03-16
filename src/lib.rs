//! # Flare

//! Flare is a high-level Rust code generation library for compiler plugins
//! and things. Its goal is to allow complex expressions, functions, and types
//! to be implemented easily and reliably with minimal overhead on the
//! programmer.
//!
//! Flare is based around the concept of "twigs". These are abstract, typed
//! representations of all actions Rust can perform. For example: accessing a
//! variable or argument, performing arthritic, branching, borrowing, etc.
//! Smaller twigs can also be composited together to describe more and more
//! advanced expressions.
//!
//! Twigs are sort of like mini-trees, each one relies on a sequence of 
//! sub-computations (e.g. `1 + 2 * 3` first takes the value of `2 * 3` and then
//! adds `1` to it). The twig as a whole is not initially a part of any
//! function or code block, it is simply the representation of something Rust
//! could perform at some point.
//!
//! Once a useful twig is constructed, it can be attached to a line of
//! execution, providing context and relative timing (i.e. before vs after
//! some other twig). Think of the branches of a tree. Small "twigs" being
//! attached to a larger branch. This provides an abstract representation of
//! the sequence of execution, one that Flare can turn into AST or textual
//! Rust code.
//! 
//! # Hello World example
//! 
//! ```
//! let func = flare::function().named("main"); // create a branch that twigs can be attached to
//! let a_twig = flare::println("{}").arg("Hello, world!"); // flare has no state, this twig could be created at any time
//! func.attach(a_twig); // the main function should execute our created statement
//! func.gen_ast()... // create the Rust syntax
//! ```
//! 
//! The above flare stuff generates
//! 
//! ```
//! fn main() {
//!     println!("{}", "Hello, world!");
//! }
//! ```
//! 
//! # For loop example
//! 
//! ```
//! let func = flare::function().named("example_loop"); // create a branch that twigs can be attached to
//! let count = func.arg(flare::ty::u32); // the function should take a u32 argument, create a twig that accesses the value of that argument
//! let (lop, index) = flare::for_(flare::range(Some(0u32), Some(count))); // loop branch
//! lop.attach(flare::println("{}").arg(index)); // print out index
//! func.attach(lop); // the loop should run in the function
//! func.attach(flare::println("End.")); // print end
//! func.gen_ast()... // create the Rust syntax
//! ```
//! 
//! The above flare stuff generates
//! 
//! ```
//! fn example_loop(randomargname: u32) {
//!     for randomvarname in 0..randomargname {
//!         println!("{}", randomvarname);
//!     }
//!     println!("End.");
//! }
//! ```

#![cfg_attr(feature = "nightly", feature(rustc_private))]

extern crate aster;

#[cfg(feature = "nightly")]
extern crate syntax;

#[cfg(not(feature = "nightly"))]
extern crate syntex_syntax as syntax;

mod twigs;
pub mod ty;

pub use ty::{Ty};
pub use twigs::bind::*;
// pub use twigs::lit::{literal};
pub use twigs::{BoxedExpressible, Expressible, Attachable};
pub use twigs::ops::*;
pub use twigs::stmt::*;

type PExpr = syntax::ptr::P<syntax::ast::Expr>;
type PStmt = syntax::ptr::P<syntax::ast::Stmt>;