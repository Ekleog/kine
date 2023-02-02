use crate::{Time, TimeResult, WrittenTimeResult};

/// A calendar system, including timezone if need be
pub trait Calendar {
    /// The data needed to represent a time in this calendar
    type Time: CalendarTime;

    /// The data needed to represent a duration in this calendar
    type Duration;
}

/// Time as represented by a calendar
pub trait CalendarTime: Sized {
    /// Error raised when trying to parse an invalid string as a time in this calendar
    type ParseError;

    /// Find the possible ways of writing time `t` in this calendar system
    fn from_time(t: &Time) -> crate::Result<WrittenTimeResult<Self>>;

    /// Find the possible times this written time could be about
    fn as_time(&self) -> crate::Result<TimeResult>;

    /// Retrieve the current time in this calendar
    ///
    /// This function is allowed to panic if the current time is not representable
    /// in this calendar. If this is a problem for you, please use `from_time`.
    fn now() -> WrittenTimeResult<Self> {
        Self::from_time(&Time::now()).expect("Time should not go out of range before")
    }
}
