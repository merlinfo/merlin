mod plane;
mod volume;
mod commands;
mod parse;

fn main() {
	let mut p = plane::Plane::new();

	p.genesis("hello\nworld\nto\nyou");
	p.genesis("bruh");

	p.focus("1");

	println!("{}", p.biblio());
}
