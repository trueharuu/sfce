use crate::{grid::Grid, piece::{Piece, Rotation}, placement::Placement, program::Sfce};

impl Sfce {
    pub fn test_command(&mut self) -> anyhow::Result<()> {
        let mut b = Grid::new("G2E2|G4|G4|GE2G").page();
        b.skim_place(Placement::new(Piece::S, 2, 0, Rotation::North));
        println!("{}", self.tetfu(&b.grid()));
        Ok(())
        
    }
}
