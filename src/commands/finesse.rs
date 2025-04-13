use std::fmt::Write as _;

use itertools::Itertools;

use crate::{
    board_parser::Tetfu,
    piece::{Piece, Rotation},
    placement::Placement,
    program::Sfce,
};

impl Sfce {
    pub fn finesse(
        &mut self,
        tetfu: &Tetfu,
        piece: Piece,
        x: usize,
        y: usize,
        rotation: Rotation,
    ) -> anyhow::Result<()> {
        let b = self.resize(tetfu.grid()).page();
        let p = Placement::new(piece, x, y, rotation);
        let ks = p.finesse(&b, b.spawn(), self.handling());
        if let Some(k) = ks {
            write!(self.buf, "{}", k.iter().join(","))?;
        } else {
            anyhow::bail!("no finesse found")
        }
        Ok(())
    }
}
