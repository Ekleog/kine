#![no_std]

mod calendar;
mod duration;
mod results;
mod time;

pub use calendar::{Calendar, CalendarTime};
pub use duration::Duration;
pub mod leap_seconds;
pub mod posix;
pub use results::{Error, Result, TimeResult, WrittenTimeResult};
pub use time::Time;

/// A date and time as indicated by a calendar
///
/// Anything user-visible should probably be handled using this type. However,
/// remember that operations on it can be vastly different from operations on
/// regular times, durations are often not commutative, etc.
pub type WrittenTime<Cal> = <Cal as Calendar>::Time;
