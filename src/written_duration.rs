use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::{calendar::CalendarDuration, Calendar};

/// A written duration in some calendar.
///
/// Note that this is not a `Duration`: a duration as-written can actually vary
/// wildly in meaning. For instance, one month in the gregorian calendar can be
/// between 29 and 31 days. But even one minute can be 59 or 60 seconds in UTC.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WrittenDuration<Cal: Calendar>(Cal::Duration);

impl<Cal: Calendar> WrittenDuration<Cal> {
    /// A duration that spans no time
    pub const ZERO: Self = Self(<Cal::Duration as CalendarDuration<Cal>>::ZERO);

    /// Add `rhs`, returning `None` on overflow
    pub fn checked_add(&self, rhs: &WrittenDuration<Cal>) -> Option<Self> {
        self.0.checked_add(&rhs.0).map(Self)
    }

    /// Subtract `rhs`, returning `None` on overflow
    pub fn checked_sub(&self, rhs: &WrittenDuration<Cal>) -> Option<Self> {
        self.0.checked_sub(&rhs.0).map(Self)
    }
}

impl<Cal: Calendar> Add<WrittenDuration<Cal>> for WrittenDuration<Cal> {
    type Output = WrittenDuration<Cal>;

    fn add(self, rhs: WrittenDuration<Cal>) -> Self::Output {
        Self(self.0.add(&rhs.0))
    }
}

impl<Cal: Calendar> AddAssign<WrittenDuration<Cal>> for WrittenDuration<Cal> {
    fn add_assign(&mut self, rhs: WrittenDuration<Cal>) {
        self.0.add_assign(&rhs.0)
    }
}

impl<Cal: Calendar> Sub<WrittenDuration<Cal>> for WrittenDuration<Cal> {
    type Output = WrittenDuration<Cal>;

    fn sub(self, rhs: WrittenDuration<Cal>) -> Self::Output {
        Self(self.0.sub(&rhs.0))
    }
}

impl<Cal: Calendar> SubAssign<WrittenDuration<Cal>> for WrittenDuration<Cal> {
    fn sub_assign(&mut self, rhs: WrittenDuration<Cal>) {
        self.0.sub_assign(&rhs.0)
    }
}

impl<Cal: Calendar> Default for WrittenDuration<Cal> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<Cal: Calendar> Debug for WrittenDuration<Cal> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.display(f)
    }
}

impl<Cal: Calendar> Display for WrittenDuration<Cal> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.display(f)
    }
}
