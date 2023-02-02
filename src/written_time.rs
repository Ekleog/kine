use crate::Calendar;

/// A date and time as indicated by a calendar
///
/// Anything user-visible should probably be handled using this type. However,
/// remember that operations on it can be vastly different from oper
pub struct WrittenTime<Cal: Calendar> {
    cal: Cal,
    data: Cal::Time,
}
