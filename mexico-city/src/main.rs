//! The Mexico City enclave
//!
//! ## Authors
//!
//! The Veracruz Development Team.
//!
//! ## Licensing and copyright notice
//!
//! See the `LICENSE.markdown` file in the Veracruz root directory for
//! information on licensing and copyright.

#![no_main]
#![crate_name = "mexico_city_enclave"]
#![feature(rustc_private)]

#[cfg(feature = "tz")]
pub mod mc_tz;
#[cfg(feature = "tz")]
pub use crate::mc_tz::*;
pub mod managers;
