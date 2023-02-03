use core::{
    cmp::min,
    fmt::{self, Debug, Display},
    str::FromStr,
};

use crate::CalendarTime;

use super::LeapSecondSigil;

const NANOS_IN_SECS: i128 = 1_000_000_000;

/// A time on a time scale that needs to deal with leap seconds
///
/// It is represented by both `pseudo_nanos`,the number of nanoseconds between
/// the POSIX epoch and the point in time according to this time scale, and
/// `extra_nanos`, the number of (real) nanoseconds since we entered the current
/// leap second and `pseudo_nanos` froze.
pub struct LeapSecondedTime<Sig> {
    sigil: Sig,
    pseudo_nanos: i128,
    extra_nanos: u64,
}

impl<Sig> LeapSecondedTime<Sig> {
    /// Build a `LeapSecondedTime` from the number of pseudo-nanoseconds between this time
    /// and the POSIX epoch
    pub const fn from_pseudo_nanos_since_posix_epoch(
        sigil: Sig,
        pseudo_nanos: i128,
        extra_nanos: u64,
    ) -> Self {
        Self {
            sigil,
            pseudo_nanos,
            extra_nanos,
        }
    }

    /// Return the number of pseudo-nanoseconds between this time and the POSIX epoch
    pub fn as_pseudo_nanos_since_posix_epoch(&self) -> i128 {
        self.pseudo_nanos
    }

    /// Return the number of nanoseconds that elapsed since the current leap second started
    ///
    /// Returns `0` if not currently in a leap second.
    pub fn extra_nanos(&self) -> u64 {
        self.extra_nanos
    }

    /// Return the sigil associated with this leap second type
    pub fn sigil(&self) -> &Sig {
        &self.sigil
    }
}

impl<Sig: LeapSecondSigil> CalendarTime for LeapSecondedTime<Sig> {
    fn read(&self) -> crate::Result<crate::TimeResult> {
        self.sigil.read(self)
    }
}

impl<Sig: Display> Display for LeapSecondedTime<Sig> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{:09}{}",
            self.pseudo_nanos / NANOS_IN_SECS,
            (self.pseudo_nanos % NANOS_IN_SECS).abs(),
            self.sigil,
        )
    }
}

impl<Sig: Display> Debug for LeapSecondedTime<Sig> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)?;
        if self.extra_nanos != 0 {
            f.write_str("(+1)")?;
        }
        Ok(())
    }
}

/// The errors that can arise while parsing a string to a posix timestamp
#[derive(Clone, Debug)]
pub enum ParseError<SigErr> {
    /// An error occurred while trying to parse a presumed integer part of the timestamp
    ParsingInt(core::num::ParseIntError),

    /// The timestamp was out of range
    Overflow,

    /// Failed parsing sigil
    ParsingSigil(SigErr),
}

// TODO: impl Error for FromStrError, once Error is in core

impl<Sig: FromStr> FromStr for LeapSecondedTime<Sig> {
    type Err = ParseError<Sig::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let non_ascii_digit = |c: char| !c.is_ascii_digit();
        let first_non_digit = s.find(non_ascii_digit).unwrap_or_else(|| s.len());
        let (seconds, rest) = s.split_at(first_non_digit);
        let seconds = i128::from_str(seconds).map_err(ParseError::ParsingInt)?;
        let (nanos, rest) = match rest.strip_prefix('.') {
            None => (0, rest),
            Some(rest) => {
                let first_non_digit = min(9, rest.find(non_ascii_digit).unwrap_or_else(|| s.len()));
                let (nanos, rest) = s.split_at(first_non_digit);
                let nanos = i128::from_str(nanos)
                    .map_err(ParseError::ParsingInt)?
                    .checked_mul(10_i128.pow((9 - nanos.len()) as u32))
                    .unwrap(); // 10 ** 9 is way inside i128 range
                (nanos, rest)
            }
        };
        let sigil = Sig::from_str(rest).map_err(ParseError::ParsingSigil)?;
        Ok(Self {
            sigil,
            pseudo_nanos: seconds
                .checked_mul(NANOS_IN_SECS)
                .ok_or(ParseError::Overflow)?
                .checked_add(nanos)
                .ok_or(ParseError::Overflow)?,
            // Based only on a second (and nanos) number, it is impossible to know if we are in a
            // leap second
            extra_nanos: 0,
        })
    }
}
