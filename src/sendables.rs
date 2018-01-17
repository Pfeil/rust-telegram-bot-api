//! This module contains Structs that can be serialized
//! to send parameters to the telegram servers using the Bot class.

extern crate serde_json;
extern crate tg_bot_models;

use self::tg_bot_models::{KeyboardButton, ReplyKeyboardMarkup};
use self::tg_bot_models::{InlineKeyboardButton, InlineKeyboardMarkup};
use self::tg_bot_models::{ForceReply, Message, ReplyKeyboardRemove};

use std::marker::PhantomData;
use std::cell::Cell;


/// This struct contains all parameters available for the send method. It directly serializes to
/// JSON and offers a builder pattern to configure.
#[derive(Debug, Clone, Serialize)]
pub struct MessageParams<K: Keyboard> {
    chat_id: String,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")] parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")] reply_markup: Option<K>,
}

pub trait Keyboard {}

struct InlineKeyboardBuilder<S = FirstButtonStage> {
    layout: Cell<Vec<Vec<InlineKeyboardButton>>>,
    builder_stage: PhantomData<S>,
}

struct FirstButtonStage;
struct NormalButtonStage;

#[allow(dead_code)] // TODO can I remove this later on?
impl InlineKeyboardBuilder<FirstButtonStage> {
    fn new() -> Self {
        let mut layout = Cell::new(Vec::new());
        layout.get_mut().append(&mut Vec::new());
        InlineKeyboardBuilder {
            layout: layout,
            builder_stage: PhantomData,
        }
    }

    fn add_pay_button(self, text: String) -> InlineKeyboardBuilder<NormalButtonStage> {
        let button = InsecureInlineButtonBuilder::new(text).as_pay_button();
        let mut layout = self.layout.into_inner();
        layout[0].push(button);
        InlineKeyboardBuilder::from_layout(layout)
    }

    fn add_game_button(
        self,
        text: String,
        description: String,
    ) -> InlineKeyboardBuilder<NormalButtonStage> {
        let button = InsecureInlineButtonBuilder::new(text).as_game_button(description);
        let mut layout = self.layout.into_inner();
        layout[0].push(button);
        InlineKeyboardBuilder::from_layout(layout)
    }
}

impl InlineKeyboardBuilder<NormalButtonStage> {
    fn from_layout(layout: Vec<Vec<InlineKeyboardButton>>) -> Self {
        InlineKeyboardBuilder {
            layout: Cell::new(layout),
            builder_stage: PhantomData,
        }
    }
}

#[allow(dead_code)] // TODO can I remove this later on?
impl<S> InlineKeyboardBuilder<S> {
    fn add_url_button<'a>(&'a mut self, text: String, url: String) -> &'a mut Self {
        let button = InsecureInlineButtonBuilder::new(text).as_url_button(url);
        self.layout.get_mut()[0].push(button);
        self
    }

    fn add_callback_button<'a>(&'a mut self, text: String, callback: String) -> &'a mut Self {
        let button = InsecureInlineButtonBuilder::new(text).as_callback_button(callback);
        self.layout.get_mut()[0].push(button);
        self
    }

    fn add_inline_query_button<'a>(&'a mut self, text: String, query: String) -> &'a mut Self {
        let button = InsecureInlineButtonBuilder::new(text).as_inline_query_button(query);
        self.layout.get_mut()[0].push(button);
        self
    }

    fn add_inline_query_to_current_chat_button<'a>(
        &'a mut self,
        text: String,
        query: String,
    ) -> &'a mut Self {
        let button =
            InsecureInlineButtonBuilder::new(text).as_inline_query_button_in_current_chat(query);
        self.layout.get_mut()[0].push(button);
        self
    }
}

/// Only intended for use within the library. Do not expose this to the lib.rs.
/// Reason: The result is a raw button which can be modified in a way
/// that is not confirm with the telegram library. This is for internal use only.
/// All functionality you can get is in `InlineKeyboardBuilder`, which is 100% safe!
struct InsecureInlineButtonBuilder {
    button: InlineKeyboardButton,
}

impl InsecureInlineButtonBuilder {
    fn new(text: String) -> Self {
        let button = InlineKeyboardButton {
            text: text,
            pay: None,
            url: None,
            callback_data: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
        };
        InsecureInlineButtonBuilder { button }
    }

    fn as_url_button(self, url: String) -> InlineKeyboardButton {
        let mut button = self.button;
        button.url = Some(url);
        button
    }

    fn as_callback_button(self, callback: String) -> InlineKeyboardButton {
        let mut button = self.button;
        button.callback_data = Some(callback);
        button
    }

    fn as_inline_query_button(self, inline_query: String) -> InlineKeyboardButton {
        let mut button = self.button;
        button.switch_inline_query = Some(inline_query);
        button
    }

    fn as_inline_query_button_in_current_chat(self, inline_query: String) -> InlineKeyboardButton {
        let mut button = self.button;
        button.switch_inline_query_current_chat = Some(inline_query);
        button
    }

    fn as_game_button(self, game_description: String) -> InlineKeyboardButton {
        let mut button = self.button;
        button.callback_game = Some(game_description);
        button
    }

    fn as_pay_button(self) -> InlineKeyboardButton {
        let mut button = self.button;
        button.pay = Some(true);
        button
    }
}

/*
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Keyboard {
    Writing(ReplyKeyboardRemove),
    Custom(ReplyKeyboardMarkup),
    AtMessage(InlineKeyboardMarkup),
    Reply(ForceReply),
}

impl Keyboard {
    fn new_message_buttons(num_rows: usize, num_cols: usize) -> Keyboard {
        let mut inline_keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();
        Keyboard::AtMessage( InlineKeyboardMarkup {inline_keyboard} )
    }

    fn append_row(&mut self, row: Vec<InlineKeyboardButton>) {
        use self::Keyboard::*;
        match self {
            &mut AtMessage(kbd_markup) => kbd_markup.inline_keyboard.push(row),
            &mut Custom(kbd_markup) => {
                kbd_markup.keyboard.push(row);
            }
        }
    }

    // fn resize custom keyboard
}
*/

impl<K: Keyboard> MessageParams<K> {
    // TODO Should these methods offer to reset values to None?
    pub fn new(chat_id: String, text: String) -> Self {
        MessageParams {
            chat_id,
            text,
            parse_mode: Some("Markdown".into()),
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn as_html<'a>(&'a mut self) -> &'a mut Self {
        self.parse_mode = Some("HTML".to_owned());
        self
    }

    pub fn as_markdown<'a>(&'a mut self) -> &'a mut Self {
        self.parse_mode = Some("Markdown".to_owned());
        self
    }

    pub fn hide_link_preview<'a>(&'a mut self, preview: bool) -> &'a mut Self {
        self.disable_web_page_preview = Some(preview);
        self
    }

    pub fn hide_notification<'a>(&'a mut self, notify: bool) -> &'a mut Self {
        self.disable_notification = Some(notify);
        self
    }

    pub fn reply_to_message_id<'a>(&'a mut self, message_id: i64) -> &'a mut Self {
        self.reply_to_message_id = Some(message_id);
        self
    }

    pub fn reply_to_message<'a>(&'a mut self, message: &Message) -> &'a mut Self {
        self.reply_to_message_id = Some(message.message_id);
        self
    }

    pub fn set_keyboard<'a>(&'a mut self, keyboard: K) -> &'a mut Self {
        self.reply_markup = Some(keyboard);
        self
    }

    pub fn build(self) -> Self {
        self
    }
}
