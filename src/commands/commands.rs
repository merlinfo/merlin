// assorted commands...

use crate::{error::MerlinError, util};

use std::{
	process::{Command, Child, Stdio},
	env,
	thread::Builder,
	io::Write,
};

// wait for, and capture an output stream of a child process, dealing with all of the errors

macro_rules! capture_output {
	($command:ident, $stream:ident) => {
		String::from_utf8($command.wait_with_output()
			.or(Err(MerlinError::InvalidExternal))?.$stream)
			.or(Err(MerlinError::InvalidExternal))
	}
}

// make a "*cant" command: run a command and capture either stdout or stderr

macro_rules! make_incant {
	($name:ident, $out:expr, $err:expr, $stream:ident) => {
		pub fn $name(script: &str) -> Result<String, MerlinError> {
			let command = make_command(script,
				Stdio::inherit(),
				$out,
				$err)?;

			capture_output!(command, $stream)
		}
	}
}

make_incant!(incant, Stdio::piped(), Stdio::inherit(), stdout);
make_incant!(decant, Stdio::inherit(), Stdio::piped(), stderr);

// send text to an external command

fn send_and_receive(command: &mut Child, input: &str) -> Result<(), MerlinError> {
	// send data to stdin
	
	let mut stdin = command.stdin.take()
		.ok_or(MerlinError::InvalidExternal)?; // stdin is not captured
	
	// write to stdin

	let owned_input = input.to_owned();

	Builder::new().spawn(move || {
		util::err_msg(stdin.write_all(owned_input.as_bytes()), // terrible badness :pensive:
			"couldn't write to stdin")
	}).or(Err(MerlinError::InvalidExternal))?;

	Ok(())
}

macro_rules! make_infuse {
	($name:ident, $out:expr, $err:expr, $stream:ident) => {
		pub fn $name(input: &str, script: &str) -> Result<String, MerlinError> {
			let mut command = make_command(script, Stdio::piped(), $out, $err)?;
			send_and_receive(&mut command, input)?;
	
			capture_output!(command, $stream)
		}
	}
}

make_infuse!(infuse, Stdio::piped(), Stdio::inherit(), stdout);
make_infuse!(defuse, Stdio::inherit(), Stdio::piped(), stderr);

// "make" a command from arguments, changing how it deals with io

fn make_command(script: &str, stdin: Stdio, stdout: Stdio, stderr: Stdio) -> Result<Child, MerlinError> {
	let args: Vec<&str> = script.split_whitespace().collect();

	if !args.is_empty() { // don't attempt to run if the command name is a blank string
		Command::new(&args[0])
			.args(&args[1..])
			.envs(env::vars_os())
			.stdin(stdin)
			.stdout(stdout)
			.stderr(stderr)
			.spawn()
			.or(Err(MerlinError::InvalidExternal))
	} else {
		return Err(MerlinError::InvalidExternal);
	}
}
