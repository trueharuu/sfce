use std::fmt::Write as _;

use crate::{
    board::Board,
    input::{Input, Key},
    piece::{Piece, Rotation},
    program::Sfce,
};

pub fn command(s: &mut Sfce) -> anyhow::Result<()> {
    let mut x = Board::new("G3E5G2|G3E4G3|G3E3G4|G3E4G3");
    x.set_height(6);
    x.set_margin(2);
    let mut i = Input::new(&x, Piece::I, x.spawn(), Rotation::North, s.handling());

    write!(
        s.buf,
        "{}",
        s.tetfu(&i.show_inputs(&[Key::CW, Key::SonicDrop, Key::DasLeft, Key::CW]))
    )?;
    Ok(())
}
