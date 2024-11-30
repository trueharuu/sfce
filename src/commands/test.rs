use std::fmt::Write;

use crate::{
    page::Page,
    piece::{Piece, Placement, Rotation},
    program::Sfce,
};

pub fn test_command(s: &mut Sfce) -> anyhow::Result<()> {
    let a = Page::new("E4|E3G");
    let p = Placement {
        x: 1,
        y: 1,
        rotation: Rotation::North,
        piece: crate::piece::Piece::I,
    };

    
    write!(s.buf, "{} {}", s.tetfu(a.with_placement(p).grid()), a.is_valid_placement(p))?;

    
    Ok(())
}
