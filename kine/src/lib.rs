//! All-in-one re-exporter for crates of the kine ecosystem
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use kine_core::*;

#[cfg(feature = "icu")]
#[cfg_attr(docsrs, doc(cfg(feature = "icu")))]
#[doc(inline)]
pub use kine_icu as icu;
