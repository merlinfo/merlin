// commands relating to the volume structure

use std::iter::FromIterator;
use std::{path::Path, fs::File};
use std::io::Write;
use crate::commands::MerlinError;
use super::{Volume, VolumeState, selection::Selection};

impl Volume {
	// return the number of the current line

	pub fn spot(&self) -> usize {
		self.line + 1
	}

	// return the length of the file

	pub fn span(&self) -> usize {
		self.buffer.len()
	}

	// return the cursor location

	pub fn pin(&self) -> usize {
		self.cursor + 1
	}

	// return the length of the line

	pub fn columns(&self) -> usize {
		self.buffer[self.line].chars().count()
	}
	
	// move up or down a line

	pub fn traverse(&mut self, n: isize) {
		self.line = move_respect_bounds(self.line, self.buffer.len(), n);
		self.cursor = 0;
	}

	// move left or right a character

	pub fn shift(&mut self, n: isize) {
		self.cursor = move_respect_bounds(self.cursor, self.columns(), n);
	}

	// move to specific line

	pub fn appear(&mut self, n: usize) -> Result<(), MerlinError> {
		if n <= self.buffer.len() && n >= 1 {
			self.line = n-1;
			self.cursor = 0;
		} else {
			return Err(MerlinError::OutOfBounds)
		}

		Ok(())
	}

	// view a piece of text

	pub fn peer(&self, part: Selection) -> String {
		match part {
			Selection::One(i)    => self.buffer.get(i).unwrap().to_string(),
			Selection::Few(b, e) => self.buffer.as_slice()[b..e].join("\n"),
		}
	}

	// inset some text into the buffer

	pub fn inscribe(&mut self, s: String) {
		let mut lines = s.lines();

		// remove text after the cursor and push the first line to the end of the current line

		let mut chars: Vec<char> = self.buffer[self.line].chars().collect();
		let remainder = String::from_iter(chars.split_off(self.cursor));

		self.buffer[self.line] = String::from_iter(chars);
		self.current().push_str(lines.next().unwrap());

		self.cursor += 1;

		// loop through the remaining lines and intersplice them in the buffer

		for line in lines {
			self.line += 1;
			self.buffer.insert(self.line, line.to_string());
		}

		self.cursor = self.columns()-1;
		self.current().push_str(&remainder);
	}

	// overwrite text

	pub fn trample(&mut self, s: String) {
		for (i, line) in s.lines().enumerate() {
			if i >= self.buffer.len() { // the length of the piece of text excedes the length of the buffer
				self.buffer.push(line.to_string());
			} else {
				self.buffer[self.line+i] = line.to_string();
			}
		}
	}

	// replace a part of the buffer

	pub fn transmute(&mut self, part: Selection, replace: String) {
		match part {
			Selection::One(l)    => self.insert_lines(l, replace),
			Selection::Few(b, e) => {
				// remove the selected lines

				for i in b+1..e {
					self.buffer.remove(i);
				}

				self.insert_lines(b, replace);
			}
		}
	}

	// shave off parts of text from a line

	pub fn shave(&mut self, amount: usize) {
		let to_remove = amount;

		if self.cursor == 0 {
			if self.line > 0 {
				let old_line = self.buffer.remove(line);

				self.line -= 1;
				to_remove -= 1;

				self.cursor = self.columns();
				self.current().push_str(&old_line);
			}
		}

		
	}

	// "dub" a buffer

	pub fn dub(&mut self, f_name: String) -> Result<(), MerlinError> {
		let err = Err(MerlinError::FileAlreadyExists);

		match self.name {
			VolumeState::NoFile(_) => {
				if Path::new(&f_name).exists() {
					return err
				}

				self.name = VolumeState::File(f_name);
			
				Ok(())
			}
			VolumeState::File(_) => err,
		}
	}

	// write out a file

	pub fn carve(&mut self) -> Result<(), MerlinError> {
		match &self.name {
			VolumeState::File(name) => {
				let mut file = File::create(&name).or(Err(MerlinError::CreationFailed))?;
				file.write_all(&(self.buffer.join("\n") + "\n").as_bytes()).or(Err(MerlinError::WriteFailed))?;

				self.set_written(true);
				Ok(())
			}
			VolumeState::NoFile(_)  => Err(MerlinError::BufferNotNamed),
		}
	}

	// remove a line with 1+ other lines

	fn insert_lines(&mut self, index: usize, lines: String) {
		self.buffer.remove(index);

		for line in lines.lines() {
			self.buffer.insert(index + 1, line.to_string());
		}
	}
}

fn move_respect_bounds(curr: usize, len: usize, n: isize) -> usize {
	let modified = (curr as isize) + n;

	if modified < len as isize && modified >= 0 {
		return modified as usize
	} else if n < 0 {
		return 0
	}

	len-1
}
