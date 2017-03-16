use ::{ToExpressible, Expressible};

pub trait Copiable<T: Expressible> {
    fn copied(&self) -> T;
}

#[derive(Debug)]
pub struct OuterIdentTwig {
    name: String,
}

impl Copiable<OuterIdentTwig> for OuterIdentTwig {
    fn copied(&self) -> OuterIdentTwig {
        OuterIdentTwig {
            name: self.name.clone(),
        }
    }
}

impl Expressible for OuterIdentTwig {

}

pub fn existing_ident(name: &str) -> OuterIdentTwig {
    OuterIdentTwig {
        name: name.to_owned(),
    }
}

#[derive(Debug)]
pub struct VariableTwig {
    id: u64,
    force_mut: Option<bool>,
    name: Option<String>,
    mut_needed: bool,
}

impl VariableTwig {
    pub fn named(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn set_mut(&mut self, mutable: bool) -> &mut Self {
        self.force_mut = Some(mutable);
        self
    }
}

impl Copiable<CopiedTwig> for VariableTwig {
    fn copied(&self) -> CopiedTwig {
        CopiedTwig {
            to: self.id,
        }
    }
}

impl ::Expressible for VariableTwig {

}

#[derive(Clone, Copy, Debug)]
pub struct CopiedTwig {
    to: u64,
}

impl Copiable<CopiedTwig> for CopiedTwig {
    fn copied(&self) -> CopiedTwig {
        CopiedTwig {
            to: self.to,
        }
    }
}

impl Expressible for CopiedTwig {
    
}

pub struct ArgumentTwig {
    var: VariableTwig,
    pos: u32,
    ty: ::BoxedTy,
}

impl Copiable<CopiedTwig> for ArgumentTwig {
    fn copied(&self) -> CopiedTwig {
        CopiedTwig {
            to: self.var.id,
        }
    }
}

impl Expressible for ArgumentTwig {
    
}