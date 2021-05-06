use crate::{volume::Volume, nomen::Nomen, stack::Stack};
use std::fmt;

mod plane_commands;
mod parse;
mod input;

// an enum that represents the various modes of merlin

pub enum Vision {
	Atom,
	Scribe,
}

// display the vision

impl fmt::Display for Vision {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Vision::Atom   => write!(f, ", "),
			Vision::Scribe => write!(f, ""),
		}
	}
}

// a structure representing our enviroment or "plane"

pub struct Plane {
	stack: Stack,

	volumes: Vec<Volume>,
	current_volume: usize,

	highest_buff_index: usize, // detirmines how we should label new buffers

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
			highest_buff_index: 0,

			vision: Vision::Atom,

			print_result: false,
			show_errors: se,

			running: true,

			nomens: vec![
				Nomen::new(String::from("new"), vec![String::from("\n")]),
				Nomen::new(String::from("tab"), vec![String::from("\t")]),
				Nomen::new(String::from("space"), vec![String::from(" ")]),
				Nomen::new(String::from("blank"), vec![String::from("")]),
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
		self.highest_buff_index += 1;
		self.volumes.push(v);
	
		if self.volumes.len() > 1 {
			self.current_volume += 1;
		}
	}
}
