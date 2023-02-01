/// A calendar system, including timezone if need be
pub trait Calendar {
    /// The data needed to represent a time in this calendar
    type Time;

    /// The data needed to represent a duration in this calendar
    type Duration;
}
