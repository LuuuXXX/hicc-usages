#![no_std]
#![feature(specialization)]
//! Framework for exposing Rust types and functions as C ABI interfaces.
//!
//! See [README.md](../README.md) for full documentation.

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod class;
pub use class::*;

pub mod cabi;
pub use cabi::*;

pub mod foreign;

#[path = "alloc/mod.rs"]
mod alloc_types;

#[path = "core/mod.rs"]
pub mod core_types;
pub use core_types::*;

#[cfg(feature = "std")]
#[path = "std/mod.rs"]
mod std_types;

pub use hicc_rs_macros::*;

#[cfg(feature = "cbindgen")]
pub mod cbindgen;
#[cfg(feature = "cbindgen")]
pub use crate::cbindgen::{ExportType, TypeRegistry};
