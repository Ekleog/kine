mod builtin_iers;

/// A provider of leap seconds
pub trait LeapSecondProvider {}

pub use builtin_iers::BuiltinIers;

/// Leap second provider for the system clock
///
/// If this is not set properly then all `Time`s (and dependent calculations) will be off
///
/// Stability note: This is currently exposed only because type_alias_impl_trait is still
/// unstable. IT WILL BE TURNED INTO `impl LeapSecondProvider` IN A MINOR RELEASE! And once
/// extern existential types get done and stable, the various feature flags will be replaced
/// with making this existential type extern.
pub type SystemProvider = BuiltinIers;
