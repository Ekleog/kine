use core::{
    fmt::{self, Debug, Display},
    str::FromStr,
};

use crate::{
    leap_seconds::{self, LeapSecondedTime, SystemLeapSecondProvider},
    Calendar, CalendarTime,
};

/// A calendar that counts time like the current system clock
///
/// On most platforms this will be either POSIX accounting or UTC-SLS accounting, depending
/// on how NTP is set.
pub struct System;

/// A calendar that counts time like the current system clock
///
/// On most platforms this will be either POSIX accounting or UTC-SLS accounting, depending
/// on how NTP is set.
///
/// Note that as this does not know of a calendar, its string functions will be giving out
/// raw numbers that may not be user-friendly.
pub struct SystemTime(LeapSecondedTime<leap_seconds::SystemProvider>);

// TODO: introduce a PosixDuration type with all the afferent Add/Sub(Assign) implementations

impl Calendar for System {
    type Time = SystemTime;

    fn write(&self, t: &crate::Time) -> crate::Result<crate::WrittenTimeResult<Self::Time>> {
        <leap_seconds::SystemProvider as SystemLeapSecondProvider>::write(t)
            .map(|r| r.map(SystemTime))
    }
}

impl CalendarTime for SystemTime {
    fn read(&self) -> crate::Result<crate::TimeResult> {
        <leap_seconds::SystemProvider as SystemLeapSecondProvider>::read(&self.0)
    }
}

impl Display for SystemTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for SystemTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl FromStr for SystemTime {
    type Err = leap_seconds::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(FromStr::from_str(s)?))
    }
}
