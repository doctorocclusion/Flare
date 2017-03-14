pub trait Twig {

}

pub trait ToTwig<T: Twig> {
	fn to_twig(self) -> T;
}

impl<T: Twig> ToTwig<T> for T {
	fn to_twig(self) -> T {
		self
	}
}

pub struct Binding {
	
}

pub struct LiteralTwig {

}