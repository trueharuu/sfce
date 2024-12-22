use std::fmt::Write;

use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::{
    board_parser::Tetfu, input::Input, piece::Rotation, placement::Placement,
    program::Sfce,
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
            eprintln!("no placement was found");
            return Ok(());
        }

        for (x, y, piece) in colored_cells {
            for rotation in Rotation::iter() {
                let p = Placement::new(piece, x, y, rotation);
                let trial = board.clone().only_gray();
                // let trial_p = board.clone().only_gray().with_placement(p);
                // println!("{p:?}");
                // println!("{board}");
                // println!("{trial}");
                // println!("{trial_p}");
                // println!();

                if trial.with_placement(p) == board {
                    placements.push(p);
                }
            }
        }

        let mut hit = false;
        for p in placements {
            println!("identified {p:?}");
            let keys = self.finesse(p, &og);
            let mut i = Input::new(&og, p.piece(), board.spawn(), Rotation::North, self.handling());
            if let Some(k) = keys {
                let v = i.show_inputs(&k);
                write!(self.buf, "{}", self.tetfu(&v))?;
                println!("{}", k.iter().map(|x| format!("{x:?}")).join(", "));
                hit = true;
                break;
            }
        }

        if !hit {
            eprintln!("no placement was possible");
        }
        // println!("{:?}", placement);

        Ok(())
    }
}
