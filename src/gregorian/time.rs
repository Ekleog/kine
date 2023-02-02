use crate::{CalendarTime, Gregorian, TimeZone};

pub struct GregorianTime<Tz: TimeZone> {
    tz: Tz,
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    nanos: u32,
}

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

    fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:09}",
            self.year, self.month, self.day, self.hour, self.minute, self.second, self.nanos,
        )?;
        self.tz.write_designator(f)
    }
}
