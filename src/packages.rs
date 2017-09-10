//! This module contains all container classes
//! for the communication with the Telegram Servers.
//!
//! Since members can't use the name `type`,
//! the deserialization of serde_json can not be used
//! in every case and is therefore manually implemented
//! or wrapped.


extern crate serde_json; // json parser

use self::serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub ok: bool,
    pub error_code: u32,
    pub description: String,
}

impl Error {
    pub fn from_json(json: Value) -> Error {
        serde_json::from_value(json).unwrap()
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Update {
    pub update_id: u64,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>,
    // TODO implement functionality below
    //inline_query: Option<InlineQuery>,
    //pub chosen_inline_result: Option<ChosenInlineResult>,
    //pub callback_query: Option<CallbackQuery>,
    //pub shipping_query: Option<ShippingQuery>,
    //pub pre_checkout_query: Option<PreCheckoutQuery>,
}

#[allow(dead_code)]
impl Update {
    pub fn from_json(json: Value) -> Option<Update> {
        let update_id = serde_json::from_value(json["update_id"].to_owned());
        if update_id.is_err() {
            return Option::None;
        }
        Some(Update {
                 update_id: update_id.unwrap(),
                 message: Message::from_json(json["message"].to_owned()),
                 edited_message: Message::from_json(json["edited_message"].to_owned()),
                 channel_post: Message::from_json(json["channel_post"].to_owned()),
                 edited_channel_post: Message::from_json(json["edited_channel_post"].to_owned()),
             })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    // TODO placeholder. maybe use an enum?
    pub message_id: i64,
    pub from: Option<User>,
    pub date: i64, // unix time
    pub chat: Chat,
    pub text: Option<String>,
}

impl Message {
    pub fn from_json(json: Value) -> Option<Message> {
        let id = serde_json::from_value(json["message_id"].to_owned());
        if id.is_err() {
            return Option::None;
        }
        //let from = match json["from"] {
        //    Value::Object(_) => Some(User::from_json(json["from"].to_owned())),
        //    _ => Option::None,
        //};
        let chat = Chat::from_json(json["chat"].to_owned());
        Some(Message {
                 message_id: id.unwrap(),
                 from: User::from_json(json["from"].to_owned()),
                 date: serde_json::from_value(json["date"].to_owned()).unwrap(),
                 chat: chat.unwrap(),
                 text: serde_json::from_value(json["text"].to_owned()).unwrap(),
             })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
}

impl User {
    pub fn from_json(json: Value) -> Option<User> {
        serde_json::from_value(json).unwrap()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chat {
    pub id: i64,
    pub chat_type: String,
    // TODO add optional members
}

impl Chat {
    fn from_json(json: Value) -> Option<Chat> {
        let chat_type = match json["type"] {
            Value::String(ref s) => s.to_owned(),
            _ => return Option::None,
        };
        Some(Chat {
                 id: serde_json::from_value(json["id"].to_owned()).unwrap(),
                 chat_type: chat_type,
             })
    }
}
