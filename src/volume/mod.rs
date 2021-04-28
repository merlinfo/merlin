use std::fmt;

mod commands;
pub mod selection;

use std::{fs::File, path::Path, io::{BufRead, BufReader}};
use crate::commands::MerlinError;

// an enum representing the states of a volume buffer

pub enum VolumeState {
	// the volume is a file, with a path

	File(String),

	// the volume is an unamed buffer, with a numerical id

	NoFile(usize),
}

// now we can display the VolumeState enum

impl fmt::Display for VolumeState {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			VolumeState::File(n)    => write!(f, "{}", n),
			VolumeState::NoFile(n)  => write!(f, "*buffer {}*", n+1),
		}
	}
}

// a structure representing a document, or "volume"

pub struct Volume {
	name: VolumeState,
	buffer: Vec<String>,

	line: usize,
	written: bool,
}

impl Volume {
	// return the length of the buffer

	pub fn len(&self) -> usize {
		self.buffer.len()
	}

	// set written to false

	pub fn set_unsaved(&mut self) {
		self.written = false;
	}

	// create a buffer with some existing text

	pub fn from_text(num: usize, contents: String) -> Volume {
		let mut buff = contents.lines().map(|s| s.to_owned()).collect::<Vec<String>>();
		
		if buff.len() == 0 {
			buff = vec![String::new()];
		}

		Volume {
			name: VolumeState::NoFile(num),
			buffer: buff,
			line: 0,
			written: false
		}
	}

	pub fn from_file(fpath: String) -> Result<Volume, MerlinError> {
		let mut buff = Vec::new();
		let path = Path::new(&fpath);
		
		if path.exists() {
			match File::open(&fpath) {
				Ok(file) => {
					let reader = BufReader::new(file);

					for line in reader.lines() {
						buff.push(line.or(Err(MerlinError::ReadFailed))?);
					}
				},
				Err(_)   => return Err(MerlinError::ReadFailed),
			}
		} else {
			File::create(path).or(Err(MerlinError::CreationFailed))?;
			buff.push(String::new());
		}

		Ok(Volume {
			name: VolumeState::File(fpath),
			buffer: buff,
			line: 0,
			written: true
		})
	}

	// return a mutable reference to the current line

	fn current(&mut self) -> &mut String {
		&mut self.buffer[self.line]
	}
}

impl fmt::Display for Volume {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let show_written = || {
			if self.written {
				return ""
			}

			"."
		};

		write!(f, "{} {}", self.name, show_written())
	}
}
