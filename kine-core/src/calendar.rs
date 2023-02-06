use crate::{Time, TimeResult};

/// A calendar system, including timezone if need be
pub trait Calendar {
    /// The data needed to represent a time in this calendar
    type Time: CalendarTime;

    /// Find the possible ways of writing time `t` in this calendar system
    fn write(&self, t: &Time) -> crate::Result<Self::Time>;

    /// Retrieve the current time in this calendar
    fn try_now(&self) -> crate::Result<Self::Time> {
        self.write(&Time::try_now()?.any_approximate())
    }

    /// Retrieve the current time in this calendar
    ///
    /// This function is allowed to panic if the current time is not representable
    /// in this calendar. If this is a problem for you, please use `write`.
    fn now(&self) -> Self::Time {
        self.write(&Time::now())
            .expect("Trying to write out-of-range time")
    }
}

/// Time as represented by a calendar
pub trait CalendarTime {
    /// Find the possible times this written time could be about
    fn read(&self) -> crate::Result<TimeResult>;
}
