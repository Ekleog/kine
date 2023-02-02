mod time;
mod timezone;

use std::marker::PhantomData;

use kine_core::Calendar;

pub use timezone::TimeZone;

/// Gregorian calendar (as defined by ISO 8601) for timezone `Tz`
pub struct Gregorian<Tz: TimeZone>(PhantomData<Tz>);

impl<Tz: TimeZone> Calendar for Gregorian<Tz> {
    type Time = time::GregorianTime<Tz>;

    fn write(
        &self,
        _t: &kine_core::Time,
    ) -> kine_core::Result<kine_core::WrittenTimeResult<Self::Time>> {
        todo!()
    }
}
