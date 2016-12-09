#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate chrono;

pub use game::Game;

mod game;
mod board;
mod common;
mod manual;
mod archive;
