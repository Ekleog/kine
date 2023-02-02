use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::{Time, TimeResult, WrittenTimeResult};

/// A calendar system, including timezone if need be
pub trait Calendar {
    /// The data needed to represent a time in this calendar
    type Time: CalendarTime;

    /// Find the possible ways of writing time `t` in this calendar system
    fn write(&self, t: &Time) -> crate::Result<WrittenTimeResult<Self::Time>>;

    /// Retrieve the current time in this calendar
    ///
    /// This function is allowed to panic if the current time is not representable
    /// in this calendar. If this is a problem for you, please use `write`.
    fn now(&self) -> WrittenTimeResult<Self::Time> {
        self.write(&Time::now())
            .expect("Trying to write out-of-range time")
    }

    /// Parse this written time from the default human-readable format
    fn from_str(s: &str) -> Result<Self::Time, <Self::Time as FromStr>::Err> {
        Self::Time::from_str(s)
    }
}

/// Time as represented by a calendar
pub trait CalendarTime: Debug + Display + FromStr + Sized {
    /// Find the possible times this written time could be about
    fn read(&self) -> crate::Result<TimeResult>;
}
