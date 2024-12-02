#![feature(let_chains, is_none_or)]
use clap::Parser;

pub mod board_parser;
pub mod commands;
pub mod data;
pub mod fumen;
pub mod grid;
pub mod text;
pub mod board;
pub mod pattern;
pub mod piece;
pub mod program;
pub mod traits;
pub mod ranged;
pub mod input;

fn main() {
    let mut p = program::Sfce::parse();
    if let Err(e) = p.run() {
        println!("\x1b[1;31merror\x1b[0m\x1b[1m:\x1b[0m {e}")
    }
}
