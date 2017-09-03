/*
 * This module encapsulates the communication with Telegram servers
 * by providing a public Bot class with the necessary functionality.
 */
extern crate futures; // needed by hyper i think
extern crate hyper; // http library
extern crate hyper_tls; // https support lol
extern crate tokio_core; // app loop
extern crate serde_json; // json parser


use std::io;
use std::string::String;
use self::futures::{Future, Stream}; // needed for http response handling (indirect at least)
use self::hyper::Client; // http client functionality
use self::hyper::client::HttpConnector;
use self::hyper_tls::HttpsConnector;
use self::tokio_core::reactor::Core; // application loop
use self::serde_json::Value;
use packages::User;



pub struct Bot {
    token: String,
    base_url: String,
    http: Client<HttpsConnector<HttpConnector>, hyper::Body>, // (hyper http implementation)
    core: Core, // for executing http calls
}


impl Bot {
    pub fn new(token: String, base_url: String, core: Core) -> Bot {
        let handle = core.handle();
        let http = Client::configure()
            .connector(HttpsConnector::new(2, &handle).unwrap())
            .build(&handle);
        Bot {
            token: token,
            base_url: base_url,
            http: http, // from hyper
            core: core,
        }
    }

    pub fn get_updates(&mut self) -> Value {
        self.http_get("getUpdates")
    }

    pub fn get_me(&mut self) -> User {
        User::from_json(self.http_get("getMe"))
    }

    fn http_get(&mut self, method: &str) -> Value {
        let uri = (self.base_url.to_owned() + method).parse().unwrap();
        println!("GET({:?})", uri);
        let content = self.http
            .get(uri)
            .and_then(|res| {
                println!("Response: {}", res.status());
                res.body()
                    .concat2()
                    .and_then(move |body| {
                                  //io::stdout().write_all(&body);
                                  let body_content: Value =
                                      serde_json::from_slice(&body.to_owned())
                                          .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                                  Ok(body_content)
                              })
            });
        self.core.run(content).unwrap() // TODO handle errors
    }
}
