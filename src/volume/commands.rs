// commands relating to the volume structure

use crate::commands::MerlinError;
use super::{Volume, selection::Selection};

impl<'a> Volume<'a> {
	// return the number of the current line

	pub fn spot(&self) -> usize {
		self.line + 1
	}

	// return the length of the file

	pub fn span(&self) -> usize {
		self.buffer.len()
	}
	
	// move up or down a line

	pub fn traverse(&mut self, n: isize) {
		let len = self.buffer.len();
		let mut edge = 0; // our "edge" is set to the beginning of the list by default

		// we are dealing with negative integers, so we must change our edge

		if n < 0 {
			edge = len-1;
		}

		let modified = (self.line as isize) + n;

		if modified < len as isize && modified >= 0 {
			self.line = modified as usize;
		} else {
			self.line = edge;
		}
	}

	// move to specific line

	pub fn appear(&mut self, n: usize) -> Result<(), MerlinError> {
		if n < self.buffer.len() {
			self.line = n;
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

	pub fn inscribe(&mut self, s: &str) {
		let mut lines = s.lines();

		// push the first line to the end of the current lines

		self.current().push_str(lines.next().unwrap());

		// loop through the remaining lines and intersplice them in the buffer

		for line in lines {
			self.line += 1;
			self.buffer.insert(self.line, line.to_string());
		}
	}

	// overwrite text

	pub fn trample(&mut self, s: &str) {
		for (i, line) in s.lines().enumerate() {
			if i >= self.buffer.len() { // the length of the piece of text excedes the length of the buffer
				self.buffer.push(line.to_string());
			} else {
				self.buffer[self.line+i] = line.to_string();
			}
		}
	}

	// replace a part of the buffer

	pub fn transmute(&mut self, part: Selection, replace: &str) {
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

	pub fn shave(&mut self, amount: isize) {
		let chars = self.current().chars();
		let len = chars.count();
		let abs = amount.abs() as usize;

		if abs >= len { // clear the line if the amount we want to remove is equal or greater than the length of the line
			self.current().clear();
		} else if amount > 0 { // remove from the end...
			self.current().replace_range(len-abs.., "");
		} else { // remove from the beginning
			self.current().replace_range(..abs, "");
		}
	}

	// remove a line with 1+ other lines

	fn insert_lines(&mut self, index: usize, lines: &str) {
		self.buffer.remove(index);

		for line in lines.lines() {
			self.buffer.insert(index + 1, line.to_string());
		}
	}
}
