use std::marker::PhantomData;

use crate::{CalendarDuration, Gregorian, TimeZone};

pub struct GregorianDuration<Tz: TimeZone> {
    tz: PhantomData<Tz>,
    years: i64,
    months: i64,
    weeks: i64,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
    nanoseconds: i128,
}

impl<Tz: TimeZone> CalendarDuration<Gregorian<Tz>> for GregorianDuration<Tz> {
    const ZERO: Self = todo!();

    fn checked_add(&self, rhs: &<Gregorian<Tz> as crate::Calendar>::Duration) -> Option<Self> {
        Some(Self {
            tz: PhantomData,
            years: self.years.checked_add(rhs.years)?,
            months: self.months.checked_add(rhs.months)?,
            weeks: self.weeks.checked_add(rhs.weeks)?,
            days: self.days.checked_add(rhs.days)?,
            hours: self.hours.checked_add(rhs.hours)?,
            minutes: self.minutes.checked_add(rhs.minutes)?,
            seconds: self.seconds.checked_add(rhs.seconds)?,
            nanoseconds: self.nanoseconds.checked_add(rhs.nanoseconds)?,
        })
    }

    fn checked_sub(&self, rhs: &<Gregorian<Tz> as crate::Calendar>::Duration) -> Option<Self> {
        Some(Self {
            tz: PhantomData,
            years: self.years.checked_sub(rhs.years)?,
            months: self.months.checked_sub(rhs.months)?,
            weeks: self.weeks.checked_sub(rhs.weeks)?,
            days: self.days.checked_sub(rhs.days)?,
            hours: self.hours.checked_sub(rhs.hours)?,
            minutes: self.minutes.checked_sub(rhs.minutes)?,
            seconds: self.seconds.checked_sub(rhs.seconds)?,
            nanoseconds: self.nanoseconds.checked_sub(rhs.nanoseconds)?,
        })
    }

    fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: think of i18n?
        let mut first = true;
        macro_rules! show {
            ( $var:ident ) => {
                #[allow(unused_assignments)]
                if self.$var != 0 {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, concat!("{} ", stringify!($var)), self.$var)?;
                    first = false;
                }
            };
        }
        show!(years);
        show!(months);
        show!(weeks);
        show!(days);
        show!(hours);
        show!(minutes);
        show!(seconds);
        show!(nanoseconds);
        Ok(())
    }
}
