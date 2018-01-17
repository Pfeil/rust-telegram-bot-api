# A Telegram Bot Library, written in Rust

> This library shall make it easy to code flexible Telegram bots in Rust. It's still in development, though, and the API will still change a lot!

Note that I don't have much experience in both, the rust language and bots or similar stuff. Therefore, contributions and tips are always welcome.


## Usage
Take a look in the examples folder to see runnable and well documented examples. Remember that you will need to register a bot and store the API key in an environment variable (the default in the examples is `$TELEGRAM_BOT_TOKEN`).


## Current features
* Polling Updates (currently no Webhook support).
	* I just do not need Webhooks for my current use case. Feel free to implement this.
* Asynchonous API via Futures and Tokio_Core. This API is not yet stable, though. It will change a lot.
* Send and receive simple messages and edited messages from users and channels.
	* This includes link previews,
	* Markdown (default) and HTML format support (no builder for Markdown or HTML (yet), just use Strings),
	* Notification hiding (default=false),
	* Replies,
	* and Message Buttons, custom Keyboards, etc. (but no builder yet, so it is currently not as comfortable as it should and will be).
	* probably stuff I forgot.


## Todo & Contribution

As written above, I'd love to see some contribution. You may take a look at my todo list, but feel free to suggest your own improvements. Also, watch out for "TODO" comments in the code to see what needs to be done.

(in rough priority)

1. More Feature support
	* [DONE] Use a more or less maintained telegram model library like ( https://github.com/gtors/tg_bot_models )
	* add support for custom buttons and keyboards
	* include some logging feature/lib
2. Simplier API
	* The examples look still too exhaustive to me. I want to reduce boilerplate code and provide convenience methods for common needed tasks. Maybe even handler methods for different types of things.
2. code improvements in api module
	* Do all the TODO notations in the code.
	* Better readable code, more compact code
3. How could we test something like this? I have no (simple) idea.
	* It should be possible to use the Telegram Client API to create a client that uses the bot and verifies it's answers to get some sanity checks.
3. Webhook support
