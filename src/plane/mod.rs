use crate::volume::Volume;

mod commands;

// a structure representing our enviroment or "plane"

pub struct Plane<'a> {
	volumes: Vec<Volume<'a>>,
	current_volume: usize,

	highest_buff_index: usize, // detirmines how we should label new buffers
}

impl<'a> Plane<'a> {
	// create a new plane with no open volumes

	pub fn new() -> Plane<'a> {
		Plane { volumes: Vec::new(), current_volume: 0, highest_buff_index: 0 }
	}

	// return an index of a volume based on a search

	fn search_volumes(&self, search: &str) -> Option<usize> {
		for (i, v) in self.volumes.iter().enumerate() {
			if format!("{}", v.name).contains(search) {
				return Some(i);
			}
		}

		None
	}
}
