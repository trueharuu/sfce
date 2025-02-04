use crate::{
    board_parser::Tetfu,
    grid::Grid,
    piece::{Piece, Rotation},
    program::Sfce,
};

impl Sfce {
    pub fn possible(&mut self, tetfu: &Tetfu, piece: Piece, rotation: Rotation) {
        let binding = self.resize(tetfu.grid());
        let board = binding.pages().first().unwrap();
        let m = board.fast().possible_placements(piece, rotation);

        println!(
            "{}",
            self.tetfu(&Grid::from_pages([board.clone().to_gray() | m.tint(piece)]))
        );
    }
}
