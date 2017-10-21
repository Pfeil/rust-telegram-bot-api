//! A simple example of a bot, reacting on input by returning a message.

extern crate futures;
extern crate telebot_rs;
extern crate tokio_core;

use telebot_rs::api::Bot;
use telebot_rs::packages::*;
use telebot_rs::packages::Update::*;
use telebot_rs::parameters::*;
use std::time::{Duration, Instant};
use std::env; // to read shell variables (telegram token)
use tokio_core::reactor::Core; // application loop
use futures::future::Future;

/// This example is a simple loop that fetches Updates
/// and responds to new (sent after starting the bot) updates with "pong".
fn main() {
    // get token via environment variable. Never store your keys in the code!
    let token: String = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let mut core = Core::new().unwrap(); // event loop (like qApp and similar)
    // use a `Handle` to queue (spawn) something into the event loop.
    // later, give the core time ("turn") to handle the queued futures.
    let handle = core.handle();
    let bot = Bot::new(token, &core.handle());
    let interval = Duration::from_millis(3000); // set polling interval

    // mark all messages in first update as already read,
    // this fixes a lot of pong responses at bot start.
    let mut old_messages_done = false;

    // remember already processed updates with this vector.
    // TODO a data structure with unique elements would be better here.
    let mut processed: Vec<u64> = Vec::new();

    // handling a single update is implemented (in this case) as
    // a closure (a function without a name, stored in a variable)
    // this will answer "pong" to messages that are both, unprocessed and new.
    // Not that it does this not immediately, but spawns a future
    // to the event loop.
    let mut process_update = |id: u64, m: Message, old_done: bool| -> () {
        if !processed.contains(&id) {
            processed.push(id);
            if old_done {
                let chat_id = m.chat.id.to_string();
                let text = "pong".to_owned();
                let ack = bot.send_message(MessageParams {
                    chat_id: chat_id,
                    text: text,
                }).and_then(|response| {
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
            println!("received {:?}", updates);
            for upd in updates {
                match upd {
                    TextMessage(id, m) => process_update(id, m, old_messages_done),
                    EditedTextMessage(id, m) => process_update(id, m, old_messages_done),
                    ChannelPost(id, m) => process_update(id, m, old_messages_done),
                    EditedChannelPost(id, m) => process_update(id, m, old_messages_done),
                    _ => {}
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
