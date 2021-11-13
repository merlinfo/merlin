use super::{Plane, Vision};
use std::str::FromStr;
use crate::commands::{commands, Command};
use crate::error::MerlinError;

const COMMAND_PREFIX: &str = ";";

impl Plane {
	// parse a line based on what mode the user is in

	pub fn parse_line(&mut self, line: &str) {
		match self.vision {
			Vision::Atom    => self.parse_line_atom(line),
			Vision::Scribe  => self.parse_line_scribe(line),
		}
	}

	// scribe mode

	fn parse_line_scribe(&mut self, line: &str) {
		if self.atom_push(line) {
			self.parse_atom(";scribe-nomen"); // we run this nomen after every line entered in scribe mode (excluding command calls)
		}
	}

	// atom mode

	fn parse_line_atom(&mut self, line: &str) {
		for atom in line.split_whitespace() {
			self.atom_push(atom);
		}
		
		if line != ";atom-prompt" { // don't run if we are parsing the prompt
			self.parse_atom(";atom-nomen"); // nomen to run after each line of input is parsed
		}
	}

	// parse and atom and push it to the stack

	fn atom_push(&mut self, atom: &str) -> bool {
		// return true if we are pushing to the stack

		match self.parse_atom(atom) {
			Some(a) => {
				self.stack.push(a);

				true
			}
			None    => false
		}
	}

	// parse a single element, an "atom"

	fn parse_atom(&mut self, atom: &str) -> Option<String> {
		if let Some(stripped) = atom.strip_prefix(COMMAND_PREFIX) { // the atom is a command
			match self.parse_command(stripped) {
				Ok((command, data)) => if let Err(e) = self.run_and_handle(command, data) { eprintln!("{}", e) }, // run and handle the command
				Err(e)              => { // the command isn't valid...
					match self.nomens.remove_entry(stripped) { // check if it is a nomen
						Some((k, v)) => { // add it if it is
							for atom in &v {
								self.atom_push(&atom);
							}

							self.nomens.insert(k, v);
						},
						None    => eprintln!("{}", e),
					}
				}
			}
		} else {
			// add the text to the stack...

			let mut out = atom.to_string();

			// remove leading '\'

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
			self.stack.push(t);
		}

		Ok(())
	}

	// parse a command, returning a command and the required data.

	fn parse_command(&mut self, name: &str) -> Result<(Command, Vec<String>), MerlinError> {
		let command = Command::from_str(name)?;
		let needed = command.get_needed(self.stack.len())?;

		Ok((command, self.stack.grab(needed)))
	}

	// run a single command with plain text arguments

	fn run_command(&mut self, command: Command, mut data: Vec<String>) -> Result<Option<String>, MerlinError> {
		match command { // check what command is being used
			Command::Genesis                           => if data.len() > 0 { self.genesis(&data[0]); } else { self.genesis(""); },
			Command::Spine                             => return Ok(Some(self.spine(parse_pos::<usize>(&data[0])?)?)),
			Command::Incant                            => return Ok(Some(commands::incant(&data[0])?)),
			Command::Decant                            => return Ok(Some(commands::decant(&data[0])?)),
			Command::Infuse                            => return Ok(Some(commands::infuse(&data[0], &data[1])?)),
			Command::Defuse                            => return Ok(Some(commands::defuse(&data[0], &data[1])?)),
			Command::Molecule                          => self.stack.molecule(),
			Command::Pen                               => self.stack.pen(),
			Command::Orbit                             => self.stack.orbit()?,
			Command::Decay                             => self.stack.decay(),
			Command::Destroy                           => self.stack.destroy(),
			Command::Tether                            => return Ok(Some(data[..data.len()-1].join(&data.last().unwrap()))),
			Command::Fray                              => self.fray(&data[data.len()-2], &data.last().unwrap()),
			Command::Atom                              => self.vision = Vision::Atom,
			Command::Scribe                            => self.vision = Vision::Scribe,
			Command::Adieu                             => self.running = false,
			Command::Nomen | Command::Bottle                            => {
					// create a new nomen, popping the name from the data vector

					let n = data.pop().unwrap();
					self.nomens.insert(n, data);
				}
			Command::Disenchant                        => self.disenchant(&data[0])?,
			Command::Smash                             => self.smash(&data[0])?,
			Command::Merlin                            => {
					// make sure we are always *starting* in atom mode, but preserving the original mode
					// for when we finish parsing 

					let old_mode = self.vision;
					self.vision = Vision::Atom;

					// parse each line of the input, respecting the current mode

					for line in data[0].lines() {
						self.parse_line(line);
					}

					self.vision = old_mode; // reset mode
				}
			Command::Summon                            => self.summon(&data[0])?,
			Command::Spellbook                         => self.spellbook(&data[0])?,
			Command::Volume                            => return Ok(Some(self.volume().to_string())),
			Command::Volumes                           => return Ok(Some(self.volumes.len().to_string())),
			Command::Atoms                             => return Ok(Some(self.stack.len().to_string())),
			_                                          => { // the following commands require buffers to be open
				if self.volumes.len() > 0 { // buffers / files are open
					let cvol = &mut self.volumes[self.current_volume]; // current volume

					match command {
						Command::Shelve   => self.shelve(parse_pos::<usize>(&data[0])?)?,
						Command::Focus    => self.focus(parse_pos::<usize>(&data[0])?)?,
						Command::Spot     => return Ok(Some(cvol.spot().to_string())),
						Command::Span     => return Ok(Some(cvol.span().to_string())),
						Command::Pin      => return Ok(Some(cvol.pin().to_string())),
						Command::Columns  => return Ok(Some(cvol.columns().to_string())),
						Command::Traverse => cvol.traverse(parse_pos::<isize>(&data[0])?),
						Command::Shift    => cvol.shift(parse_pos::<isize>(&data[0])?),
						Command::Appear   => cvol.appear(parse_pos::<usize>(&data[0])?),
						Command::Infix    => cvol.infix(parse_pos::<usize>(&data[0])?),
						Command::Peer     => return Ok(Some(cvol.peer(parse_pos::<usize>(&data[0])?,
											parse_pos::<usize>(&data[1])?)?)),
						Command::Dub      => cvol.dub(&data[0])?,
						Command::Carve    => cvol.carve()?,
						Command::Carved   => return Ok(Some(cvol.carved())),
						_ => { // we are modifying the buffer...
							cvol.written = false;

							match command {
								Command::Inscribe => cvol.inscribe(&data[0]),
								Command::Trample  => cvol.trample(&data[0]),
								Command::Burn     => cvol.burn(),
								Command::Shave    => cvol.shave(parse_pos::<usize>(&data[0])?),
								_                 => (),
							}
						}
					}
				} else { // allow index commands to work when no buffers are open
					let n = match command {
						Command::Spot    => 0,
						Command::Span    => 0,
						Command::Pin     => 0,
						Command::Columns => 0,
						_                => 1,
					};

					if n == 0 {
						return Ok(Some("0".to_string()));
					}
					
					return Err(MerlinError::NoVolumes);
				}
			}
		}

		Ok(None)
	}
}

// convert a str (containing positional information) into a T, converting the error into type MerlinError

fn parse_pos<T: std::str::FromStr>(s: &str) -> Result<T, MerlinError> {
	s.trim().parse::<T>().or(Err(MerlinError::InvalidSyntax))
}
