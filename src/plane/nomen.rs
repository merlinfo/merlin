use std::cmp::PartialEq;

pub struct Nomen<'a> {
	name: &'a str,
	atoms: String,
}

impl<'a> Nomen<'a> {
	pub fn new(n: &'a str, a: String) -> Nomen<'a> {
		Nomen { name: n, atoms: a }
	}

	pub fn matches_command(&self, c: &str) -> Option<&str> {
		if ";".to_owned() + self.name == c {
			return Some(&self.atoms)
		}

		None
	}
}

impl<'a> PartialEq for Nomen<'a> {
	fn eq (&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl <'a> PartialEq<&'a str> for Nomen<'a> {
	fn eq(&self, name: &'a str ) {
		self.name == name
	}
}
