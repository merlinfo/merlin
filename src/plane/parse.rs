use super::{Plane, Vision};
use std::str::FromStr;
use crate::commands::{commands, MerlinError, Command};

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
			if self.print_result {
				print!("{}", t);
			} else {
				self.push(t);
			}
		}

		Ok(())
	}

	// parse a command, returning a command and the required data.

	fn parse_command(&mut self, name: &str) -> Result<(Command, Vec<String>), MerlinError> {
		let command = Command::from_str(name)?;
		let needed = command.get_needed(self.stack.len())?;

		Ok((command, self.stack.split_off(self.stack.len() - needed)))
	}

	// run a single command with plain text arguments

	fn run_command(&mut self, command: Command, mut data: Vec<String>) -> Result<Option<String>, MerlinError> {
		let oksome  = |t: String| Ok(Some(t));

		match command { // check what command is being used
			Command::Genesis                           => if data.len() > 0 { self.genesis(data.remove(0)); } else { self.genesis(String::new()); },
			Command::Biblio                            => self.biblio(),
			Command::Incant                            => return oksome(commands::incant(&data[0])?),
			Command::Infuse                            => return oksome(commands::infuse(&data[0], &data[1])?),
			Command::Molecule                          => self.molecule(),
			Command::Pen                               => self.pen(),
			Command::Orbit                             => self.orbit()?,
			Command::Decay                             => self.decay(),
			Command::Destroy                           => self.destroy(),
			Command::Tether                            => return oksome(commands::tether(&data[..data.len()-1], &data.last().unwrap())),
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
						Command::Spot     => return oksome(cvol.spot().to_string()),
						Command::Span     => return oksome(cvol.span().to_string()),
						Command::Pin      => return oksome(cvol.pin().to_string()),
						Command::Columns  => return oksome(cvol.columns().to_string()),
						Command::Traverse => cvol.traverse(parse_pos::<isize>(&data[0])?),
						Command::Shift    => cvol.shift(parse_pos::<isize>(&data[0])?),
						Command::Appear   => cvol.appear(parse_pos::<usize>(&data[0])?),
						Command::Infix    => cvol.infix(parse_pos::<usize>(&data[0])?),
						Command::Peer     => return oksome(cvol.peer(parse_pos::<usize>(&data[0])?, parse_pos::<usize>(&data[1])?)?),
						Command::Peek     => return oksome(cvol.peek(parse_pos::<usize>(&data[0])?)?),
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
