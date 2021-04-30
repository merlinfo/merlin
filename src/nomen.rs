use std::cmp::PartialEq;

pub struct Nomen {
	name: String,
	atoms: Vec<String>,
}

impl Nomen {
	pub fn new(n: String, a: Vec<String>) -> Nomen {
		Nomen { name: n, atoms: a }
	}

	pub fn expand(&self) -> &Vec<String> {
		&self.atoms
	}
}

impl<'a> PartialEq<str> for Nomen {
	fn eq(&self, name: &str) -> bool {
		self.name == name
	}
}
