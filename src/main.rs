/*
	ᛗ ᛘ - Merliin
*/

extern crate clap;
extern crate ctrlc;

use clap::{Arg, App, crate_version};

mod plane;
mod volume;
mod commands;
mod error;
mod util;

fn main() {
	let merlin_args = App::new("merlin:")
		.about("An esoteric, programmable text editor")
		.version(crate_version!())
		.arg(Arg::with_name("interrupt")
			.short("i")
			.long("interrupt")
			.help("Block interrupts (Ctrl-C)"))
		.arg(Arg::with_name("NOTATION")
			.index(1)
			.multiple(true)
			.help("Merlin notation to evaluate before entering the shell"))
		.get_matches();

	let mut p = plane::Plane::new();
	
	// parse the first argument(s) as MN

	if let Some(n) = merlin_args.values_of("NOTATION") {
		p.parse_line(&n.collect::<Vec<&str>>().join(" "));
	}

	// only handle interrupts when the -i flag is present

	if merlin_args.is_present("interrupt") {
		util::err_msg(ctrlc::set_handler(|| ()), "can't handle Ctrl-C events");
	}

	// start the shell

	p.repl();
}
