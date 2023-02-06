use core::{fmt::Display, str::FromStr};

use crate::{Calendar, CalendarTime, OffsetTime, Sigil, TimeZone};

use super::InvalidSigil;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct System;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SystemSigil;

impl TimeZone for System {
    type Sigil = SystemSigil;

    fn get_sigil(&self) -> &Self::Sigil {
        &SystemSigil
    }
}

impl Calendar for System {
    type Time = OffsetTime<SystemSigil>;

    fn write(&self, t: &crate::Time) -> crate::Result<crate::WrittenTimeResult<Self::Time>> {
        Ok(crate::providers::UTC.write(t)?.map(|t| {
            Self::Time::from_pseudo_nanos_since_posix_epoch(
                SystemSigil,
                t.as_pseudo_nanos_since_posix_epoch(),
                t.extra_nanos(),
            )
        }))
    }
}

impl Sigil for SystemSigil {
    fn read(&self, t: &OffsetTime<Self>) -> crate::Result<crate::TimeResult> {
        OffsetTime::<crate::providers::SystemSigil>::from_pseudo_nanos_since_posix_epoch(
            *crate::providers::UTC.get_sigil(),
            t.as_pseudo_nanos_since_posix_epoch(),
            t.extra_nanos(),
        )
        .read()
    }
}

impl Display for SystemSigil {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Z")
    }
}

impl FromStr for SystemSigil {
    type Err = InvalidSigil;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Z" => Ok(SystemSigil),
            " UTC" => Ok(SystemSigil),
            _ => Err(InvalidSigil),
        }
    }
}
