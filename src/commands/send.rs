use std::fmt::Write as _;

use crate::{
    board_parser::Tetfu,
    input::{Input, Key},
    piece::{Piece, Rotation},
    program::Sfce,
};

impl Sfce {
    pub fn send_command(
        &mut self,
        tetfu: &Tetfu,
        piece: Piece,
        keys: &[Key],
    ) -> anyhow::Result<()> {
        let binding = self.resize(tetfu.grid());
        let board = binding.pages().first().unwrap();
        let mut i = Input::new(
            board,
            piece,
            board.spawn(),
            Rotation::North,
            self.handling(),
        );
        let g = i.show_inputs(keys);
        write!(self.buf, "{}", self.tetfu(&g))?;
        Ok(())
    }
}
