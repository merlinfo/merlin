use crate::util;
use super::{Plane, Vision};
use std::io::{self, Write, BufRead};

// flush stdout, handling errors

macro_rules! flush_stdout {
	() => {
		util::err_msg(io::stdout().flush(), "unable to flush stdout")
	}
}

impl Plane {
	pub fn repl(&mut self) {
		let mut input = String::new();

		while self.running {
			// run our prompt nomen

			match self.vision {
				Vision::Atom   => self.parse_line(";atom-prompt"),
				Vision::Scribe => self.parse_line(";scribe-prompt"),
			};

			flush_stdout!();                                                         // flush stdout, handling any errors
			util::err_msg(io::stdin().read_line(&mut input), "unable to read line"); // read a line of input (handle any errors)

			// parse our line, stripping newlines

			self.parse_line(strip_nl(&input));

			input.clear();
		}
	}

	pub fn parse_stdin(&mut self) {
		for line in io::stdin().lock().lines() {
			self.parse_line(strip_nl(&line
				.expect(&format!("{} {}", util::ERROR_PREFIX, "can't read stdin"))));
			
			flush_stdout!();

			if !self.running {
				break;
			}
		}
	}
}

// strip newlines

fn strip_nl(input: &str) -> &str {
	input.strip_suffix("\r\n")
		.or_else(|| input.strip_suffix("\n"))
		.unwrap_or(&input)
}
