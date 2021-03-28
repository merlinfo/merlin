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
	Traverse,
	Appear,
	Peer,
	Inscribe,
	Trample,
	Transmute,
	Shave,
	Molecule,
	Pen,
	Orbit,
	Decay,
	Destroy,
	Tether,
	Newline,
	Space,
	Tab,
	Mirror,
	Atom,
	Scribe,
	Adieu,
	Nomen,
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
			"traverse"  => Ok(Command::Traverse),
			"appear"    => Ok(Command::Appear),
			"peer"      => Ok(Command::Peer),
			"inscribe"  => Ok(Command::Inscribe),
			"trample"   => Ok(Command::Trample),
			"transmute" => Ok(Command::Transmute),
			"shave"     => Ok(Command::Shave),
			"molecule"  => Ok(Command::Molecule),
			"pen"       => Ok(Command::Pen),
			"orbit"     => Ok(Command::Orbit),
			"decay"     => Ok(Command::Decay),
			"destroy"   => Ok(Command::Destroy),
			"tether"    => Ok(Command::Tether),
			"new"       => Ok(Command::Newline),
			"space"     => Ok(Command::Space),
			"tab"       => Ok(Command::Tab),
			"mirror"    => Ok(Command::Mirror),
			"atom"      => Ok(Command::Atom),
			"scribe"    => Ok(Command::Scribe),
			"adieu"     => Ok(Command::Adieu),
			"nomen"     => Ok(Command::Nomen),
			_           => Err(MerlinError::UnknownCommand),
		}
	}
}

impl Command {
	// check if the right amount of arguments have been supplied

	fn valid(&self, args: usize) -> bool {
		match self {
			Command::Genesis | Command::Biblio    | Command::Spot    | Command::Span   | Command::Molecule | Command::Pen    | Command::Orbit  | Command::Decay    | Command::Destroy | 
			Command::Newline | Command::Space     | Command::Tab     | Command::Mirror | Command::Atom     | Command::Scribe | Command::Adieu                                           => true,
			Command::Focus   | Command::Traverse  | Command::Appear  | Command::Shave  | Command::Shelve   | Command::Peer   | Command::Incant | Command::Inscribe | Command::Trample | 
			Command::Nomen                                                                                                                                                              => args >= 1,
			Command::Infuse  | Command::Transmute | Command::Tether                                                                                                                     => args >= 2,
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
				Command::Incant  | Command::Infuse   | Command::Tether   | Command::Nomen                                                                                  => return Ok(args),
				Command::Biblio  | Command::Spot     | Command::Span     | Command::Molecule | Command::Pen    | Command::Orbit    | Command::Decay   | Command::Destroy |
				Command::Newline | Command::Space    | Command::Tab      | Command::Mirror   | Command::Atom   | Command::Scribe   | Command::Adieu                        => return Ok(0),
				Command::Focus   | Command::Traverse | Command::Appear   | Command::Shave    | Command::Shelve | Command::Inscribe | Command::Trample                      => return Ok(1),
				Command::Peer                                                                                                                                              => return Ok(choose_mm(2, 1)),
				Command::Transmute                                                                                                                                         => return Ok(choose_mm(3, 2)),
				Command::Genesis                                                                                                                                           => return Ok(choose_mm(1, 0))
			}
		}

		return Err(MerlinError::InvalidOrNoArguments);
	}

}
