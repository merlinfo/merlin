use std::fmt;

mod vol_commands;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{BufRead, BufReader};
use crate::commands::MerlinError;

// a structure representing a document, or "volume"

pub struct Volume {
	name: Option<PathBuf>,
	buffer: Vec<String>,

	line: usize,
	cursor: usize,

	written: bool,
}

impl Volume {
	// set written to a boolean value

	pub fn set_written(&mut self, w: bool) {
		self.written = w;
	}

	// create a buffer with some existing text

	pub fn from_text(contents: String) -> Volume {
		let mut buff = contents.lines().map(|s| s.to_owned()).collect::<Vec<String>>();
		
		if buff.len() == 0 {
			buff = vec![String::new()];
		}

		Volume {
			name: None,
			buffer: buff,
			line: 0,
			cursor: 0,
			written: false
		}
	}

	pub fn from_file(fpath: &str) -> Result<Volume, MerlinError> {
		let mut buff = Vec::new();
		let mut w = true;

		let path = Path::new(fpath);

		if path.exists() {
			match File::open(fpath) {
				Ok(file) => {
					let reader = BufReader::new(file);

					for line in reader.lines() {
						buff.push(line.or(Err(MerlinError::ReadFailed))?);
					}
				},
				Err(_)   => return Err(MerlinError::ReadFailed),
			}
		} else {
			buff.push(String::new());
			w = false;
		}

		Ok(Volume {
			name: Some(path.to_path_buf()),
			buffer: buff,
			line: 0,
			cursor: 0,
			written: w
		})
	}

	// return a mutable reference to the current line

	fn current(&mut self) -> &mut String {
		&mut self.buffer[self.line]
	}
}

impl fmt::Display for Volume {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self.name {
			Some(p) => write!(f, "{}", p.display()),
			&None    => write!(f, "*volume*"),
		}
	}
}
