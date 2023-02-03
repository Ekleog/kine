use core::{fmt::Display, str::FromStr};

use crate::{Calendar, Time, TimeResult, WrittenTimeResult};

use super::{LeapSecondProvider, LeapSecondSigil, LeapSecondedTime};

/// Name of the IERS Bulletin from which this list was taken (as a sigil)
const BULLETIN: &str = " IERS-C65";

const NANOS_IN_SECS: i128 = 1_000_000_000;

macro_rules! make_table {
    ( $( ( $posix:expr, $offset:expr ), )* ) => {
        [ $(
            (
                Time::from_posix_secs($posix + $offset),
                LeapSecondedTime::from_pseudo_nanos_since_posix_epoch(
                    BuiltinIersSigil,
                    $posix * NANOS_IN_SECS,
                    0,
                )
            )
        ),* ]
    }
}

/// List of leap seconds. Between -infinity and the first item here, the two are assumed to
/// be the same. Between item N (included) and N + 1 (excluded, or +infinity), the two times
/// advance linearly, in sync.
///
/// A leap second happens when the offset between the elements of item N + 1 and N are not
/// the same.
static LEAP_SECS: &[(Time, LeapSecondedTime<BuiltinIersSigil>)] = &make_table![
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
#[derive(Clone, Copy)]
pub struct BuiltinIers;

impl BuiltinIers {
    // TODO: remove once it's possible to make it explicit that Default is implemented constly
    pub(super) const fn const_default() -> BuiltinIers {
        BuiltinIers
    }
}

impl LeapSecondProvider for BuiltinIers {
    type Sigil = BuiltinIersSigil;

    fn get_sigil(&self) -> &BuiltinIersSigil {
        &BuiltinIersSigil
    }
}

impl Calendar for BuiltinIers {
    type Time = LeapSecondedTime<BuiltinIersSigil>;

    fn write(&self, _t: &Time) -> crate::Result<WrittenTimeResult<Self::Time>> {
        todo!()
    }
}

impl Default for BuiltinIers {
    fn default() -> Self {
        Self
    }
}

/// The sigil for the built-in IERS table
#[derive(Clone, Copy, Debug)]
pub struct BuiltinIersSigil;

impl LeapSecondSigil for BuiltinIersSigil {
    fn read(&self, _t: &LeapSecondedTime<Self>) -> crate::Result<TimeResult> {
        todo!()
    }
}

impl Display for BuiltinIersSigil {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(BULLETIN)
    }
}

/// The passed sigil was not one of the valid ones
///
/// Valid ones are "", " IERS" and " IERS-C??" where "??" is the number of the bulletin
/// that is currently being built-in.
#[derive(Clone, Copy, Debug)]
pub struct InvalidSigil;

// TODO: derive Error for InvalidSigil

impl FromStr for BuiltinIersSigil {
    type Err = InvalidSigil;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" | " IERS" | BULLETIN => Ok(BuiltinIersSigil),
            _ => Err(InvalidSigil),
        }
    }
}
