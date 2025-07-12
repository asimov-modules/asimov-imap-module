// This is free and unencumbered software released into the public domain.

//#![no_std]
#![forbid(unsafe_code)]

mod capabilities;
pub use capabilities::*;

mod config;
pub use config::*;

mod error;
pub use error::*;

mod iterator;
pub use iterator::*;

mod message;
pub use message::*;

mod reader;
pub use reader::*;

mod url;
pub use url::*;
