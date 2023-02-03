use crate::Time;

// TODO: consider removing WrittenTimeResult altogether as I don't think it's
// actually useful compared to just always returning one written time

/// The result of trying to write a time in a calendar system
///
/// Note that it is assumed that a calendar will return [`Error::OutOfRange`] if
/// it cannot represent the time. Calendars that would not support some times in
/// the middle of their range are assumed not to exist.
#[derive(Clone, Debug)]
pub enum WrittenTimeResult<T> {
    /// There was exactly one way of writing the time
    ///
    /// This is the "normal path."
    One(T),

    /// There were many ways of writing the time (more than two)
    ///
    /// This should not happen with well-formed calendars (any real-world instant
    /// should have a unique way of writing it), but it can theoretically happen
    /// in some calendars. It will probably never actually happen in practice. Only
    /// one example way of writing the time is provided in this case.
    Many(T),
}

impl<T> WrittenTimeResult<T> {
    /// Returns any way of writing the point in time
    ///
    /// Note that this panics if the value of `Many` is an empty vector, but this
    /// would be an ill-formed value
    pub fn any(self) -> T {
        match self {
            WrittenTimeResult::One(t) => t,
            WrittenTimeResult::Many(t) => t,
        }
    }

    /// Maps this function on all instances of T in this result
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> WrittenTimeResult<U> {
        match self {
            WrittenTimeResult::One(t) => WrittenTimeResult::One(f(t)),
            WrittenTimeResult::Many(t) => WrittenTimeResult::Many(f(t)),
        }
    }
}

/// The result of trying to figure out what real-world time matches a given written time
#[derive(Clone, Debug)]
pub enum TimeResult {
    /// The written time matches exactly one real-world time
    ///
    /// This is the "normal path."
    One(Time),

    /// The written time matches two real-world times
    ///
    /// This happens relatively often, eg. when a timezone goes back in time.
    Two(Time, Time),

    /// The written time matches many real-world times
    ///
    /// This should happen very rarely, but can theoretically happen, eg. if a backwards
    /// leap second happens while a timezone is going back in time then there would be three
    /// possible real-world times matching this Time. Given how rare this should be (basically,
    /// probably never actually happen until all programs using this crate are long forgotten),
    /// in this scenario a single example value is returned.
    Many(Time),

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
            TimeResult::One(t)
            | TimeResult::Two(t, _)
            | TimeResult::Many(t)
            | TimeResult::DidNotExist(t, _) => t,
        }
    }
}

/// A specialized Result type for `kine`.
pub type Result<T> = core::result::Result<T, Error>;

/// Represents all the ways a function can fail within `kine`.
#[derive(Clone, Copy, Debug)]
pub enum Error {
    /// Overflowed the allowed range for the return type
    OutOfRange,
}

// TODO: implement Error once error_in_core is stable
// impl core::error::Error for Error {}
