//! Utilities for pinning

#![no_std]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

#[macro_use]
mod stack_pin;
#[macro_use]
mod projection;

// Not public API.
#[doc(hidden)]
pub mod __private {
    pub use core::pin::Pin;
}
