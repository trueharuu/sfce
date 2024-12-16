use std::fmt::Write as _;

use crate::{
    board::Board,
    input::{Input, Key},
    piece::{Piece, Rotation},
    program::Sfce,
};

impl Sfce {
    pub fn test_command(&mut self) -> anyhow::Result<()> {
        let mut x = Board::new("G3E5G2|G3E4G3|G3E3G4|G3E4G3");
        x.set_height(6);
        x.set_margin(2);
        let mut i = Input::new(&x, Piece::I, x.spawn(), Rotation::North, self.handling());

        write!(
            self.buf,
            "{}",
            self.tetfu(&i.show_inputs(&[Key::CW, Key::SonicDrop, Key::DasLeft, Key::CW]))
        )?;
        Ok(())
    }
}
