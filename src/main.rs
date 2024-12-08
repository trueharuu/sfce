#![feature(let_chains, is_none_or)]
#![warn(clippy::pedantic)]
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use program::Sfce;

pub mod board;
pub mod board_parser;
pub mod caches;
pub mod commands;
pub mod data;
pub mod fumen;
pub mod grid;
pub mod input;
pub mod pattern;
pub mod piece;
pub mod placement;
pub mod program;
pub mod ranged;
pub mod text;
pub mod traits;

fn main() {
    let mut p = Sfce::new();
    // println!("?");
    if let Err(e) = p.run() {
        println!("\x1b[1;31merror\x1b[0m\x1b[1m:\x1b[0m {e}");
    };
}
