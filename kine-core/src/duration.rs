use core::{
    fmt::{self, Debug, Display},
    ops::{Add, AddAssign, Sub, SubAssign},
};

/// A (signed) Duration.
///
/// It can represent time intervals of roughly 10^22 years either way.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Duration {
    pub(crate) nanos: i128,
}

const NANOS_IN_MICROS: i128 = 1000;
const NANOS_IN_MILLIS: i128 = NANOS_IN_MICROS * 1000;
const NANOS_IN_SECS: i128 = NANOS_IN_MILLIS * 1000;
const NANOS_IN_MINS: i128 = NANOS_IN_SECS * 60;
const NANOS_IN_HOURS: i128 = NANOS_IN_MINS * 60;
const NANOS_IN_DAYS: i128 = NANOS_IN_HOURS * 24;
const NANOS_IN_WEEKS: i128 = NANOS_IN_DAYS * 7;

impl Duration {
    pub const ZERO: Duration = Duration { nanos: 0 };

    /// Create a Duration that lasts for `nanos` nanoseconds
    ///
    /// This cannot panic.
    pub fn from_nanos(nanos: i128) -> Duration {
        Duration { nanos }
    }

    /// Create a Duration that lasts for `micros` microseconds
    ///
    /// This cannot panic.
    pub fn from_micros(micros: i64) -> Duration {
        Duration {
            nanos: micros as i128 * NANOS_IN_MICROS,
        }
    }

    /// Create a Duration that lasts for `millis` milliseconds
    ///
    /// This cannot panic.
    pub fn from_millis(millis: i64) -> Duration {
        Duration {
            nanos: millis as i128 * NANOS_IN_MILLIS,
        }
    }

    /// Create a Duration that lasts for `secs` seconds
    ///
    /// This cannot panic.
    pub fn from_secs(secs: i64) -> Duration {
        Duration {
            nanos: secs as i128 * NANOS_IN_SECS,
        }
    }

    /// Create a Duration that lasts for `mins` minutes
    ///
    /// This cannot panic.
    pub fn from_minutes(mins: i64) -> Duration {
        Duration {
            nanos: mins as i128 * NANOS_IN_MINS,
        }
    }

    /// Create a Duration that lasts for `hours` hours
    ///
    /// This cannot panic.
    pub fn from_hours(hours: i64) -> Duration {
        Duration {
            nanos: hours as i128 * NANOS_IN_HOURS,
        }
    }

    /// Create a Duration that lasts for `days` days
    ///
    /// This cannot panic.
    pub fn from_days(days: i64) -> Duration {
        Duration {
            nanos: days as i128 * NANOS_IN_DAYS,
        }
    }

    /// Create a Duration that lasts for `weeks` weeks
    ///
    /// This cannot panic.
    pub fn from_weeks(weeks: i64) -> Duration {
        Duration {
            nanos: weeks as i128 * NANOS_IN_WEEKS,
        }
    }

    /// Retrieve the number of nanoseconds this Duration is
    pub fn nanos(&self) -> i128 {
        self.nanos
    }

    /// Retrieve the number of microseconds this Duration is (rounded to zero)
    pub fn micros(&self) -> i128 {
        self.nanos / NANOS_IN_MICROS
    }

    /// Retrieve the number of milliseconds this Duration is (rounded to zero)
    pub fn millis(&self) -> i128 {
        self.nanos / NANOS_IN_MILLIS
    }

    /// Retrieve the number of seconds this Duration is (rounded to zero)
    pub fn secs(&self) -> i128 {
        self.nanos / NANOS_IN_SECS
    }

    /// Retrieve the number of minutes this Duration is (rounded to zero)
    pub fn mins(&self) -> i128 {
        self.nanos / NANOS_IN_MINS
    }

    /// Retrieve the number of hours this Duration is (rounded to zero)
    pub fn hours(&self) -> i128 {
        self.nanos / NANOS_IN_HOURS
    }

    /// Retrieve the number of days this Duration is (rounded to zero)
    pub fn days(&self) -> i128 {
        self.nanos / NANOS_IN_DAYS
    }

    /// Retrieve the number of weeks this Duration is (rounded to zero)
    pub fn weeks(&self) -> i128 {
        self.nanos / NANOS_IN_WEEKS
    }

    /// Add `rhs`, returning `None` on overflow
    pub fn checked_add(&self, rhs: &Duration) -> Option<Self> {
        self.nanos
            .checked_add(rhs.nanos)
            .map(|nanos| Duration { nanos })
    }

    /// Subtract `rhs`, returning `None` on overflow
    pub fn checked_sub(&self, rhs: &Duration) -> Option<Self> {
        self.nanos
            .checked_sub(rhs.nanos)
            .map(|nanos| Duration { nanos })
    }
}

impl Add<Duration> for Duration {
    type Output = Duration;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(&rhs)
            .expect("overflow while adding durations")
    }
}

impl AddAssign<Duration> for Duration {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs
    }
}

impl Sub<Duration> for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Self::Output {
        self.checked_sub(&rhs)
            .expect("overflow while subtracting durations")
    }
}

impl SubAssign<Duration> for Duration {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs
    }
}

impl Default for Duration {
    fn default() -> Self {
        Duration::ZERO
    }
}

impl Debug for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Duration as Display>::fmt(self, f)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.nanos % NANOS_IN_HOURS == 0 {
            write!(f, "{}h", self.nanos / NANOS_IN_HOURS)
        } else if self.nanos % NANOS_IN_MINS == 0 {
            write!(f, "{}min", self.nanos / NANOS_IN_MINS)
        } else if self.nanos % NANOS_IN_SECS == 0 {
            write!(f, "{}s", self.nanos / NANOS_IN_SECS)
        } else if self.nanos % NANOS_IN_MILLIS == 0 {
            write!(f, "{}ms", self.nanos / NANOS_IN_MILLIS)
        } else {
            // Do not actually check micros: it's probably not worth it and greek
            // letters may be surprising
            write!(f, "{}ns", self.nanos)
        }
    }
}
