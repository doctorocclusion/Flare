use ::{Expressible};

// TODO: Change Copyable to apply to any situation where a twig can be duplicated without obvious consequence
/// While many types in rust can only be moved or borrowed, rust also has Copy
/// types. Variables containing these types can be passed directly while
/// allowing the variable to be used again. Twigs containing named data
/// implement this trait so that the associated variable/arg can be used
/// multiple times without borrowing or cloning.
///
/// # Example
///
/// ```
/// let my_variable = my_code.attach(flare::let_from(5)); // initialize a new variable using a let statement
/// let use_again = my_variable.copied();
/// let my_tuple = flare::tuple()
///     .with(my_variable)                // variable is used once
///     .with(flare::add(use_again, 5));  // copy of variable is used again
/// my_code.attach(flare::return_(my_tuple)); // return the tuple
/// ```
///
/// Will generate:
///
/// ```
/// let randomname = 5i32;
/// return (randomname, randomname + 5i32);
/// ```
pub trait Copyable<T: Expressible> {
    /// Create a twig that generates the same identifier as self
    fn copied(&self) -> T;
}

/// An twig used to access previously defined variables by name
#[derive(Debug)]
pub struct ExistingVarTwig {
    name: String,
}

impl Copyable<ExistingVarTwig> for ExistingVarTwig {
    fn copied(&self) -> ExistingVarTwig {
        ExistingVarTwig {
            name: self.name.clone(),
        }
    }
}

impl Expressible for ExistingVarTwig {

}

/// Create twig that accesses a previously defined variable by name
pub fn existing_var(name: &str) -> ExistingVarTwig {
    ExistingVarTwig {
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

/// A twig used to access, borrow, modify, and move variables
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

impl Copyable<CopiedTwig> for VariableTwig {
    fn copied(&self) -> CopiedTwig {
        CopiedTwig {
            to: self.id,
        }
    }
}

impl ::Expressible for VariableTwig {

}

/// A twig used to move variables by copy
#[derive(Clone, Copy, Debug)]
pub struct CopiedTwig {
    to: u64,
}

impl Copyable<CopiedTwig> for CopiedTwig {
    fn copied(&self) -> CopiedTwig {
        CopiedTwig {
            to: self.to,
        }
    }
}

impl Expressible for CopiedTwig {
    
}

/// A twig used to access, borrow, modify, and move function arguments
pub struct ArgumentTwig {
    var: VariableTwig,
    pos: u32,
    ty: ::ty::BoxedTy,
}

impl Copyable<CopiedTwig> for ArgumentTwig {
    fn copied(&self) -> CopiedTwig {
        CopiedTwig {
            to: self.var.id,
        }
    }
}

impl Expressible for ArgumentTwig {
    
}