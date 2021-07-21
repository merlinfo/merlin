// assorted commands...

//extern crate shellexpand;

use std::process::{Command, Stdio};
//use std::env;
use super::MerlinError;
use std::io::{Write, Read};

// run an external command and capture its output

pub fn incant(script: &str) -> Result<String, MerlinError> {
	let args = script.split_whitespace().collect::<Vec<&str>>();
	//let enviroment = env::vars_os().collect::<HashMap>()

	let command = Command::new(&args[0])
		.args(&args[1..])
		//.envs(enviroment)
		//.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.or(Err(MerlinError::InvalidExternal))?;

	String::from_utf8(command.wait_with_output()
		.or(Err(MerlinError::InvalidExternal))?.stdout)
		.or(Err(MerlinError::InvalidExternal))
}

// send text to an external command

pub fn infuse(input: &str, script: &str) -> Result<String, MerlinError> {
	let args = script.split_whitespace().collect::<Vec<&str>>();

	let command = Command::new(&args[0])
		.args(&args[1..])
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.or(Err(MerlinError::InvalidExternal))?;
	
	command.stdin
		.ok_or(MerlinError::InvalidExternal)? // stdin is not captured
		.write_all(input.as_bytes())
		.or(Err(MerlinError::InvalidExternal))?; // can't send data to stdin

	let mut s = String::new();

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
