use core::{
    fmt::{self, Debug},
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::{Calendar, CalendarTime, Duration, System, TimeResult, WrittenTime, WrittenTimeResult};

const NANOS_IN_SEC: i128 = 1_000_000_000;

/// One instant in real-life
///
/// It can hold any time between roughly 10^22 years before and after posix epoch.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Time {
    /// Offset with the POSIX epoch
    ///
    /// This is basically TAI - 10.
    nanos: i128,
}

impl Time {
    /// The posix epoch (1970-01-01T00:00:00 Gregorian UTC)
    pub const POSIX_EPOCH: Time = Time { nanos: 0 };

    pub(crate) const fn from_posix_nanos(nanos: i128) -> Time {
        Time { nanos }
    }

    pub(crate) const fn from_posix_secs(secs: i64) -> Time {
        Time {
            nanos: secs as i128 * NANOS_IN_SEC,
        }
    }

    /// Return the current time
    pub fn now() -> Time {
        System::now()
            .read()
            .expect("System time out of range")
            .any_approximate()
    }

    pub fn read<Tim: CalendarTime>(t: Tim) -> crate::Result<TimeResult> {
        t.read()
    }

    pub fn write<Cal: Calendar>(
        &self,
        cal: Cal,
    ) -> crate::Result<WrittenTimeResult<WrittenTime<Cal>>> {
        cal.write(self)
    }

    /// Offset by a duration, returning `None` on (however unlikely) overflow
    pub fn checked_add(&self, rhs: &Duration) -> Option<Time> {
        self.nanos
            .checked_add(rhs.nanos())
            .map(Time::from_posix_nanos)
    }

    /// Offset by a duration, returning `None` on (however unlikely) overflow
    pub fn checked_sub(&self, rhs: &Duration) -> Option<Time> {
        self.nanos
            .checked_sub(rhs.nanos())
            .map(Time::from_posix_nanos)
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
        self.nanos.checked_sub(rhs.nanos).map(Duration::from_nanos)
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const NANOS_IN_SEC: i128 = 1_000_000_000;

        write!(
            f,
            "{}.{}",
            self.nanos / NANOS_IN_SEC,
            (self.nanos % NANOS_IN_SEC).abs(),
        )
    }
}
