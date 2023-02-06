# Kine

Kine is a crate for time handling in Rust.

It aims to be **correct** first and foremost, but also to provide time formatting options that match basically any needs in adapters.

Correctness is done with the `Time` type, which is the main type of this crate. The `Time` crate deals with time that (assuming the system is properly configured) should only ever move forward at a constant rate, including across restarts of the computer (at which time `Instant` is no longer monotonic) and leap seconds (at which time `SystemTime` may no be longer monotonic). This is done by knowing about leap seconds, thanks to providers that must be configured in feature flags by the binary, through feature flags until extern existential types become reality. `Time` is able to represent time with nanosecond accuracy, though not all systems are able to provide a clock this precise, and the time taken to run the code itself would probably already make the measurement off anyway.

The important point is, using `Duration` operations on `Time` can assume the time is linear, as it will run through leap seconds properly. For instance, one minute before the POSIX epoch, the time was `1969-12-31T23:59:10Z`, because there were 10 leap seconds at that time.

Formatting options are handled through the `Calendar` trait. A `Calendar` is a way of displaying the time. In particular, `kine-icu` (available under the `icu` feature of `kine`) offers all the formatting options of `icu4x` with the precise time handling of `kine`. These crates also allow for "intuitive" arithmetic of times, with the problematic results this could generate. For instance, once converted to a UTC timezone, one "pseudo-minute" before POSIX epoch would become `1969-12-31T23:59:00Z`

Each `Calendar` can "read" and "write" precise times. The "read" operation corresponds to identifying to which instant in real life a written moment corresponds. And the "write" operation corresponds to identifying how to write an identified instant from real life.
