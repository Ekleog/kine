#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub use kine_core::*;

#[cfg(feature = "icu")]
#[doc(inline)]
pub use kine_icu as icu;
