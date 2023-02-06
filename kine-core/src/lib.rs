#![cfg_attr(not(feature = "std"), no_std)]

mod calendar;
mod duration;
mod providers;
mod results;
mod system;
mod time;

pub use calendar::{Calendar, CalendarTime};
pub use duration::Duration;
pub mod leap_seconds;
pub use results::{Error, Result, TimeResult, WrittenTimeResult};
pub use system::{System, SystemTime};
pub use time::Time;
pub mod timezone;
pub use timezone::{OffsetTime, Sigil, TimeZone};

pub mod tz;

/// A date and time as indicated by a calendar
///
/// Anything user-visible should probably be handled using this type. However,
/// remember that operations on it can be vastly different from operations on
/// regular times, durations are often not commutative, etc.
pub type WrittenTime<Cal> = <Cal as Calendar>::Time;
