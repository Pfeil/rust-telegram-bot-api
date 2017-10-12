//! This is a library, encapsulating the telegram bot api.
#![feature(conservative_impl_trait)]


//#![feature(use_extern_macros)]
pub mod api;
pub mod packages;
pub mod parameters;
#[macro_use]
extern crate serde_derive;


// === EXAMPLE ===
extern crate futures;
//extern crate shoppingbot;
extern crate tokio_core;

use api::Bot;
use packages::*;
use packages::Update::*;
use std::time::{Duration, Instant};
use std::thread;
use std::env; // to read shell variables (telegram token)
use tokio_core::reactor::Core; // application loop
                               //use futures::future::Future;

/// This example is a simple loop that fetches Updates
/// every 10 seconds and prints them.
fn main() {
    let token: String = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let mut bot = Bot::new(token, &core.handle());
    let interval = Duration::from_millis(10000);
    loop {
        let start = Instant::now();

        println!("---START---");
        let value = bot.get_updates();
        let mut value_plus = bot.get_updates().and_then(|updates| {
            for update in updates {
                match update {
                    TextMessage(id, ref m) => println!("{}: {:?}", id, m),
                    EditedTextMessage(id, ref m) => println!("{}: {:?}", id, m),
                    ChannelPost(id, ref m) => println!("{}: {:?}", id, m),
                    EditedChannelPost(id, ref m) => println!("{}: {:?}", id, m),
                    _ => {
                        return Err(Error {
                            ok: false,
                            error_code: 0u32,
                            description: "asd".to_owned(),
                        });
                    }
                }
            }
            Ok(Error {
                ok: false,
                error_code: 0u32,
                description: "asd".to_owned(),
            })
        });
        //core.run(value);
        handle.spawn(value);
        core.turn(None);
        println!("---STOP---");

        // TODO handle value
        //println!("value is {:?}", value);

        let elapsed = start.elapsed();
        if elapsed <= interval {
            thread::sleep(interval - elapsed);
        }
    }
}
