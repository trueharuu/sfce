#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc, clippy::struct_excessive_bools)]

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
pub mod set;

fn main() {
    let mut p = Sfce::new();
    // println!("?");
    if let Err(e) = p.run() {
        println!("\x1b[1;31merror\x1b[0m\x1b[1m:\x1b[0m {e}");
    };
}
