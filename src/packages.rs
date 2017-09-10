/*
 * This module shall contain all container classes
 * for the communication with the Telegram Servers.
 *
 * Since members can't use the name `type`,
 * the deserialization of serde_json can not be used
 * in every case and is therefore manually implemented
 * or wrapped.
 */

extern crate serde_json; // json parser
extern crate time;


use self::time::Timespec; // TODO use this to represent time in the final Message struct somehow!
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
    // TODO implement functionality below
    //edited_message: Option<Message>,
    //channel_post: Option<Message>,
    //edited_channel_post: Option<Message>,
    //inline_query: Option<InlineQuery>,
    //chosen_inline_result: Option<ChosenInlineResult>,
    //callback_query: Option<CallbackQuery>,
    //shipping_query: Option<ShippingQuery>,
    //pre_checkout_query: Option<PreCheckoutQuery>,
}

#[allow(dead_code)]
impl Update {
    pub fn from_json(json: Value) -> Update {
        let message = match json["message"] {
            Value::Object(_) => Some(Message::from_json(json["message"].to_owned())),
            _ => Option::None,
        };
        Update {
            update_id: serde_json::from_value(json["update_id"].to_owned()).unwrap(),
            message: message,
        }
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
    pub fn from_json(json: Value) -> Message {
        let from = match json["from"] {
            Value::Object(_) => Some(User::from_json(json["from"].to_owned())),
            _ => Option::None,
        };
        let chat = match json["chat"] {
            Value::Object(_) => Chat::from_json(json["chat"].to_owned()),
            _ => {
                Chat {
                    id: 0,
                    chat_type: "wrong".to_owned(),
                }
            }
        };
        Message {
            message_id: serde_json::from_value(json["message_id"].to_owned()).unwrap(),
            from: from,
            date: serde_json::from_value(json["date"].to_owned()).unwrap(),
            chat: chat,
            text: serde_json::from_value(json["text"].to_owned()).unwrap(),
        }
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
    pub fn from_json(json: Value) -> User {
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
    fn from_json(json: Value) -> Chat {
        let chat_type = match json["type"] {
            Value::String(ref s) => s.to_owned(),
            _ => "wrong".to_owned(),
        };
        Chat {
            id: serde_json::from_value(json["id"].to_owned()).unwrap(),
            chat_type: chat_type,
        }
    }
}
