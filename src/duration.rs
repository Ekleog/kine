use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Sub, SubAssign},
};

/// A (signed) Duration.
///
/// It can represent time intervals of roughly 10^22 years either way.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Duration {
    nanos: i128,
}

impl Duration {
    pub const ZERO: Duration = Duration { nanos: 0 };

    /// Create a Duration that lasts for `nanos` nanoseconds
    ///
    /// This cannot panic.
    pub fn from_nanos(_nanos: i128) -> Duration {
        todo!()
    }

    /// Create a Duration that lasts for `micros` microseconds
    ///
    /// This cannot panic.
    pub fn from_micros(_micros: i64) -> Duration {
        todo!()
    }

    /// Create a Duration that lasts for `millis` milliseconds
    ///
    /// This cannot panic.
    pub fn from_millis(_millis: i64) -> Duration {
        todo!()
    }

    /// Create a Duration that lasts for `secs` seconds
    ///
    /// This cannot panic.
    pub fn from_secs(_secs: i64) -> Duration {
        todo!()
    }

    /// Create a Duration that lasts for `mins` minutes
    ///
    /// This cannot panic.
    pub fn from_minutes(_mins: i64) -> Duration {
        todo!()
    }

    /// Create a Duration that lasts for `hours` hours
    ///
    /// This cannot panic.
    pub fn from_hours(_hours: i64) -> Duration {
        todo!()
    }

    /// Retrieve the number of nanoseconds this Duration is
    pub fn nanos(&self) -> i128 {
        todo!()
    }

    /// Retrieve the number of microseconds this Duration is
    pub fn micros(&self) -> i128 {
        todo!()
    }

    /// Retrieve the number of milliseconds this Duration is
    pub fn millis(&self) -> i128 {
        todo!()
    }

    /// Retrieve the number of seconds this Duration is
    pub fn secs(&self) -> i128 {
        todo!()
    }

    /// Retrieve the number of minutes this Duration is
    pub fn mins(&self) -> i128 {
        todo!()
    }

    /// Retrieve the number of hours this Duration is
    pub fn hours(&self) -> i128 {
        todo!()
    }

    /// Add `rhs`, returning `None` on overflow
    pub fn checked_add(&self, _rhs: &Duration) -> Option<Self> {
        todo!()
    }

    /// Subtract `rhs`, returning `None` on overflow
    pub fn checked_sub(&self, _rhs: &Duration) -> Option<Self> {
        todo!()
    }
}

impl Add<Duration> for Duration {
    type Output = Duration;

    fn add(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl AddAssign<Duration> for Duration {
    fn add_assign(&mut self, _rhs: Duration) {
        todo!()
    }
}

impl Sub<Duration> for Duration {
    type Output = Duration;

    fn sub(self, _rhs: Duration) -> Self::Output {
        todo!()
    }
}

impl SubAssign<Duration> for Duration {
    fn sub_assign(&mut self, _rhs: Duration) {
        todo!()
    }
}

impl Default for Duration {
    fn default() -> Self {
        todo!()
    }
}

impl Debug for Duration {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Duration {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
