// commands relating to the plane structure 

use super::Plane;
use crate::{volume::Volume, commands::MerlinError};

impl<'a> Plane<'a> {
	// list all of the open volumes

	pub fn biblio(&self) -> String {
		let mut output = String::from("");

		for (i, v) in self.volumes.iter().enumerate() {
			if i == self.current_volume {
				output.push_str("> ");
			}

			output.push_str(&format!("{}\n", v));
		}

		output.pop();
		output
	}

	// insert a buffer created from text

	pub fn genesis(&mut self, text: &str) {
		self.highest_buff_index += 1;
		self.volumes.push(Volume::from_text(self.highest_buff_index, text));
	
		if self.volumes.len() > 1 {
			self.current_volume += 1;
		}
	}

	// close a file / buffer

	pub fn shelve(&mut self, search: &str) -> Result<(), MerlinError> {
		if search.is_empty() { // if there are no arguments, close the currently open volume
			self.volumes.remove(self.current_volume);
		} else {
			if let Some(i) = self.search_volumes(search) {
				self.volumes.remove(i);
			} else {
				return Err(MerlinError::UnmatchedSearch);
			}
		}

		// if the current volume index is out of bounds, shift it to the last volume

		let len = self.volumes.len();

		if self.current_volume >= len && self.current_volume > 0 {
			self.current_volume = len - 1;
		}

		Ok(())
	}

	// brings a volume into focus

	pub fn focus(&mut self, search: &str) -> Result<(), MerlinError> {
		match self.search_volumes(search) {
			Some(i) => self.current_volume = i,
			None    => return Err(MerlinError::UnmatchedSearch),
		}

		Ok(())
	}
}
