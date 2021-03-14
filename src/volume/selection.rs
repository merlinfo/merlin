use crate::commands::MerlinError;

// enum that represents a bit of text

pub enum Selection {
	One(usize),
	Few(usize, usize),
}

impl Selection {
	pub fn new(coords: &[String], len: usize) -> Result<Selection, MerlinError> {
		let last = coords.len() - 1;
		let mut sel: Selection;

		if let Ok(y) = coords[last].parse::<usize>() {
			sel = Selection::One(y - 1);

			if coords.len() > 1 {
				if let Some(f) = coords.get(last - 1) {
					if let Ok(x) = f.parse::<usize>() {
						if x > 0 {
							sel = Selection::Few(x - 1, y);
						}
					}
				}
			}
		} else {
			return Err(MerlinError::InvalidOrNoArguments)
		}

		if !sel.valid(len) {
			return Err(MerlinError::OutOfBounds);
		}

		Ok(sel)
	}

	// check to make sure that the selection is valid in our buffer

	fn valid(&self, len: usize) -> bool {
		match self {
			Selection::One(l)    => l < &len,
			Selection::Few(b, e) => b < &len && b < e && e <= &len
		}
	}
}
