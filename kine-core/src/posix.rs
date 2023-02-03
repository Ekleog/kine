use core::{
    cmp::min,
    fmt::{self, Debug, Display},
    str::FromStr,
};

use crate::{Calendar, CalendarTime};

const NANOS_IN_SEC: i128 = 1_000_000_000;

/// A calendar that counts POSIX timestamps
///
/// This is included in `kine-core` because most platforms currently only allow a program to
/// know what posix time is now, and not directly what actual time it is now, even though
/// posix is not a linear time system and thus doing so is actually more complex for them than
/// just giving out a linear time difference to some real-world epoch. Legacy being what it is,
/// this will probably not change in the foreseeable future.
pub struct Posix;

/// A posix timestamp, precise to the nanosecond
pub struct PosixTime {
    posix_nanos: i128,
}

// TODO: introduce a PosixDuration type with all the afferent Add/Sub(Assign) implementations

impl Calendar for Posix {
    type Time = PosixTime;

    fn write(&self, _t: &crate::Time) -> crate::Result<crate::WrittenTimeResult<Self::Time>> {
        todo!()
    }
}

impl CalendarTime for PosixTime {
    fn read(&self) -> crate::Result<crate::TimeResult> {
        todo!()
    }
}

impl Display for PosixTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}",
            self.posix_nanos / NANOS_IN_SEC,
            (self.posix_nanos % NANOS_IN_SEC).abs(),
        )
    }
}

impl Debug for PosixTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

/// The errors that can arise while parsing a string to a posix timestamp
#[derive(Clone, Debug)]
pub enum ParseError {
    /// An error occurred while trying to parse a presumed integer part of the timestamp
    ParsingInt(core::num::ParseIntError),

    /// The timestamp was out of range
    Overflow,
}

// TODO: impl Error for FromStrError, once Error is in core

impl FromStr for PosixTime {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = s.split('.');
        let seconds = items.next().unwrap(); // split always returns at least one result
        let seconds = i128::from_str(seconds).map_err(ParseError::ParsingInt)?;
        let nanos = match items.next() {
            None | Some("") => 0,
            Some(nanos) => i128::from_str(&nanos[..min(nanos.len(), 9)])
                .map_err(ParseError::ParsingInt)?
                .checked_mul(10_i128.pow((9 - nanos.len()) as u32))
                .unwrap(), // 10 ** 9 is way inside i128 range
        };
        Ok(Self {
            posix_nanos: seconds
                .checked_mul(NANOS_IN_SEC)
                .ok_or(ParseError::Overflow)?
                .checked_add(nanos)
                .ok_or(ParseError::Overflow)?,
        })
    }
}
