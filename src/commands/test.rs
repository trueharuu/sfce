use std::{fmt::Write, time::Instant};

use crate::{
    board::Board,
    piece::{Piece, Rotation},
    placement::Placement,
    program::Sfce,
};

pub fn test_command(s: &mut Sfce) -> anyhow::Result<()> {
    let a = Board::new("E4|E4|E4|GE2G|GE3|G2EG");
    let p = Placement {
        x: 2,
        y: 1,
        rotation: Rotation::South,
        piece: Piece::T,
    };

    let e = Instant::now();
    let kl = p.inputs(a, (1, 4), 6);

    write!(s.buf, "{kl:?} {:.3}s", e.elapsed().as_secs_f64())?;

    Ok(())
}
