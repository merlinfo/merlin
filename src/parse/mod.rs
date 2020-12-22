pub use crate::volume::Selection;
use crate::commands::MerlinError;

const SEPERATOR: &str = ".";

impl Selection {
	pub fn from(s: &str, l: usize) -> Result<Selection, MerlinError> {
		match s {
			""        => return Ok(Selection::Current), // no specification means the current line
			SEPERATOR => return Ok(Selection::Entire), // a single comma means the entire buffer
			_   => {
				if let Ok(v) = s.parse::<usize>() { // we have a single line number
					// we check if it is in an acceptable range

					if v > l || v == 0 {
						return Err(MerlinError::OutOfBounds);
					}

					return Ok(Selection::One(v-1))
				} else {
					let split = s.split(SEPERATOR).collect::<Vec<&str>>();

					if split.len() == 2 {
						if let Ok(b) = split[0].parse::<usize>() {
							if let Ok(e) = split[1].parse::<usize>() {
								if b < e && e <= l && b != 0 {
									return Ok(Selection::Few(b-1, e))
								}
							}
						}
					}

					return Err(MerlinError::InvalidSyntax);
				}
			}
		}
	}
}
