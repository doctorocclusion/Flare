use {Expressible, BoxedExpressible, Attachable};
use super::bind::VariableTwig;

pub struct LetTwig {
    right: Option<BoxedExpressible>,
}

impl Attachable for LetTwig {
    type After = VariableTwig;
}

pub fn let_from<E: Expressible + 'static>(right: E) -> LetTwig {
    LetTwig {
        right: Some(right.to_boxed()),
    }
}

pub fn let_() -> LetTwig {
    LetTwig {
        right: None,
    }
}