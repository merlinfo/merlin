use std::fmt;
use crate::util::ERROR_PREFIX;

// our error structure

#[derive(Debug)]
pub enum MerlinError {
	OutOfBounds,
	InvalidSyntax,
	UnknownCommand,
	InvalidExternal,
	InvalidOrNoArguments,
	NoVolumes,
	CreationOrWriteFailed,
	ReadFailed,
	FileAlreadyExists,
	BufferNotNamed,
	UnknownNomen,
}

impl fmt::Display for MerlinError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let msg = match self {
			MerlinError::OutOfBounds           => "index out of bounds",
			MerlinError::InvalidSyntax         => "invalid syntax",
			MerlinError::UnknownCommand        => "unknown command",
			MerlinError::InvalidExternal       => "invalid or failing external command",
			MerlinError::InvalidOrNoArguments  => "invalid number of arguments",
			MerlinError::NoVolumes             => "no open volumes",
			MerlinError::CreationOrWriteFailed => "failed to create / write a file",
			MerlinError::ReadFailed            => "failed to read a file",
			MerlinError::FileAlreadyExists     => "file already exists or buffer is already named",
			MerlinError::BufferNotNamed        => "buffer is not named",
			MerlinError::UnknownNomen          => "unknown nomen",
		};

		write!(f, "{} {}", ERROR_PREFIX, msg)
	}
}
