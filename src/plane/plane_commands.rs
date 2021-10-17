// commands relating to the plane structure 

extern crate shellexpand;

use std::{fs::File, io::{BufRead, BufReader}};
use crate::{volume::Volume, error::MerlinError};
use super::Plane;

impl Plane {
	// return the name of a volume

	pub fn spine(&self, index: usize) -> Result<String, MerlinError> {
		match self.volumes.get(index - 1) {
			Some(v) => Ok(format!("{}", v)),
			None    => Err(MerlinError::OutOfBounds),
		}
	}

	// insert a buffer created from text

	pub fn genesis(&mut self, text: &str) {
		self.push_volume(Volume::from_text(text));
	}

	// open a new file

	pub fn summon(&mut self, path: &str) -> Result<(), MerlinError> {
		Ok(self.push_volume(Volume::from_file(&shellexpand::tilde(path))?))
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

	// "clear" a nomen

	pub fn disenchant(&mut self, name: &str) -> Result<(), MerlinError> {
		// empty the vector of atoms for a certain nomen, reutrn an error if we can't find it

		Ok(self.nomens.get_mut(name)
			.ok_or(MerlinError::UnknownNomen)?
			.clear())
	}

	// remove a nomen

	pub fn smash(&mut self, name: &str) -> Result<(), MerlinError> {
		// remove the nomen from the list if it exists, otherwise return an error

		self.nomens.remove(name)
			.ok_or(MerlinError::UnknownNomen)?;

		Ok(())
	}

	// read a file an parse its contents 

	pub fn spellbook(&mut self, file_path: &str) -> Result<(), MerlinError> {
		match File::open(&*shellexpand::tilde(file_path)) {
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

	// return the index of the current volume, 0 if none are present

	pub fn volume(&self) -> usize {
        	match self.volumes.len() {
	            0 => 0,
	            _ => self.current_volume + 1,
	        }
	}

	// split atom by another atom

	pub fn fray(&mut self, atom: &str, s: &str) {
		for a in atom.split(s) {
			if !a.is_empty() {
				self.stack.push(a.to_string())
			}
		}
	}
}

