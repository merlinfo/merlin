// commands relating to the volume structure

use gapbuf::{GapBuffer, gap_buffer};
use std::{path::PathBuf, fs};
use crate::error::MerlinError;
use super::{Volume, gb_of_chars};

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
		self.buffer[self.line].len()
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
			Ok(self.buff_to_string(b, e))
		} else {
			Err(MerlinError::OutOfBounds)
		}
	}

	// insert some text into the buffer

	pub fn inscribe(&mut self, s: &str) {
		if !s.is_empty() {
			let mut lines = s.lines().peekable(); // iterator over the lines in the input

			self.insert_iter(lines.next().unwrap().chars()); // insert the first line
			self.cursor = self.buffer[self.line].gap();

			// there are some remaining lines...
			
			if lines.peek().is_some() {
				let append_later: Vec<char> = self.buffer[self.line].drain(
					self.cursor..).collect(); // take the text after the cursor

				// insert the lines

				for line in lines {
					self.line += 1;
					self.buffer.insert(self.line, gb_of_chars(line));
				}

				self.cursor = self.buffer[self.line].len(); // move the cursor the end of the line

				// append the remaining text

				for ch in append_later {
					self.buffer[self.line].push_back(ch);
				}
			}
		}
	}

	// overwrite text

	pub fn trample(&mut self, s: &str) {
		if s.is_empty() { // if our text is empty, clear the line
			self.buffer[self.line].clear();
		} else { // else, clear the line(s)
			let mut chars: GapBuffer<char>;
			
			for (i, line) in s.lines().enumerate() {
				chars = gb_of_chars(line);

				if self.line + i >= self.buffer.len() { // the length of the piece of text excedes the length of the buffer
					self.buffer.push_back(chars);
				} else {
					self.buffer[self.line+i] = chars;
				}
			}
		}

		self.cursor = 0;
	}

	// clear the buffer

	pub fn burn(&mut self) {
		self.cursor = 0;
		self.line = 0;

		self.buffer = gap_buffer![GapBuffer::new()];
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
				fs::write(&name, &(self.buff_to_string(1, self.buffer.len()) + "\n")
					.as_bytes()).or(Err(MerlinError::CreationOrWriteFailed))?;

				self.written = true;
				Ok(())
			}
			None       => Err(MerlinError::BufferNotNamed),
		}
	}

	// return a character based on if the buffer is saved or not

	pub fn carved(&self) -> String {
		if self.written {
			String::from("@")
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

				// combine the current line and the old line.

				for (i, ch) in old_line.iter().enumerate() {
					self.buffer[self.line].insert(self.cursor+i, *ch);
				}
			} else {
				return false;
			}
		} else { // remove a single char
			self.cursor -= 1;
			self.buffer[self.line].remove(self.cursor);
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

	// convert (a part of) the buffer into a string

	fn buff_to_string(&self, b: usize, e: usize) -> String {
		self.buffer
			.range(b-1..e)
			.iter()
			.map(|l| l.iter().collect::<String>())
			.collect::<Vec<String>>()
			.join("\n")
	}

	// insert some text into the current line

	fn insert_iter<T: std::iter::Iterator<Item = char>>(&mut self, iter: T) {
		self.buffer[self.line].insert_many(self.cursor, iter);
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
