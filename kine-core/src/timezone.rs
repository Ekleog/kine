use core::{
    cmp::min,
    fmt::{self, Debug, Display},
    str::FromStr,
};

use crate::{Calendar, CalendarTime, TimeResult};

const NANOS_IN_SECS: i128 = 1_000_000_000;

/// A time zone
///
/// Time zones usually offset the visible time by some amount, and do not
pub trait TimeZone: Calendar<Time = OffsetTime<Self::Sigil>> + Eq + PartialEq {
    /// The sigil type associated to this time zone
    ///
    /// This is basically metadata added to all `OffsetTime`s.
    type Sigil: Sigil;
}

/// The sigil for a time zone
pub trait Sigil: Display + Eq + FromStr + PartialEq {
    /// Read the given time-with-offset
    fn read(&self, t: &OffsetTime<Self>) -> crate::Result<TimeResult>;
}

/// A time on a time scale that needs to deal with leap seconds
///
/// It is represented by both `pseudo_nanos`,the number of nanoseconds between
/// the POSIX epoch and the point in time according to this time scale, and
/// `extra_nanos`, the number of (real) nanoseconds since we entered the current
/// leap second and `pseudo_nanos` froze.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OffsetTime<Sig> {
    sigil: Sig,
    pseudo_nanos: i128,
    extra_nanos: u64,
}

impl<Sig> OffsetTime<Sig> {
    /// Build a `LeapSecondedTime` from the number of pseudo-nanoseconds between this time
    /// and the POSIX epoch
    ///
    /// `pseudo_nanos` represent the number of nanoseconds since the POSIX epoch, and
    /// `extra_nanos` the number of real-world nanoseconds that elapsed since the time at
    /// which `pseudo-nanos` froze, which can be used to represent leap seconds.
    ///
    /// Note that no attempt is made to validate that this time is actually correct for
    /// timezone, be it due to leap seconds being invalidly set or to `pseudo_nanos`
    /// requesting a time that never existed eg. due to a timezone shift. If you manually
    /// build an `OffsetTime` with invalid values, you may see strange results. This
    /// function is mostly exposed for the implementers of `TimeZone` themselves.
    // TODO: Get rid of the command above by introducing a "token" that is
    // timezone-specific to prove that the function call came from the `TimeZone`? But
    // then how is eg. `kine-icu` supposed to generate the `OffsetTime` when reading
    // a calendar date?
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

impl<Sig: Sigil> CalendarTime for OffsetTime<Sig> {
    fn read(&self) -> crate::Result<crate::TimeResult> {
        self.sigil.read(self)
    }
}

impl<Sig: Display> Display for OffsetTime<Sig> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.pseudo_nanos / NANOS_IN_SECS;
        let nanos = (self.pseudo_nanos % NANOS_IN_SECS).abs();
        if self.pseudo_nanos < 0 && secs == 0 {
            f.write_str("-")?; // seconds will display without the minus if it is 0
        }
        write!(f, "{secs}.{nanos:09}{}", self.sigil)
    }
}

impl<Sig: Display> Debug for OffsetTime<Sig> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.pseudo_nanos / NANOS_IN_SECS;
        let nanos = (self.pseudo_nanos % NANOS_IN_SECS).abs();
        if self.pseudo_nanos < 0 && secs == 0 {
            f.write_str("-")?; // seconds will display without the minus if it is 0
        }
        write!(
            f,
            "{secs}.{nanos:09}(+{}ns){}",
            self.extra_nanos, self.sigil
        )
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

impl<Sig: FromStr> FromStr for OffsetTime<Sig> {
    type Err = ParseError<Sig::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let non_ascii_digit = |c: char| !c.is_ascii_digit();
        let first_non_digit = s.find(non_ascii_digit).unwrap_or(s.len());
        let (seconds, rest) = s.split_at(first_non_digit);
        let seconds = i128::from_str(seconds).map_err(ParseError::ParsingInt)?;
        let (nanos, rest) = match rest.strip_prefix('.') {
            None => (0, rest),
            Some(rest) => {
                let first_non_digit = min(9, rest.find(non_ascii_digit).unwrap_or(s.len()));
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

#[cfg(test)]
mod tests {
    use crate::{leap_seconds::BuiltinIersSigil, OffsetTime};

    #[test]
    fn display_small_negative_values_properly() {
        let t = OffsetTime::from_pseudo_nanos_since_posix_epoch(BuiltinIersSigil, -1, 10);
        let iers_sigil = BuiltinIersSigil;
        assert_eq!(format!("{t}"), format!("-0.000000001{iers_sigil}"));
        assert_eq!(format!("{t:?}"), format!("-0.000000001(+10ns){iers_sigil}"));
    }
}
