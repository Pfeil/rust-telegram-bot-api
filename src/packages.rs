/*
 * This module shall contain all container classes
 * for the communication with the Telegram Servers.
 */

extern crate serde_json; // json parser
extern crate time;

use self::time::Timespec;
use self::serde_json::Value;

#[derive(Debug)]
pub struct Updates {
    ok: bool,
    result: Vec<Update>,
}

impl Updates {
    pub fn from_json(json: Value) -> Updates {
        let ok: bool = match json["ok"] {
            Value::Bool(b) => b,
            _ => panic!("Panic: ok was not a bool in Updates"),
        };
        let result = match json["result"] {
            Value::Array(ref vec) => {
                let mut updates = Vec::new();
                for val in vec {
                    updates.push(Update::from_json(val.to_owned()));
                }
                updates
            }
            Value::Object(_) => vec![Update::from_json(json["result"].to_owned())],
            _ => Vec::new(),
        };
        Updates {
            ok: ok,
            result: result,
        }
    }
}

#[derive(Debug)]
pub struct Update {
    update_id: u64,
    message: Option<Message>,
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

impl Update {
    pub fn from_json(json: Value) -> Update {
        let id = match json["update_id"] {
            Value::Number(ref s) => s.as_u64().unwrap(),
            _ => panic!("Panic: id was not a number in Update"),
        };
        let message = match json["message"] {
            Value::Object(_) => Some(Message::from_json(json["message"].to_owned())),
            _ => Option::None,
        };
        Update {
            update_id: id,
            message: message,
        }
    }
}

#[derive(Debug)]
pub struct Message {
    // TODO placeholder. maybe use an enum?
    message_id: i64,
    from: Option<User>,
    date: Timespec, // unix time
    /*chat: Chat,*/
    text: Option<String>,
}

impl Message {
    pub fn from_json(json: Value) -> Message {
        let id = match json["message_id"] {
            Value::Number(ref n) => n.as_i64().unwrap(),
            _ => panic!("Panic: id was not a number in Message"),
        };
        let from = match json["from"] {
            Value::Object(_) => Some(User::from_json(json["from"].to_owned())),
            _ => Option::None,
        };
        let date = match json["date"] {
            Value::Number(ref n) => Timespec::new(n.as_i64().unwrap(), 0),
            _ => Timespec::new(0, 0),
        };
        let text = match json["text"] {
            Value::String(ref s) => Some(s.to_owned()),
            _ => None,
        };
        Message {
            message_id: id,
            from: from,
            date: date,
            text: text,
        }
    }
}

#[derive(Debug)]
pub struct User {
    id: i64,
    is_bot: bool,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
    language_code: Option<String>,
}

impl User {
    pub fn from_json(json: Value) -> User {
        let id = match json["id"] {
            Value::Number(ref n) => n.as_i64().unwrap(),
            _ => panic!("Panic: id was not a Number in User"),
        };
        let is_bot = match json["is_bot"] {
            Value::Bool(b) => b,
            _ => false,
        };
        let first_name = match json["first_name"] {
            Value::String(ref s) => s.to_owned(),
            _ => String::from("First Name Unknown"),
        };
        let last_name = match json["last_name"] {
            Value::String(ref s) => Some(s.to_owned()),
            _ => Option::None,
        };
        let username = match json["username"] {
            Value::String(ref s) => Some(s.to_owned()),
            _ => Option::None,
        };
        let language_code = match json["language_code"] {
            Value::String(ref s) => Some(s.to_owned()),
            _ => Option::None,
        };

        User {
            id: id,
            is_bot: is_bot,
            first_name: first_name,
            last_name: last_name,
            username: username,
            language_code: language_code,
        }
    }
}
