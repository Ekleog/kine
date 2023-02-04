use core::{
    fmt::{self, Debug},
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::{Calendar, CalendarTime, Duration, TimeResult, WrittenTime, WrittenTimeResult};

const NANOS_IN_SECS: i128 = 1_000_000_000;

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

    pub(crate) const fn from_tai_nanos(nanos: i128) -> Option<Time> {
        Time::from_nanos_since_posix_epoch(nanos).checked_sub(&Duration::from_secs(10))
    }

    pub(crate) const fn as_tai_nanos(&self) -> Option<i128> {
        self.nanos.checked_add(10 * NANOS_IN_SECS)
    }

    pub(crate) const fn from_nanos_since_posix_epoch(nanos: i128) -> Time {
        Time { nanos }
    }

    /// Return the current time as indicated by the system timezone
    ///
    /// Note that this will panic in no-std environments if an alternative way of getting
    /// the time is not known for the platform.
    pub fn try_now() -> crate::Result<TimeResult> {
        crate::System::now().read()
    }

    /// Return the current time
    ///
    /// If the system clock (as returned by Rust's `SystemTime::now()`) is in a non-UTC
    /// timezone mode, then this will return any one of the possible `Time`s corresponding
    /// to the time returned by the system clock. This should not happen anyway as Rust's
    /// `SystemTime` should return POSIX timestamps (so, UTC) even when the system clock
    /// is set to local time, so there should be no issue in relying on this function.
    ///
    /// Note that this will panic in no-std environments if an alternative way of getting
    /// the time is not known for the platform.
    pub fn now() -> Time {
        Self::try_now()
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
    pub const fn checked_add(&self, rhs: &Duration) -> Option<Time> {
        // TODO: replace with .map() once const_option_ext is stable
        match self.nanos.checked_add(rhs.nanos()) {
            Some(n) => Some(Time::from_nanos_since_posix_epoch(n)),
            None => None,
        }
    }

    /// Offset by a duration, returning `None` on (however unlikely) overflow
    pub const fn checked_sub(&self, rhs: &Duration) -> Option<Time> {
        // TODO: replace with .map() once const_option_ext is stable
        match self.nanos.checked_sub(rhs.nanos()) {
            Some(n) => Some(Time::from_nanos_since_posix_epoch(n)),
            None => None,
        }
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
        write!(
            f,
            "{}.{}",
            self.nanos / NANOS_IN_SECS,
            (self.nanos % NANOS_IN_SECS).abs(),
        )
    }
}
