// assorted commands...

use std::io::process::Command;

// run an external command and capture its output

pub fn incant(cmd: &str) -> String {
	let args = cmd.split_whitespace.collect::<[&str]>();

	let command = Command::new(args[0])
		.args(args[1..])
}
