use crate::Calendar;

/// A date and time as indicated by a calendar
///
/// Anything user-visible should probably be handled using this type. However,
/// remember that operations on it can be vastly different from operations on
/// regular times, durations are often not commutative, etc.
pub type WrittenTime<Cal> = <Cal as Calendar>::Time;
