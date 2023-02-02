use crate::{CalendarDuration, Gregorian, TimeZone};

pub struct GregorianDuration<Tz: TimeZone>(Tz);

impl<Tz: TimeZone> CalendarDuration<Gregorian<Tz>> for GregorianDuration<Tz> {
    const ZERO: Self = todo!();

    fn checked_add(&self, _rhs: &<Gregorian<Tz> as crate::Calendar>::Duration) -> Option<Self> {
        todo!()
    }

    fn checked_sub(&self, _rhs: &<Gregorian<Tz> as crate::Calendar>::Duration) -> Option<Self> {
        todo!()
    }

    fn display(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
