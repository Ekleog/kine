# Kine

Kine is a crate for time handling in Rust.

It aims to be **correct** first and foremost, but also to provide time formatting options that match basically any needs in adapters.

Correctness is done with the `Time` type, which is the main type of this crate. The `Time` crate deals with time that (assuming the system is properly configured) should only ever move forward, including across restarts of the computer (at which time `Instant` is no longer monotonic) and leap seconds (at which time `SystemTime` may no be longer monotonic). This is done by knowing about leap seconds, thanks to providers that must be configured in feature flags by the binary, through feature flags until extern existential types become reality.

Formatting options are handled through the `Calendar` trait. A `Calendar` is a way of displaying the time. In particular, `kine-icu` (available under the `icu` feature of `kine`) offers all the formatting options of `icu4x` with the precise time handling of `kine`.
