mod utc;

/// The passed sigil was not one of the valid ones for this timezone
#[derive(Clone, Copy, Debug)]
pub struct InvalidSigil;

// TODO: derive Error for InvalidSigil

pub use utc::{Utc, UtcSigil};
