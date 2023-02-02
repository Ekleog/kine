use crate::{CalendarTime, Gregorian, TimeZone};

pub struct GregorianTime<Tz: TimeZone>(Tz);

impl<Tz: TimeZone> CalendarTime<Gregorian<Tz>> for GregorianTime<Tz> {
    fn read(&self) -> crate::Result<crate::TimeResult> {
        todo!()
    }

    fn checked_add(&self, _rhs: &<Gregorian<Tz> as crate::Calendar>::Duration) -> Option<Self> {
        todo!()
    }

    fn checked_sub(&self, _rhs: &<Gregorian<Tz> as crate::Calendar>::Duration) -> Option<Self> {
        todo!()
    }

    fn checked_duration_since(
        &self,
        _rhs: &Self,
    ) -> Option<<Gregorian<Tz> as crate::Calendar>::Duration> {
        todo!()
    }

    fn display(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
