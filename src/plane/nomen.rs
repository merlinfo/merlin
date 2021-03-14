pub struct Nomen<'a> {
	name: &'a str,
	atoms: &'a str,
}

impl<'a> Nomen<'a> {
	pub fn new(n: &'a str, a: &'a str) -> Nomen<'a> {
		Nomen { name: n, atoms: a }
	}

	pub fn matches_command(&self, c: &str) -> Option<&str> {
		if ";".to_owned() + self.name == c {
			return Some(self.atoms)
		}

		None
	}
} 
