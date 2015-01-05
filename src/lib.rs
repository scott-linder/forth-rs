#![feature(phase, unboxed_closures)]
#[phase(link, plugin)] extern crate log;

use std::result;

pub type Result = result::Result<(), error::Error>;

pub mod context;
pub mod dict;
pub mod error;
pub mod stack;
pub mod word;
