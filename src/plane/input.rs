use super::Plane;
use std::io::{self, Write};

impl<'a> Plane<'a> {
	pub fn repl(&mut self) {
		let mut input = String::new();

		while self.running {
			print!("{}", self.vision);

			std::io::stdout()
				.flush()
				.expect("merlin: unable to flush stdout");

			io::stdin()
				.read_line(&mut input)
				.expect("merlin: unable to read line");

			self.parse_line(&input);

			input.clear();
		}
	} 
}
