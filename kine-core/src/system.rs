use core::{
    fmt::{self, Debug, Display},
    str::FromStr,
};

use crate::{
    leap_seconds::{self, LeapSecondProvider, LeapSecondedTime, SYSTEM_PROVIDER},
    Calendar, CalendarTime, TimeResult, WrittenTimeResult,
};

/// A calendar that counts time like the current system clock
///
/// On most platforms this will be either POSIX accounting or UTC-SLS accounting, depending
/// on how NTP is set.
///
/// Note that POSIX time accounting leads to the clock being stalled for one second, which
/// means that during that second it will become impossible to know how much time has elapsed.
/// Unfortunately, this means that on such systems it is basically impossible to have precise
/// time accounting during leap seconds. As such, even `Time::now()` may return wrong results
/// at these times, in a way similar to if the system clock were not set correctly.
///
/// The best idea to improve on this would be to juggle between different clocks so that we
/// could use a fallback clock (like `CLOCK_MONOTONIC`) during a leap seconds. Contributions
/// are welcome.
pub struct System;

/// A time clock that ticks like the current system clock
///
/// On most platforms this will be either POSIX accounting or UTC-SLS accounting, depending
/// on how NTP is set.
///
/// Note that as this does not know of a calendar, its string functions will be giving out
/// raw numbers that may not be user-friendly.
pub struct SystemTime(LeapSecondedTime<leap_seconds::SystemProvider>);

// TODO: introduce a SystemDuration type with all the afferent Add/Sub(Assign) implementations?

impl System {
    /// Retrieve the current date according to the system clock
    ///
    /// Note that this is infallible, as the system calendar is by definition always able to
    /// store a time returned by the system clock.
    #[cfg(feature = "std")]
    pub fn now() -> SystemTime {
        let duration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Current time was before posix epoch");
        let pseudo_nanos = i128::try_from(duration.as_nanos())
            .expect("Overflow trying to retrieve the current time");
        SystemTime(LeapSecondedTime::from_pseudo_nanos_since_posix_epoch(
            SYSTEM_PROVIDER.clone(),
            pseudo_nanos,
            false,
        ))
    }
}

impl Calendar for System {
    type Time = SystemTime;

    fn now(&self) -> WrittenTimeResult<Self::Time> {
        WrittenTimeResult::One(Self::now())
    }

    fn write(&self, t: &crate::Time) -> crate::Result<WrittenTimeResult<Self::Time>> {
        SYSTEM_PROVIDER.write(t).map(|r| r.map(SystemTime))
    }
}

impl CalendarTime for SystemTime {
    fn read(&self) -> crate::Result<TimeResult> {
        SYSTEM_PROVIDER.read(&self.0)
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
