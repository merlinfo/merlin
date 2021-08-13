use super::{Plane, Vision};
use std::io::{self, Write};

impl Plane {
	pub fn repl(&mut self) {
		let mut input = String::new();

		while self.running {
			// run our prompt nomen

			match self.vision {
				Vision::Atom   => self.parse_line(";atom-prompt"),
				Vision::Scribe => self.parse_line(";scribe-prompt"),
			};

			handle_io_err(io::stdout().flush(), "unable to flush stdout");          // flush stdout, handling any errors
			handle_io_err(io::stdin().read_line(&mut input), "unable to readline"); // read a line of input (handle any errors)

			// parse our line, stripping newlines

			self.parse_line(strip_nl(&input));

			input.clear();
		}
	} 
}

// strip newlines

fn strip_nl(input: &str) -> &str {
	input.strip_suffix("\r\n")
		.or_else(|| input.strip_suffix("\n"))
		.unwrap_or(&input)
}

// print a nice error message on an io error event

fn handle_io_err<T>(res: io::Result<T>, msg: &str) {
	if let Err(_) = res {
		eprintln!("merlin: {}", msg);
	}
}
