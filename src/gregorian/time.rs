use crate::{CalendarTime, Gregorian, TimeZone};

use super::duration::GregorianDuration;

pub struct GregorianTime<Tz: TimeZone>(Tz);

pub enum GregorianParseError {}

impl<Tz: TimeZone> CalendarTime<Gregorian<Tz>> for GregorianTime<Tz> {
    type ParseError = GregorianParseError;

    fn from_time(_t: &crate::Time) -> crate::Result<crate::WrittenTimeResult<Self>> {
        todo!()
    }

    fn as_time(&self) -> crate::Result<crate::TimeResult> {
        todo!()
    }

    fn checked_add(&self, _rhs: &GregorianDuration<Tz>) -> Option<Self> {
        todo!()
    }

    fn checked_sub(&self, _rhs: &GregorianDuration<Tz>) -> Option<Self> {
        todo!()
    }

    fn checked_duration_since(&self, _rhs: &Self) -> Option<GregorianDuration<Tz>> {
        todo!()
    }

    fn display(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }

    fn from_str(_s: &str) -> Result<Self, Self::ParseError> {
        todo!()
    }
}
