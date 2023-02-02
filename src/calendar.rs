use crate::{Time, TimeResult, WrittenTimeResult};

/// A calendar system, including timezone if need be
pub trait Calendar: Sized {
    /// The data needed to represent a time in this calendar
    type Time: CalendarTime<Self>;

    /// The data needed to represent a duration in this calendar
    type Duration: CalendarDuration<Self>;
}

/// Time as represented by a calendar
pub trait CalendarTime<Cal: Calendar>: Sized {
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

    /// Add a duration to this time, returning `None` in case of overflow
    fn checked_add(&self, rhs: &Cal::Duration) -> Option<Self>;

    /// Add a duration to this time
    fn add(&self, rhs: &Cal::Duration) -> Self {
        self.checked_add(rhs)
            .expect("overflow while adding a duration to a time")
    }

    /// Add a duration to this time
    fn add_assign(&mut self, rhs: &Cal::Duration) {
        *self = self.add(rhs);
    }

    /// Subtract a duration to this time, returning `None` in case of overflow
    fn checked_sub(&self, rhs: &Cal::Duration) -> Option<Self>;

    /// Subtract a duration to this time
    fn sub(&self, rhs: &Cal::Duration) -> Self {
        self.checked_sub(rhs)
            .expect("overflow while subtracting a duration from a time")
    }

    /// Subtract a duration to this time
    fn sub_assign(&mut self, rhs: &Cal::Duration) {
        *self = self.sub(rhs);
    }

    /// Return the duration elapsed since the other time
    ///
    /// Returns `None` on overflow
    fn checked_duration_since(&self, _rhs: &Self) -> Option<Cal::Duration>;

    /// Return the duration elapsed since the other time
    fn duration_since(&self, rhs: &Self) -> Cal::Duration {
        self.checked_duration_since(rhs)
            .expect("overflow while subtracting two times")
    }

    /// Show this written time in the default human-readable format
    fn display(&self, f: &mut std::fmt::Formatter<'_>);

    /// Parse this written time from the default human-readable format
    fn from_str(&self, s: &str);
}

/// Duration as represented by a calendar
pub trait CalendarDuration<Cal: Calendar>: Sized {
    /// A duration that spans no time
    const ZERO: Self;
}
