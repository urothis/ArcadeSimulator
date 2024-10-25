use std::fmt::{self, Display, Formatter};

use clap::ValueEnum;

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum App {
	#[default]
    Desktop,
    Wasm,
}

impl Display for App {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Desktop => write!(f, "desktop"),
			Self::Wasm => write!(f, "wasm"),
		}
	}
}

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum Target {
	Diorama,
	#[default]
	Spooky,
}

impl Display for Target {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Diorama => write!(f, "diorama"),
			Self::Spooky => write!(f, "spooky"),
		}
	}
}
