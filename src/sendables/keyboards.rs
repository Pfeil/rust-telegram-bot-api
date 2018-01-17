use std::marker::PhantomData;
use std::cell::Cell;

use sendables::tg_bot_models::{InlineKeyboardButton, InlineKeyboardMarkup};
use sendables::tg_bot_models::{KeyboardButton, ReplyKeyboardMarkup};



pub trait Keyboard {}

/// Neat builder for inline keyboards.
/// To restrict the keyboard to be allowed by the Telegram Bot API, it uses some kind of
/// compile time state machine. This way, you always get a valid keyboard at compile time!
/// This is implemented using Generics, but you may just forget and ignore this fact.
/// TODO EXAMPLE
pub struct InlineKeyboardBuilder<S = FirstButtonStage> {
    layout: Cell<Vec<Vec<InlineKeyboardButton>>>,
    builder_stage: PhantomData<S>,
}

/// This is a helper type for the incredibly neat `InlineKeyboardBuilder`.
/// It won't help you, so just ignore it.
pub struct FirstButtonStage;
/// This is a helper type for the incredibly neat `InlineKeyboardBuilder`.
/// It won't help you, so just ignore it.
pub struct NormalButtonStage;

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
