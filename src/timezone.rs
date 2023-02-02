/// A time zone
pub trait TimeZone {
    /// Write the designator of this timezone
    ///
    /// The designator is the string indicating which timezone a written date is in.
    /// For instance, it is "Z" for UTC, "+01:00" for a fixed 1h offset, and " CET"
    /// for Central European Time.
    fn write_designator(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
