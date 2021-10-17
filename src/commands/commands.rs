// assorted commands...

use crate::{error::MerlinError, util};

use std::{
	process::{Command, Child, Stdio},
	env,
	thread::Builder,
	io::Write,
};

// wait for, and capture of a child process, dealing with all of the errors

macro_rules! capture_output {
	($command:ident) => {
		String::from_utf8($command.wait_with_output()
			.or(Err(MerlinError::InvalidExternal))?.stdout)
			.or(Err(MerlinError::InvalidExternal))
	}
}

// run an external command and capture its output

pub fn incant(script: &str) -> Result<String, MerlinError> {
	let command = make_command(script, Stdio::inherit())?;
	capture_output!(command)
}

// send text to an external command

pub fn infuse(input: &str, script: &str) -> Result<String, MerlinError> {
	let mut command = make_command(script, Stdio::piped())?;

	// send data to stdin
	
	let mut stdin = command.stdin.take()
		.ok_or(MerlinError::InvalidExternal)?; // stdin is not captured
	
	// write to stdin

	let owned_input = input.to_owned();

	Builder::new().spawn(move || {
		util::err_msg(stdin.write_all(owned_input.as_bytes()), // terrible badness :pensive:
			"couldn't write to stdin")
	}).or(Err(MerlinError::InvalidExternal))?;

	capture_output!(command)
}

// "make" a command from arguments, changing how it deals with io

fn make_command(script: &str, sio: Stdio) -> Result<Child, MerlinError> {
	let args: Vec<&str> = script.split_whitespace().collect();

	Command::new(&args[0])
		.args(&args[1..])
		.envs(env::vars_os())
		.stdin(sio)
		.stdout(Stdio::piped())
		.spawn()
		.or(Err(MerlinError::InvalidExternal))
}
