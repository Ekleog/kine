use std::fmt::{self, Debug, Display};

use icu_calendar::types::{IsoSecond, NanoSecond};
use kine_core::{Calendar, CalendarTime, OffsetTime, TimeResult, TimeZone, WrittenTimeResult};

const NANOS_IN_SECS: i128 = 1_000_000_000;
const NANOS_IN_SECS_U64: u64 = 1_000_000_000;
const NANOS_IN_MINS: i128 = 60 * NANOS_IN_SECS;

pub struct Cal<Ca: icu_calendar::AsCalendar, Tz: TimeZone> {
    cal: Ca,
    tz: Tz,
}

/// A time represented with calendar `Ca` and timezone `Tz`
#[derive(Clone, Eq, PartialEq)]
pub struct Time<Ca: icu_calendar::AsCalendar, Tz: TimeZone> {
    tz: Tz::Sigil,
    time: icu_calendar::DateTime<Ca>,
}

impl<Ca: icu_calendar::AsCalendar, Tz: TimeZone> Cal<Ca, Tz> {
    pub fn new(cal: Ca, tz: Tz) -> Self {
        Self { cal, tz }
    }
}

impl<Ca: icu_calendar::AsCalendar, Tz: TimeZone> Time<Ca, Tz> {
    pub fn new(tz: Tz::Sigil, time: icu_calendar::DateTime<Ca>) -> Self {
        Self { tz, time }
    }
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
        let (minutes, submin_pseudo_nanos) =
            num_integer::div_mod_floor(pseudo_nanos, NANOS_IN_MINS);
        let (seconds, nanos) =
            num_integer::div_mod_floor(submin_pseudo_nanos + extra_nanos, NANOS_IN_SECS);

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
        let seconds_with_leap = time.time.second.number();
        let seconds_without_leap = std::cmp::min(seconds_with_leap, 59);
        let nanos_outside_leap = match seconds_with_leap > 59 {
            true => NANOS_IN_SECS - 1,
            false => i128::from(time.time.nanosecond.number()),
        };
        let extra_nanos = match seconds_with_leap > 59 {
            true => {
                u64::from(time.time.nanosecond.number())
                    + u64::from(seconds_with_leap - 60) * NANOS_IN_SECS_U64
                    + 1
            }
            false => 0,
        };
        let local_nanos = local_mins * NANOS_IN_MINS
            + i128::from(seconds_without_leap) * NANOS_IN_SECS
            + nanos_outside_leap;
        let offset_time = OffsetTime::from_pseudo_nanos_since_posix_epoch(
            self.tz.clone(),
            local_nanos,
            extra_nanos,
        );
        offset_time.read()
    }
}

impl<Tz: TimeZone> Display for Time<icu_calendar::Iso, Tz> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let date = &self.time.date;
        let time = &self.time.time;
        write!(
            f,
            "{}-{:02}-{:02}T{:02}:{:02}:{:02}.{:09}{}",
            date.year().number,
            date.month().ordinal,
            date.day_of_month().0,
            time.hour.number(),
            time.minute.number(),
            time.second.number(),
            time.nanosecond.number(),
            self.tz,
        )
    }
}

impl<Tz: TimeZone> Debug for Time<icu_calendar::Iso, Tz> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use icu_calendar::{
        types::{IsoSecond, NanoSecond},
        Iso,
    };
    use kine_core::{
        tz::{Utc, UTC},
        Calendar, CalendarTime, Duration, TimeResult, TimeZone, WrittenTimeResult,
    };

    use crate::{Cal, Time, NANOS_IN_MINS, NANOS_IN_SECS};

    // icu4x works based on i32 minutes from epoch
    const MIN_NANOS: i128 = -(i32::MIN as i128 * NANOS_IN_MINS);
    const MAX_NANOS: i128 = -(i32::MAX as i128 * NANOS_IN_MINS);

    const NANOS_IN_SECS_U32: u32 = 1_000_000_000;

    fn mktime(nanos: i128) -> kine_core::Time {
        kine_core::Time::POSIX_EPOCH + Duration::from_nanos(nanos)
    }

    #[test]
    fn negative_time_writes_correctly() {
        let time = mktime(-NANOS_IN_MINS);
        let written = time.write(Cal::new(Iso, UTC.clone()));
        let expected =
            icu_calendar::DateTime::try_new_iso_datetime(1969, 12, 31, 23, 59, 10).unwrap();
        assert_eq!(
            written,
            Ok(WrittenTimeResult::One(Time::new(
                UTC.get_sigil().clone(),
                expected
            )))
        );
    }

    #[test]
    fn leap_second_reads_correctly() {
        // Normal behavior with one second
        let mut time: Time<Iso, Utc> = Time::new(
            UTC.get_sigil().clone(),
            icu_calendar::DateTime::try_new_iso_datetime(1969, 12, 31, 23, 59, 60).unwrap(),
        );
        assert_eq!(
            time.read(),
            Ok(TimeResult::One(mktime(-10 * NANOS_IN_SECS)))
        );
        time.time.time.nanosecond = NanoSecond::try_from(NANOS_IN_SECS_U32 / 2).unwrap();
        assert_eq!(
            time.read(),
            Ok(TimeResult::One(mktime(-19 * NANOS_IN_SECS / 2)))
        );

        // Love the 10 seconds at posix epoch
        time.time.time.second = IsoSecond::try_from(61_u8).unwrap();
        assert_eq!(
            time.read(),
            Ok(TimeResult::One(mktime(-17 * NANOS_IN_SECS / 2)))
        );
        time.time.time.second = IsoSecond::try_from(65_u8).unwrap();
        assert_eq!(
            time.read(),
            Ok(TimeResult::One(mktime(-9 * NANOS_IN_SECS / 2)))
        );
        time.time.time.second = IsoSecond::try_from(69_u8).unwrap();
        assert_eq!(time.read(), Ok(TimeResult::One(mktime(-NANOS_IN_SECS / 2))));
    }

    #[test]
    fn negative_time_reads_correctly() {
        let time: Time<Iso, Utc> = Time::new(
            UTC.get_sigil().clone(),
            icu_calendar::DateTime::try_new_iso_datetime(1969, 12, 31, 23, 59, 10).unwrap(),
        );
        let read = time.read();
        let expected = mktime(-NANOS_IN_MINS);
        assert_eq!(read, Ok(TimeResult::One(expected)));
    }

    #[test]
    fn iso_conversion_round_trip() {
        bolero::check!().with_type::<i128>().for_each(|&t| {
            let assert_out_of_range = |t| {
                assert!(
                    t < MIN_NANOS || t > MAX_NANOS,
                    "Returned out of range for time {t} that is not close to the ends of the range"
                )
            };
            let time = kine_core::Time::POSIX_EPOCH + Duration::from_nanos(t);
            let cal = Cal::new(Iso, UTC.clone());
            let formatted = match cal.write(&time) {
                Err(kine_core::Error::OutOfRange) => {
                    assert_out_of_range(t);
                    return;
                }
                Ok(WrittenTimeResult::Many(r)) => {
                    panic!("Converting time to formatted ISO time returned multiple results, like {r:?}")
                }
                Ok(WrittenTimeResult::One(t)) => t,
            };
            let time_bis = match formatted.read() {
                Err(kine_core::Error::OutOfRange) => {
                    assert_out_of_range(t);
                    return;
                }
                Ok(TimeResult::One(t)) => t,
                Ok(t) => panic!(
                    "Converting formatted ISO time to time did not return exactly one result: {t:?}"
                ),
            };
            assert_eq!(
                time, time_bis,
                "Round-tripping through formatted ISO time lost information"
            );
        })
    }
}
