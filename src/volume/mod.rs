use std::fmt;

mod commands;
pub mod selection;

// an enum representing the states of a volume buffer

pub enum VolumeState<'a> {
	// the volume is a file, with a path

	File(&'a str),

	// the volume is an unamed buffer, with a numerical id

	NoFile(usize),
}

// now we can display the VolumeState enum

impl<'a> fmt::Display for VolumeState<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			VolumeState::File(n)    => write!(f, "{}", n),
			VolumeState::NoFile(n)  => write!(f, "*buffer {}*", n),
		}
	}
}

// a structure representing a document, or "volume"

pub struct Volume<'a> {
	pub name: VolumeState<'a>,
	buffer: Vec<String>,

	line: usize,
	pub written: bool,
}

impl<'a> Volume<'a> {
	// return the length of the buffer

	pub fn len(&self) -> usize {
		self.buffer.len()
	}

	// create a buffer with some existing text

	pub fn from_text(num: usize, contents: &str) -> Volume<'a> {
		let mut buff = contents.lines().map(|s| s.to_string()).collect::<Vec<String>>();
		
		if buff.len() == 0 {
			buff = vec![String::from("")];
		}

		Volume {
			name: VolumeState::NoFile(num),
			buffer: buff,
			line: 0,
			written: false
		}
	}

	// create a named buffer from a file

	/*pub fn from_file(path: &str) -> Result<Volume<'a>> {
		Volume {
			name: VolumeSate::File(path),
			buffer: 
			line: 0,
			written: true
		}
	}*/

	// return a mutable reference to the current line

	fn current(&mut self) -> &mut String {
		&mut self.buffer[self.line]
	}
}

impl<'a> fmt::Display for Volume<'a> {
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
