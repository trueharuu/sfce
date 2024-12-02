use std::fmt::Write;

use crate::{
    board::Board,
    piece::{Placement, Rotation},
    program::Sfce,
};

pub fn test_command(s: &mut Sfce) -> anyhow::Result<()> {
    let a = Board::new("E4|E4|Z2E2|EZ2E");
    let p = Placement {
        x: 3,
        y: 1,
        rotation: Rotation::West,
        piece: crate::piece::Piece::T,
    };

    
    write!(s.buf, "{} {}", s.tetfu(a.with_placement(p).grid()), a.is_valid_placement(p))?;

    
    Ok(())
}
