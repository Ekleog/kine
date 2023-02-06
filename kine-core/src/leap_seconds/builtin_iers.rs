use core::{fmt::Display, str::FromStr};

use crate::{
    tz::InvalidSigilError, Calendar, Duration, Error, OffsetTime, Sigil, Time, TimeResult, TimeZone,
};

const NANOS_IN_SECS: i128 = 1_000_000_000;

macro_rules! make_table {
    ( $( ( $posix:expr, $offset:expr ), )* ) => {
        [ $(
            (
                // TODO: replace with .unwrap() when const_option_ext is stable
                match Time::from_tai_nanos(($posix + $offset) * NANOS_IN_SECS) {
                    Some(t) => t,
                    None => panic!("Ill-formed leap second table"),
                },
                OffsetTime::from_pseudo_nanos_since_posix_epoch(
                    BuiltinIersSigil,
                    $posix * NANOS_IN_SECS,
                    0,
                )
            )
        ),* ]
    }
}

/// Name of the IERS Bulletin from which this list was taken (as a sigil)
const BULLETIN: &str = " IERS-C65";

/// LeapSecondedTime::as_pseudo_nanos_from_posix_epoch() - Time::as_tai_nanos() before the
/// first leap second
const OFFSET_BEFORE_FIRST_LEAP: i128 = 0;

/// List of leap seconds. Between -infinity and the first item here, the two are assumed to
/// be the same. At item N, `LeapSecondedTime` jumps to the value specified in `Time`.
/// Between item N (included) and N + 1 (excluded, or +infinity), the two times advance
/// linearly, in sync.
///
/// A leap second happens when the offset between the elements of item N + 1 and N are not
/// the same.
static LEAP_SECS: [(Time, OffsetTime<BuiltinIersSigil>); 28] = make_table![
    (0, 10),
    (15_724_800, 11),
    (31_622_400, 12),
    (63_158_400, 13),
    (94_694_400, 14),
    (126_230_400, 15),
    (157_852_800, 16),
    (189_388_800, 17),
    (220_924_800, 18),
    (252_460_800, 19),
    (299_721_600, 20),
    (331_257_600, 21),
    (362_793_600, 22),
    (425_952_000, 23),
    (504_921_600, 24),
    (568_080_000, 25),
    (599_616_000, 26),
    (646_876_800, 27),
    (678_412_800, 28),
    (709_948_800, 29),
    (757_382_400, 30),
    (804_643_200, 31),
    (852_076_800, 32),
    (1_073_001_600, 33),
    (1_167_696_000, 34),
    (1_278_028_800, 35),
    (1_372_636_800, 36),
    (1_420_156_800, 37),
];
// TODO: add a(n impure) test checking that this is up-to-date with the latest
// downloaded file

/// Leap second provider that uses the builtin, latest-at-the-time-of-last-`kine-core`-update
/// IERS leap second table.
// TODO: Make Copy again when clippy no longer complains about an unused clone
// (ie. when System will be a proper existential trait)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BuiltinIers;

impl BuiltinIers {
    // TODO: remove once it's possible to make it explicit that Default is implemented constly
    pub(crate) const fn const_default() -> BuiltinIers {
        BuiltinIers
    }
}

impl TimeZone for BuiltinIers {
    type Sigil = BuiltinIersSigil;
}

impl Calendar for BuiltinIers {
    type Time = OffsetTime<BuiltinIersSigil>;

    fn write(&self, t: &Time) -> crate::Result<Self::Time> {
        // Find the time in the leap seconds table
        let search = LEAP_SECS.binary_search_by_key(t, |(p, _)| *p);

        // Handle the easy cases of time at a leap second or after the last leap second
        let id_after = match search {
            Ok(i) => return Ok(LEAP_SECS[i].1),
            Err(i) if i == LEAP_SECS.len() => {
                let (base, leaped) = LEAP_SECS.last().unwrap();
                let pseudo_nanos =
                    leaped.as_pseudo_nanos_since_posix_epoch() + (*t - *base).nanos(); // TODO: remove derefs once Add correctly impl'd
                return Ok(OffsetTime::from_pseudo_nanos_since_posix_epoch(
                    BuiltinIersSigil,
                    pseudo_nanos,
                    0, // No extra nanos after last leap second
                ));
            }
            Err(i) => i,
        };

        // Handle the hard case of time that may be on a leap second
        // First, figure out what the nanoseconds _should be_ if there were no future leap second
        let should_be = match id_after {
            0 => t.as_tai_nanos().ok_or(Error::OutOfRange)? + OFFSET_BEFORE_FIRST_LEAP,
            i => {
                let (base, leaped) = &LEAP_SECS[i - 1];
                leaped.as_pseudo_nanos_since_posix_epoch() + (*t - *base).nanos()
            }
        };

        // Then, figure out whether the "should be" actually is, or is in the middle of a leap
        let (_next, next_leaped) = &LEAP_SECS[id_after];
        let next_leaped_nanos = next_leaped.as_pseudo_nanos_since_posix_epoch();
        if should_be >= next_leaped_nanos {
            Ok(OffsetTime::from_pseudo_nanos_since_posix_epoch(
                BuiltinIersSigil,
                next_leaped_nanos - 1,
                u64::try_from(should_be - next_leaped_nanos + 1).expect("ill-formed IERS table"),
            ))
        } else {
            Ok(OffsetTime::from_pseudo_nanos_since_posix_epoch(
                BuiltinIersSigil,
                should_be,
                0,
            ))
        }
    }
}

