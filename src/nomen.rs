use std::cmp::PartialEq;

// nomen structure

pub struct Nomen {
	name: String,
	pub atoms: Vec<String>,
}

// methods

impl Nomen {
	pub fn new(n: String, a: Vec<String>) -> Nomen {
		Nomen { name: n, atoms: a }
	}
}

// compare our nomen to strings

impl<'a> PartialEq<str> for Nomen {
	fn eq(&self, name: &str) -> bool {
		self.name == name
	}
}
