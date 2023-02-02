use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::Duration;

/// One instant in real-life
///
/// `Time` guarantees being able to hold any time between 10.000 BC and 10.000 AD.
/// Any time beyond these bounds may be considered by later versions of this crate
/// as being out of bounds.
/// Currently, it can actually hold any time between roughly 10^22 years before and
/// after posix epoch.
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

    /// Offset by a duration, returning `None` on (however unlikely) overflow
    pub fn checked_add(&self, _rhs: Duration) -> Option<Time> {
        todo!()
    }

    /// Offset by a duration, returning `None` on (however unlikely) overflow
    pub fn checked_sub(&self, _rhs: Duration) -> Option<Time> {
        todo!()
    }

    /// Return the duration elapsed since the other point in time
    pub fn duration_since(&self, _rhs: Time) -> Duration {
        todo!()
    }

    /// Return the duration elapsed since the other point in time
    ///
    /// Returns `None` on the (however unlikely) overflow
    pub fn checked_duration_since(&self, _rhs: Time) -> Option<Duration> {
        todo!()
    }
}

impl Add<Duration> for Time {
    type Output = Time;

    fn add(self, _rhs: Duration) -> Self::Output {
        todo!()
    }
}

impl AddAssign<Duration> for Time {
    fn add_assign(&mut self, _rhs: Duration) {
        todo!()
    }
}

impl Sub<Duration> for Time {
    type Output = Time;

    fn sub(self, _rhs: Duration) -> Self::Output {
        todo!()
    }
}

impl SubAssign<Duration> for Time {
    fn sub_assign(&mut self, _rhs: Duration) {
        todo!()
    }
}

impl Sub<Time> for Time {
    type Output = Duration;

    fn sub(self, _rhs: Time) -> Self::Output {
        todo!()
    }
}

impl Debug for Time {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
