use core::{
    cmp::min,
    fmt::{self, Debug, Display},
    str::FromStr,
};

use crate::CalendarTime;

use super::LeapSecondProvider;

const NANOS_IN_SEC: i128 = 1_000_000_000;

/// A time relative
pub struct LeapSecondedTime<Src> {
    source: Src,
    pseudo_nanos: i128,
    in_leap_second: bool,
}

impl<Src> LeapSecondedTime<Src> {
    /// Build a `LeapSecondedTime` from the number of pseudo-nanoseconds between this time
    /// and the POSIX epoch
    pub(crate) fn from_pseudo_nanos_since_posix_epoch(
        source: Src,
        pseudo_nanos: i128,
        in_leap_second: bool,
    ) -> Self {
        Self {
            source,
            pseudo_nanos,
            in_leap_second,
        }
    }

    /// Return the number of pseudo-nanoseconds between this time and the POSIX epoch
    pub fn as_pseudo_nanos_since_posix_epoch(&self) -> i128 {
        self.pseudo_nanos
    }

    /// Return `true` iff this time is currently undergoing a leap second
    pub fn in_leap_second(&self) -> bool {
        self.in_leap_second
    }
}

impl<Src> CalendarTime for LeapSecondedTime<Src>
where
    Src: LeapSecondProvider + Default,
{
    fn read(&self) -> crate::Result<crate::TimeResult> {
        self.source.read(self)
    }
}

impl<Src: LeapSecondProvider> Display for LeapSecondedTime<Src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}{}",
            self.pseudo_nanos / NANOS_IN_SEC,
            (self.pseudo_nanos % NANOS_IN_SEC).abs(),
            self.source.sigil(),
        )
    }
}

impl<Src: LeapSecondProvider> Debug for LeapSecondedTime<Src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)?;
        if self.in_leap_second {
            f.write_str("(+1)")?;
        }
        Ok(())
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

impl<Src: Default> FromStr for LeapSecondedTime<Src> {
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
            source: Src::default(),
            pseudo_nanos: seconds
                .checked_mul(NANOS_IN_SEC)
                .ok_or(ParseError::Overflow)?
                .checked_add(nanos)
                .ok_or(ParseError::Overflow)?,
            // Based only on a second number, it is impossible to differentiate between leap
            // and non-leap
            in_leap_second: false,
        })
    }
}
