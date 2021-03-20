use crate::volume::Volume;
use std::fmt;

mod commands;
mod parse;
mod input;
mod nomen;

// an enum that represents the various modes of merlin

pub enum Vision {
	Atom,
	Scribe,
}

// display the vision

impl fmt::Display for Vision {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Vision::Atom => write!(f, "| "),
			Vision::Scribe => write!(f, ""),
		}
	}
}

// a structure representing our enviroment or "plane"

pub struct Plane<'a> {
	stack: Vec<String>,

	volumes: Vec<Volume<'a>>,
	current_volume: usize,

	highest_buff_index: usize, // detirmines how we should label new buffers

	vision: Vision, // current vision
	print_result: bool,
	show_errors: bool,

	running: bool,

	nomens: Vec<nomen::Nomen<'a>>,
}

impl<'a> Plane<'a> {
	// create a new plane with no open volumes
 
	pub fn new() -> Plane<'a> {
		Plane {
			stack: Vec::new(),
			volumes: Vec::new(),

			current_volume: 0,
			highest_buff_index: 0,

			vision: Vision::Atom,

			print_result: false,
			show_errors: true,

			running: true,

			nomens: Vec::new(),
		}
	}

	// push a string to the stack

	pub fn push(&mut self, item: String) {
		self.stack.push(item);
	}

	/*pub fn expand_nomen(&self, nomen: &str) -> Option<&str> {
		for n in self.nomens
	}*/
}
