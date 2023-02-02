use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::{Calendar, Duration, WrittenTime, WrittenTimeResult};

/// One instant in real-life
///
/// It can hold any time between roughly 10^22 years before and after posix epoch.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Time {
    /// Offset with the POSIX epoch
    nanos: i128,
}

impl Time {
    /// The posix epoch (1970-01-01T00:00:00 Gregorian UTC)
    pub const POSIX_EPOCH: Time = Time { nanos: 0 };

    /// Return the current time
    pub fn now() -> Time {
        todo!()
    }

    pub fn write<Cal: Calendar>(
        &self,
        cal: Cal,
    ) -> crate::Result<WrittenTimeResult<WrittenTime<Cal>>> {
        cal.write(self).map(|r| r.map(WrittenTime))
    }

    /// Offset by a duration, returning `None` on (however unlikely) overflow
    pub fn checked_add(&self, rhs: &Duration) -> Option<Time> {
        self.nanos
            .checked_add(rhs.nanos)
            .map(|nanos| Time { nanos })
    }

    /// Offset by a duration, returning `None` on (however unlikely) overflow
    pub fn checked_sub(&self, rhs: &Duration) -> Option<Time> {
        self.nanos
            .checked_sub(rhs.nanos)
            .map(|nanos| Time { nanos })
    }

    /// Return the duration elapsed since the other point in time
    pub fn duration_since(&self, rhs: &Time) -> Duration {
        self.checked_duration_since(rhs)
            .expect("overflow computing duration since another time")
    }

    /// Return the duration elapsed since the other point in time
    ///
    /// Returns `None` on the (however unlikely) overflow
    pub fn checked_duration_since(&self, rhs: &Time) -> Option<Duration> {
        self.nanos
            .checked_sub(rhs.nanos)
            .map(|nanos| Duration { nanos })
    }
}

impl Add<Duration> for Time {
    type Output = Time;

    fn add(self, rhs: Duration) -> Self::Output {
        self.checked_add(&rhs)
            .expect("overflow adding duration to a time")
    }
}

impl AddAssign<Duration> for Time {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub<Duration> for Time {
    type Output = Time;

    fn sub(self, rhs: Duration) -> Self::Output {
        self.checked_sub(&rhs)
            .expect("overflow subtracting duration from a time")
    }
}

impl SubAssign<Duration> for Time {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Sub<Time> for Time {
    type Output = Duration;

    fn sub(self, rhs: Time) -> Self::Output {
        self.duration_since(&rhs)
    }
}

// TODO: also introduce all the & variants

impl Debug for Time {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!() // TODO: format as TAI?
    }
}
