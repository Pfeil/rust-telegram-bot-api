//! This is a library, encapsulating the telegram bot api.
#![feature(conservative_impl_trait)]


pub mod api;
pub mod packages;
pub mod parameters;
#[macro_use]
extern crate serde_derive;
