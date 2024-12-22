use crate::{
    board_parser::Tetfu,
    grid::Grid,
    piece::{Piece, Rotation},
    placement::Placement,
    program::Sfce,
};

impl Sfce {
    pub fn possible(&mut self, tetfu: &Tetfu, piece: Piece, rotation: Rotation) {
        let binding = self.resize(tetfu.grid());
        let board = binding.pages().first().unwrap();

        let mut g = Grid(vec![]);
        let m = board.fast().possible_placements(piece, rotation);

        for i in m.filled_cells() {
            let mut c = board.clone();
            c.skim_place(Placement::new(piece, i.0, i.1, rotation));
            g.add_page(c);
        }

        println!("{}", self.tetfu(&g));
    }
}
