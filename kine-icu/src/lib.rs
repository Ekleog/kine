use kine_core::{CalendarTime, OffsetTime, TimeZone};

const NANOS_PER_SECS: i128 = 1_000_000_000;
const NANOS_PER_MINS: i128 = 60 * NANOS_PER_SECS;

/// A time represented with calendar `Cal` and timezone `Tz`
pub struct Time<Cal: icu_calendar::AsCalendar, Tz: TimeZone> {
    tz: Tz::Sigil,
    time: icu_calendar::DateTime<Cal>,
}

impl<Cal, Tz> CalendarTime for Time<Cal, Tz>
where
    Cal: icu_calendar::AsCalendar,
    Tz: TimeZone,
{
    fn read(&self) -> kine_core::Result<kine_core::TimeResult> {
        let time = self.time.to_calendar(icu_calendar::Iso);
        let local_mins = i128::from(time.minutes_since_local_unix_epoch());
        // TODO: revisit after https://github.com/unicode-org/icu4x/issues/3085 answered
        let seconds_outside_leap = time.time.second.number();
        let nanos_outside_leap = match seconds_outside_leap {
            60 => 0, // Already counted in seconds
            _ => i128::from(time.time.nanosecond.number()),
        };
        let extra_nanos = match seconds_outside_leap {
            60 => u64::from(time.time.nanosecond.number()),
            _ => 0,
        };
        let local_nanos = local_mins * NANOS_PER_MINS
            + i128::from(seconds_outside_leap) * NANOS_PER_SECS
            + nanos_outside_leap;
        let offset_time = OffsetTime::from_pseudo_nanos_since_posix_epoch(
            self.tz.clone(),
            local_nanos,
            extra_nanos,
        );
        offset_time.read()
    }
}
