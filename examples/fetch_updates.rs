//! A simple example of a bot fetching updates (messages)
//! and printing out the results via stdout.

extern crate shoppingbot;
extern crate tokio_core; // app loop

use shoppingbot::api::Bot;
use std::time::{Instant, Duration};
use std::thread;
use std::env; // to read shell variables (telegram token)
use tokio_core::reactor::Core; // application loop

/// This example is a simple loop that fetches Updates
/// every 10 seconds and prints them.
fn main() {
    let token: String = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let core = Core::new().unwrap();
    let mut bot = Bot::new(token, core);
    let interval = Duration::from_millis(10000);
    loop {
        let start = Instant::now();
        let value = bot.get_updates();
        // TODO handle value
        println!("value is {:?}", value);
        let elapsed = start.elapsed();
        if elapsed <= interval {
            thread::sleep(interval - elapsed);
        }
    }
}
