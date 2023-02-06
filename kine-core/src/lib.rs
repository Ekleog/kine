#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../../README.md")]
#![warn(missing_docs)]

mod calendar;
mod duration;
mod providers;
mod results;
mod time;
mod timezone;

pub mod leap_seconds;
pub mod tz;

pub use calendar::{Calendar, CalendarTime};
pub use duration::Duration;
pub use results::{Error, Result, TimeResult};
pub use time::Time;
pub use timezone::{OffsetTime, ParseError, Sigil, TimeZone};

/// A date and time as indicated by a calendar
///
/// Anything user-visible should probably be handled using this type. However,
/// remember that operations on it can be vastly different from operations on
/// regular times, durations are often not commutative, etc.
pub type WrittenTime<Cal> = <Cal as Calendar>::Time;
