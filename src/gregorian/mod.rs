mod duration;
mod time;

use std::marker::PhantomData;

use crate::{Calendar, TimeZone};

/// Gregorian calendar (as defined by ISO 8601) for timezone `Tz`
pub struct Gregorian<Tz: TimeZone>(PhantomData<Tz>);

impl<Tz: TimeZone> Calendar for Gregorian<Tz> {
    type Time = time::GregorianTime<Tz>;

    type Duration = duration::GregorianDuration<Tz>;
}
