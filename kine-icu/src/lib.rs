use icu_calendar::types::{IsoSecond, NanoSecond};
use kine_core::{Calendar, CalendarTime, OffsetTime, TimeResult, TimeZone, WrittenTimeResult};

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

impl<Ca, Tz> Cal<Ca, Tz>
where
    Ca: Clone + icu_calendar::AsCalendar,
    Tz: Clone + TimeZone,
    <Tz as TimeZone>::Sigil: Clone,
{
    fn write_impl(
        &self,
        offset_time: OffsetTime<Tz::Sigil>,
    ) -> kine_core::Result<WrittenTimeResult<Time<Ca, Tz>>> {
        let pseudo_nanos = offset_time.as_pseudo_nanos_since_posix_epoch();
        let extra_nanos = i128::from(offset_time.extra_nanos());
        let minutes = pseudo_nanos / NANOS_PER_MINS;
        let submin_pseudo_nanos = pseudo_nanos % NANOS_PER_MINS + extra_nanos;
        let seconds = submin_pseudo_nanos / NANOS_PER_SECS;
        let nanos = submin_pseudo_nanos % NANOS_PER_SECS;

        let minutes = i32::try_from(minutes).map_err(|_| kine_core::Error::OutOfRange)?;
        let mut res = icu_calendar::DateTime::from_minutes_since_local_unix_epoch(minutes);
        res.time.second = IsoSecond::try_from(u8::try_from(seconds).unwrap()).unwrap();
        res.time.nanosecond = NanoSecond::try_from(u32::try_from(nanos).unwrap()).unwrap();

        Ok(WrittenTimeResult::One(Time {
            tz: offset_time.sigil().clone(),
            time: res.to_calendar(self.cal.clone()),
        }))
    }
}

impl<Ca, Tz> Calendar for Cal<Ca, Tz>
where
    Ca: Clone + icu_calendar::AsCalendar,
    Tz: Clone + TimeZone,
    <Tz as TimeZone>::Sigil: Clone,
{
    type Time = Time<Ca, Tz>;

    fn write(&self, t: &kine_core::Time) -> kine_core::Result<WrittenTimeResult<Self::Time>> {
        Ok(match t.write(self.tz.clone())? {
            WrittenTimeResult::One(t) => self.write_impl(t)?,
            WrittenTimeResult::Many(t) => WrittenTimeResult::Many(self.write_impl(t)?.any()),
        })
    }
}

impl<Ca, Tz> CalendarTime for Time<Ca, Tz>
where
    Ca: icu_calendar::AsCalendar,
    Tz: TimeZone,
    <Tz as TimeZone>::Sigil: Clone,
{
    fn read(&self) -> kine_core::Result<TimeResult> {
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
