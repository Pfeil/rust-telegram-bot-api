//! This module encapsulates the communication with Telegram servers
//! by providing a public `Bot` class with the available functionality.

// needed by hyper i think
extern crate futures;
// http library
extern crate hyper;
// https support lol
extern crate hyper_tls;
// json parser
extern crate serde_json;
// app loop
extern crate tokio_core;


use std::io;
use std::string::String;
use self::futures::{Future, Stream};
use self::hyper::{Body, Client, Method, Request};
use self::hyper::client::HttpConnector;
use self::hyper::header::{ContentLength, ContentType};
use self::hyper_tls::HttpsConnector;
use self::tokio_core::reactor::Handle;
use self::serde_json::Value;
use packages::*;
use parameters::*;


/// This struct offers access to all implemented bot functionality.
pub struct Bot {
    base_url: String,
    http: Client<HttpsConnector<HttpConnector>, hyper::Body>,
}

#[allow(dead_code)]
impl Bot {
    pub fn new(token: String, core_handle: Handle) -> Bot {
        //! Creates a new bot using the token and a tokio core.
        //! It is recommended not to hard code the token but use
        //! `let token: String = env::var("TELEGRAM_BOT_TOKEN").unwrap();`
        // TODO can I somehow remove the core_handle?
        // TODO how many threads should be used? Expose `with_threads(n: usize)`
        let http = Client::configure()
            .connector(HttpsConnector::new(2, &core_handle).unwrap())
            .build(&core_handle);
        let base_url = "https://api.telegram.org/bot".to_owned() + token.as_str() + "/";
        Bot {
            base_url: base_url,
            http: http, // from hyper
        }
    }

    pub fn get_updates(&mut self) -> impl Future<Item = Vec<Update>, Error = Error> {
        //! Fetches updates and returns a `Vec<packages::Update>` or an `packages::Error`
        // TODO enable optional parameters
        // TODO cleanup, write code more compact
        self.http_post("getUpdates", "{}")
        .map_err(|_| {
            Error {
                ok: false,
                error_code: 0,
                description: "Http / Hyper Error".to_owned(),
            }
        })
        .and_then(|json| {
        match json {
            Value::Null => {
                // TODO log this error
                Ok(Vec::new())
            }
            Value::Object(obj) => {
                if obj["ok"].as_bool().unwrap() {
                    match obj["result"] {
                        Value::Array(ref array) => {
                            let mut result: Vec<Update> = Vec::new();
                            for object in array {
                                let upd = Update::from_json(object.to_owned());
                                if upd.is_some() {
                                    result.push(upd.unwrap());
                                }
                            }
                            Ok(result)
                        }
                        _ => {
                            // TODO log error!
                            Ok(Vec::new())
                        }
                    }
                } else {
                    Err(Error::from_json(Value::Object(obj)))
                }
            }
            _ => Ok(Vec::new()),
        }})
    }

    pub fn get_me(&mut self) -> impl Future<Item = User, Error = Error> {
        //! Fetches information about this bot from the telegram server.
        //! This is a testing functionalty offered by the telegram bot api.
        self.http_post("getMe", "{}")
            .map_err(|_| {
                Error {
                    ok: false,
                    error_code: 0,
                    description: "Http / Hyper Error".to_owned(),
                }
            })
            .and_then(|json| {
                if json["ok"] == true {
                Ok(User::from_json(json["result"].to_owned()).unwrap())
            } else {
                Err(Error::from_json(json))
            }
        })
    }

    pub fn send_message(&mut self, parameters: MessageParams) -> impl Future<Item = Value, Error = hyper::Error> {
        //! Sends a message and returns what the telegram servers received.
        // TODO enable optional parameters
        // TODO map the return value to some useful struct (message?)
        self.http_post("sendMessage", parameters.to_json().as_str())
    }

    fn http_get(&mut self, method: &str) -> impl Future<Item = Value, Error = hyper::Error> {
        //! Sends a GET request at `base_url/method`.
        let uri = (self.base_url.to_owned() + method).parse().unwrap();
        println!("GET({:?})", uri);
        let content = self.http.get(uri).and_then(|res| {
            println!("Response: {}", res.status());
            res.body().concat2().and_then(move |body| {
                //io::stdout().write_all(&body);
                let body_content: Value = serde_json::from_slice(&body.to_owned())
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                //println!("Received:\n\t{:?}", body_content); // DEBUG
                Ok(body_content)
            })
        });
        content
    }

    fn http_post(&mut self, method: &str, json: &str) -> impl Future<Item = Value, Error = hyper::Error> {
        //! Sends a POST request at `base_url/method` with a given `&str`
        //! which must be valid JSON.
        let uri = (self.base_url.to_owned() + method).parse().unwrap();
        println!("POST({:?}): {:?}", uri, json);
        let mut request: Request<Body> = Request::new(Method::Post, uri);
        request.headers_mut().set(ContentType::json());
        request.headers_mut().set(ContentLength(json.len() as u64));
        request.set_body(json.to_owned());
        let content = self.http.request(request).and_then(|res| {
            println!("Response: {}", res.status());
            res.body().concat2().and_then(move |body| {
                //io::stdout().write_all(&body);
                let body_content: Value = serde_json::from_slice(&body.to_owned())
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                //println!("Received:\n\t{:?}", body_content); // DEBUG
                Ok(body_content)
            })
        });
        content
    }
}
