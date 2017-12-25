//! This module contains all container classes
//! for the communication with the Telegram Servers.
//!
//! Since members can't use the name `type`,
//! the deserialization of serde_json can not be used
//! in every case and is therefore manually implemented
//! or wrapped in the "from_json" functions.


extern crate serde_json;
extern crate tg_bot_models;

use error::Error;

/// Exposes the raw structs representing the JSON API.
pub use self::tg_bot_models::*;


#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Receivable {
    NewMessage(i64, Message),
    EditedMessage(i64, Message),
    ChannelPost(i64, Message),
    EditedChannelPost(i64, Message),
    InlineQuery(i64, InlineQuery),
    ChosenInlineResult(i64, ChosenInlineResult),
    CallbackQuery(i64, CallbackQuery),
    ShippingQuery(i64, ShippingQuery),
    PreCheckoutQuery(i64, PreCheckoutQuery),
}

impl Receivable {
    pub fn from_update(update: Update) -> Result<Receivable, Error> {
        use self::Receivable::*;

        let id = update.update_id;
        if let Some(m) = update.message {
            return Ok(NewMessage(id, m));
        }
        if let Some(m) = update.edited_message {
            return Ok(EditedMessage(id, m));
        }
        if let Some(m) = update.channel_post {
            return Ok(ChannelPost(id, m));
        }
        if let Some(m) = update.edited_channel_post {
            return Ok(EditedChannelPost(id, m));
        }
        if let Some(m) = update.inline_query {
            return Ok(InlineQuery(id, m));
        }
        if let Some(m) = update.chosen_inline_result {
            return Ok(ChosenInlineResult(id, m));
        }
        if let Some(m) = update.callback_query {
            return Ok(CallbackQuery(id, m));
        }
        if let Some(m) = update.shipping_query {
            return Ok(ShippingQuery(id, m));
        }
        if let Some(m) = update.pre_checkout_query {
            return Ok(PreCheckoutQuery(id, m));
        }

        Err(Error::Api("Update was not any known Receivable. Has the API been changed?"))
    }
}
