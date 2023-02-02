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
pub struct WrittenTime<Cal: Calendar>(Cal::Time);

impl<Cal: Calendar> WrittenTime<Cal> {
    /// Retrieve the current time in this calendar
    ///
    /// This function is allowed to panic if the current time is not representable
    /// in this calendar. If this is a problem for you, please use `from_time`.
    pub fn now() -> WrittenTimeResult<Self> {
        Cal::Time::now().map(Self)
    }

    /// Find the possible ways of writing time `t` in this calendar system
    pub fn from_time(t: &Time) -> crate::Result<WrittenTimeResult<Self>> {
        Cal::Time::from_time(t).map(|r| r.map(Self))
    }

    /// Find the possible times this written time could be about
    pub fn as_time(&self) -> crate::Result<TimeResult> {
        self.0.as_time()
    }

    /// Offset by a duration, returning `None` on overflow
    pub fn checked_add(&self, rhs: &WrittenDuration<Cal>) -> Option<WrittenTime<Cal>> {
        self.0.checked_add(&rhs.0).map(Self)
    }

    /// Offset by a duration, returning `None` on overflow
    pub fn checked_sub(&self, rhs: &WrittenDuration<Cal>) -> Option<WrittenTime<Cal>> {
        self.0.checked_sub(&rhs.0).map(Self)
    }

    /// Return the duration elapsed since the other written time
    pub fn duration_since(&self, rhs: &WrittenTime<Cal>) -> WrittenDuration<Cal> {
        WrittenDuration(self.0.duration_since(&rhs.0))
    }

    /// Return the duration elapsed since the other written time
    ///
    /// Returns `None` on the (however unlikely) overflow
    pub fn checked_duration_since(&self, rhs: &WrittenTime<Cal>) -> Option<WrittenDuration<Cal>> {
        self.0.checked_duration_since(&rhs.0).map(WrittenDuration)
    }
}

impl<Cal: Calendar> Add<WrittenDuration<Cal>> for WrittenTime<Cal> {
    type Output = WrittenTime<Cal>;

    fn add(self, rhs: WrittenDuration<Cal>) -> Self::Output {
        Self(self.0.add(&rhs.0))
    }
}

impl<Cal: Calendar> AddAssign<WrittenDuration<Cal>> for WrittenTime<Cal> {
    fn add_assign(&mut self, rhs: WrittenDuration<Cal>) {
        self.0.add_assign(&rhs.0);
    }
}

impl<Cal: Calendar> Sub<WrittenDuration<Cal>> for WrittenTime<Cal> {
    type Output = WrittenTime<Cal>;

    fn sub(self, rhs: WrittenDuration<Cal>) -> Self::Output {
        Self(self.0.sub(&rhs.0))
    }
}

impl<Cal: Calendar> SubAssign<WrittenDuration<Cal>> for WrittenTime<Cal> {
    fn sub_assign(&mut self, rhs: WrittenDuration<Cal>) {
        self.0.sub_assign(&rhs.0);
    }
}

impl<Cal: Calendar> Sub<WrittenTime<Cal>> for WrittenTime<Cal> {
    type Output = WrittenDuration<Cal>;

    fn sub(self, rhs: WrittenTime<Cal>) -> Self::Output {
        self.duration_since(&rhs)
    }
}

// TODO: also introduce all the & variants

impl<Cal: Calendar> Debug for WrittenTime<Cal> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl<Cal: Calendar> Display for WrittenTime<Cal> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.display(f)
    }
}

impl<Cal: Calendar> FromStr for WrittenTime<Cal> {
    type Err = <Cal::Time as CalendarTime<Cal>>::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Cal::Time::from_str(s).map(Self)
    }
}
