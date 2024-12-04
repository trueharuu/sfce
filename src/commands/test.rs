use std::{fmt::Write, time::Instant};

use crate::{
    board::Board,
    input::{Input, Key},
    piece::{Piece, Rotation},
    placement::Placement,
    program::Sfce,
    traits::contiguous_subsequences,
};

pub fn test_command(s: &mut Sfce) -> anyhow::Result<()> {
    let a = Board::new("E4|E4|E4|GE2G|GE3|G2EG");
    let mut i = Input::new(a.clone(), Piece::T, a.spawn());

    println!("{}", s.tetfu(i.place().grid()));

    let k: &[Key] = &[Key::MoveRight, Key::MoveLeft, Key::MoveLeft, Key::MoveLeft];
    println!("{k:?} {:?}", i.remove_all_noops(k));
    Ok(())
}
