mod builtin_iers;

pub use builtin_iers::BuiltinIers;

cfg_if::cfg_if! {
    if #[cfg(feature = "leap-seconds-provider-builtin")] {
        /// Leap second provider for the system clock
        ///
        /// If this is not set properly then all `Time`s (and dependent calculations) will be off
        ///
        /// Stability note: This is currently exposed only because type_alias_impl_trait is still
        /// unstable. IT WILL BE TURNED INTO `impl SystemLeapSecondProvider` IN A MINOR RELEASE!
        /// And once extern existential types get done and stable, the various feature flags will
        /// be replaced with making this existential type extern.
        pub type SystemProvider = BuiltinIers;
    } else {
        compile_error!("Please define (exactly) one leap second provider feature");
    }
}

/// A provider of leap seconds
pub trait LeapSecondProvider {}

/// A provider of leap seconds, usable as a system provider
pub trait SystemLeapSecondProvider {}

impl<T: SystemLeapSecondProvider> LeapSecondProvider for T {}

pub struct Systemify<T>(pub T);

impl<T: LeapSecondProvider> SystemLeapSecondProvider for Systemify<T> {}
