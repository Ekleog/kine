use crate::{leap_seconds::BuiltinIers, TimeZone};

cfg_if::cfg_if! {
    if #[cfg(doc)] {
        /// Time zone the `std::time::SystemTime` clock is in
        ///
        /// If this is not set properly, then all `Time`s (and dependent calculations) will
        /// be off.
        ///
        /// Usually, `SystemTime` returns POSIX timestamps, so this should be an IERS leap
        /// seconds timezone. Which one you choose depends on until which date you want time
        /// calculations to be precise.
        ///
        /// Stability note: The real type of is currently exposed only because
        /// type_alias_impl_trait is still unstable. IT WILL BE TURNED INTO
        /// `impl ~const Default + Clone + TimeZone` IN A MINOR RELEASE!
        /// And once extern existential types get done and stable, the various feature flags
        /// will be replaced with making this existential type extern (but will be kept there
        /// for backwards compatibility until the next major release)
        pub type System = BuiltinIers;

    } else if #[cfg(feature = "tz-system-provider-builtin-iers")] {
        pub type System = BuiltinIers;
    } else {
        compile_error!("Please define (exactly) one system timezone provider feature");
    }
}

/// Time zone for the system clock
///
/// If this is not set properly then all `Time`s (and dependent calculations) will be off
///
/// Stability note: See the stability note of `System`. This line will also need to
/// change once it's possible to encode the fact that `default()` needs to be `const`, but
/// the things you can do with it will mostly change when `System` will change.
pub static SYSTEM: System = System::const_default();

/// Sigil for the system timezone provider, convenience
///
/// Stability note: See SYSTEM.
pub type SystemSigil = <System as TimeZone>::Sigil;
