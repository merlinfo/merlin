use std::cmp::PartialEq;

pub struct Nomen {
	name: String,
	atoms: String,
}

impl Nomen {
	pub fn new(n: String, a: String) -> Nomen {
		Nomen { name: n, atoms: a }
	}

	pub fn expand(&self) -> &str {
		self.atoms.as_str()
	}
}

impl<'a> PartialEq<str> for Nomen {
	fn eq(&self, name: &str) -> bool {
		self.name == name
	}
}
