use crate::Time;

/// A calendar system, including timezone if need be
pub trait Calendar {
    /// The data needed to represent a time in this calendar
    type Time: CalendarTime;

    /// The data needed to represent a duration in this calendar
    type Duration;
}

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

/// Time as represented by a calendar
pub trait CalendarTime: Sized {
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
