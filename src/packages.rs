/*
 * This module shall contain all container classes
 * for the communication with the Telegram Servers.
 * STATUS: Not used yet, just some rapid ideas.
 */

use std::{fmt, string};
use std::collections::LinkedList;

trait JsonValue {
    fn name(&self) -> String;
    fn value(&self) -> String;
    fn as_json_value(&self) -> String {
        let mut result: String = String::new();
        if self.value().is_empty() {
            return result;
        }
        result.push_str(self.name().as_str());
        result.push_str(": ");
        result.push_str(self.value().as_str());
        result
    }
}

trait UrlValue {
    fn name(&self) -> String;
    fn value(&self) -> String;
    fn as_url_value(&self) -> String {
        let mut result: String = String::new();
        if self.value().is_empty() {
            return result;
        }
        result.push_str(self.name().as_str());
        result.push_str(": ");
        result.push_str(self.value().as_str());
        result
    }
}

struct Parameter<T: fmt::Display> {
    name: String,
    value: T,
}

struct Optional<T: fmt::Display> {
    name: String,
    value: Option<T>,
}

impl<T: fmt::Display + string::ToString> JsonValue for Parameter<T> {
    fn name(&self) -> String {
        self.name.to_owned()
    }
    fn value(&self) -> String {
        self.value.to_string()
    }
}

impl<T: fmt::Display + string::ToString> UrlValue for Parameter<T> {
    fn name(&self) -> String {
        self.name.to_owned()
    }
    fn value(&self) -> String {
        self.value.to_string()
    }
}

impl<T: fmt::Display + string::ToString> JsonValue for Optional<T> {
    fn name(&self) -> String {
        self.name.to_owned()
    }
    fn value(&self) -> String {
        match self.value {
            Some(ref v) => v.to_string(),
            None => String::new(),
        }
    }
}

impl<T: fmt::Display + string::ToString> UrlValue for Optional<T> {
    fn name(&self) -> String {
        self.name.to_owned()
    }
    fn value(&self) -> String {
        match self.value {
            Some(ref v) => v.to_string(),
            None => String::new(),
        }
    }
}


#[allow(unused)]
enum Methods {
    GetMe, // about this bot
    GetUpdates {
        // usual last received message + 1 or less if data from the past is needed.
        offset: Optional<i32>,
        limit: Optional<u8>, // TODO 1-100 is the accepted range, handle it!
        timeout: Optional<u32>,
        //allowed_updates: Optional<[T]>,
    },
    SendMessage,
}

fn getUpdates<T: fmt::Display>(offset: Optional<i32>,
                               limit: Optional<u8>, // TODO 1-100 is the accepted range, handle it!
                               timeout: Optional<u32>,
                               allowed_updates: Optional<LinkedList<T>>) {

}

#[allow(non_camel_case_types)]
enum UpdateTypes {
    Message,
    EditedMessage,
    ChannelPost,
    EditedChannelPost,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    ShippingQuery,
    PreCheckoutQuery,
}
