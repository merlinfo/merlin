use std::fmt;

mod vol_commands;

use std::{
	fs::File,
	path::PathBuf,
	io::{BufRead, BufReader},
	iter::FromIterator,
};

use gapbuf::{GapBuffer, gap_buffer};
use crate::error::MerlinError;

// create a blank line GapBuffer

fn blank_line() -> GapBuffer<GapBuffer<char>> {
	gap_buffer![GapBuffer::new()]
}

// create a GapBuffer of characters

fn gb_of_chars(s: &str) -> GapBuffer<char> {
	GapBuffer::from_iter(s.chars())
}

// a structure representing a document, or "volume"

pub struct Volume {
	name: Option<PathBuf>,
	buffer: GapBuffer<GapBuffer<char>>,

	line: usize,
	cursor: usize,

	pub written: bool,
}

impl Volume {
	// create a buffer with some existing text

	pub fn from_text(contents: &str) -> Volume {
		/*
			create our buffer, spliting our input into lines,
			and split those up into unicode scalars.
		*/

		let mut buff: GapBuffer<GapBuffer<char>> = contents
			.lines()
			.map(gb_of_chars)
			.collect();

		// fall back if there isn't any text supplied

		if buff.is_empty() {
			buff = blank_line();
		}

		Volume {
			name: None,
			buffer: buff,
			line: 0,
			cursor: 0,
			written: false
		}
	}

	// create a buffer from a file

	pub fn from_file(fpath: &str) -> Result<Volume, MerlinError> {
		let mut buff = blank_line();
		let mut w = true;

		let path = PathBuf::from(fpath);

		// if the file exists, read it. Otherwise, just make an empty buffer 

		if path.exists() {
			match File::open(fpath) {
				Ok(file) => {
					for line in BufReader::new(file).lines() {
						buff.push_back(gb_of_chars(&line.or(Err(MerlinError::ReadFailed))?));
					}

					// remove the blank line if we have successfully populated the buffer

					if buff.len() > 1 {
						buff.pop_front();
					}
				}
				Err(_)   => return Err(MerlinError::ReadFailed),
			}
		} else {
			w = false;
		}

		Ok(Volume {
			name: Some(path),
			buffer: buff,
			line: 0,
			cursor: 0,
			written: w
		})
	}
}

// display our volume

impl fmt::Display for Volume {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self.name {
			Some(p) => write!(f, "{}", p.display()),
			None    => write!(f, "*volume*"),
		}
	}
}
