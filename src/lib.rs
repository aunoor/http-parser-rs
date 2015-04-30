#![crate_name = "http_parser"]

pub use self::parser::{HttpParser, HttpParserType};
pub use self::http_version::HttpVersion;
pub use self::error::HttpErrno;
pub use self::http_method::HttpMethod;
pub use self::callback::{HttpParserCallback, CallbackResult, CallbackDecision};

mod parser;
mod http_version;
mod error;
mod state;
mod flags;
mod http_method;
mod callback;