impl Default for BuiltinIers {
    fn default() -> Self {
        Self
    }
}

/// The sigil for the built-in IERS table
///
/// Valid sigils, in written form, are "", " IERS" and " IERS-C??" where "??"
/// is the number of the bulletin that is currently being built-in.
// TODO: Make Copy again when clippy no longer complains about an unused clone
// (ie. when System will be a proper existential trait)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BuiltinIersSigil;

impl BuiltinIersSigil {
    pub(crate) const fn const_default() -> BuiltinIersSigil {
        BuiltinIersSigil
    }
}

impl Sigil for BuiltinIersSigil {
    fn read(&self, t: &OffsetTime<Self>) -> crate::Result<TimeResult> {
        // Find the interval in the leap seconds table (excluding extra time)
        let search = LEAP_SECS
            .binary_search_by_key(&t.as_pseudo_nanos_since_posix_epoch(), |(_, p)| {
                p.as_pseudo_nanos_since_posix_epoch()
            });

        // Then, figure out the associated time, not counting leap seconds yet
        let without_extra_nanos = match search {
            Ok(i) => LEAP_SECS[i].0,
            Err(0) => Time::from_tai_nanos(
                t.as_pseudo_nanos_since_posix_epoch() - OFFSET_BEFORE_FIRST_LEAP,
            )
            .ok_or(Error::OutOfRange)?,
            Err(i) => {
                let (base, leaped) = &LEAP_SECS[i - 1];
                base.checked_add(&Duration::from_nanos(
                    t.as_pseudo_nanos_since_posix_epoch()
                        - leaped.as_pseudo_nanos_since_posix_epoch(),
                ))
                .ok_or(Error::OutOfRange)?
            }
        };

        // Finally, also count the extra nanoseconds
        without_extra_nanos
            .checked_add(&Duration::from_nanos(i128::from(t.extra_nanos())))
            .ok_or(Error::OutOfRange)
            .map(TimeResult::One)
    }
}

impl Display for BuiltinIersSigil {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(BULLETIN)
    }
}

impl FromStr for BuiltinIersSigil {
    type Err = InvalidSigilError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" | " IERS" | BULLETIN => Ok(BuiltinIersSigil),
            _ => Err(InvalidSigilError),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Calendar, CalendarTime, Error, OffsetTime, Time, TimeResult};

    use super::{BuiltinIers, BuiltinIersSigil, NANOS_IN_SECS};

    #[test]
    fn convert_close_to_posix_epoch() {
        const HALF_SEC: i128 = NANOS_IN_SECS / 2;

        // After the posix epoch, all is nice: there is no offset.
        assert_eq!(
            Time::from_nanos_since_posix_epoch(HALF_SEC).write(BuiltinIers),
            Ok(OffsetTime::from_pseudo_nanos_since_posix_epoch(
                BuiltinIersSigil,
                HALF_SEC,
                0
            ))
        );
        assert_eq!(
            OffsetTime::from_pseudo_nanos_since_posix_epoch(BuiltinIersSigil, HALF_SEC, 0).read(),
            Ok(TimeResult::One(Time::from_nanos_since_posix_epoch(
                HALF_SEC
            ))),
        );

        // Before the posix epoch, nothing is nice: there is a 10s offset.
        assert_eq!(
            Time::from_nanos_since_posix_epoch(-HALF_SEC).write(BuiltinIers),
            Ok(OffsetTime::from_pseudo_nanos_since_posix_epoch(
                BuiltinIersSigil,
                -1,
                u64::try_from(19 * HALF_SEC + 1).unwrap(),
            ))
        );
        assert_eq!(
            OffsetTime::from_pseudo_nanos_since_posix_epoch(BuiltinIersSigil, -HALF_SEC, 0).read(),
            Ok(TimeResult::One(Time::from_nanos_since_posix_epoch(
                -21 * HALF_SEC
            ))),
        );
    }

    #[test]
    fn leap_conversion_round_trip() {
        bolero::check!().with_type::<i128>().for_each(|&t| {
            let assert_out_of_range = |t| {
                assert!(
                    t < i128::MIN + 15 * NANOS_IN_SECS || t > i128::MAX - 15 * NANOS_IN_SECS,
                    "Returned out of range for time {t} that is not close to the ends of the range"
                )
            };
            let time = Time::from_nanos_since_posix_epoch(t);
            let leaped = match BuiltinIers.write(&time) {
                Err(Error::OutOfRange) => {
                    assert_out_of_range(t);
                    return;
                }
                Ok(t) => t,
            };
            let time_bis = match leaped.read() {
                Err(Error::OutOfRange) => {
                    assert_out_of_range(t);
                    return;
                }
                Ok(TimeResult::One(t)) => t,
                Ok(t) => panic!(
                    "Converting leaped time to time did not return exactly one result: {t:?}"
                ),
            };
            assert_eq!(
                time, time_bis,
                "Round-tripping through leaped time lost information"
            );
        })
    }
}
