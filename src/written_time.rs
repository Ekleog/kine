use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

use crate::{Calendar, CalendarTime, Time, TimeResult, WrittenDuration, WrittenTimeResult};

/// A date and time as indicated by a calendar
///
/// Anything user-visible should probably be handled using this type. However,
/// remember that operations on it can be vastly different from operations on
/// regular times.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WrittenTime<Cal: Calendar> {
    _data: Cal::Time,
}

impl<Cal: Calendar> WrittenTime<Cal> {
    /// Retrieve the current time in this calendar
    ///
    /// This function is allowed to panic if the current time is not representable
    /// in this calendar. If this is a problem for you, please use `from_time`.
    pub fn now() -> WrittenTimeResult<Self> {
        todo!()
    }

    /// Find the possible ways of writing time `t` in this calendar system
    pub fn from_time(_t: &Time) -> crate::Result<WrittenTimeResult<Self>> {
        todo!()
    }

    /// Find the possible times this written time could be about
    pub fn as_time(&self) -> crate::Result<TimeResult> {
        todo!()
    }

    /// Offset by a duration, returning `None` on overflow
    pub fn checked_add(&self, _rhs: &WrittenDuration<Cal>) -> Option<WrittenTime<Cal>> {
        todo!()
    }

    /// Offset by a duration, returning `None` on overflow
    pub fn checked_sub(&self, _rhs: &WrittenDuration<Cal>) -> Option<WrittenTime<Cal>> {
        todo!()
    }

    /// Return the duration elapsed since the other written time
    pub fn duration_since(&self, _rhs: &WrittenTime<Cal>) -> WrittenDuration<Cal> {
        todo!()
    }

    /// Return the duration elapsed since the other written time
    ///
    /// Returns `None` on the (however unlikely) overflow
    pub fn checked_duration_since(&self, _rhs: &WrittenTime<Cal>) -> Option<WrittenDuration<Cal>> {
        todo!()
    }
}

impl<Cal: Calendar> Add<WrittenDuration<Cal>> for WrittenTime<Cal> {
    type Output = WrittenTime<Cal>;

    fn add(self, _rhs: WrittenDuration<Cal>) -> Self::Output {
        todo!()
    }
}

impl<Cal: Calendar> AddAssign<WrittenDuration<Cal>> for WrittenTime<Cal> {
    fn add_assign(&mut self, _rhs: WrittenDuration<Cal>) {
        todo!()
    }
}

impl<Cal: Calendar> Sub<WrittenDuration<Cal>> for WrittenTime<Cal> {
    type Output = WrittenTime<Cal>;

    fn sub(self, _rhs: WrittenDuration<Cal>) -> Self::Output {
        todo!()
    }
}

impl<Cal: Calendar> SubAssign<WrittenDuration<Cal>> for WrittenTime<Cal> {
    fn sub_assign(&mut self, _rhs: WrittenDuration<Cal>) {
        todo!()
    }
}

impl<Cal: Calendar> Sub<WrittenTime<Cal>> for WrittenTime<Cal> {
    type Output = WrittenDuration<Cal>;

    fn sub(self, _rhs: WrittenTime<Cal>) -> Self::Output {
        todo!()
    }
}

// TODO: also introduce all the & variants

impl<Cal: Calendar> Debug for WrittenTime<Cal> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<Cal: Calendar> Display for WrittenTime<Cal> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<Cal: Calendar> FromStr for WrittenTime<Cal> {
    type Err = <Cal::Time as CalendarTime>::ParseError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
