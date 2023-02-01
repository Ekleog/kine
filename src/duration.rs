/// A (signed) Duration.
/// 
/// It can currently represent time intervals of roughly 10^22 years either way.
pub struct Duration {
    nanos: i128,
}
