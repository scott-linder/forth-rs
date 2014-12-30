#![feature(phase, unboxed_closures)]
#[phase(link, plugin)] extern crate log;

use std::result;

pub type ForthResult = result::Result<(), error::Error>;

pub mod error;
pub mod word;
pub mod stack;
pub mod context;
