//! A simple example of a bot fetching updates (messages)
//! and printing out the results via stdout.

extern crate futures;
extern crate telebot_rs;
extern crate tokio_core;

use telebot_rs::api::Bot;
use telebot_rs::packages::*;
use telebot_rs::packages::Update::*;
use std::time::{Duration, Instant};
use std::thread; // to let the application sleep
use std::env; // to read shell variables (telegram token)
use tokio_core::reactor::Core; // application loop
use futures::future::Future;

/// This example is a simple loop that fetches Updates
/// every 10 seconds and prints them.
fn main() {
    // get token via environment variable. Never store your keys in the code!
    let token: String = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let mut core = Core::new().unwrap(); // event loop (like qApp and similar)
    let bot = Bot::new(token, &core.handle());
    let interval = Duration::from_millis(10000); // set polling interval
    loop {
        let start = Instant::now();
        let update_handler = bot.get_updates().and_then(|updates| {
            for update in updates {
                match update {
                    TextMessage(id, m) => println!("+{}\n{}: {:?}", id, m.from.unwrap(), m.text),
                    EditedTextMessage(id, m) => {
                        println!("*{}\n{}: {:?}", id, m.from.unwrap(), m.text)
                    }
                    ChannelPost(id, m) => {
                        println!("pub +{}\n{}: {:?}", id, m.from.unwrap(), m.text)
                    }
                    EditedChannelPost(id, m) => {
                        println!("pub *{}\n{}: {:?}", id, m.from.unwrap(), m.text)
                    }
                    _ => {
                        return Err(Error {
                            ok: false,
                            error_code: 0u32,
                            description: "asd".to_owned(),
                        });
                    }
                }
            }
            Ok(())
        });

        // run the update_handler one time, until it is finished.
        // using a variable with underscore for the result,
        // prevents warnings vor not using the result.
        let _result = core.run(update_handler);

        if start.elapsed() <= interval {
            // if there is time, handle other events in the event queue.
            // this is useful if you plan to extend this example.
            core.turn(Some(interval - start.elapsed()));
        }
        if start.elapsed() <= interval {
            // if there is still time, go to sleep.
            // this is due to the polling,
            // which is flexible (no static IP adress needed),
            // but quiet an overhead if you do it too excessively.
            thread::sleep(interval - start.elapsed());
        }
    }
}
