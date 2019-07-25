//! Core structs and traits for paperclip.

#[macro_use] extern crate serde;

#[cfg(feature = "v2")]
pub mod v2;
pub mod im;
