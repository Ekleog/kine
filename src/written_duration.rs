use crate::Calendar;

/// A written duration in some calendar.
///
/// Note that this is not a [`Duration`]: a duration as-written can actually vary
/// wildly in meaning. For instance, one month in the gregorian calendar can be
/// between 29 and 31 days. But even one minute can be 59 or 60 seconds in UTC.
pub struct WrittenDuration<Cal: Calendar> {
    _data: Cal::Duration,
}
