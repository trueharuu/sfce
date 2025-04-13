use std::fmt::Write as _;

use strum::IntoEnumIterator;

use crate::{
    board_parser::Tetfu,
    grid::Grid,
    piece::{Piece, Rotation},
    program::Sfce,
};

impl Sfce {
    pub fn possible(
        &mut self,
        tetfu: &Tetfu,
        piece: Piece,
    ) -> anyhow::Result<()> {
        let binding = self.resize(tetfu.grid());
        let board = binding.pages().first().unwrap();
        let zz = Rotation::iter().map(|x| {
            (board.clone().to_gray() | board.fast().possible_placements(piece, x).tint(piece)).with_comment(x)
        });
        

        writeln!(
            self.buf,
            "{}",
            self.tetfu(&Grid::from_pages(zz))
        )?;
        Ok(())
    }
}
