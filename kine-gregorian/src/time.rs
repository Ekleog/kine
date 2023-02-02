use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    str::FromStr,
};

use kine_core::CalendarTime;

use crate::TimeZone;

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

impl<Tz: TimeZone> CalendarTime for GregorianTime<Tz> {
    fn read(&self) -> kine_core::Result<kine_core::TimeResult> {
        todo!()
    }
}

impl<Tz: TimeZone> Debug for GregorianTime<Tz> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl<Tz: TimeZone> Display for GregorianTime<Tz> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:09}",
            self.year, self.month, self.day, self.hour, self.minute, self.second, self.nanos,
        )?;
        self.tz.write_designator(f)
    }
}

impl<Tz: TimeZone> FromStr for GregorianTime<Tz> {
    type Err = Infallible;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
