use crate::error::MerlinError;

// stack struct

pub struct Stack {
	stack: Vec<String>
}

impl Stack {
	// new, empty stack

	pub fn new() -> Stack {
		Stack { stack: Vec::new() }
	}

	pub fn len(&self) -> usize {
		self.stack.len()
	}

	pub fn push(&mut self, s: String) {
		self.stack.push(s)
	}

	// grap some values from the struct

	pub fn grab(&mut self, needed: usize) -> Vec<String> {
		self.stack.split_off(self.stack.len() - needed)
	}

	// display a list of the items in the stack

	pub fn molecule(&mut self) {
		if !&self.stack.is_empty() {
			for item in &self.stack {
				print!("{} ", item);
			}

			print!("\n");
		}
	}

	// print the last atom in the stack

	pub fn pen(&mut self) {
		if let Some(s) =  self.stack.last() {
			print!("{}", s);
		}
	}

	// swap the last two items in the stack

	pub fn orbit(&mut self) -> Result<(), MerlinError> {
		let len = self.len();

		if len < 2 {
			return Err(MerlinError::OutOfBounds);
		}

		self.stack.swap(len-2, len-1);

		Ok(())
	}

	// remove the last item from the stack

	pub fn decay(&mut self) {
		self.stack.pop();
	}

	// clear the stack

	pub fn destroy(&mut self) {
		self.stack.clear();
	}
}
