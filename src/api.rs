//! This module encapsulates the communication with Telegram servers
//! by providing a public `Bot` class with the available functionality.

// http library
extern crate hyper;
extern crate hyper_rustls;

extern crate serde_json;
extern crate tg_bot_models;

extern crate futures;
extern crate tokio_core;



use std::{io, env};
use std::string::String;
use self::futures::{Future, Stream};
use self::hyper::{Body, Client, Method, Request};
use self::hyper::header::{ContentLength, ContentType};
use self::hyper_rustls::HttpsConnector;
use self::tokio_core::reactor::Handle;
use self::serde_json::Value;
// use packages::*;
use sendables::*;
use receivables::*;
//use receivables::Receivable;
use error::Error;

/// This struct offers access to all implemented bot functionality.
pub struct Bot {
    base_url: String,
    http: Client<HttpsConnector, Body>,
}

#[allow(dead_code)]
impl Bot {
    pub fn new(token_env_var: &str, handle: &Handle) -> Bot {
        //! Creates a new bot using the token and a tokio core.
        //! It is recommended not to hard code the token but use
        //! `let token: String = env::var("TELEGRAM_BOT_TOKEN").unwrap();`
        // TODO can I somehow remove the core_handle?
        // TODO how many threads should be used? Expose `with_threads(n: usize)`
        let token: String = env::var(token_env_var)
            .expect( format!("Environment Variable {} for your bot is not set!", token_env_var)
                .as_str()
        );
        let http = Client::configure()
            .connector(HttpsConnector::new(2, &handle))
            .build(&handle);
        let base_url = "https://api.telegram.org/bot".to_owned() + token.as_str() + "/";
        Bot { base_url, http }
    }

    pub fn get_updates(&self) -> impl Future<Item = Vec<Receivable>, Error = Error> {
        //! Fetches updates and returns a `Vec<Receivable>` or an `Error`
        // TODO enable optional parameters
        self.http_post("getUpdates", "{}")
            .map_err(|e| {
                println!("{}", e);
                Error::Hyper("POST on getUpdates failed.")
            })
            .and_then(|json| {
                let array = extract_result_vector(json);
                if array.is_err() {
                    return Err(array.err().unwrap().clone());
                }
                let mut result: Vec<Receivable> = Vec::new();
                for object in array.unwrap() {
                    let upd = serde_json::from_value(object.to_owned());
                    if upd.is_ok() {
                        result.push(Receivable::from_update(upd.unwrap()).unwrap());
                    }
                }
                Ok(result)
            })
    }

    pub fn get_me(&self) -> impl Future<Item = User, Error = Error> {
        //! Fetches information about this bot from the telegram server.
        //! This is a testing functionalty offered by the telegram bot api.
        self.http_post("getMe", "{}")
            .map_err(|_| {
                Error::Hyper("POST on getMe failed.")
            })
            .and_then(|json| {
                if json["ok"] == true {
                    Ok(serde_json::from_value(json["result"].to_owned()).unwrap())
                } else {
                    Err(Error::Api("getMe: Field \"ok\" in JSON was false."))
                }
            })
    }

    pub fn send_message(
        &self,
        parameters: MessageParams,
    ) -> impl Future<Item = Value, Error = hyper::Error> {
        //! Sends a message and returns what the telegram servers received.
        // TODO enable optional parameters
        // TODO map the return value to some useful struct (message?)
        self.http_post(
            "sendMessage",
            serde_json::to_string(&parameters).unwrap().as_str(),
        )
    }

    fn http_get(&self, method: &str) -> impl Future<Item = Value, Error = hyper::Error> {
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

    fn http_post(
        &self,
        method: &str,
        json: &str,
    ) -> impl Future<Item = Value, Error = hyper::Error> {
        //! Sends a POST request at `base_url/method` with a given `&str`,
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

fn extract_result_vector(json: Value) -> Result<Vec<Value>, Error> {
    if let Value::Object(obj) = json {
        if !obj["ok"].as_bool().expect("No ok in JSON.") {
            return Err(Error::Api("Field \"ok\" in JSON was false."))
        }
        match obj["result"].clone() {
            Value::Array(array) => Ok(array),
            _ => return Err(Error::Api("Field \"result\" in JSON was not an array.")),
        }
    } else {
        Err(Error::Api("Received JSON was not an object."))
    }
}
