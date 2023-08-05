/*
	The Merlin Text Editor
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
		.about("an esoteric text editor for wizards")
		.version(crate_version!())
		.arg(Arg::with_name("interrupt")
			.short("i")
			.long("interrupt")
			.help("Don't Block interrupts (Ctrl-C)"))
		.arg(Arg::with_name("stdin")
			.short("s")
			.long("stdin")
			.help("Parse stdin as merlin notation"))
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

	// only handle interrupts when the -i flag is NOT present
 
	if !merlin_args.is_present("interrupt") {
		util::err_msg(ctrlc::set_handler(|| ()), "can't handle Ctrl-C events");
	}

	// check to see if we should parse stdin...

	if merlin_args.is_present("stdin") {
		p.parse_stdin()
	} else { // otherwise start the REPL
		p.repl();
	}
}
