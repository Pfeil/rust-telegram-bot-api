//! This is a library, encapsulating the telegram bot api.
#![feature(conservative_impl_trait)]


//#![feature(use_extern_macros)]
pub mod api;
pub mod packages;
pub mod parameters;
#[macro_use]
extern crate serde_derive;
