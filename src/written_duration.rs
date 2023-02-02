use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::{calendar::CalendarDuration, Calendar};

/// A written duration in some calendar.
///
/// Note that this is not a [`Duration`]: a duration as-written can actually vary
/// wildly in meaning. For instance, one month in the gregorian calendar can be
/// between 29 and 31 days. But even one minute can be 59 or 60 seconds in UTC.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WrittenDuration<Cal: Calendar> {
    data: Cal::Duration,
}

impl<Cal: Calendar> WrittenDuration<Cal> {
    /// A duration that spans no time
    pub const ZERO: Self = Self {
        data: <Cal::Duration as CalendarDuration<Cal>>::ZERO,
    };

    /// Add `rhs`, returning `None` on overflow
    pub fn checked_add(&self, _rhs: &WrittenDuration<Cal>) -> Option<Self> {
        todo!()
    }

    /// Subtract `rhs`, returning `None` on overflow
    pub fn checked_sub(&self, _rhs: &WrittenDuration<Cal>) -> Option<Self> {
        todo!()
    }
}

impl<Cal: Calendar> Add<WrittenDuration<Cal>> for WrittenDuration<Cal> {
    type Output = WrittenDuration<Cal>;

    fn add(self, _rhs: WrittenDuration<Cal>) -> Self::Output {
        todo!()
    }
}

impl<Cal: Calendar> AddAssign<WrittenDuration<Cal>> for WrittenDuration<Cal> {
    fn add_assign(&mut self, _rhs: WrittenDuration<Cal>) {
        todo!()
    }
}

impl<Cal: Calendar> Sub<WrittenDuration<Cal>> for WrittenDuration<Cal> {
    type Output = WrittenDuration<Cal>;

    fn sub(self, _rhs: WrittenDuration<Cal>) -> Self::Output {
        todo!()
    }
}

impl<Cal: Calendar> SubAssign<WrittenDuration<Cal>> for WrittenDuration<Cal> {
    fn sub_assign(&mut self, _rhs: WrittenDuration<Cal>) {
        todo!()
    }
}

impl<Cal: Calendar> Default for WrittenDuration<Cal> {
    fn default() -> Self {
        todo!()
    }
}

impl<Cal: Calendar> Debug for WrittenDuration<Cal> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<Cal: Calendar> Display for WrittenDuration<Cal> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
