use core::fmt::{self, Debug};

use crate::{providers, Calendar, CalendarTime, OffsetTime, TimeResult, WrittenTimeResult};

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
pub struct SystemTime(OffsetTime<providers::SystemSigil>);

// TODO: introduce a SystemDuration type with all the afferent Add/Sub(Assign) implementations?

impl System {
    /// Retrieve the current date according to the system clock
    ///
    /// Note that this is infallible, as the system calendar is by definition always able to
    /// store a time returned by the system clock.
    ///
    /// However, it will panic if running on no-std and not having a known alternative
    /// implementation for time retrieval.
    pub fn now() -> SystemTime {
        Self::now_impl()
    }

    // TODO: split this into a separate feature so that later on when extern existential types come
    // in we can allow extern implementations of this?
    #[cfg(feature = "std")]
    fn now_impl() -> SystemTime {
        use crate::TimeZone;

        let duration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Current time was before posix epoch");
        let pseudo_nanos = i128::try_from(duration.as_nanos())
            .expect("Overflow trying to retrieve the current time");
        // No extra nanoseconds for the `std`-based implementation because it'd be impossible to know
        let extra_nanos = 0;
        // TODO: Introduce an implementation of `Time::now()` based on other clocks for platforms that
        // can so this is not the best precision we could have.
        SystemTime(OffsetTime::from_pseudo_nanos_since_posix_epoch(
            providers::SYSTEM.get_sigil().clone(),
            pseudo_nanos,
            extra_nanos,
        ))
    }

    #[cfg(not(feature = "std"))]
    fn now_impl() -> SystemTime {
        panic!("Running on no-std with no known implementation of time-getting");
    }
}

impl Calendar for System {
    type Time = SystemTime;

    fn try_now(&self) -> crate::Result<WrittenTimeResult<Self::Time>> {
        Ok(WrittenTimeResult::One(Self::now()))
    }

    fn now(&self) -> WrittenTimeResult<Self::Time> {
        WrittenTimeResult::One(Self::now())
    }

    fn write(&self, t: &crate::Time) -> crate::Result<WrittenTimeResult<Self::Time>> {
        providers::SYSTEM.write(t).map(|r| r.map(SystemTime))
    }
}

impl CalendarTime for SystemTime {
    fn read(&self) -> crate::Result<TimeResult> {
        self.0.read()
    }
}

impl Debug for SystemTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}
