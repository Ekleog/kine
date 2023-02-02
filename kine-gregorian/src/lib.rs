mod duration;
mod time;
mod timezone;

use std::{convert::Infallible, marker::PhantomData};

use kine_core::Calendar;

pub use timezone::TimeZone;

/// Gregorian calendar (as defined by ISO 8601) for timezone `Tz`
pub struct Gregorian<Tz: TimeZone>(PhantomData<Tz>);

impl<Tz: TimeZone> Calendar for Gregorian<Tz> {
    type Time = time::GregorianTime<Tz>;

    type Duration = duration::GregorianDuration<Tz>;

    type ParseError = Infallible;

    fn write(
        &self,
        _t: &kine_core::Time,
    ) -> kine_core::Result<kine_core::WrittenTimeResult<Self::Time>> {
        todo!()
    }

    fn from_str(_s: &str) -> Result<Self::Time, Self::ParseError> {
        todo!()
    }
}
