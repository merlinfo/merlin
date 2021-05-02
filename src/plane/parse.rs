use super::{Plane, Vision};
use std::str::FromStr;
use crate::{volume::selection::Selection, commands::{commands, MerlinError, Command}};

enum Text {
	Value(String),
	Display(String),
}

impl Plane {
	// parse a line based on what mode the user is in

	pub fn parse_line(&mut self, line: &str) {
		match &self.vision {
			Vision::Atom    => self.parse_line_atom(line),
			Vision::Scribe  => self.parse_line_scribe(line),
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
			match self.parse_command(stripped) {
				Ok((command, data)) => if self.run_and_handle(command, data).is_err() { self.error() },
				Err(_)              => {
					match self.get_nomen(stripped) {
						Some(i) => {
							let nomen = self.nomens.remove(i);
							
							for atom in nomen.expand() {
								self.atom_push(atom);
							}
							
							self.nomens.push(nomen);
						} 
						None    => self.error(),
					}
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

	// run and handle!!

	fn run_and_handle(&mut self, command: Command, data: Vec<String>) -> Result<(), MerlinError> {
		if let Some(t) = self.run_command(command, data)? {
			match t {
				Text::Value(s)   => {
					if self.print_result {
						print!("{}", s);
					} else {
						self.push(s);
					}
				}
				Text::Display(s) => print!("{}", s),
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

	fn run_command(&mut self, command: Command, mut data: Vec<String>) -> Result<Option<Text>, MerlinError> {
		let oksome  = |t: Text  | Ok(Some(t));
		let oksval  = |s: String| oksome(Text::Value(s));

		match command { // check what command is being used
			Command::Genesis                           => if data.len() > 0 { self.genesis(data.remove(0)); } else { self.genesis(String::new()); },
			Command::Biblio if self.volumes.len() > 0  => return oksome(Text::Display(self.biblio())),
			Command::Incant                            => return oksval(commands::incant(&data[0])?),
			Command::Infuse                            => return oksval(commands::infuse(&data[0], &data[1])?),
			Command::Molecule                          => if let Some(s) = self.molecule() { return oksome(Text::Display(s));             },
			Command::Pen                               => if let Some(s) = self.pen()      { return oksome(Text::Display(s.to_string())); },
			Command::Orbit                             => self.orbit()?,
			Command::Decay                             => self.decay(),
			Command::Destroy                           => self.destroy(),
			Command::Tether                            => return oksval(commands::tether(&data[..data.len()-1], &data.last().unwrap())),
			Command::Newline                           => self.push(String::from("\n")),
			Command::Space                             => self.push(String::from(" ")),
			Command::Tab                               => self.push(String::from("\t")),
			Command::Blank                             => self.push(String::from("")),
			Command::Atom                              => self.vision = Vision::Atom,
			Command::Scribe                            => self.vision = Vision::Scribe,
			Command::Mirror                            => self.print_result = !self.print_result,
			Command::Adieu                             => self.running = false,
			Command::Nomen                             => {
				let n = data.pop().unwrap();
				self.nomen(data, n);
				}
			Command::Summon                            => self.summon(data.remove(0))?,
			Command::Spellbook                         => self.spellbook(&data[0])?,
			_                                          => { // the following commands require buffers to be open
				if self.volumes.len() > 0 { // buffers / files are open
					let cvol = &mut self.volumes[self.current_volume]; // current volume

					match command {
						Command::Shelve   => self.shelve(parse_pos::<usize>(&data[0])?)?,
						Command::Focus    => self.focus(parse_pos::<usize>(&data[0])?)?,
						Command::Spot     => return oksval(cvol.spot().to_string()),
						Command::Span     => return oksval(cvol.span().to_string()),
						Command::Pin      => return oksval(cvol.pin().to_string()),
						Command::Columns  => return oksval(cvol.columns().to_string()),
						Command::Traverse => cvol.traverse(parse_pos::<isize>(&data[0])?),
						Command::Shift    => cvol.shift(parse_pos::<isize>(&data[0])?),
						Command::Appear   => cvol.appear(parse_pos::<usize>(&data[0])?)?,
						Command::Peer     => return oksval(cvol.peer(Selection::new(&data, cvol.len())?)),
						Command::Dub      => cvol.dub(data.remove(0))?,
						Command::Carve    => cvol.carve()?,
						_ => {
							cvol.set_written(false);

							match command {
								Command::Inscribe => cvol.inscribe(data.remove(0)),
								Command::Trample  => cvol.trample(data.remove(0)),
								Command::Burn     => cvol.burn(),
								Command::Shave    => cvol.shave(parse_pos::<usize>(&data[0])?),
								_                 => (),
							}
						}
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
