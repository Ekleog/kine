//! Provided time zones
//!
//! As this crate tries to reduce its API to the minimum, only the timezones that
//! actually need to be implemented are here. See other crates in the ecosystem
//! for more timezone implementations.

mod system;
mod utc;

/// The passed sigil was not one of the valid ones for this timezone
#[derive(Clone, Copy, Debug)]
pub struct InvalidSigilError;

// TODO: derive Error for InvalidSigil

pub use system::{System, SystemSigil};
pub use utc::{Utc, UtcSigil};
