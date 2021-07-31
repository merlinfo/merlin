// our error structure

pub enum MerlinError {
	OutOfBounds,
	InvalidSyntax,
	UnknownCommand,
	InvalidExternal,
	InvalidOrNoArguments,
	NoVolumes,
	CreationFailed,
	ReadFailed,
	FileAlreadyExists,
	BufferNotNamed,
}

