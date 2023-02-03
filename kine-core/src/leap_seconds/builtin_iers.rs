use crate::Time;

use super::LeapSecondProvider;

/// List of leap seconds. Between -infinity and the first item here, the offset is 0.
/// Between items n (included) and n + 1 (excluded, or +infinity) here, the offset is
/// IERS_LEAP_SECS[n].1
static _IERS_LEAP_SECS: &[(Time, i32)] = &[
    (Time::from_posix_nanos(0), 10),
    (Time::from_posix_nanos(15_724_800_000_000_000), 11),
    (Time::from_posix_nanos(31_622_400_000_000_000), 12),
    (Time::from_posix_nanos(63_158_400_000_000_000), 13),
    (Time::from_posix_nanos(94_694_400_000_000_000), 14),
    (Time::from_posix_nanos(126_230_400_000_000_000), 15),
    (Time::from_posix_nanos(157_852_800_000_000_000), 16),
    (Time::from_posix_nanos(189_388_800_000_000_000), 17),
    (Time::from_posix_nanos(220_924_800_000_000_000), 18),
    (Time::from_posix_nanos(252_460_800_000_000_000), 19),
    (Time::from_posix_nanos(299_721_600_000_000_000), 20),
    (Time::from_posix_nanos(331_257_600_000_000_000), 21),
    (Time::from_posix_nanos(362_793_600_000_000_000), 22),
    (Time::from_posix_nanos(425_952_000_000_000_000), 23),
    (Time::from_posix_nanos(504_921_600_000_000_000), 24),
    (Time::from_posix_nanos(568_080_000_000_000_000), 25),
    (Time::from_posix_nanos(599_616_000_000_000_000), 26),
    (Time::from_posix_nanos(646_876_800_000_000_000), 27),
    (Time::from_posix_nanos(678_412_800_000_000_000), 28),
    (Time::from_posix_nanos(709_948_800_000_000_000), 29),
    (Time::from_posix_nanos(757_382_400_000_000_000), 30),
    (Time::from_posix_nanos(804_643_200_000_000_000), 31),
    (Time::from_posix_nanos(852_076_800_000_000_000), 32),
    (Time::from_posix_nanos(1_073_001_600_000_000_000), 33),
    (Time::from_posix_nanos(1_167_696_000_000_000_000), 34),
    (Time::from_posix_nanos(1_278_028_800_000_000_000), 35),
    (Time::from_posix_nanos(1_372_636_800_000_000_000), 36),
    (Time::from_posix_nanos(1_420_156_800_000_000_000), 37),
];
// TODO: add a(n impure) test checking that this is up-to-date with the latest
// downloaded file

/// Leap second provider that uses the builtin, latest-at-the-time-of-last-`kine-core`-update
/// IERS leap second table.
pub struct BuiltinIers;

impl LeapSecondProvider for BuiltinIers {}
