//! This module contains Structs that can be serialized
//! to send parameters to the telegram servers using the Bot class.

extern crate serde_json;
extern crate tg_bot_models;

mod keyboards;
mod message;

// export only what is safe and needed.
pub use self::keyboards::{InlineKeyboardBuilder, Keyboard};
pub use self::message::MessageParams;
