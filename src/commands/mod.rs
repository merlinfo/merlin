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
	Decant,
	Infuse,
	Defuse,
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
	Atoms,
	Pen,
	Orbit,
	Decay,
	Destroy,
	Tether,
	Fray,
	Atom,
	Scribe,
	Adieu,
	Nomen,
	Bottle,
	Disenchant,
	Smash,
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
			"genesis"    => Ok(Command::Genesis),
			"spine"      => Ok(Command::Spine),
			"carved"     => Ok(Command::Carved),
			"incant"     => Ok(Command::Incant),
			"decant"     => Ok(Command::Decant),
			"infuse"     => Ok(Command::Infuse),
			"defuse"     => Ok(Command::Defuse),
			"shelve"     => Ok(Command::Shelve),
			"focus"      => Ok(Command::Focus),
			"volume"     => Ok(Command::Volume),
			"volumes"    => Ok(Command::Volumes),
			"spot"       => Ok(Command::Spot),
			"span"       => Ok(Command::Span),
			"pin"        => Ok(Command::Pin),
			"columns"    => Ok(Command::Columns),
			"traverse"   => Ok(Command::Traverse),
			"shift"      => Ok(Command::Shift),
			"appear"     => Ok(Command::Appear),
			"infix"      => Ok(Command::Infix),
			"peer"       => Ok(Command::Peer),
			"inscribe"   => Ok(Command::Inscribe),
			"trample"    => Ok(Command::Trample),
			"burn"       => Ok(Command::Burn),
			"shave"      => Ok(Command::Shave),
			"molecule"   => Ok(Command::Molecule),
			"atoms"      => Ok(Command::Atoms),
			"pen"        => Ok(Command::Pen),
			"orbit"      => Ok(Command::Orbit),
			"decay"      => Ok(Command::Decay),
			"destroy"    => Ok(Command::Destroy),
			"tether"     => Ok(Command::Tether),
			"fray"       => Ok(Command::Fray),
			"atom"       => Ok(Command::Atom),
			"scribe"     => Ok(Command::Scribe),
			"adieu"      => Ok(Command::Adieu),
			"nomen"      => Ok(Command::Nomen),
			"bottle"     => Ok(Command::Bottle),
			"disenchant" => Ok(Command::Disenchant),
			"smash"      => Ok(Command::Smash),
			"merlin"     => Ok(Command::Merlin),
			"summon"     => Ok(Command::Summon),
			"dub"        => Ok(Command::Dub),
			"carve"      => Ok(Command::Carve),
			"spellbook"  => Ok(Command::Spellbook),
			_            => Err(MerlinError::UnknownCommand),
		}
	}
}

impl Command {
	// check if the number of arguments are valid, and if so return the needed amount of arguments

	pub fn get_needed(&self, args: usize) -> Result<usize, MerlinError> {
		// choose the number of atoms we need, based on those available

		let choose_mm = |max, min| {
			if args >= max {
				max
			} else {
				min
			}
		};

		let all_with_min = |min| {
			if args >= min {
				args
			} else {
				args + 1
			}
		};

		let needed = match self {
			Command::Tether                                                                                                                                                => all_with_min(3), // we need a minimum of 3
			Command::Nomen                                                                                                                                                 => all_with_min(1), // min of 1
			Command::Spot    | Command::Span     | Command::Molecule  | Command::Pen      | Command::Orbit  | Command::Decay    | Command::Destroy |
			Command::Atom    | Command::Scribe   | Command::Adieu     | Command::Carve    | Command::Pin    | Command::Columns  | Command::Burn    | Command::Volume     |
			Command::Volumes | Command::Carved   | Command::Atoms                                                                                                          => 0,
			Command::Focus   | Command::Traverse | Command::Appear    | Command::Shave    | Command::Shelve | Command::Inscribe | Command::Trample | Command::Incant     | 
			Command::Summon  | Command::Dub      | Command::Spellbook | Command::Shift    | Command::Infix  | Command::Spine    | Command::Merlin  | Command::Disenchant |
			Command::Smash   | Command::Decant                                                                                                                             => 1,
			Command::Infuse  | Command::Peer     | Command::Fray      | Command::Defuse                                                                                    => 2,
			Command::Bottle                                                                                                                                                => choose_mm(2, 1),
			Command::Genesis                                                                                                                                               => choose_mm(1, 0),
		};

		if needed <= args {
			Ok(needed)
		} else {
			Err(MerlinError::InvalidOrNoArguments)
		}
	}

}
