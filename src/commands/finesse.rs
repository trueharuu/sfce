use std::fmt::Write;
use chumsky::extra::Err;
use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::{
    board_parser::Tetfu, grid::Grid, input::{Input, Key}, piece::{Piece, Rotation}, placement::Placement, program::Sfce
};

impl Sfce {
    pub fn finesse_command(&mut self, tetfu: &Tetfu) -> anyhow::Result<()> {
        let board = self
            .resize(tetfu.grid())
            .pages()
            .last()
            .cloned()
            .unwrap_or_default();

        let og = board.clone().only_gray();
        let mut placements = vec![];
        let mut colored_cells = vec![];
        for x in 0..board.width() {
            for y in 0..board.height() {
                // println!("{:?}", board.get(x, y));
                if board.get(x, y).is_filled_with_piece() {
                    colored_cells.push((x, y, board.get(x, y)));
                }
            }
        }

        // println!("{colored_cells:?}");
        if colored_cells.len() != 4 {
            writeln!(self.buf, "no placement was found")?;
            return Ok(());
        }

        for &(x, y, piece) in &colored_cells {
            for rotation in Rotation::iter() {
                let p = Placement::new(piece, x, y, rotation);
                let trial_p = board.clone().only_gray().with_skimmed_placement(p);
                // println!("{p:?}");
                // println!("{board}");
                // println!("{trial}");
                // println!("{trial_p}");
                // println!();
                if trial_p == board {
                    placements.push(p);
                }
            }
        }

        if placements.is_empty() {
            writeln!(self.buf, "no placement was found")?;
            return Ok(());
        }

        let mut z: Option<(Vec<Key>, Grid)> = None;
        for mut pl in placements {
            println!("{pl:?}");
            let py = pl.y();
            for (_, _) in og.removed_lines().iter().filter(|x| x.0 < py) {
                pl.set_y(pl.y() - 1);
            }

            let so = og.clone().skimmed();
            let f = pl.finesse(&so, so.spawn(), self.handling());
            if let Some(k) = f {
                let mut i =
                    Input::new(&so, pl.piece(), so.spawn(), Rotation::North, self.handling());

                let g = i.show_inputs(&k);
                if let Some(ref p) = z {
                    if p.0.len() < k.len() {
                        continue;
                    } else {
                        z = Some((k, g))
                    }
                } else {
                    z = Some((k, g));
                }
            }
        }

        if let Some((_, g)) = z {
            writeln!(self.buf, "{}", self.tetfu(&g))?;
        } else {
            writeln!(self.buf, "no finesse found")?;
        }

        Ok(())
    }

    pub fn inputs(&mut self, tetfu: &Tetfu, piece: Piece, x: usize, y: usize, r: Rotation) -> anyhow::Result<()> {
        let board = self.resize(tetfu.grid()).page();
        let p = Placement::new(piece, x, y, r);
        let ks = p.finesse(&board, board.spawn(), self.handling());

        if let Some(k) = ks {
            writeln!(self.buf, "{}", k.iter().join(","))?;
            return Ok(())
        } else {
            anyhow::bail!("no inputs found");
        }
    }
}
