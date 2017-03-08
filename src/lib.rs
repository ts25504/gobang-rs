#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![crate_name = "gobang"]
#![crate_type = "lib"]

#![warn(non_camel_case_types)]

extern crate chrono;

#[macro_use(quick_error)]
extern crate quick_error;

pub use game::Game;

mod game;
mod board;
mod common;
mod manual;
mod archive;
mod errors;
