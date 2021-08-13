/*
	ᛗ ᛘ - Merliin
*/

extern crate clap;
use clap::{Arg, App, crate_version};

mod plane;
mod volume;
//mod stack;
mod commands;
mod error;

fn main() {
	let merlin_args = App::new("merlin:")
		.about("An esoteric, programmable text editor")
		.version(crate_version!())
		.arg(Arg::with_name("no-errors")
			.short("n")
			.long("no-errors")
			.help("Silence errors"))
		.arg(Arg::with_name("NOTATION")
			.index(1)
			.multiple(true)
			.help("Merlin notation to evaluate before entering the shell"))
		.get_matches();

	let mut p = plane::Plane::new(!merlin_args.is_present("no-errors"));
	
	// parse the first argument(s) as MN

	if let Some(n) = merlin_args.values_of("NOTATION") {
		p.parse_line(&n.collect::<Vec<&str>>().join(" "));
	}

	// start the shell

	p.repl();
}
