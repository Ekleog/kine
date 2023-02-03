mod builtin_iers;
mod time;

use crate::{Calendar, Time, TimeResult, WrittenTimeResult};

pub use builtin_iers::BuiltinIers;
pub use time::{LeapSecondedTime, ParseError};

cfg_if::cfg_if! {
    if #[cfg(feature = "leap-seconds-provider-builtin")] {
        /// Leap second provider for the system clock
        ///
        /// If this is not set properly then all `Time`s (and dependent calculations) will be off
        ///
        /// Stability note: This is currently exposed only because type_alias_impl_trait is still
        /// unstable. IT WILL BE TURNED INTO `impl SystemLeapSecondProvider` IN A MINOR RELEASE!
        /// And once extern existential types get done and stable, the various feature flags will
        /// be replaced with making this existential type extern (but will be kept there for
        /// backwards compatibility until the next major release)
        pub type SystemProvider = BuiltinIers;
    } else {
        compile_error!("Please define (exactly) one leap second provider feature");
    }
}

/// A provider of leap seconds
pub trait LeapSecondProvider: Sized + Calendar<Time = LeapSecondedTime<Self>> {
    /// Read a time with leap seconds as a Time
    fn read(&self, t: &LeapSecondedTime<Self>) -> crate::Result<TimeResult>;

    /// Return the sigil this leap second provider can be identified with
    fn sigil(&self) -> &str;
}

/// A provider of leap seconds, usable as a system provider
pub trait SystemLeapSecondProvider: Sized + Default {
    /// Read a time with leap seconds as a Time
    fn read(t: &LeapSecondedTime<Self>) -> crate::Result<TimeResult>;

    /// Write a time to a time with leap seconds
    fn write(t: &Time) -> crate::Result<WrittenTimeResult<LeapSecondedTime<Self>>>;

    /// Return the sigil this leap second provider can be identified with
    fn sigil() -> &'static str;
}

impl<T: SystemLeapSecondProvider> LeapSecondProvider for T {
    fn read(&self, t: &LeapSecondedTime<Self>) -> crate::Result<TimeResult> {
        Self::read(t)
    }

    fn sigil(&self) -> &str {
        Self::sigil()
    }
}

impl<T: SystemLeapSecondProvider> Calendar for T {
    type Time = LeapSecondedTime<Self>;

    fn write(&self, t: &Time) -> crate::Result<WrittenTimeResult<Self::Time>> {
        Self::write(t)
    }
}
