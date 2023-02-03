use crate::{Duration, Time};

use super::LeapSecondProvider;

/// List of leap seconds. Between -infinity and the first item here, the offset is 0.
/// Between items n (included) and n + 1 (excluded, or +infinity) here, the offset is
/// IERS_LEAP_SECS[n].1
static IERS_LEAP_SECS: &[(Time, i32)] = &[
    (Time::POSIX_EPOCH, 10),
    // TODO
];

/// Leap second provider that uses the builtin, latest-at-the-time-of-last-`kine-core`-update
/// IERS leap second table.
pub struct BuiltinIers;

impl LeapSecondProvider for BuiltinIers {}
