use core::{
    fmt::{self, Debug, Display},
    str::FromStr,
};

use crate::{
    leap_seconds::{self, LeapSecondedTime, SystemLeapSecondProvider},
    Calendar, CalendarTime,
};

/// A calendar that counts POSIX timestamps
///
/// This is included in `kine-core` because most platforms currently only allow a program to
/// know what posix time is now, and not directly what actual time it is now, even though
/// posix is not a linear time system and thus doing so is actually more complex for them than
/// just giving out a linear time difference to some real-world epoch. Legacy being what it is,
/// this will probably not change in the foreseeable future.
pub struct Posix;

/// A posix timestamp, precise to the nanosecond
pub struct PosixTime(LeapSecondedTime<leap_seconds::SystemProvider>);

// TODO: introduce a PosixDuration type with all the afferent Add/Sub(Assign) implementations

impl Calendar for Posix {
    type Time = PosixTime;

    fn write(&self, t: &crate::Time) -> crate::Result<crate::WrittenTimeResult<Self::Time>> {
        <leap_seconds::SystemProvider as SystemLeapSecondProvider>::write(t)
            .map(|r| r.map(PosixTime))
    }
}

impl CalendarTime for PosixTime {
    fn read(&self) -> crate::Result<crate::TimeResult> {
        <leap_seconds::SystemProvider as SystemLeapSecondProvider>::read(&self.0)
    }
}

impl Display for PosixTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for PosixTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl FromStr for PosixTime {
    type Err = leap_seconds::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(FromStr::from_str(s)?))
    }
}
