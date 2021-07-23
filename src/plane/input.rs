use super::{Plane, Vision};
use std::io::{self, Write};

impl Plane {
	pub fn repl(&mut self) {
		let mut input = String::new();

		while self.running {
			// run our prompt nomen

			match self.vision {
				Vision::Atom   => self.parse_atom(";atom-prompt"),
				Vision::Scribe => self.parse_atom(";scribe-prompt"),
			};

			std::io::stdout()
				.flush()
				.expect("merlin: unable to flush stdout");

			io::stdin()
				.read_line(&mut input)
				.expect("merlin: unable to read line");
			
			// parse our line, stripping newlines

			self.parse_line(strip_nl(&input));

			input.clear();
		}
	} 
}

// strip newlines

fn strip_nl(input: &str) -> &str {
	input.strip_suffix("\r\n")
		.or(input.strip_suffix("\n"))
		.unwrap_or(&input)
}
