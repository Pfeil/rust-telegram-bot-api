//! This module defines this crates error handling.
//! It consists of an error type that wraps all possible occuring errors.

extern crate serde_json;
extern crate hyper;

use std;
use std::fmt;
use std::fmt::Display;

//#[derive(Clone, Debug, Serialize, Deserialize)]
//pub struct Error {
//    // TODO need ENUM with Error Types, like HttpError and JsonError
//    pub ok: bool,
//    pub error_code: u32,
//    pub description: String,
//}
//
//impl Error {
//    pub fn from_json(json: Value) -> Error {
//        serde_json::from_value(json).unwrap()
//    }
//}

#[derive(Clone, Debug)]
pub enum Error {
    Hyper(&'static str),
    Api(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match self {
            &Hyper(str) => write!(f, "Hyper Error: {}", str),
            &Api(str) => write!(f, "Unexpected API response: {}", str),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match self {
            &Hyper(str) => str,
            &Api(str) => str,
        }
    }
}
