#![cfg_attr(not(feature = "std"), no_std)]

mod calendar;
mod duration;
mod results;
mod system;
mod time;

pub use calendar::{Calendar, CalendarTime};
pub use duration::Duration;
pub mod leap_seconds;
pub mod providers;
pub use results::{Error, Result, TimeResult, WrittenTimeResult};
pub use system::{System, SystemTime};
pub use time::Time;
pub mod timezone;
pub use timezone::{OffsetTime, Sigil, TimeZone};

/// A date and time as indicated by a calendar
///
/// Anything user-visible should probably be handled using this type. However,
/// remember that operations on it can be vastly different from operations on
/// regular times, durations are often not commutative, etc.
pub type WrittenTime<Cal> = <Cal as Calendar>::Time;

// TODO: before release, consider discarding all the trait bounds related to
// display and strings; but still keep them implemented for our types where it
// makes sense.
