use crate::{volume::Volume, nomen::Nomen, stack::Stack};

mod plane_commands;
mod parse;
mod input;

// an enum that represents the various modes of merlin

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
	show_errors: bool,

	running: bool,

	nomens: Vec<Nomen>,
}

impl Plane {
	// create a new plane with no open volumes
 
	pub fn new(se: bool) -> Plane {
		Plane {
			stack: Stack::new(),
			volumes: Vec::new(),

			current_volume: 0,

			vision: Vision::Atom,

			print_result: false,
			show_errors: se,

			running: true,

			nomens: vec![
				Nomen::new(String::from("new"), vec![String::from("\n")]),
				Nomen::new(String::from("tab"), vec![String::from("\t")]),
				Nomen::new(String::from("space"), vec![String::from(" ")]),
				Nomen::new(String::from("blank"), vec![String::from("")]),
			
				Nomen::new(String::from("scribe-notation"), Vec::new()),
			
				Nomen::new(String::from("atom-prompt"), vec![", ".to_string(), ";pen".to_string(), ";decay".to_string()]),
				Nomen::new(String::from("scribe-prompt"), Vec::new()),
			],
		}
	}

	// push a string to the stack

	fn push(&mut self, item: String) {
		self.stack.push(item);
	}

	// get the index of a nomen

	fn get_nomen(&self, name: &str) -> Option<usize> {
		for (i, n) in self.nomens.iter().enumerate() {
			if n == name {
				return Some(i);
			}
		}

		None
	}

	// print an error if they are turned on

	fn error(&self) {
		if self.show_errors {
			eprintln!("?");
		}
	}

	// add a new volume

	fn push_volume(&mut self, v: Volume) {
		self.volumes.push(v);
	
		if self.volumes.len() > 1 {
			self.current_volume += 1;
		}
	}
	
}
