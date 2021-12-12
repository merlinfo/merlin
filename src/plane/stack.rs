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

	// connect items on the stack

	pub fn tether(&mut self) -> Result<(), MerlinError> {
		if self.stack.len() >= 3 {
			let last = self.stack.pop().unwrap();
			let tethered = self.stack.join(&last);

			self.stack.clear();

			return Ok(self.stack.push(tethered))
		}

		Err(MerlinError::InvalidOrNoArguments)
	}

	// split an atom by another atom

	pub fn fray(&mut self) -> Result<(), MerlinError> {
		// get the last two elements from the stack

		let (splitter, atom) = (self.pop()?, self.pop()?);

		// loop through each element separated by "splitter" and add it to the stack

		for a in atom.split(&splitter) {
			if !a.is_empty() {
				self.stack.push(a.to_string())
			}
		}

		Ok(())
	}

	// connect exactly two items together

	pub fn stitch(&mut self) -> Result<(), MerlinError> {	
		let (connector, b, mut a) = (self.pop()?, self.pop()?, self.pop()?);
		
		a.push_str(&connector);
		a.push_str(&b);

		Ok(self.stack.push(a))
	}

	fn pop(&mut self) -> Result<String, MerlinError> {
		self.stack.pop()
			.ok_or(MerlinError::InvalidOrNoArguments)
	}
}
