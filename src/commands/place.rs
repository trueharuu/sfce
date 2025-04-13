use std::fmt::Write as _;

use crate::{
    board_parser::Tetfu, grid::Grid, piece::{Piece, Rotation}, placement::Placement, program::Sfce
};

impl Sfce {
    pub fn place(&mut self, tetfu: &Tetfu, piece: Piece, x: usize, y: usize, rotation: Rotation) -> anyhow::Result<()> {
        let binding = self.resize(tetfu.grid());
        let mut board = binding.pages().first().cloned().unwrap().to_gray();
        let p = Placement::new(piece, x, y, rotation);
        if !board.is_valid_placement(p, true) {
            anyhow::bail!("invalid placement");
        }
        board.place(p);

        writeln!(self.buf, "{}", self.tetfu(&Grid::from_pages([board.clone()])))?;

        Ok(())
    }
}
