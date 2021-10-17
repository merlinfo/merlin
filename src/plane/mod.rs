use std::{
	collections::HashMap,
	array::IntoIter,
	iter::FromIterator,
};

use crate::volume::Volume;
use stack::Stack;

mod plane_commands;
mod parse;
mod input;
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

	running: bool,

	nomens: HashMap<String, Vec<String>>,
}

impl Plane {
	// create a new plane with no open volumes
 
	pub fn new() -> Plane {
		Plane {
			stack: Stack::new(),
			volumes: Vec::new(),

			current_volume: 0,

			vision: Vision::Atom,

			running: true,

			// built in nomens

			nomens: HashMap::<String, Vec<String>>::from_iter(IntoIter::new([
				(String::from("new"), vec![String::from("\n")]),
				(String::from("tab"), vec![String::from("\t")]),
				(String::from("space"), vec![String::from(" ")]),
				(String::from("blank"), vec![String::from("")]),
			
				// notation to be executing after each line of input is entered 

				(String::from("scribe-nomen"), Vec::new()),
				(String::from("atom-nomen"), Vec::new()),

				// our prompts

				(String::from("atom-prompt"), vec![", ".to_string(), ";pen".to_string(), ";decay".to_string()]),
				(String::from("scribe-prompt"), Vec::new()),
			])),
		}
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
