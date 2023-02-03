mod builtin_iers;
mod time;

use crate::{Calendar, TimeResult};

pub use builtin_iers::BuiltinIers;
pub use time::{LeapSecondedTime, ParseError};

cfg_if::cfg_if! {
    if #[cfg(feature = "leap-seconds-provider-builtin")] {
        /// Leap second provider for the system clock
        ///
        /// If this is not set properly then all `Time`s (and dependent calculations) will be off
        ///
        /// Stability note: This is currently exposed only because type_alias_impl_trait is still
        /// unstable. IT WILL BE TURNED INTO `impl ~const Default + Clone + LeapSecondProvider`
        /// IN A MINOR RELEASE!
        /// And once extern existential types get done and stable, the various feature flags will
        /// be replaced with making this existential type extern (but will be kept there for
        /// backwards compatibility until the next major release)
        pub type SystemProvider = BuiltinIers;
    } else {
        compile_error!("Please define (exactly) one leap second provider feature");
    }
}

/// Leap second provider for the system clock
///
/// If this is not set properly then all `Time`s (and dependent calculations) will be off
///
/// Stability note: See the stability note of `SystemProvider`. This line will also need to
/// change once it's possible to encode the fact that `default()` needs to be `const`, but
/// the things you can do with it will mostly change when `SystemProvider` will change.
pub static SYSTEM_PROVIDER: SystemProvider = SystemProvider::const_default();

/// A provider of leap seconds
pub trait LeapSecondProvider: Sized + Calendar<Time = LeapSecondedTime<Self>> {
    /// Read a time with leap seconds as a Time
    fn read(&self, t: &LeapSecondedTime<Self>) -> crate::Result<TimeResult>;

    /// Return the sigil this leap second provider can be identified with
    fn sigil(&self) -> &str;
}
