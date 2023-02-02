use crate::Time;

/// The result of trying to write a time in a calendar system
///
/// Note that it is assumed that a calendar will return [`Error::OutOfRange`] if
/// it cannot represent the time. Calendars that would not support some times in
/// the middle of their range are assumed not to exist.
pub enum WrittenTimeResult<T> {
    /// There was exactly one way of writing the time
    One(T),

    /// There were many ways of writing the time
    Many(Vec<T>),
}

/// The result of trying to figure out what real-world time matches a given written time
pub enum TimeResult {
    /// The written time matches exactly one real-world time
    One(Time),

    /// The written time matches many real-world times
    Many(Vec<Time>),

    /// The written time never actually happened
    ///
    /// In this case, the two times returned are the one just before the time at which
    /// this written time would have been if it existed, and the one just after.
    DidNotExist(Time, Time),
}

/// A specialized Result type for `kine`.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents all the ways a function can fail within `kine`.
#[derive(Debug)]
pub enum Error {
    /// Went out of the allowed range for the return type
    OutOfRange,
}
