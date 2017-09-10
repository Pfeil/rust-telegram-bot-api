//! This module contains Structs that can be serialized
//! to send parameters to the telegram servers using the Bot class.

extern crate serde_json; // json parser


#[derive(Debug, Serialize, Deserialize)]
pub struct MessageParams {
    pub chat_id: String,
    pub text: String,
    // TODO add optionals
}

impl MessageParams {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
