//! A simple example of a bot, reacting on input by returning a message.

extern crate futures;
extern crate telebot_rs;
extern crate tokio_core;

use telebot_rs::api::Bot;
use telebot_rs::receivables::*;
use telebot_rs::receivables::Receivable as recv;
use telebot_rs::sendables::*;
//use telebot_rs::error::Error;

use std::time::{Duration, Instant};
use tokio_core::reactor::Core; // application loop
use futures::future::Future;

/// This example is a simple loop that fetches Updates
/// and responds to new (sent after starting the bot) updates with "pong".
fn main() {
    let mut core = Core::new().unwrap(); // event loop
    // use a `Handle` to queue (spawn) something into the event loop.
    // later, give the core time ("turn") to handle the queued futures.
    let handle = core.handle();
    // TELEGRAM_BOT_TOKEN is the name of the environment variable the API key should be stored.
    // Feel free to change it. Especially if you need to run multiple bots.
    let bot = Bot::new("TELEGRAM_BOT_TOKEN", &core.handle());

    let interval = Duration::from_millis(3000); // set polling interval

    // mark all messages in first update as already read,
    // this fixes a lot of pong responses at bot start.
    let mut old_messages_done = false;

    // remember already processed updates with this vector.
    // TODO a data structure with unique elements would be better here.
    let mut processed: Vec<i64> = Vec::new();

    // handling a single update is implemented (in this case) as
    // a closure (a function without a name, stored in a variable)
    // this will answer "pong" to messages that are both, unprocessed and new.
    // Not that it does this not immediately, but spawns a future
    // to the event loop.
    let mut process_update = |id: i64, m: Message, old_done: bool| -> () {
        if !processed.contains(&id) {
            processed.push(id);
            if old_done {
                let chat_id = m.chat.id.to_string();
                let text = "This bot is written with *telebot-rs* in _pure rust_. ".to_owned()
                    + "It offers link previews and markdown formatting by default.\n"
                    + "Since there is not yet any other feature here, just watch this cute cat :)"
                    + "\nhttps://youtu.be/Mvdwo3D6VBw";
                let ack = bot.send_message(
                        MessageParams::new(chat_id, text).hide_link_preview(false).build()
                    ).and_then(|response| {
                        println!("{:?}", response);
                        Ok(())
                    })
                    .map_err(|_| ());
                handle.spawn(ack);
            };
        }
    };

    // this is the main loop.
    loop {
        let start = Instant::now(); // remember the time
        // Creates a future which fetches updates and handles
        // every single one of them using the closure above.
        // Note that you can easily distinguish the update types here.
        let update_handler = bot.get_updates().and_then(|updates| {
            for upd in updates {
                match upd {
                    recv::NewMessage(id, m) => process_update(id, m, old_messages_done),
                    recv::EditedMessage(id, m) => process_update(id, m, old_messages_done),
                    recv::ChannelPost(id, m) => process_update(id, m, old_messages_done),
                    recv::EditedChannelPost(id, m) => process_update(id, m, old_messages_done),
                    _ => {},  // ignore the rest for demonstration purposes
                    //InlineQuery(i64, ext::InlineQuery),
                    //ChosenInlineResult(i64, ext::ChosenInlineResult),
                    //CallbackQuery(i64, ext::CallbackQuery),
                    //ShippingQuery(i64, ext::ShippingQuery),
                    //PreCheckoutQuery(i64, ext::PreCheckoutQuery),
                }
            }
            old_messages_done = true;
            Ok(())
        });

        // The next command runs the `update_handler` one time until it's done.
        // Note that the handler only queues the handling of single updates
        // in the event loop. You still have to give the loop time to handle
        // the updates!
        // using a variable with underscore for the result,
        // prevents warnings vor not using the result.
        let _result = core.run(update_handler);

        while start.elapsed() <= interval {
            // if there is time, handle other events in the event queue.
            // this MUST exist in this example, since the handle queues
            // the handling of every single update in the event loop (core).
            core.turn(Some(interval - start.elapsed()));
        }
    }
}
