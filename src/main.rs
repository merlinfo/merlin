mod plane;

mod volume;
mod commands;

fn main() {
	let mut p = plane::Plane::new();

	p.repl();
}
