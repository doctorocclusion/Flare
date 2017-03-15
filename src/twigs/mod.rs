pub mod lit;
pub mod bind;
pub mod ops;

use aster::AstBuilder;
use std::ops::*;

pub struct CodePaver {

}

pub trait Attachable<T> {
    fn attach(&self, pave: &mut CodePaver) -> T;
}

pub trait ToAttachable<T: Attachable<R>, R> {
    fn to_attachable(self) -> T;
}

pub trait Expressible {
}

pub trait ToExpressible<T: Expressible>: ToExpressibleBox {
    fn to_expressible(self) -> T;
}

pub trait ToExpressibleBox {
    fn to_expressible_box(self) -> ::Exprable;
}

impl<T: Expressible> ToExpressible<T> for T {
    fn to_expressible(self) -> T {
        self
    }
}

impl<T: Expressible> ToExpressibleBox for T {
    fn to_expressible_box(self) -> Box<T> {
        Box::new(self)
    }
}