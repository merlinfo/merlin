// commands relating to the plane structure 

use super::Plane;
use std::{fs::File, io::{BufRead, BufReader}};
use crate::{volume::Volume, commands::MerlinError, nomen::Nomen};

impl Plane {
	// list all of the open volumes

	pub fn biblio(&self) {
		for (i, v) in self.volumes.iter().enumerate() {
			if i == self.current_volume {
				print!("> ");
			} else {
				print!("  ");
			}

			println!("{}", v);
		}
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

	// create a new nomen

	pub fn nomen(&mut self, atoms: Vec<String>, name: String) {
		if let Some(i) = self.get_nomen(&name) {
			self.nomens.remove(i);
		}

		self.nomens.push(Nomen::new(name, atoms))
	}

	// read a file an parse its contents 

	pub fn spellbook(&mut self, file_path: &str) -> Result<(), MerlinError> {
		match File::open(file_path) {
			Ok(file) => {
				let reader = BufReader::new(file);

				for line in reader.lines() {
					self.parse_line(&line.or(Err(MerlinError::ReadFailed))?);
				}

				Ok(())
			},
			Err(_)   => Err(MerlinError::ReadFailed),

		}
	}
}
