//! This is a library, encapsulating the telegram bot api.

//#![feature(use_extern_macros)]
pub mod api;
pub mod packages;
pub mod parameters;
#[macro_use]
extern crate serde_derive;
