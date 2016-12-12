#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
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
