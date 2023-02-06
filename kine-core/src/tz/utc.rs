use core::{fmt::Display, str::FromStr};

use crate::{Calendar, CalendarTime, OffsetTime, Sigil, TimeZone};

use super::InvalidSigil;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Utc;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UtcSigil;

impl TimeZone for Utc {
    type Sigil = UtcSigil;

    fn get_sigil(&self) -> &Self::Sigil {
        &UtcSigil
    }
}

impl Calendar for Utc {
    type Time = OffsetTime<UtcSigil>;

    fn write(&self, t: &crate::Time) -> crate::Result<crate::WrittenTimeResult<Self::Time>> {
        Ok(crate::providers::UTC.write(t)?.map(|t| {
            Self::Time::from_pseudo_nanos_since_posix_epoch(
                UtcSigil,
                t.as_pseudo_nanos_since_posix_epoch(),
                t.extra_nanos(),
            )
        }))
    }
}

impl Sigil for UtcSigil {
    fn read(&self, t: &OffsetTime<Self>) -> crate::Result<crate::TimeResult> {
        OffsetTime::<crate::providers::UtcSigil>::from_pseudo_nanos_since_posix_epoch(
            *crate::providers::UTC.get_sigil(),
            t.as_pseudo_nanos_since_posix_epoch(),
            t.extra_nanos(),
        )
        .read()
    }
}

impl Display for UtcSigil {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Z")
    }
}

impl FromStr for UtcSigil {
    type Err = InvalidSigil;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Z" => Ok(UtcSigil),
            " UTC" => Ok(UtcSigil),
            _ => Err(InvalidSigil),
        }
    }
}
