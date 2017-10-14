# A Telegram Bot Library, written in Rust

> This library shall make it easy to code Telegram bots in Rust.

Note that I don't have much experience in both, the rust language and bots or similar stuff. Therefore, contributions and tips are very welcome. Check the commit messages to get an overview of the current state.


## Usage
Take a look in the examples folder to see runnable and well documented examples. Remember that you will need to register a bot and store the API key in an environment variable (the default in the examples is `$TELEGRAM_BOT_TOKEN`).


## Current features
* Polling Updates (currently no webhook support).
	* This is because I want to be able to run bots without a public ip/adress to my raspberry pi. There was currently no need for Webhooks, then.
* Asynchonous API via Futures and Tokio_Core.
* Send and receive simple messages and edited messages from users and channels.


## TODO
(in rough priority)

1. More Feature support
	* especially custom buttons and keyboards
2. code improvements in api module
3. Webhook support
