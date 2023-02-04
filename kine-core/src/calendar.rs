use crate::{Time, TimeResult, WrittenTimeResult};

/// A calendar system, including timezone if need be
pub trait Calendar {
    /// The data needed to represent a time in this calendar
    type Time: CalendarTime;

    /// Find the possible ways of writing time `t` in this calendar system
    fn write(&self, t: &Time) -> crate::Result<WrittenTimeResult<Self::Time>>;

    /// Retrieve the current time in this calendar
    fn try_now(&self) -> crate::Result<WrittenTimeResult<Self::Time>> {
        let (time, time_many) = match Time::try_now()? {
            TimeResult::One(t) => (t, false),
            TimeResult::Two(t, _) => (t, true),
            TimeResult::Many(t) => (t, true),
            TimeResult::DidNotExist(t, _) => (t, false),
        };
        let (res, res_many) = match self.write(&time)? {
            WrittenTimeResult::One(t) => (t, false),
            WrittenTimeResult::Many(t) => (t, true),
        };
        match time_many || res_many {
            true => Ok(WrittenTimeResult::Many(res)),
            false => Ok(WrittenTimeResult::One(res)),
        }
    }

    /// Retrieve the current time in this calendar
    ///
    /// This function is allowed to panic if the current time is not representable
    /// in this calendar. If this is a problem for you, please use `write`.
    fn now(&self) -> WrittenTimeResult<Self::Time> {
        self.write(&Time::now())
            .expect("Trying to write out-of-range time")
    }
}

/// Time as represented by a calendar
pub trait CalendarTime {
    /// Find the possible times this written time could be about
    fn read(&self) -> crate::Result<TimeResult>;
}
