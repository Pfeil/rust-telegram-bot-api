/*
 * This module shall contain all container classes
 * for the communication with the Telegram Servers.
 * STATUS: Not used yet, just some rapid ideas.
 */

extern crate serde_json; // json parser

use self::serde_json::Value;
//use std::{fmt, string};
//use std::collections::LinkedList;

#[derive(Debug)]
pub struct User {
    pub ok: bool,
    pub is_bot: bool,
    pub id: u64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
}

impl User {
    pub fn from_json(json: Value) -> User {
        let ok = match json["ok"] {
            Value::Bool(b) => b,
            _ => false,
        };
        let is_bot = match json["result"]["is_bot"] {
            Value::Bool(b) => b,
            _ => false,
        };
        let first_name = match json["result"]["first_name"] {
            Value::String(ref s) => s.to_owned(),
            _ => String::from("First Name Unknown"),
        };
        let last_name = match json["result"]["last_name"] {
            Value::String(ref s) => Some(s.to_owned()),
            _ => Option::None,
        };
        let username = match json["result"]["username"] {
            Value::String(ref s) => Some(s.to_owned()),
            _ => Option::None,
        };
        let language_code = match json["result"]["language_code"] {
            Value::String(ref s) => Some(s.to_owned()),
            _ => Option::None,
        };

        User {
            ok: ok,
            id: json["result"]["id"].as_u64().unwrap(), // TODO if id goes wrong, it may be better to crash anyway?
            is_bot: is_bot,
            first_name: first_name,
            last_name: last_name,
            username: username,
            language_code: language_code,
        }
    }
}

/*
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
*/
