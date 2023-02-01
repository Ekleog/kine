/// One instant in real-life
/// 
/// `Time` guarantees being able to hold any time between 10.000 BC and 10.000 AD.
/// Any time beyond these bounds may be considered by later versions of this crate
/// as being out of bounds.
/// Currently, it can actually hold any time between roughly 10^22 years before and
/// after posix epoch.
pub struct Time {
    /// Offset with the POSIX epoch
    nanos: i128,
}
