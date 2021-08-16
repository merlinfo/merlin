// commands relating to the volume structure

use std::iter::FromIterator;
use std::{path::PathBuf, fs};
use crate::error::MerlinError;
use super::Volume;

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
		self.update_cursor()
	}

	// move left or right a character

	pub fn shift(&mut self, n: isize) {
		self.cursor = move_respect_bounds(self.cursor, self.columns()+1, n);
	}

	// move to specific line

	pub fn appear(&mut self, n: usize) {
		self.line = goto_respect_bounds(self.buffer.len()-1, n);
		self.update_cursor();
	}

	// move to a certain character

	pub fn infix(&mut self, n: usize) {
		self.cursor = goto_respect_bounds(self.columns(), n);
	}

	// view a piece of text

	pub fn peer(&self, b: usize, e: usize) -> Result<String, MerlinError> {
		if b >= 1 && e <= self.buffer.len() && b <= e { // coords to view are valid
			Ok(self.buffer.as_slice()[b-1..e].join("\n"))
		} else {
			Err(MerlinError::OutOfBounds)
		}
	}

	// inset some text into the buffer

	pub fn inscribe(&mut self, s: &str) {
		if !s.is_empty() {
			let mut lines = s.lines();

			// remove text after the cursor and push the first line to the end of the current line

			let (chars, remainder) = self.split_line_chars();

			self.buffer[self.line] = String::from_iter(chars);
			self.current().push_str(lines.next().unwrap());

			// loop through the remaining lines and intersplice them in the buffer

			for line in lines {
				self.line += 1;
				self.buffer.insert(self.line, line.to_string());
			}

			self.cursor = self.columns();
			self.current().push_str(&remainder);
		}
	}

	// overwrite text

	pub fn trample(&mut self, s: &str) {
		if s.is_empty() { // if our text is empty, clear the line
			self.buffer[self.line].clear();
		} else { // else, clear the line(s)
			for (i, line) in s.lines().enumerate() {
				if self.line + i >= self.buffer.len() { // the length of the piece of text excedes the length of the buffer
					self.buffer.push(line.to_string());
				} else {
					self.buffer[self.line+i] = line.to_string();
				}
			}
		}

		self.update_cursor();
	}

	// clear the buffer

	pub fn burn(&mut self) {
		self.cursor = 0;
		self.line = 0;

		self.buffer = vec![String::new()]
	}

	// shave off parts of text from a line

	pub fn shave(&mut self, amount: usize) {
		for _ in 0..amount {
			if !self.backspace() {
				break;
			}
		}
	}

	// "dub" a buffer

	pub fn dub(&mut self, f_name: &str) -> Result<(), MerlinError> {
		let err = Err(MerlinError::FileAlreadyExists);

		// name the buffer, or return an error if it is already named

		match self.name {
			None    => {
				let path = PathBuf::from(f_name);

				if path.exists() {
					return err
				}

				self.name = Some(path);

				Ok(())
			}
			Some(_) => err,
		}
	}

	// write out a file

	pub fn carve(&mut self) -> Result<(), MerlinError> {
		match &self.name {
			Some(name) => {
				fs::write(&name, &(self.buffer.join("\n") + "\n").as_bytes()).or(Err(MerlinError::CreationOrWriteFailed))?;

				self.written = true;
				Ok(())
			}
			None       => Err(MerlinError::BufferNotNamed),
		}
	}

	// return a character based on if the buffer is saved or not

	pub fn carved(&self) -> String {
		if self.written {
			String::from("")
		} else {
			String::from("!")
		}
	}

	// remove a single character or line break

	fn backspace(&mut self) -> bool {
		// return true if we have text to delete

		if self.cursor == 0 { 
			if self.line > 0 { // append what is left of the old line to the one above it
				let old_line = self.buffer.remove(self.line);

				self.line -= 1;

				self.cursor = self.columns();
				self.current().push_str(&old_line); // combine!
			} else {
				return false;
			}
		} else { // remove a single char
			let (mut chars, remainder) = self.split_line_chars();
			
			chars.pop();
			self.buffer[self.line] = String::from_iter(chars); 

			self.current().push_str(&remainder);

			self.cursor -= 1;
		}

		true
	}

	// update the cursor position

	fn update_cursor(&mut self) {
		let len = self.columns();

		if self.cursor > len {
			self.cursor = len
		}
	}

	// convert the current line into a vec of chars

	fn curr_into_chars(&self) -> Vec<char> {
		self.buffer[self.line].chars().collect()
	}

	// split the chars in half

	fn split_line_chars(&self) -> (Vec<char>, String) {
		let mut chars = self.curr_into_chars();
		let remainder = String::from_iter(chars.split_off(self.cursor));
		
		(chars, remainder)
	}
}

// move a certain amount (+/-) respecting max and min

fn move_respect_bounds(curr: usize, len: usize, n: isize) -> usize {
	let modified = (curr as isize) + n;

	if modified < len as isize && modified >= 0 {
		return modified as usize
	} else if n < 0 {
		return 0
	}

	len-1
}

// got to a spot, cutting off extra past the max

fn goto_respect_bounds(len: usize, n: usize) -> usize {
	if n > len {
		len
	} else if n < 1 {
		0
	} else {
		n-1
	}
}
