use ::{Attachable, Expressible, BoxedExpressible};

pub struct ForTwig {
    iter: BoxedExpressible,
}

impl ForTwig {
    pub fn attach<R, A: Attachable<After = R>>(attch: A) -> R {
        unimplemented!();
    }
}

pub fn for_<I: Expressible + 'static>(iter: I) -> ForTwig {
    ForTwig {
        iter : iter.to_boxed()
    }
}