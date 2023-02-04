use kine_core::{Calendar, CalendarTime, OffsetTime, TimeZone};

const NANOS_PER_SECS: i128 = 1_000_000_000;
const NANOS_PER_MINS: i128 = 60 * NANOS_PER_SECS;

pub struct Cal<Ca: icu_calendar::AsCalendar, Tz: TimeZone> {
    cal: Ca,
    tz: Tz,
}

/// A time represented with calendar `Ca` and timezone `Tz`
pub struct Time<Ca: icu_calendar::AsCalendar, Tz: TimeZone> {
    tz: Tz::Sigil,
    time: icu_calendar::DateTime<Ca>,
}

impl<Ca, Tz> Calendar for Cal<Ca, Tz>
where
    Ca: icu_calendar::AsCalendar,
    Tz: Clone + TimeZone,
    <Tz as TimeZone>::Sigil: Clone,
{
    type Time = Time<Ca, Tz>;

    fn write(
        &self,
        t: &kine_core::Time,
    ) -> kine_core::Result<kine_core::WrittenTimeResult<Self::Time>> {
        let offset_time = t.write(self.tz.clone());
        todo!()
    }
}

impl<Ca, Tz> CalendarTime for Time<Ca, Tz>
where
    Ca: icu_calendar::AsCalendar,
    Tz: TimeZone,
    <Tz as TimeZone>::Sigil: Clone,
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
