use crate::Time;

/// The result of trying to write a time in a calendar system
///
/// Note that it is assumed that a calendar will return [`Error::OutOfRange`] if
/// it cannot represent the time. Calendars that would not support some times in
/// the middle of their range are assumed not to exist.
#[derive(Clone, Debug)]
pub enum WrittenTimeResult<T> {
    /// There was exactly one way of writing the time
    One(T),

    /// There were many ways of writing the time
    Many(Vec<T>),
}

impl<T> WrittenTimeResult<T> {
    /// Returns any way of writing the point in time
    ///
    /// Note that this panics if the value of `Many` is an empty vector, but this
    /// would be an ill-formed value
    pub fn any(self) -> T {
        match self {
            WrittenTimeResult::One(t) => t,
            WrittenTimeResult::Many(v) => v.into_iter().next().unwrap(),
        }
    }

    /// Maps this function on all instances of T in this result
    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> WrittenTimeResult<U> {
        match self {
            WrittenTimeResult::One(t) => WrittenTimeResult::One(f(t)),
            WrittenTimeResult::Many(v) => WrittenTimeResult::Many(v.into_iter().map(f).collect()),
        }
    }
}

/// The result of trying to figure out what real-world time matches a given written time
#[derive(Clone, Debug)]
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

impl TimeResult {
    /// Returns any point in time that matches the given written time
    ///
    /// Note that if the written time did not exist, this will return an approximate version
    /// of what it would have been if it did actually exist.
    pub fn any_approximate(self) -> Time {
        match self {
            TimeResult::One(t) => t,
            TimeResult::Many(v) => v.into_iter().next().unwrap(),
            TimeResult::DidNotExist(t, _) => t,
        }
    }
}

/// A specialized Result type for `kine`.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents all the ways a function can fail within `kine`.
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum Error {
    /// Overflowed the allowed range for the return type
    #[error("Overflowed the allowed range for the return type")]
    OutOfRange,
}
