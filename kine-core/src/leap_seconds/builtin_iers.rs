use crate::{Time, TimeResult, WrittenTimeResult};

use super::{LeapSecondedTime, SystemLeapSecondProvider};

/// Name of the IERS Bulletin from which this list was taken (as a sigil)
const BULLETIN: &str = " IERS-C65";

/// List of leap seconds. Between -infinity and the first item here, the offset is 0.
/// Between items n (included) and n + 1 (excluded, or +infinity) here, the offset is
/// IERS_LEAP_SECS[n].1
static LEAP_SECS: &[(Time, i32)] = &[
    (Time::from_posix_nanos(0), 10),
    (Time::from_posix_secs(15_724_800), 11),
    (Time::from_posix_secs(31_622_400), 12),
    (Time::from_posix_secs(63_158_400), 13),
    (Time::from_posix_secs(94_694_400), 14),
    (Time::from_posix_secs(126_230_400), 15),
    (Time::from_posix_secs(157_852_800), 16),
    (Time::from_posix_secs(189_388_800), 17),
    (Time::from_posix_secs(220_924_800), 18),
    (Time::from_posix_secs(252_460_800), 19),
    (Time::from_posix_secs(299_721_600), 20),
    (Time::from_posix_secs(331_257_600), 21),
    (Time::from_posix_secs(362_793_600), 22),
    (Time::from_posix_secs(425_952_000), 23),
    (Time::from_posix_secs(504_921_600), 24),
    (Time::from_posix_secs(568_080_000), 25),
    (Time::from_posix_secs(599_616_000), 26),
    (Time::from_posix_secs(646_876_800), 27),
    (Time::from_posix_secs(678_412_800), 28),
    (Time::from_posix_secs(709_948_800), 29),
    (Time::from_posix_secs(757_382_400), 30),
    (Time::from_posix_secs(804_643_200), 31),
    (Time::from_posix_secs(852_076_800), 32),
    (Time::from_posix_secs(1_073_001_600), 33),
    (Time::from_posix_secs(1_167_696_000), 34),
    (Time::from_posix_secs(1_278_028_800), 35),
    (Time::from_posix_secs(1_372_636_800), 36),
    (Time::from_posix_secs(1_420_156_800), 37),
];
// TODO: add a(n impure) test checking that this is up-to-date with the latest
// downloaded file

/// Leap second provider that uses the builtin, latest-at-the-time-of-last-`kine-core`-update
/// IERS leap second table.
pub struct BuiltinIers;

impl SystemLeapSecondProvider for BuiltinIers {
    fn read(_t: &LeapSecondedTime<Self>) -> crate::Result<TimeResult> {
        todo!()
    }

    fn write(_t: &Time) -> crate::Result<WrittenTimeResult<LeapSecondedTime<Self>>> {
        todo!()
    }

    fn sigil() -> &'static str {
        BULLETIN
    }
}

impl Default for BuiltinIers {
    fn default() -> Self {
        Self
    }
}
