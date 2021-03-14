use super::{Plane, Vision};
use std::str::FromStr;
use crate::{volume::selection::Selection, commands::{commands, MerlinError, Command}};

enum Text {
	Value(String),
	Display(String),
}

impl<'a> Plane<'a> {
	// parse a line based on what mode the user is in

	pub fn parse_line(&mut self, line: &str) {
		match self.vision {
			Vision::Atom   => self.parse_line_atom(line),
			Vision::Scribe => self.parse_line_scribe(line),
		}
	}

	// scribe mode

	fn parse_line_scribe(&mut self, line: &str) {
		self.atom_push(line);
	}

	// atom mode

	fn parse_line_atom(&mut self, line: &str) {
		for atom in line.split_whitespace() {
			self.atom_push(atom);
		}
	}

	// parse and atom and push it to the stack

	fn atom_push(&mut self, atom: &str) {
		if let Some(a) = self.parse_atom(atom) {
			self.push(a);
		}
	}

	// parse a single element, an "atom"

	fn parse_atom(&mut self, atom: &str) -> Option<String> {
		if let Some(stripped) = atom.strip_prefix(";") { // the atom is a command
			if let Err(_) = self.parse_and_run(strip_nl(stripped)) { // the command wasn't successful
				if self.show_errors {
					eprintln!("?");
				}
			}
		} else {
			let mut out = atom.to_string();

			if let Some(stripped) = atom.strip_prefix("\\") {
				out = stripped.to_string();
			}

			return Some(out);
		}

		None
	}

	// parse and run!

	fn parse_and_run(&mut self, name: &str) -> Result<(), MerlinError> {
		let (command, data) = self.parse_command(name)?;

		if let Some(t) = self.run_command(command, &data)? {
			match t {
				Text::Value(s)   => {
					if self.print_result {
						println!("{}", s);
					} else {
						self.push(s);
					}
				}
				Text::Display(s) => println!("{}", s),
			}
		}

		Ok(())
	}

	// parse a command, returning a command and the required data.

	fn parse_command(&mut self, name: &str) -> Result<(Command, Vec<String>), MerlinError> {
		let command = Command::from_str(name)?;
		let needed = command.get_needed(self.stack.len())?;

		let mut data = Vec::new();

		for _ in (self.stack.len() - needed)..(self.stack.len()) {
			data.insert(0, self.stack.pop().unwrap());
		}

		Ok((command, data))
	}

	// run a single command with plain text arguments

	fn run_command(&mut self, command: Command, data: &Vec<String>) -> Result<Option<Text>, MerlinError> {
		let oksome  = |t: Text  | Ok(Some(t));
		let oksval  = |s: String| oksome(Text::Value(s));

		match command { // check what command is being used
			Command::Genesis                           => self.genesis(&data[0]),
			Command::Biblio if self.volumes.len() > 0  => return oksome(Text::Display(self.biblio())),
			Command::Incant                            => return oksval(commands::incant(data)?),
			Command::Infuse                            => return oksval(commands::infuse(&data[0], &data[1..])?),
			Command::Molecule                          => if let Some(s) = self.molecule() { return oksome(Text::Display(s));             },
			Command::Pen                               => if let Some(s) = self.pen()      { return oksome(Text::Display(s.to_string())); },
			Command::Orbit                             => self.orbit()?,
			Command::Decay                             => self.decay(),
			Command::Destroy                           => self.destroy(),
			Command::Tether                            => return oksval(commands::tether(&data[..data.len()-1], &data.last().unwrap())),
			Command::Newline                           => self.newline(),
			Command::Space                             => self.space(),
			Command::Tab                               => self.tab(),
			Command::Atom                              => self.vision = Vision::Atom,
			Command::Scribe                            => self.vision = Vision::Scribe,
			Command::Error                             => self.show_errors = !self.show_errors,
			Command::Mirror                            => self.print_result = !self.print_result,
			Command::Adieu                             => self.running = false,
			_                                          => { // the following commands require buffers to be open
				if self.volumes.len() > 0 { // buffers / files are open
					let cvol = &mut self.volumes[self.current_volume]; // current volume

					match command {
						Command::Shelve    => self.shelve(parse_pos::<usize>(&data[0])?)?,
						Command::Focus     => self.focus(parse_pos::<usize>(&data[0])?)?,
						Command::Spot      => return oksval(cvol.spot().to_string()),
						Command::Span      => return oksval(cvol.span().to_string()),
						Command::Traverse  => cvol.traverse(parse_pos::<isize>(&data[0])?),
						Command::Appear    => cvol.appear(parse_pos::<usize>(&data[0])?)?,
						Command::Peer      => return oksval(cvol.peer(Selection::new(data, cvol.len())?)),
						Command::Inscribe  => cvol.inscribe(&data[0]),
						Command::Trample   => cvol.trample(&data[0]),
						Command::Transmute => cvol.transmute(Selection::new(&data[1..], cvol.len())?, &data[0]),
						Command::Shave     => cvol.shave(parse_pos::<isize>(&data[0])?),
						_                  => (),
					}
				} else {
					return Err(MerlinError::NoVolumes)
				}
			}
		}

		Ok(None)
	}
}

// convert a str (containing positional information) into a T, converting the error into type MerlinError

fn parse_pos<T: std::str::FromStr>(s: &str) -> Result<T, MerlinError> {
	s.parse::<T>().or(Err(MerlinError::InvalidSyntax))
}

// remove first trailing newline

fn strip_nl(input: &str) -> &str {
	input.strip_suffix("\r\n")
		.or(input.strip_suffix("\n"))
		.unwrap_or(&input)
}
