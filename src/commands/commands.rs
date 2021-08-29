// assorted commands...

use std::process::{Command, Child, Stdio};
use std::env;
use crate::error::MerlinError;
use std::io::{Write, Read};

// run an external command and capture its output

pub fn incant(script: &str) -> Result<String, MerlinError> {
	let command = make_command(script, Stdio::inherit())?;

	String::from_utf8(command.wait_with_output()
		.or(Err(MerlinError::InvalidExternal))?.stdout)
		.or(Err(MerlinError::InvalidExternal))
}

// send text to an external command

pub fn infuse(input: &str, script: &str) -> Result<String, MerlinError> {
	let command = make_command(script, Stdio::piped())?;

	// send data to stdin

	command.stdin
		.ok_or(MerlinError::InvalidExternal)? // stdin is not captured
		.write_all(input.as_bytes())
		.or(Err(MerlinError::InvalidExternal))?; // can't send data to stdin

	let mut s = String::new();

	// capture data from stdout

	command.stdout
		.ok_or(MerlinError::InvalidExternal)? // stdout is not captured
		.read_to_string(&mut s)
		.or(Err(MerlinError::InvalidExternal))?; // can't read datwa

	Ok(s)
}

// "tether" elements together

pub fn tether(elems: &[String], teth: &str) -> String {
	elems.join(teth)
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
