use crate::{
    grid::Grid,
    piece::{Piece, Rotation},
    placement::Placement,
    program::Sfce,
};

impl Sfce {
    pub fn test_command(&mut self) -> anyhow::Result<()> {
        let b = Grid::new("E10|G3E6G|G2T3I4G|G10|G3TG2E2G2").page();

        let p = Placement::new(Piece::T, 7, 0, Rotation::North);
        println!("valid? {}", b.is_valid_placement(p, true));
        println!(
            "{}",
            self.tetfu(&b.with_placement(p).grid())
        );
        Ok(())
    }
}
