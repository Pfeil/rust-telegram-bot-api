extern crate shoppingbot;
extern crate tokio_core; // app loop

use shoppingbot::api::*;
use shoppingbot::packages::*;
use shoppingbot::parameters::*;
use std::time::{Instant, Duration};
use std::thread;
use std::env; // to read shell variables (telegram token)
use tokio_core::reactor::Core; // application loop

/// This example is a simple loop that fetches Updates
/// every 10 seconds and prints them.
fn main() {
    // set up telegram bot
    let token: String = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let core = Core::new().unwrap();
    let mut bot = Bot::new(token, core);
    let interval = Duration::from_millis(3000);

    // mark all messages in first update as already read,
    // this fixes s lot of pong responses at bot start.
    let mut old_messages_done = false;
    // remember already processed updates with this vector.
    // TODO a data structure with unique elements would be better here.
    let mut processed: Vec<u64> = Vec::new();
    loop {
        let start = Instant::now();
        let update = bot.get_updates();
        println!("received {:?}", update);
        if update.is_ok() {
            let result: Vec<Update> = update.unwrap();
            for upd in result {
                if !processed.contains(&upd.update_id) {
                    processed.push(upd.update_id);
                    if old_messages_done {
                        //let message: Message =
                        upd.message
                            .clone()
                            .and_then(|message| {
                                let id = message.chat.id.to_string();
                                let text = "pong".to_owned();
                                let ack = bot.send_message(MessageParams {
                                                               chat_id: id,
                                                               text: text,
                                                           });
                                println!("{:?}", ack);
                                Some(message)
                            });
                    }
                }
            }
            old_messages_done = true;
        } else {
            println!("ERR: {:?}", update.err().unwrap());
        }

        // check the remaining time you got from the interval
        // and sleep if needed.
        let elapsed = start.elapsed();
        if elapsed <= interval {
            thread::sleep(interval - elapsed);
        }
    }
}
