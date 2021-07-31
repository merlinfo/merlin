use std::str::FromStr;
use crate::error::MerlinError;

pub mod commands;

// An enum that represents commands

pub enum Command {
	Genesis,
	Spine,
	Carved,
	Shelve,
	Focus,
	Volume,
	Volumes,
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
	Fray,
	Mirror,
	Atom,
	Scribe,
	Adieu,
	Nomen,
	Merlin,
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
			"spine"     => Ok(Command::Spine),
			"carved"    => Ok(Command::Carved),
			"incant"    => Ok(Command::Incant),
			"infuse"    => Ok(Command::Infuse),
			"shelve"    => Ok(Command::Shelve),
			"focus"     => Ok(Command::Focus),
			"volume"    => Ok(Command::Volume),
			"volumes"   => Ok(Command::Volumes),
			"spot"      => Ok(Command::Spot),
			"span"      => Ok(Command::Span),
			"pin"       => Ok(Command::Pin),
			"columns"   => Ok(Command::Columns),
			"traverse"  => Ok(Command::Traverse),
			"shift"     => Ok(Command::Shift),
			"appear"    => Ok(Command::Appear),
			"infix"     => Ok(Command::Infix),
			"peer"      => Ok(Command::Peer),
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
			"fray"      => Ok(Command::Fray),
			"mirror"    => Ok(Command::Mirror),
			"atom"      => Ok(Command::Atom),
			"scribe"    => Ok(Command::Scribe),
			"adieu"     => Ok(Command::Adieu),
			"nomen"     => Ok(Command::Nomen),
			"merlin"    => Ok(Command::Merlin),
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
			Command::Genesis | Command::Spot      | Command::Span      | Command::Pin    | Command::Columns | Command::Molecule | Command::Pen      | Command::Orbit   | 
			Command::Decay   | Command::Destroy   | Command::Mirror    | Command::Atom   | Command::Scribe  | Command::Adieu    | Command::Carve    | Command::Burn    | Command::Volume |
			Command::Volumes | Command::Carved                                                                                                                                             => true,
			Command::Focus   | Command::Traverse  | Command::Appear    | Command::Shave  | Command::Shelve  | Command::Incant   | Command::Inscribe | Command::Trample | Command::Summon |
			Command::Dub     | Command::Spellbook | Command::Shift     | Command::Infix  | Command::Spine   | Command::Nomen    | Command::Merlin                                          => args >= 1,
			Command::Infuse  | Command::Tether    | Command::Fray      | Command::Peer                                                                                                     => args >= 2,
		}
	}

	// check if the number of arguments are valid, and if so return the needed amount of arguments

	pub fn get_needed(&self, args: usize) -> Result<usize, MerlinError> {
		if self.valid(args) {
			// choose the number of atoms we need, based on those available

			let choose_mm = |max, min| {
				if args >= max {
					return max;
				}

				return min;
			};

			match self {
				Command::Tether  | Command::Nomen                                                                                                                           => return Ok(args),
				Command::Spot    | Command::Span     | Command::Molecule  | Command::Pen      | Command::Orbit  | Command::Decay    | Command::Destroy | Command::Mirror  |
				Command::Atom    | Command::Scribe   | Command::Adieu     | Command::Carve    | Command::Pin    | Command::Columns  | Command::Burn    | Command::Volume  |
				Command::Volumes | Command::Carved                                                                                                                          => return Ok(0),
				Command::Focus   | Command::Traverse | Command::Appear    | Command::Shave    | Command::Shelve | Command::Inscribe | Command::Trample | Command::Incant  | 
				Command::Summon  | Command::Dub      | Command::Spellbook | Command::Shift    | Command::Infix  | Command::Spine    | Command::Merlin                       => return Ok(1),
				Command::Infuse  | Command::Peer     | Command::Fray                                                                                                        => return Ok(2),
				Command::Genesis                                                                                                                                            => return Ok(choose_mm(1, 0))
			}
		}

		return Err(MerlinError::InvalidOrNoArguments);
	}

}
