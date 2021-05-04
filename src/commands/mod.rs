use std::str::FromStr;

pub mod commands;

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
	WriteFailed,
}

// An enum that represents commands

pub enum Command {
	Genesis,
	Biblio,
	Shelve,
	Focus,
	Incant,
	Infuse,
	Spot,
	Span,
	Pin,
	Columns,
	Traverse,
	Shift,
	Appear,
	Infix,
	Peer,
	Peek,
	Inscribe,
	Trample,
	Burn,
	Shave,
	Molecule,
	Pen,
	Orbit,
	Decay,
	Destroy,
	Tether,
	Mirror,
	Atom,
	Scribe,
	Adieu,
	Nomen,
	Summon,
	Dub,
	Carve,
	Spellbook,
}

impl FromStr for Command {
	type Err = MerlinError;

	fn from_str(cmd: &str) -> Result<Self, Self::Err> {
		match cmd {
			"genesis"   => Ok(Command::Genesis),
			"biblio"    => Ok(Command::Biblio),
			"incant"    => Ok(Command::Incant),
			"infuse"    => Ok(Command::Infuse),
			"shelve"    => Ok(Command::Shelve),
			"focus"     => Ok(Command::Focus),
			"spot"      => Ok(Command::Spot),
			"span"      => Ok(Command::Span),
			"pin"       => Ok(Command::Pin),
			"columns"   => Ok(Command::Columns),
			"traverse"  => Ok(Command::Traverse),
			"shift"     => Ok(Command::Shift),
			"appear"    => Ok(Command::Appear),
			"infix"     => Ok(Command::Infix),
			"peer"      => Ok(Command::Peer),
			"peek"      => Ok(Command::Peek),
			"inscribe"  => Ok(Command::Inscribe),
			"trample"   => Ok(Command::Trample),
			"burn"      => Ok(Command::Burn),
			"shave"     => Ok(Command::Shave),
			"molecule"  => Ok(Command::Molecule),
			"pen"       => Ok(Command::Pen),
			"orbit"     => Ok(Command::Orbit),
			"decay"     => Ok(Command::Decay),
			"destroy"   => Ok(Command::Destroy),
			"tether"    => Ok(Command::Tether),
			"mirror"    => Ok(Command::Mirror),
			"atom"      => Ok(Command::Atom),
			"scribe"    => Ok(Command::Scribe),
			"adieu"     => Ok(Command::Adieu),
			"nomen"     => Ok(Command::Nomen),
			"summon"    => Ok(Command::Summon),
			"dub"       => Ok(Command::Dub),
			"carve"     => Ok(Command::Carve),
			"spellbook" => Ok(Command::Spellbook),
			_           => Err(MerlinError::UnknownCommand),
		}
	}
}

impl Command {
	// check if the right amount of arguments have been supplied

	fn valid(&self, args: usize) -> bool {
		match self {
			Command::Genesis | Command::Biblio    | Command::Spot      | Command::Span   | Command::Pin    | Command::Columns | Command::Molecule | Command::Pen     | Command::Orbit   | 
			Command::Decay   | Command::Destroy   | Command::Mirror    | Command::Atom   | Command::Scribe | Command::Adieu   | Command::Carve    | Command::Burn                         => true,
			Command::Focus   | Command::Traverse  | Command::Appear    | Command::Shave  | Command::Shelve | Command::Incant  | Command::Inscribe | Command::Trample | Command::Peek    |
			Command::Summon  | Command::Dub       | Command::Spellbook | Command::Shift  | Command::Infix                                                                                 => args >= 1,
			Command::Infuse  | Command::Tether    | Command::Nomen     | Command::Peer                                                                                                    => args >= 2,
		}
	}

	// check if the number of arguments are valid, and if so return the needed amount of arguments

	pub fn get_needed(&self, args: usize) -> Result<usize, MerlinError> {
		if self.valid(args) {
			let choose_mm = |max, min| {
				if args >= max {
					return max;
				}

				return min;
			};

			match self {
				Command::Tether  | Command::Nomen                                                                                                                           => return Ok(args),
				Command::Biblio  | Command::Spot     | Command::Span      | Command::Molecule | Command::Pen    | Command::Orbit    | Command::Decay   | Command::Destroy |
				Command::Mirror  | Command::Atom     | Command::Scribe    | Command::Adieu    | Command::Carve  | Command::Pin      | Command::Columns | Command::Burn      => return Ok(0),
				Command::Focus   | Command::Traverse | Command::Appear    | Command::Shave    | Command::Shelve | Command::Inscribe | Command::Trample | Command::Incant  | 
				Command::Summon  | Command::Dub      | Command::Spellbook | Command::Shift    | Command::Infix  | Command::Peek                                             => return Ok(1),
				Command::Infuse  | Command::Peer                                                                                                                            => return Ok(2),
				Command::Genesis                                                                                                                                            => return Ok(choose_mm(1, 0))
			}
		}

		return Err(MerlinError::InvalidOrNoArguments);
	}

}
