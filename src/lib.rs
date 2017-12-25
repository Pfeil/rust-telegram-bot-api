//! This is a library, encapsulating the telegram bot api.
//! It shall **make it easy** to create **fully featured** bots for Telegram.
#![feature(conservative_impl_trait)]

#[macro_use]
extern crate serde_derive;

pub mod api;  // TODO properly rewrite most of this
pub mod receivables;
pub mod sendables;
pub mod error;  // TODO adjust to api implementation, rename to TeleApiError?

// TODO rewrite examples, make them short and easy, test them properly!
// TODO make examples to use more features or write more examples!
