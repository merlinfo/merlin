use crate::{volume::Volume, error::MerlinError};
use stack::Stack;
use nomen::Nomen;

mod plane_commands;
mod parse;
mod input;
mod nomen;
mod stack;

// an enum that represents the various modes of merlin

#[derive(Copy, Clone)]
enum Vision {
	Atom,
	Scribe,
}

// a structure representing our enviroment or "plane"

pub struct Plane {
	stack: Stack,

	volumes: Vec<Volume>,
	current_volume: usize,

	vision: Vision, // current vision
	print_result: bool,

	running: bool,

	nomens: Vec<Nomen>,
}

impl Plane {
	// create a new plane with no open volumes
 
	pub fn new() -> Plane {
		Plane {
			stack: Stack::new(),
			volumes: Vec::new(),

			current_volume: 0,

			vision: Vision::Atom,

			print_result: false,

			running: true,

			// built in nomens

			nomens: vec![
				Nomen::new(String::from("new"), vec![String::from("\n")]),
				Nomen::new(String::from("tab"), vec![String::from("\t")]),
				Nomen::new(String::from("space"), vec![String::from(" ")]),
				Nomen::new(String::from("blank"), vec![String::from("")]),
			
				// notation to be executing after each line of input is entered 

				Nomen::new(String::from("scribe-nomen"), Vec::new()),
				Nomen::new(String::from("atom-nomen"), Vec::new()),

				// our prompts

				Nomen::new(String::from("atom-prompt"), vec![", ".to_string(), ";pen".to_string(), ";decay".to_string()]),
				Nomen::new(String::from("scribe-prompt"), Vec::new()),
			],
		}
	}

	// get the index of a nomen

	fn get_nomen(&self, name: &str) -> Option<usize> {
		self.nomens.iter().position(|n| n == name)
	}

	// get_nomen but it returns an error if it can't find what it's looking for

	fn get_nomen_err(&self, name: &str) -> Result<usize, MerlinError> {
		 self.get_nomen(name).ok_or(MerlinError::UnknownNomen)
	}

	// add a new volume

	fn push_volume(&mut self, v: Volume) {
		self.volumes.push(v);
	
		// only increment when there are more than one valumes open 

		if self.volumes.len() > 1 {
			self.current_volume += 1;
		}
	}
	
}
