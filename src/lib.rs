mod calendar;
mod duration;
mod time;
mod written_duration;
mod written_time;

pub use calendar::{Calendar, CalendarTime, TimeResult, WrittenTimeResult};
pub use duration::Duration;
pub use time::Time;
pub use written_duration::WrittenDuration;
pub use written_time::WrittenTime;

/// A specialized Result type for `kine`.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents all the ways a function can fail within `kine`.
#[derive(Debug)]
pub enum Error {
    /// Went out of the allowed range for the return type
    OutOfRange,
}
