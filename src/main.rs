mod plane;
mod volume;
mod commands;
mod nomen;

fn main() {
	let mut p = plane::Plane::new();

	p.repl();
}
