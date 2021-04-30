// commands relating to the plane structure 

use super::Plane;
use crate::{volume::Volume, commands::MerlinError, nomen::Nomen};

impl Plane {
	// list all of the open volumes

	pub fn biblio(&self) -> String {
		let mut output = String::from("");

		for (i, v) in self.volumes.iter().enumerate() {
			if i == self.current_volume {
				output.push_str("> ");
			}

			output.push_str(&format!("{}\n", v));
		}

		output
	}

	// insert a buffer created from text

	pub fn genesis(&mut self, text: String) {
		self.push_volume(Volume::from_text(self.highest_buff_index, text));
	}

	// open a new file

	pub fn summon(&mut self, path: String) -> Result<(), MerlinError> {
		Ok(self.push_volume(Volume::from_file(path)?))
	}

	// close a file / buffer

	pub fn shelve(&mut self, index: usize) -> Result<(), MerlinError> {
		if index <= self.volumes.len() && index > 0 {
			self.volumes.remove(index - 1);
		} else {
			return Err(MerlinError::OutOfBounds);
		}

		// if the current volume index is out of bounds, shift it to the last volume

		let len = self.volumes.len();

		if self.current_volume >= len && self.current_volume > 0 {
			self.current_volume = len - 1;
		}

		Ok(())
	}

	// brings a volume into focus

	pub fn focus(&mut self, index: usize) -> Result<(), MerlinError> {
		if index <= self.volumes.len() && index > 0 {
			self.current_volume = index - 1;
		} else {
			return Err(MerlinError::OutOfBounds);
		}

		Ok(())
	}

	// display a list of the items in the stack

	pub fn molecule(&mut self) -> Option<String> {
		let mut output = String::new();

		for item in &self.stack {
			output.push_str(&(item.to_owned() + " "));
		}

		if output.is_empty() {
			return None;
		}

		output.push_str("\n");

		Some(output)
	}

	// print the last atom in the stack

	pub fn pen(&mut self) -> Option<&str> {
		match self.stack.last() {
			Some(s) => Some(s.as_str()),
			None    => None,
		}
	}

	// swap the last two items in the stack

	pub fn orbit(&mut self) -> Result<(), MerlinError> {
		if self.stack.len() < 2 {
			return Err(MerlinError::OutOfBounds);
		}

		let new_last = self.stack.remove(self.stack.len() - 2);
		self.push(new_last);

		Ok(())
	}

	// remove the last item from the stack

	pub fn decay(&mut self) {
		self.stack.pop();
	}

	// clear the stack

	pub fn destroy(&mut self) {
		self.stack.clear();
	}
	
	// create a new nomen

	pub fn nomen(&mut self, atoms: Vec<String>, name: String) {
		if let Some(i) = self.get_nomen(&name) {
			self.nomens.remove(i);
		}

		self.nomens.push(Nomen::new(name, atoms))
	}

	// read a file an parse its contents 

	//spell
}
