use crate::Time;

/// The result of trying to figure out what real-world time matches a given written time
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Overflowed the allowed range for the return type
    OutOfRange,
}

// TODO: implement Error once error_in_core is stable
// impl core::error::Error for Error {}
